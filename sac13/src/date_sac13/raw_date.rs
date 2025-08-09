use core::fmt::Display;

use crate::{
    scalars::{CycleEpochDay, Year},
    traits::CalendarDate,
    Month,
};

use super::Date;

pub const RAW_YEAR_OFFSET: u16 = 199;

macro_rules! days_in_cycle {
    ($leap:literal, $total:literal) => {
        const { $leap * 366 + ($total - $leap) * 365 }
    };
}

macro_rules! to_years {
    ($y:ident, $d:ident, $leap:literal, $total:literal) => {
        let f = $d / days_in_cycle!($leap, $total);
        $d -= f * days_in_cycle!($leap, $total);
        $y += f * $total;
    };
}

macro_rules! to_years_rest {
    ($y:ident, $d:ident) => {
        // remaining years: CLCCC
        // C = common year
        // L = leap year

        if $d >= 731 {
            // Years: CL|CCC
            $d -= 731;
            $y += 2 + $d / 365;
            $d %= 365;
        } else if $d >= 365 {
            // Years: C|LCCC
            $d -= 365;
            $y += 1;
        }
    };
}

macro_rules! to_days {
    ($y:ident, $d:ident, $leap:literal, $total:literal) => {
        let f = $y / $total;

        $d += f * days_in_cycle!($leap, $total);
        $y -= f * $total;
    };
}

macro_rules! to_days_rest {
    ($y:ident, $d:ident) => {
        // remaining years: CLCCC
        // C = common year
        // L = leap year

        $d += $y * 365;

        if $y >= 2 {
            $d += 1;
        }

        // theoretically the year should be set to zero here,
        // but we intentionally don't do that because nobody
        // reads the year after this macro

        // $y = 0;
    };
}

/// SAC13 year ordinal. Consists of a year and the day of the year.
///
/// This is only used internally and is not exposed for library consumers.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct YearOrdinal {
    year: Year,
    day: u16,
}

impl YearOrdinal {
    #[must_use]
    pub const fn new(year: Year, day: u16) -> Option<Self> {
        if day >= year.days() {
            None
        } else {
            Some(Self { year, day })
        }
    }

    #[must_use]
    pub const fn year(&self) -> Year {
        self.year
    }

    #[must_use]
    pub const fn day(&self) -> u16 {
        self.day
    }

    #[must_use]
    const fn from_epoch_day(value: CycleEpochDay) -> Self {
        let mut d = value.value();
        let mut y = 0;

        to_years!(y, d, 71, 293);
        to_years!(y, d, 8, 33);
        to_years!(y, d, 1, 4);

        to_years_rest!(y, d);

        Self {
            day: d as u16,
            year: Year::new(y as u16 - RAW_YEAR_OFFSET).expect("Year be in range. This is a bug!"),
        }
    }

    #[must_use]
    const fn epoch_day(self) -> CycleEpochDay {
        let mut d = self.day as u32;
        let mut y = (self.year.value() + RAW_YEAR_OFFSET) as u32;

        to_days!(y, d, 71, 293);
        to_days!(y, d, 8, 33);
        to_days!(y, d, 1, 4);

        to_days_rest!(y, d);

        CycleEpochDay::new(d).unwrap()
    }
}

impl Display for YearOrdinal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}-{}", self.year(), self.day())
    }
}

impl CalendarDate for YearOrdinal {
    const MIN: Self = Self::new(year!(A000), 0).unwrap();
    const MAX: Self = Self::new(year!(Z999), 364).unwrap();

    fn as_julian(&self) -> i32 {
        self.epoch_day().as_julian()
    }

    fn from_julian(value: i32) -> Option<Self> {
        Some(Self::from_epoch_day(CycleEpochDay::from_julian(value)?))
    }
}

pub fn date_to_yo(value: Date) -> YearOrdinal {
    let year = value.year;
    let month = value.month;

    let mut day = (month.ord() as u16 - 1) * 28 + value.day as u16 - 1;

    if year.is_leap() && month > Month::August {
        day += 1;
    }

    YearOrdinal { year, day }
}

pub const fn yo_to_date(value: YearOrdinal) -> Date {
    // TODO: check all unwraps

    let mut days = value.day;
    let year = value.year;

    if year.is_leap() {
        #[allow(clippy::comparison_chain)] // more readable
        if days == 28 * 6 {
            return Date {
                year,
                month: Month::August,
                day: 29,
            };
        } else if days > 28 * 6 {
            days -= 1;
        }
    }

    if days == 364 {
        return Date {
            year,
            month: Month::Addenduary,
            day: 29,
        };
    }

    let day = ((days % 28) + 1) as u8;
    let month = ((days / 28) + 1) as u8;
    let month = Month::new(month).unwrap();

    Date { year, month, day }
}
