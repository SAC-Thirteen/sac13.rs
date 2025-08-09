use core::{cmp::Ordering, fmt::Display};

use crate::{scalars::JulianDay, traits::CalendarDate};

/// Gregorian Calendar date _(proleptic, when applicable)_.
///
/// The implementation for the Gregorian Calendar is very slim
/// and intentionally doesn't have a month or support weekdays.
/// It's only here to allow conversions from the Gregorian Calendar to SAC13 and vice-versa.
/// If you want better typing for the Gregorian Calender check out the crates `chrono` and `time`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GregorianDate {
    year: i16,
    month: u8,
    day: u8,
}

impl GregorianDate {
    // Earliest representable date in the calendar system, marking the farthest point in the past.
    // Latest representable date in the calendar system, marking the farthest point in the future.

    /// Earliest representable date: -10'000-03-22
    ///
    /// Corresponds to A000-01-01 in SAC13.
    /// We only allow GregorianDates that have a corresponding SAC13 date to simplify the API.
    pub const MIN: Self = Self {
        year: -10000,
        month: 3,
        day: 22,
    };

    /// Latest representable date: 16'000-03-17
    ///
    /// Corresponds to Z999-13-29 in SAC13.
    /// We only allow GregorianDates that have a corresponding SAC13 date to simplify the API.
    pub const MAX: Self = Self {
        year: 16000,
        month: 3,
        day: 17,
    };

    /// Creates a Gregorian Calendar date from its components _year_, _month_ and _day_.
    ///
    /// Returns [`None`] if the given date is invalid (doesn't exist in the Gregorian Calendar),
    /// like day zero or August 32th, or February 29th on non-leap (common) years.
    ///
    /// It also returns [`None`] if the date is outside the range for a valid
    #[must_use]
    pub const fn from_ymd(year: i16, month: u8, day: u8) -> Option<Self> {
        if month == 0 || month > 12 || day == 0 || day > Self::month_len(year, month).unwrap() {
            return None;
        }

        Self { year, month, day }.limit_sac13()
    }

    #[must_use]
    pub const fn is_leap_year(year: i16) -> bool {
        // TODO verify negative years, but should work because we check "== 0"

        if year % 400 == 0 {
            true
        } else if year % 100 == 0 {
            false
        } else {
            year % 4 == 0
        }
    }

    #[must_use]
    pub const fn month_len(year: i16, month: u8) -> Option<u8> {
        const DAYS_PER_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if month == 0 || month > 12 {
            return None;
        }

        Some(if month == 2 && Self::is_leap_year(year) {
            29
        } else {
            DAYS_PER_MONTH[(month - 1) as usize]
        })
    }

    #[must_use]
    pub const fn year(&self) -> i16 {
        self.year
    }

    #[must_use]
    pub const fn month(&self) -> u8 {
        self.month
    }

    #[must_use]
    pub const fn day(&self) -> u8 {
        self.day
    }

    const fn limit_sac13(self) -> Option<Self> {
        if matches!(Self::const_cmp(self, Self::MIN), Ordering::Less)
            || matches!(Self::const_cmp(self, Self::MAX), Ordering::Greater)
        {
            None
        } else {
            Some(self)
        }
    }

    const fn const_cmp(lhs: Self, rhs: Self) -> Ordering {
        if lhs.year < rhs.year {
            Ordering::Less
        } else if lhs.year > rhs.year {
            Ordering::Greater
        } else if lhs.month < rhs.month {
            Ordering::Less
        } else if lhs.month > rhs.month {
            Ordering::Greater
        } else if lhs.day < rhs.day {
            Ordering::Less
        } else if lhs.day > rhs.day {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Display for GregorianDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl CalendarDate for GregorianDate {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;

    #[must_use]
    #[allow(non_upper_case_globals)] // allowed to match nomenclature of E.G. Richards
    #[allow(clippy::let_and_return)]
    #[allow(non_snake_case)]
    #[allow(unused)]
    fn as_julian(&self) -> i32 {
        // Based on Edward Graham Richards Algorithm, Chapter 15
        // 15.11 Calendar Conversion Algorithms
        // https://aa.usno.navy.mil/downloads/c15_usb_online.pdf (page 617 ff)

        const y: i32 = 4716;
        const j: i32 = 1401;
        const m: i32 = 2;
        const n: i32 = 12;
        const r: i32 = 4;
        const p: i32 = 1461;
        const q: i32 = 0;
        const v: i32 = 3;
        const u: i32 = 5;
        const s: i32 = 153;
        const t: i32 = 2;
        const w: i32 = 2;
        const A: i32 = 184;
        const B: i32 = 274277;
        const C: i32 = -38;

        let D = i32::from(self.day);
        let M = i32::from(self.month);
        let Y = i32::from(self.year);

        let h = M - m;
        let g = Y + y - (n - h).div_euclid(n);
        let f = (h - 1 + n).rem_euclid(n);
        let e = (p * g + q).div_euclid(r) + D - 1 - j;
        let J = e + (s * f + t).div_euclid(u);

        let J = J - (3 * ((g + A).div_euclid(100))).div_euclid(4) - C;

        J
    }

    #[must_use]
    #[allow(non_upper_case_globals)] // allowed to match nomenclature of E.G. Richards
    #[allow(non_snake_case)]
    #[allow(unused)]
    fn from_julian(value: i32) -> Option<Self> {
        // Based on Edward Graham Richards Algorithm, Chapter 15
        // 15.11 Calendar Conversion Algorithms
        // https://aa.usno.navy.mil/downloads/c15_usb_online.pdf (page 617 ff)

        const y: i32 = 4716;
        const j: i32 = 1401;
        const m: i32 = 2;
        const n: i32 = 12;
        const r: i32 = 4;
        const p: i32 = 1461;
        const q: i32 = 0;
        const v: i32 = 3;
        const u: i32 = 5;
        const s: i32 = 153;
        const t: i32 = 2;
        const w: i32 = 2;
        const A: i32 = 184;
        const B: i32 = 274277;
        const C: i32 = -38;

        if !(JulianDay::MIN_INT..=JulianDay::MAX_INT).contains(&value) {
            return None;
        }

        let J = value;

        let f = J + j;
        let f = f + (((4 * J + B).div_euclid(146_097)) * 3).div_euclid(4) + C;
        let e = r * f + v;
        let g = e.rem_euclid(p).div_euclid(r);
        let h = u * g + w;

        let D = h.rem_euclid(s).div_euclid(u) + 1;
        let M = (h.div_euclid(s) + m).rem_euclid(n) + 1;
        let Y = e.div_euclid(p) - y + (n + m - M).div_euclid(n);

        Some(Self {
            year: Y as i16,
            month: M as u8,
            day: D as u8,
        })
    }

    #[must_use]
    fn tomorrow(mut self) -> Option<Self> {
        // Note: the implementation should be simple,
        // and almost trivial to show its correctness,
        // because it's used as a reference during unit testing.

        if self.day < 28 {
            self.day += 1;
            return Some(self);
        }

        let days = ok!(Self::month_len(self.year, self.month));

        if self.day < days {
            self.day += 1;
            return Some(self);
        }

        self.day = 1;
        self.month = if self.month == 12 { 1 } else { self.month + 1 };

        if self.month == 1 {
            self.year += 1;
        }

        self.limit_sac13()
    }

    #[must_use]
    fn yesterday(mut self) -> Option<Self> {
        // Note: the implementation should be simple,
        // and almost trivial to show its correctness,
        // because it's used as a reference during unit testing.

        if self.day > 1 {
            self.day -= 1;
            return Some(self);
        }

        self.month = if self.month == 1 { 12 } else { self.month - 1 };

        if self.month == 12 {
            self.year -= 1;
        }

        self.day = ok!(Self::month_len(self.year, self.month));

        self.limit_sac13()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn gregorian_julian_day_number_conversion_works_with_samples() {
        macro_rules! same {
            ($year:literal-$month:literal-$day:literal, $jdn:literal) => {
                assert_eq!(date_greg!($year - $month - $day).as_julian(), $jdn);

                assert_eq!(
                    date_greg!($year - $month - $day),
                    GregorianDate::from_julian($jdn).unwrap()
                );
            };
        }

        same!(2024 - 12 - 05, 2460650);
        same!(2000 - 01 - 01, 2451545);
        same!(2000 - 03 - 20, 2451624);
        same!(1600 - 02 - 29, 2305507);
    }
}
