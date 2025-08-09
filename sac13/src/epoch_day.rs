

// TODO: document cycle epoch day

// /// Represents a SAC13 epoch day count (like Julian Day Number, but with an offset).
// #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
// pub struct EpochDay(u32);

// // TODO: explain why it doesn't start at zero

// impl EpochDay {
//     /// Earliest representable value in SAC13 (= 72'683) which corresponds to A000-01-01.
//     pub const MIN: Self = Self(Self::MIN_EPOCH_DAY);

//     /// Latest representable value in SAC13 (= 9'568'983) which corresponds to Z999-13-29.
//     pub const MAX: Self = Self(9_568_983);

//     pub const MIN_EPOCH_DAY: u32 = 72_683;
//     pub const MAX_EPOCH_DAY: u32 = 9_568_983;

//     pub const MIN_JULIAN_DAY: i32 = -1_931_284;
//     pub const MAX_JULIAN_DAY: i32 = 7_565_016;
//     pub const JULIAN_DAY_OFFSET: i32 = 2003967;

//     pub const MIN_UNIX_DAY: i32 = -4_371_872;
//     pub const MAX_UNIX_DAY: i32 = 5_124_428;
//     pub const UNIX_DAY_OFFSET: i32 = 4444555;

//     /// Creates a new EpochDay. Returns `None` for invalid values. Valid values are between 72'683 and 9'568'983 (both inclusive).
//     #[must_use]
//     pub const fn new(value: u32) -> Option<Self> {
//         if value >= 72_683 && value <= 9_568_983 {
//             Some(Self(value))
//         } else {
//             None
//         }
//     }

//     #[must_use]
//     pub const fn from_julian_day(value: i32) -> Option<Self> {
//         if value < Self::MIN_JULIAN_DAY || value > Self::MAX_JULIAN_DAY {
//             return None;
//         }

//         Some(Self((value + Self::JULIAN_DAY_OFFSET) as u32))
//     }


//     #[must_use]
//     pub const fn from_unix_day(value: i32) -> Option<Self> {
//         if value < Self::MIN_UNIX_DAY || value > Self::MAX_UNIX_DAY {
//             return None;
//         }

//         Some(Self((value + Self::UNIX_DAY_OFFSET) as u32))
//     }

//     #[inline]
//     #[must_use]
//     pub const fn julian_day(&self) -> i32 {
//         self.0 as i32 - Self::JULIAN_DAY_OFFSET
//     }

//     #[inline]
//     #[must_use]
//     pub const fn unix_day(&self) -> i32 {
//         self.0 as i32 - Self::UNIX_DAY_OFFSET
//     }

//     /// Returns the underlying Epoch Day value.
//     #[inline(always)]
//     #[must_use]
//     pub const fn value(&self) -> u32 {
//         self.0
//     }
// }

// impl core::fmt::Display for EpochDay {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
