use core::fmt::Display;

/// A minimum set of functionality a typical calendar should provide.
pub trait CalendarDate: Sized + Display {
    /// Earliest representable date.
    const MIN: Self;

    /// Latest representable date.
    const MAX: Self;

    #[must_use]
    /// Convert the date into the corresponding Julian Day Number.
    fn as_julian(&self) -> i32;

    #[must_use]
    /// Get the date from the corresponding Julian
    fn from_julian(value: i32) -> Option<Self>;

    #[must_use]
    fn tomorrow(self) -> Option<Self> {
        Self::from_julian(self.as_julian() + 1)
    }

    #[must_use]
    fn yesterday(self) -> Option<Self> {
        Self::from_julian(self.as_julian() - 1)
    }

    /// Converts the calendar date to a different calendar system.
    ///
    /// # Examples
    ///
    /// ```
    /// use sac13::prelude::*;
    /// use sac13::day_counts::*;
    ///
    /// let date_sac13 : Date = JulianDay::new(2460000).unwrap().convert();
    /// let date_greg : GregorianDate = date_sac13.convert();
    /// ```
    ///
    /// It's basically like the [`From`] trait, but because of the orphan rule I failed
    /// to implement it generically for all types that implement [`CalendarDate`].
    #[must_use]
    fn convert<T: CalendarDate>(self) -> T {
        T::from_julian(self.as_julian()).expect("SAC13 range calendars to be convertible.")
    }
}
