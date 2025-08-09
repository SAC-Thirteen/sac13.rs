use core::fmt::Display;

/// Represents a month on the SAC13 calendar.
///
/// Months are practically the same as in the Gregorian Calendar.
/// They have the same names and order. The two main differences are that
/// SAC13 starts its year with March (so March is the 1st month) and SAC13 has
/// 13 months and this additional month is called "Addenduary" and placed after
/// February.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum Month {
    March = 1,
    April = 2,
    May = 3,
    June = 4,
    July = 5,
    August = 6,
    September = 7,
    October = 8,
    November = 9,
    December = 10,
    January = 11,
    February = 12,
    Addenduary = 13,
}

impl Month {
    /// Month from its ordinal number _(valid are 1-13, both inclusive)_.
    ///
    /// Returns `None` for invalid ordinals.
    #[must_use]
    pub const fn new(m: u8) -> Option<Self> {
        use Month::*;

        Some(match m {
            1 => March,
            2 => April,
            3 => May,
            4 => June,
            5 => July,
            6 => August,
            7 => September,
            8 => October,
            9 => November,
            10 => December,
            11 => January,
            12 => February,
            13 => Addenduary,
            _ => return None,
        })
    }

    /// The ordinal number of the month.
    ///
    /// Note that those are different from the Gregorian Calendar.  
    /// March = 1, April = 2, ... February = 12, Addenduary = 13
    #[must_use]
    pub const fn ord(self) -> u8 {
        self as u8
    }

    #[must_use]
    pub const fn next(self) -> Self {
        use Month::*;

        match self {
            March => April,
            April => May,
            May => June,
            June => July,
            July => August,
            August => September,
            September => October,
            October => November,
            November => December,
            December => January,
            January => February,
            February => Addenduary,
            Addenduary => March,
        }
    }

    #[must_use]
    pub const fn previous(self) -> Self {
        use Month::*;

        match self {
            March => Addenduary,
            April => March,
            May => April,
            June => May,
            July => June,
            August => July,
            September => August,
            October => September,
            November => October,
            December => November,
            January => December,
            February => January,
            Addenduary => February,
        }
    }

    /// Full name of the month _(international, english)_.
    ///
    /// March, April, May, ...
    #[must_use]
    pub const fn name(self) -> &'static str {
        use Month::*;

        match self {
            March => "March",
            April => "April",
            May => "May",
            June => "June",
            July => "July",
            August => "August",
            September => "September",
            October => "October",
            November => "November",
            December => "December",
            January => "January",
            February => "February",
            Addenduary => "Addenduary",
        }
    }

    // TODO: next nth Month
    // #[must_use]
    // pub const fn next_nth(self) -> Self {
    //     Self::new((self.num() % 13) + 1).unwrap()
    // }
}

impl Display for Month {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// TODO: documentation for From traits?

macro_rules! from_num_month {
    ($type:ident) => {
        impl From<Month> for $type {
            fn from(value: Month) -> Self {
                value.ord() as $type
            }
        }

        impl TryFrom<$type> for Month {
            type Error = ();

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                use Month::*;

                let m = match value {
                    1 => March,
                    2 => April,
                    3 => May,
                    4 => June,
                    5 => July,
                    6 => August,
                    7 => September,
                    8 => October,
                    9 => November,
                    10 => December,
                    11 => January,
                    12 => February,
                    13 => Addenduary,
                    _ => return Err(()),
                };

                Ok(m)
            }
        }
    };
}

from_num_month!(u8);
from_num_month!(u16);
from_num_month!(u32);
from_num_month!(u64);
from_num_month!(u128);

from_num_month!(i8);
from_num_month!(i16);
from_num_month!(i32);
from_num_month!(i64);
from_num_month!(i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_implementation_works() {
        let m: u8 = Month::March.into();
        assert_eq!(m, 1);

        let m: i32 = Month::September.into();
        assert_eq!(m, 7);
    }

    #[test]
    fn from_into_round_trip_works() {
        for m in 1..=13 {
            let typed: Month = m.try_into().unwrap();
            let num: i32 = typed.into();

            assert_eq!(m, num);
        }
    }
}
