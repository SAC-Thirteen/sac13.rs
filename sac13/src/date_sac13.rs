pub mod raw_date;

use core::fmt::Display;

use crate::{month::Month, scalars::Year, traits::CalendarDate};
use raw_date::{date_to_yo, yo_to_date, YearOrdinal};

/// SAC13 date.
///
/// Consists of the three components `year`, `month` and `day`.
/// Check the module root documentation for details about the SAC13 calendar and its dates. You can also check out the documentation for [`Year`] and [`Month`].
///
/// # Examples
///
/// ```
/// use sac13::prelude::*;
///
/// // Hard-coded values can be constructed with a compile-time checked macro.
/// let date = date!(M024 - 03 - 12);
///
/// // TODO: other ctors
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: Year,
    month: Month,
    day: u8,
}

impl Date {
    /// SAC13 date from given year, month, day.
    ///
    /// Returns `None` for invalid dates.
    ///
    /// Examples
    ///
    /// ```
    /// use sac13::prelude::*;
    ///
    /// let y = Year::new(12020).unwrap();
    /// let m = Month::new(5).unwrap();
    ///
    /// let date = Date::from_ymd(y,m,16).unwrap();
    /// ```
    #[must_use]
    pub const fn from_ymd(year: Year, month: Month, day: u8) -> Option<Self> {
        if day == 0 || day > Self::month_len(year, month) {
            None
        } else {
            Some(Self { year, month, day })
        }
    }

    /// Like [from_ymd()](Self::from_ymd()) but from integers instead of typed [`Year`] and [`Month`].
    ///
    /// Returns [`None`] for invalid dates.
    ///
    /// # Examples
    ///
    /// ```
    /// use sac13::{Date,date};
    ///
    /// assert_eq!(Date::from_ymd_untyped(12020,02,16), Some(date!(M020 - 02 - 16)));
    /// assert_eq!(Date::from_ymd_untyped(12020,02,29), None); // April 29th does not exist in SAC13
    /// ```
    #[must_use]
    pub const fn from_ymd_untyped(year: u16, month: u8, day: u8) -> Option<Self> {
        let y = match Year::new(year) {
            None => return None,
            Some(y) => y,
        };

        let m = match Month::new(month) {
            None => return None,
            Some(m) => m,
        };

        Self::from_ymd(y, m, day)
    }

    /// Year component of the date.
    #[must_use]
    pub const fn year(&self) -> Year {
        self.year
    }

    /// Month component of the date.
    #[must_use]
    pub const fn month(&self) -> Month {
        self.month
    }

    /// Day component of the date.
    #[must_use]
    pub const fn day(&self) -> u8 {
        self.day
    }

    /// SAC13 Weekday ordinal.
    ///
    /// Returns which day of the week it is. Typically 1-7, 8 on synchronization days.
    /// Note that 1-7 are NOT synonymous with Monday - Sunday. SAC13 doesn't have weekdays
    /// in the typical sense.
    #[must_use]
    pub const fn weekday_ordinal(&self) -> u8 {
        match self.day {
            29 => 8,
            x => (x - 1) % 7 + 1,
        }
    }

    /// All months have 28 days, except (Addenduary)[Month::Addenduary], and (August)[Month::August] on [leap years](Year::is_leap), which are 29 days long.
    #[must_use]
    pub const fn month_len(year: Year, month: Month) -> u8 {
        if matches!(month, Month::Addenduary) || (matches!(month, Month::August) && year.is_leap())
        {
            29
        } else {
            28
        }
    }
}

impl Display for Date {
    /// Displays the SAC13 date.
    ///
    /// ```
    /// use sac13::date;
    ///
    /// let formatted_date = format!("{}", date!(M020 - 05 - 21));
    /// assert_eq!(formatted_date, "M020-05-21");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month.ord(), self.day)
    }
}

impl CalendarDate for Date {
    const MIN: Self = date!(A000 - 01 - 01);
    const MAX: Self = date!(Z999 - 13 - 29);

    fn as_julian(&self) -> i32 {
        date_to_yo(*self).as_julian()
    }

    fn from_julian(value: i32) -> Option<Self> {
        Some(yo_to_date(YearOrdinal::from_julian(value)?))
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

        let days = Self::month_len(self.year, self.month);

        if self.day < days {
            self.day += 1;
            return Some(self);
        }

        self.day = 1;
        self.month = self.month.next();

        if matches!(self.month, Month::March) {
            self.year = ok!(self.year.next());
        }

        Some(self)
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

        self.month = self.month().previous();

        if matches!(self.month, Month::Addenduary) {
            self.year = ok!(self.year.previous());
        }

        self.day = Self::month_len(self.year, self.month);

        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use raw_date::YearOrdinal;

    use crate::{
        scalars::{CycleEpochDay, JulianDay, UnixDay},
        traits::CalendarDate,
        weekday::Weekday,
    };

    use super::*;

    #[test]
    fn test_date_order_and_equality() {
        assert!(date!(M020 - 05 - 16) == date!(M020 - 05 - 16));

        assert!(date!(M020 - 05 - 15) < date!(M020 - 05 - 16));
        assert!(date!(M020 - 05 - 16) > date!(M020 - 05 - 15));

        assert!(date!(M020 - 04 - 17) < date!(M020 - 05 - 16));
        assert!(date!(M019 - 06 - 17) < date!(M020 - 05 - 16));
    }

    #[test]
    pub fn reference_date_unix_epoch_works() {
        let date: Date = UnixDay::new(11036).unwrap().convert();

        assert_eq!(date.year(), year!(M000));
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
    }

    #[test]
    pub fn reference_date_julian_day_works() {
        let date: Date = Date::from_julian(2451624).unwrap();

        assert_eq!(date.year(), year!(M000));
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
    }

    #[test]
    pub fn reference_date_gregorian_converted_to_sac13_works() {
        let date: Date = date_greg!(2000 - 03 - 20).convert();

        assert_eq!(date.year(), year!(M000));
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);

        assert_eq!(date.convert::<JulianDay>().weekday(), Weekday::Monday);
    }

    #[test]
    pub fn reference_timestamp_year_zero_works() {
        let result: YearOrdinal = CycleEpochDay::new(72683).unwrap().convert();

        assert_eq!(result.year(), year!(A000));
        assert_eq!(result.day(), 0);
    }

    #[test]
    pub fn leap_year_rule_works_as_expected() {
        assert!(year!(L814).is_common());
        assert!(year!(L815).is_leap());
    }
}
