use std::prelude::rust_2024::*;

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

#[test]
fn snapshot_all_leap_years() {
    let mut y = Year::MIN;
    let mut leap_years = vec![];

    loop {
        if y.is_leap() {
            leap_years.push(format!("{y}").to_string());
        }

        match y.next() {
            Some(new_y) => y = new_y,
            None => break,
        };
    }

    insta::assert_yaml_snapshot!(leap_years);
}

// #[test]
// fn demo() {
//     let j = -1931284;

//     let jd = JulianDay::new(j).unwrap();
//     let jd_t = JulianDay::new(j + 1).unwrap();

//     assert_eq!(jd_t.yesterday(), jd)
// }
