#![doc = include_str!("../README.md")]
#![no_std]
// #![cfg_attr(not(test), no_std)]

// Please ignore the messy clippy part.
// I'm still trying to figure out which lints to enable.
#![warn(
     clippy::all,
    // clippy::restriction,
     clippy::pedantic,
    clippy::nursery,
    // clippy::cargo
)]
#![warn(clippy::trivially_copy_pass_by_ref)]

// TODO: features: serde, std, alloc, macros, formatting, wasm?, chrono, time
// TODO: maybe more const?
// TODO: difference between dates (in days)
// TODO: subtract dates (even between greg and sac13?)
// TODO: move macros?
// TODO: move date_greg macro to greg module + reexport in prelude

/// Creates a [SAC13 year](Year) with a statically known value.
///
/// This is typically used to create hard coded dates without `unwrap()` (which would be a runtime check).
///
/// # Example
///
/// ```
/// use sac13::year;
///
/// let year = year!(M020);
///
/// let year_zero = year!(A000);
/// let last_year = year!(Z999);
///
/// // The following lines are invalid years (or format) and would fail during compilation:
/// // let year = year!(m020);
/// // let year = year!(20020);
/// // let year = year!(-100);
/// // let year = year!(20);
/// ```
#[macro_export]
macro_rules! year {
    ($year:ident) => {
        const {
            $crate::Year::try_from_str(core::stringify!($year))
                .expect(concat!("Invalid SAC13 year: ", stringify!($year)))
        }
    };
}

/// Creates a [Gregorian Calendar date](GregorianDate) with a statically known value.
///
/// This is typically used to create hard coded dates without `unwrap()` (which would be a runtime check).
///
/// # Example
///
/// ```
/// use sac13::prelude::*;
///
/// let date = date_greg!(2020 - 04 - 17);
/// let date = date_greg!(2020 - 02 - 29);  // leap year
///
/// // the following line would not compile (because 2021 wasn't a leap year)
/// // let date = date_greg!(2021 - 02 - 29);
/// ```
#[macro_export]
macro_rules! date_greg {
    ($year:literal - $month:literal - $day:literal) => {
        const {
            #[allow(clippy::zero_prefixed_literal)]
            let y = $year;

            #[allow(clippy::zero_prefixed_literal)]
            let m = $month;

            #[allow(clippy::zero_prefixed_literal)]
            let d = $day;

            $crate::GregorianDate::from_ymd(y, m, d)
                .expect("The given input was not a valid Gregorian Calendar date")
        }
    };
}

/// Creates a [SAC13 date](Date) with a statically known value.
///
/// This is typically used to create hard coded dates without `unwrap()` (which would be a runtime check).
///
/// # Example
///
/// ```
/// use sac13::prelude::*;
///
/// let date = date!(M020 - 04 - 14); // "regular" day
/// let date = date!(M020 - 13 - 29); // year day
/// let date = date!(M021 - 06 - 29); // leap day
///
/// // the following lines would not compile
///
/// // date = date!(M022 - 06 - 29); // M022 is not a leap year
/// // date = date!(M022 - 04 - 29); // No month except August on leap years and Addenduary have more than 28 days
///
/// ```
#[macro_export]
macro_rules! date {
    ($year:ident - $month:literal - $day:literal) => {
        const {
            let y = $crate::year!($year);

            #[allow(clippy::zero_prefixed_literal)]
            let m = $month;

            #[allow(clippy::zero_prefixed_literal)]
            let d = $day;

            let m = $crate::Month::new(m).expect(concat!(
                "Month must be a value from 1 - 13. Given: ",
                $month
            ));

            $crate::Date::from_ymd(y, m, d).expect("The given input was not a valid SAC13 date")
        }
    };
}

macro_rules! ok {
    ($opt:expr) => {
        match $opt {
            ::core::option::Option::None => return ::core::option::Option::None,
            ::core::option::Option::Some(x) => x,
        }
    };
}

/// The type of the year.
///
/// A [`YearType::Common`] year has 365 days and a [`YearType::Leap`] year has 366 days.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum YearType {
    Common,
    Leap,
}

mod date_gregorian;
mod date_sac13;
mod epoch_day;
mod misc;
mod month;
mod parse;
mod scalars;
mod traits;
mod weekday;

pub mod prelude;

/// Primitive types for linear day counts like the [Julian Day Number](crate::scalars::JulianDay).
pub mod day_counts {
    pub use crate::date_sac13::raw_date::YearOrdinal;
    pub use crate::scalars::{CycleEpochDay, JulianDay, Sac13Day, UnixDay};
}

pub use parse::parse_date_str;
pub use parse::ComponentOrder;
pub use parse::GregorianOrSac13;
pub use parse::ParsedDate;

pub use date_gregorian::GregorianDate;
pub use date_sac13::Date;
pub use scalars::Year;
pub use traits::CalendarDate;

pub use month::Month;

#[cfg(test)]
mod tests {
    use crate::{prelude::*, scalars::JulianDay};

    #[test]
    pub fn const_year_num_is_same_as_during_construction() {
        assert_eq!(year!(B000).value(), 1000);
    }

    #[test]
    pub fn const_date_construction_works() {}

    #[test]
    pub fn greg_from_sac13_works() {
        let result: GregorianDate = date!(M000 - 01 - 01).convert();

        assert_eq!(result, date_greg!(2000 - 03 - 20));
    }

    #[test]
    pub fn sac13_from_greg_works() {
        let result: Date = date_greg!(2000 - 03 - 20).convert();
        assert_eq!(result, date!(M000 - 01 - 01));
    }
}
