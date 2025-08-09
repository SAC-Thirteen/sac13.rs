#![cfg(test)]

use crate::prelude::*;

use crate::{
    scalars::{JulianDay, UnixDay},
    traits::CalendarDate,
};

#[test]
fn automatic_max_calculation_for_scalars_works_as_expected() {
    assert_eq!(UnixDay::MAX.value(), 5124428);
}

#[test]
fn exhaustive_day_conversion_check() {
    for j in JulianDay::MIN_INT..=JulianDay::MAX_INT {
        let has_yesterday = j != JulianDay::MIN_INT;
        let has_tomorrow = j != JulianDay::MAX_INT;

        let jd = JulianDay::new(j).unwrap();
        let sac13 = Date::from_julian(j).unwrap();
        let greg = GregorianDate::from_julian(j).unwrap();

        if has_yesterday {
            let j_y = j - 1;

            let jd_y = JulianDay::new(j_y).unwrap();
            let sac13_y = Date::from_julian(j_y).unwrap();
            let greg_y = GregorianDate::from_julian(j_y).unwrap();

            assert_eq!(jd_y.tomorrow().unwrap(), jd, "JD: {}", j);
            assert_eq!(sac13_y.tomorrow().unwrap(), sac13, "JD: {}", j);
            assert_eq!(greg_y.tomorrow().unwrap(), greg, "JD: {}", j);
        } else {
            assert_eq!(jd, JulianDay::MIN, "JD: {}", j);
            assert_eq!(sac13, Date::MIN, "JD: {}", j);
            assert_eq!(greg, GregorianDate::MIN, "JD: {}", j);
        }

        if has_tomorrow {
            let j_t = j + 1;

            let jd_t = JulianDay::new(j_t).unwrap();
            let sac13_t = Date::from_julian(j_t).unwrap();
            let greg_t = GregorianDate::from_julian(j_t).unwrap();

            assert_eq!(jd_t.yesterday().unwrap(), jd, "JD: {}", j);
            assert_eq!(sac13_t.yesterday().unwrap(), sac13, "JD: {}", j);
            assert_eq!(greg_t.yesterday().unwrap(), greg, "JD: {}", j);
        } else {
            assert_eq!(jd, JulianDay::MAX, "JD: {}", j);
            assert_eq!(sac13, Date::MAX, "JD: {}", j);
            assert_eq!(greg, GregorianDate::MAX, "JD: {}", j);
        }
    }
}

// #[test]
// fn demo() {
//     let j = -1931284;

//     let jd = JulianDay::new(j).unwrap();
//     let jd_t = JulianDay::new(j + 1).unwrap();

//     assert_eq!(jd_t.yesterday(), jd)
// }
