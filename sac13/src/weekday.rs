use core::fmt::Display;

/// Represents the Gregorian weekday.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Weekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

impl Weekday {
    /// The international (english) name for the day of the week.
    ///
    /// The [`Display`](core::fmt::Display) implementation also calls this method.
    #[must_use]
    pub const fn name(self) -> &'static str {
        use Weekday::*;

        match self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday",
        }
    }

    /// The international (english) two letter abbreviation for the day of the week.
    ///
    /// Mo, Tu, We, Th, Fr, Sa, Su
    #[must_use]
    pub const fn name_abr2(self) -> &'static str {
        use Weekday::*;

        match self {
            Monday => "Mo",
            Tuesday => "Tu",
            Wednesday => "We",
            Thursday => "Th",
            Friday => "Fr",
            Saturday => "Sa",
            Sunday => "Su",
        }
    }

    /// The international (english) three letter abbreviation for the day of the week.
    ///
    /// Mon, Tue, Wed, Thu, Fri, Sat, Sun
    #[must_use]
    pub const fn name_abr3(self) -> &'static str {
        use Weekday::*;

        match self {
            Monday => "Mon",
            Tuesday => "Tue",
            Wednesday => "Wed",
            Thursday => "Thu",
            Friday => "Fri",
            Saturday => "Sat",
            Sunday => "Sun",
        }
    }

    /// Next weekday.
    #[must_use]
    pub const fn next(self) -> Self {
        use Weekday::*;

        match self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }
    }

    /// Previous weekday.
    #[must_use]
    pub const fn previous(self) -> Self {
        use Weekday::*;

        match self {
            Monday => Sunday,
            Tuesday => Monday,
            Wednesday => Tuesday,
            Thursday => Wednesday,
            Friday => Thursday,
            Saturday => Friday,
            Sunday => Saturday,
        }
    }
}

impl Display for Weekday {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}
