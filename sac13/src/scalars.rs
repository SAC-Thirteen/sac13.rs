//! Types in this module represent linear

use crate::{traits::CalendarDate, weekday::Weekday, YearType};

macro_rules! scalar {
    (
        $(#[$attr:meta])*
        name: $name:ident;
        unit: $unit:ident;
        base: $t:ty;
        min: $min:expr;
        max: $max:expr;
        future: $next:ident;
        past: $previous:ident;
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $name($t);

        impl $name {
            /// Earliest representable date (in its native integer type).
            pub const MIN_INT: $t = $min;

            /// Latest representable value (in its native integer type).
            pub const MAX_INT: $t = $max;

            /// Earliest representable date
            pub const MIN: Self = Self(Self::MIN_INT);

            /// Latest representable value
            pub const MAX: Self = Self(Self::MAX_INT);

            #[doc = concat!("Creates a new ", stringify!($name), ".","\n\n", "Returns `None` for invalid values. Valid values are between (and including) [`MIN_INT`](Self::MIN_INT) (A000-01-01) and [`MAX_INT`](Self::MAX_INT) (Z999-13-29).")]
            #[inline]
            #[must_use]
            pub const fn new(value: $t) -> Option<Self> {
                if value >= Self::MIN_INT && value <= Self::MAX_INT {
                    Some(Self(value))
                } else {
                    None
                }
            }

            /// Returns the underlying integer value.
            #[inline(always)]
            #[must_use]
            pub const fn value(&self) -> $t {
                self.0
            }


            #[doc = concat!("Returns the next ", stringify!($unit), ".")]
            #[doc = concat!("Returns `None` if the next ", stringify!($unit), " is outside the valid SAC13 range.")]
            #[inline]
            #[must_use]
            pub const fn $next(&self) -> Option<Self> {
                if self.0 < Self::MAX_INT {
                    Some(Self(self.0+1))
                } else {
                    None
                }
            }

            #[doc = concat!("Returns the previous ", stringify!($unit), ".")]
            #[doc = concat!("Returns `None` if the previous ", stringify!($unit), " is outside the valid SAC13 range.")]
            #[inline]
            #[must_use]
            pub const fn $previous(&self) -> Option<Self> {
                if self.0 > Self::MIN_INT {
                    Some(Self(self.0-1))
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! scalar_day {
    (
        $(#[$attr:meta])*
        name: $name:ident;
        base: $t:ty;
        min: $min:literal;
    ) => {
        scalar!(
            $(#[$attr])*
            name: $name;
            unit: day;
            base: $t;
            min: $min;
            max: ($min + 9496300);
            future: tomorrow;
            past: yesterday;
        );

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f,"{}", self.0)
            }
        }

        impl $name {
            const JULIAN_OFFSET: i32 = -1931284 - $min;
        }

        impl CalendarDate for $name {
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;

            fn as_julian(&self) -> i32 {
                (self.0 as i32) + Self::JULIAN_OFFSET
            }

            fn from_julian(value: i32) -> Option<Self> {
                Self::new((value - Self::JULIAN_OFFSET) as $t)
            }
        }
    };
}

// // TODO: unix day documentation details

scalar_day!(
    /// Days since the Unix Epoch. 1970-01-01 (Gregorian) is day zero.
    ///
    /// _Important: "Unix Day" is **not** a Unix timestamp! Neither in seconds nor milliseconds._
    ///
    /// # Example
    ///
    /// ```
    /// use sac13::prelude::*;
    /// use sac13::day_counts::UnixDay;
    ///
    /// const SECONDS_PER_DAY : i32 = 86400;
    ///
    /// let unix_timestamp_seconds = 1355313600;
    /// let unix_day = unix_timestamp_seconds / SECONDS_PER_DAY;
    ///
    /// let date : Date = UnixDay::new(unix_day).unwrap().convert();
    /// ```
    ///
    /// # About Unix Days
    /// The name is actually made up, but the concept is simple. It's the number of days since
    /// the Unix Epoch, so 1.1.1970 (Gregorian) is day zero. Java calls this "Epoch Day"
    /// in the documentation for `LocalDate.toEpochDay()`, but because every timescale has a
    /// different Epoch and we wanted to be precise, we chose the name "Unix Day" instead.
    ///
    /// ## Why do I have to manually convert?
    /// It was actually a very deliberate choice to not allow passing the unix timestamp directly.
    /// TODO
    ///
    ///
    name: UnixDay;
    base: i32;
    min: -4371872;
    // 5124428
);

scalar_day!(
    /// Julian Day Number. Day count since the beginning of the Julian period.
    name: JulianDay;
    base: i32;
    min: -1931284;
);

scalar_day!(
    /// SAC13 Year Cycle Epoch Day Number.
    ///
    /// Day count since the beginning of the first SAC13 cycle.
    name: CycleEpochDay;
    base: u32;
    min: 72683;
);

scalar_day!(
    /// SAC13 Day Number. Days since A000-01-01.
    name: Sac13Day;
    base: u32;
    min: 0;
);

scalar!(
    /// SAC13 year. It roughly corresponds to the Gregorian Year + 10'000.
    /// Roughly, because the Gregorian Calendar starts its year with January and
    /// SAC13 with March.
    ///
    /// The year can have any value from 0 to 25'999 (both inclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// use sac13::prelude::*;
    ///
    /// // Preferred method for hard-coded / compile-time years:
    /// let year = year!(M024);
    ///
    /// // From &str (e.g. user input):
    /// let year = Year::try_from_str("M024").unwrap();
    ///
    /// // From an integer (via `TryFrom` trait):
    /// let year = Year::new(12_024).unwrap();
    ///
    /// // Year construction via const compile-time helper function:
    /// // Prefer that method if you know the year at compile-time.
    ///
    /// ```
    ///
    /// TODO: year construction only via TryFrom?
    ///
    /// # About the limits
    /// Even though the SAC13 calendar system design could easily support negative years
    /// and years beyond 26'000 we intentionally chose not to for the following reasons:
    ///
    /// - The 10'000 year offset compared to the Gregorian Calendar the year zero
    ///   is so far in the past, there aren't really any applications for exact dates around
    ///   that time. Maybe astronomers, but they already use linear time-scales like JDN
    ///   instead of civil calendars.
    ///
    /// - The year 25'999 is so far in the future it's highly unlikely, that this calendar
    ///   would survive unaltered for that long anyway. I hope that humans (or our AI overlords)
    ///   are no longer interested in Earth-based solar calendars at that point.
    ///
    /// - SAC13 years are typically written with a millennium indicator letter
    ///   (A=0, B=1, ..., Z=25) to disambiguate between SAC13 and the Gregorian Calendar.
    ///   So the year 12'020 is written as M020 and the year 25'999 would be Z999.
    ///
    /// All SAC13 implementations have to respect those limits and handle edge cases accordingly.
    /// Having different limits than zero and 25'999 is considered a bug. Limits should be handled
    /// as graceful as possible.
    ///
    /// For example:
    ///
    ///   - If you have a UI, prevent the user from switching to dates outside the limit.
    ///   - If you process data, or handle requests, return an error if the request contains
    ///     invalid dates (dates outside the limits are invalid!).
    ///   - As a last resort you can also log, silently drop or abort the process in those cases.
    ///
    /// The benefit of such strict rules is that everybody knows what to expect and what other
    /// systems consider valid or invalid. Unlike with the Gregorian Calender where everybody
    /// decides for themselves what the limits are. Because of that, every software out there handles
    /// the cases differently on arbitrary limits.
    ///
    /// If you are implementing SAC13 according to the specification you know for a fact that
    /// using a 16 bit integer (signed or unsigned doesn't matter) would be enough.
    name: Year;
    unit: year;

    base: u16;
    min: 0;
    max: 25999;

    future: next;
    past: previous;
);

impl core::fmt::Display for Year {
    /// Displays the year with prefixed millennium indicator.
    ///
    /// ```
    /// use sac13::year;
    ///
    /// let formatted_year = format!("{}", year!(M020));
    /// assert_eq!(formatted_year, "M020");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let millennium = (self.0 / 1000) as u8;
        let sub_mill = self.0 % 1000;

        let m = (b'A' + millennium) as char;

        write!(f, "{m}{sub_mill:03}")
    }
}

macro_rules! parse_digits {
    ($var:ident, $min:literal, $max:literal) => {
        if $var < $min || $var > $max {
            return None;
        } else {
            ($var - $min) as u16
        }
    };
}

impl Year {
    /// Returns the year, given four ASCII digits
    #[inline(always)]
    const fn parse_year_digits(d0: u8, d1: u8, d2: u8, d3: u8) -> Option<Self> {
        let d0 = parse_digits!(d0, b'A', b'Z');
        let d1 = parse_digits!(d1, b'0', b'9');
        let d2 = parse_digits!(d2, b'0', b'9');
        let d3 = parse_digits!(d3, b'0', b'9');

        let year_value = d0 * 1000 + d1 * 100 + d2 * 10 + d3;

        Self::new(year_value)
    }

    /// Used internally for the `year!()` macro.
    /// TODO: details
    #[must_use]
    pub const fn try_from_str(year: &str) -> Option<Self> {
        let year_bytes = year.as_bytes();

        if year_bytes.len() != 4 {
            return None;
        }

        Self::parse_year_digits(year_bytes[0], year_bytes[1], year_bytes[2], year_bytes[3])
    }

    /// Returns the type of the year (leap year or common year).
    #[must_use]
    pub const fn year_type(&self) -> YearType {
        match (self.0 + 199) % 293 % 33 % 4 {
            1 => YearType::Leap,
            _ => YearType::Common,
        }
    }

    #[must_use]
    pub const fn is_leap(&self) -> bool {
        matches!(self.year_type(), YearType::Leap)
    }

    #[must_use]
    pub const fn is_common(&self) -> bool {
        matches!(self.year_type(), YearType::Common)
    }

    /// Returns the number of days the year has.
    ///
    /// Can only be 365 or 366.
    #[must_use]
    pub const fn days(&self) -> u16 {
        match self.year_type() {
            YearType::Common => 365,
            YearType::Leap => 366,
        }
    }
}

impl JulianDay {
    pub fn weekday(self) -> Weekday {
        use crate::weekday::Weekday::*;

        match self.0.rem_euclid(7) {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            6 => Sunday,
            _ => unreachable!(),
        }
    }
}
