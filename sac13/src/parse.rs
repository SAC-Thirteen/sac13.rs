use core::{fmt::Display, iter::Peekable};

use crate::{Date, GregorianDate};

#[derive(Debug, Clone)]
pub enum GregorianOrSac13 {
    GregorianDate(GregorianDate),
    Sac13Date(Date),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentOrder {
    YMD,
    DMY,
    MDY,
}

#[derive(Debug, Clone)]
pub struct ParsedDate {
    pub date: GregorianOrSac13,
    pub format: ParsedFormat,
}

#[derive(Debug, Clone)]
pub struct ParsedFormat {
    pub separator: u8,
    pub component_order: ComponentOrder,
    pub len_day: u8,
    pub len_month: u8,
    pub len_year: u8,
}

#[derive(Debug)]
struct ComponentParse {
    letter: bool,
    char_cnt: u8,
    value: i16,
    end: u8,
}

impl ComponentParse {
    pub fn parse<T>(i: &mut Peekable<T>) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let mut result = Self {
            letter: false,
            value: 0,
            end: 0,
            char_cnt: 0,
        };

        let invert = *i.peek()? == b'-';

        if invert {
            // consume peeked negative sign
            _ = i.next();
            result.char_cnt += 1;
        }

        if (b'A'..=b'Z').contains(i.peek()?) {
            if invert {
                // negative SAC13 years are not allowed
                return None;
            }

            // consume and process prefix letter
            result.value = (i.next().unwrap() - b'A') as i16;
            result.letter = true;
            result.char_cnt += 1;
        }

        loop {
            let x = i.next();

            let (exit, x) = match x {
                None => (true, 0),
                Some(x) => ([b'.', b'/', b'-'].contains(&x), x),
            };

            if exit {
                result.end = x;

                if invert {
                    result.value = result.value.checked_mul(-1)?;
                }

                return Some(result);
            }

            if !((b'0'..=b'9').contains(&x)) {
                return None;
            }

            result.char_cnt += 1;
            result.value = result.value.checked_mul(10)?;
            result.value = result.value.checked_add((x - b'0') as i16)?;
        }
    }
}

/// Parses various SAC13 and Gregorian Calendar formats.
///
/// ## Supported Formats
///
/// Even if the year is less than 100 it must be written
/// with leading zeros, to at least be three characters long.
///
/// - YYYY-MM-DD
/// - DD-MM-YYYY
pub fn parse_date_str(input: &str) -> Option<ParsedDate> {
    const MIN_YEAR_LENGTH: u8 = 4;

    let mut input = input.as_bytes().iter().copied().peekable();

    let c1 = ComponentParse::parse(&mut input)?;
    let c2 = ComponentParse::parse(&mut input)?;
    let c3: ComponentParse = ComponentParse::parse(&mut input)?;

    if c1.char_cnt == 3 || c2.char_cnt == 3 || c1.char_cnt == 3 {
        // No component is allowed to be three digits.
        // Days and months must be 1 or 2, and years must be 4 or more.
        return None;
    }

    if c1.end != c2.end {
        // different delimiters in same date are not allowed
        return None;
    }

    let separator = c1.end;

    if c3.end != 0 {
        // c3 must be the last component (delimiter zero)
        return None;
    }

    let year_first = c1.char_cnt >= MIN_YEAR_LENGTH;
    let year_last = c3.char_cnt >= MIN_YEAR_LENGTH;

    if c2.char_cnt >= MIN_YEAR_LENGTH {
        // middle part is never allowed to be a year
        return None;
    }

    if year_first == year_last {
        // either both ends or neither seem to be a year which is not allowed
        return None;
    }

    // determine sort order
    let (year, month, day, order) = if year_first {
        (c1, c2, c3, ComponentOrder::YMD)
    } else if c1.end == b'/' && !c3.letter {
        // US format only for Gregorian (no SAC13 millennium indicator letter)
        (c3, c1, c2, ComponentOrder::MDY)
    } else {
        (c3, c2, c1, ComponentOrder::DMY)
    };

    if day.letter || month.letter {
        return None;
    }

    if !(1..=31).contains(&day.value) || !(1..=13).contains(&month.value) {
        return None;
    }

    let format = ParsedFormat {
        separator,
        component_order: order,
        len_day: day.char_cnt,
        len_month: month.char_cnt,
        len_year: year.char_cnt,
    };

    let day = day.value as u8;
    let month = month.value as u8;

    let date = if year.letter {
        if year.value < 0 {
            return None;
        }

        GregorianOrSac13::Sac13Date(Date::from_ymd_untyped(year.value as u16, month, day)?)
    } else {
        GregorianOrSac13::GregorianDate(GregorianDate::from_ymd(year.value, month, day)?)
    };

    Some(ParsedDate { date, format })
}

impl Display for ParsedFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let separator = self.separator as char;

        let format_order = match self.component_order {
            ComponentOrder::YMD => [
                ('Y', self.len_year),
                ('M', self.len_month),
                ('D', self.len_day),
            ],
            ComponentOrder::DMY => [
                ('D', self.len_day),
                ('M', self.len_month),
                ('Y', self.len_year),
            ],
            ComponentOrder::MDY => [
                ('M', self.len_month),
                ('D', self.len_day),
                ('Y', self.len_year),
            ],
        };

        for (i, &(c, count)) in format_order.iter().enumerate() {
            if i != 0 {
                write!(f, "{}", separator)?;
            }

            for _ in 0..count {
                write!(f, "{}", c)?;
            }
        }

        Ok(())
    }
}

impl Display for GregorianOrSac13 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GregorianOrSac13::GregorianDate(x) => write!(f, "{}", x),
            GregorianOrSac13::Sac13Date(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_matches {
        ($left:expr, $right:pat) => {
            assert!(matches!($left, $right));
        };
    }

    macro_rules! assert_parse_error {
        ($inp:literal) => {
            assert_matches!(parse_date_str($inp), None);
        };
    }

    macro_rules! parse_expect_greg {
        ($inp:expr) => {
            match parse_date_str($inp) {
                Some(ParsedDate {
                    date: GregorianOrSac13::GregorianDate(x),
                    ..
                }) => x,
                _ => panic!(concat!(
                    "Expected ",
                    stringify!($inp),
                    " to parse as Gregorian date."
                )),
            }
        };
    }

    macro_rules! parse_expect_sac13 {
        ($inp:expr) => {
            match parse_date_str($inp) {
                Some(ParsedDate {
                    date: GregorianOrSac13::Sac13Date(x),
                    ..
                }) => x,
                _ => panic!(concat!(
                    "Expected ",
                    stringify!($inp),
                    " to parse as Gregorian date."
                )),
            }
        };
    }

    macro_rules! assert_sac13 {
        ($inp:expr, $y:ident - $m:literal - $d:literal) => {
            assert_eq!(parse_expect_sac13!($inp), date!($y - $m - $d));
        };
    }

    macro_rules! assert_greg {
        ($inp:expr, $y:literal - $m:literal - $d:literal) => {
            assert_eq!(parse_expect_greg!($inp), date_greg!($y - $m - $d));
        };
    }

    #[test]
    fn parsing_gregorian() {
        // DD-MM-YYYY
        assert_greg!("11-12-2000", 2000 - 12 - 11);
        assert_greg!("11.12.2000", 2000 - 12 - 11);

        // YYYY-MM-DD
        assert_greg!("2000-12-11", 2000 - 12 - 11);
        assert_greg!("2000.12.11", 2000 - 12 - 11);
        assert_greg!("2000/12/11", 2000 - 12 - 11);

        // Gregorian US Format:
        assert_greg!("12/11/2000", 2000 - 12 - 11);
    }

    #[test]
    fn negative_year_greg() {
        assert_greg!("-2000-12-11", -2000 - 12 - 11);
        assert_greg!("11-12--2000", -2000 - 12 - 11);
        assert_greg!("-2000.12.11", -2000 - 12 - 11);
        assert_greg!("11.12.-2000", -2000 - 12 - 11);
    }

    #[test]
    fn parsing_sac13() {
        assert_sac13!("M003-02-01", M003 - 02 - 01);
        assert_sac13!("M003.02.01", M003 - 02 - 01);
        assert_sac13!("M003/02/01", M003 - 02 - 01);

        assert_sac13!("01-02-M003", M003 - 02 - 01);
        assert_sac13!("01.02.M003", M003 - 02 - 01);
        assert_sac13!("01/02/M003", M003 - 02 - 01);

        // Note: SAC13 is always YMD or DMY and never the US format MDY
    }

    #[test]
    fn no_letter_allowed_as_month() {
        assert_parse_error!("2001-L-03");
    }

    #[test]
    fn no_letter_allowed_as_day() {
        assert_parse_error!("2001-02-L");
    }

    #[test]
    fn ambiguous_year_end_fails_to_parse() {
        assert_parse_error!("2020-12-2020");
    }

    #[test]
    fn no_year_end_fails_to_parse() {
        assert_parse_error!("01-01-01");
    }

    #[test]
    fn three_digit_components_fail_to_parse() {
        assert_parse_error!("001-01-01");
        assert_parse_error!("01-01-001");

        assert_parse_error!("01-001-2000");
        assert_parse_error!("001-01-2000");
    }
}
