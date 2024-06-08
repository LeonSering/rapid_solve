use super::converters::{from_days_to_yyyy_mm_dd, from_yyyy_mm_dd_to_days, is_leap_year};

use self::converters::{days_of_month, get_days_in_year};

use super::*;

#[test]
fn test_from_yyyy_mm_dd_to_days_for_1990() {
    let year = 1999;
    let month = 12;
    let day = 31;
    let mut days = 0;
    for y in 0..year {
        days += get_days_in_year(y) as u64;
    }
    for m in 1..month {
        days += days_of_month(year, m) as u64;
    }
    days = days + day as u64 - 1;
    assert_eq!(
        from_yyyy_mm_dd_to_days(year, month, day),
        days,
        "from_yyyy_mm_dd_to_days() does not return the correct number of days"
    );
}

#[test]
fn test_from_yyyy_mm_dd_to_days_for_2990() {
    let year = 2999;
    let month = 12;
    let day = 31;
    let mut days = 0;
    for y in 0..year {
        days += get_days_in_year(y) as u64;
    }
    for m in 1..month {
        days += days_of_month(year, m) as u64;
    }
    days = days + day as u64 - 1;
    assert_eq!(
        from_yyyy_mm_dd_to_days(year, month, day),
        days,
        "from_yyyy_mm_dd_to_days() does not return the correct number of days"
    );
}

#[test]
fn test_from_days_to_yyyy_mm_dd() {
    let days = 4234221321;
    let mut year = 0;
    let mut month = 1;
    let mut days_remaining = days;
    while days_remaining >= get_days_in_year(year) as u64 {
        days_remaining -= get_days_in_year(year) as u64;
        year += 1;
    }
    while days_remaining >= days_of_month(year, month) as u64 {
        days_remaining -= days_of_month(year, month) as u64;
        month += 1;
    }
    let day = days_remaining as u8 + 1;
    assert_eq!(
        from_days_to_yyyy_mm_dd(days),
        (year, month, day),
        "from_days_to_yyyy_mm_dd() does not return the correct date"
    );
}

#[test]
fn test_days_of_month() {
    assert_eq!(days_of_month(2000, 1), 31);
    assert_eq!(days_of_month(2000, 2), 29);
    assert_eq!(days_of_month(2000, 3), 31);
    assert_eq!(days_of_month(2000, 4), 30);
    assert_eq!(days_of_month(2000, 5), 31);
    assert_eq!(days_of_month(2000, 6), 30);
    assert_eq!(days_of_month(2000, 7), 31);
    assert_eq!(days_of_month(2000, 8), 31);
    assert_eq!(days_of_month(2000, 9), 30);
    assert_eq!(days_of_month(2000, 10), 31);
    assert_eq!(days_of_month(2000, 11), 30);
    assert_eq!(days_of_month(2000, 12), 31);

    assert_eq!(days_of_month(2001, 2), 28);
    assert_eq!(days_of_month(2024, 2), 29);
    assert_eq!(days_of_month(2100, 2), 28);
}

#[test]
fn test_is_leap_year() {
    assert!(is_leap_year(2000), "2000 is a leap year");
    assert!(!is_leap_year(2001), "2001 is not a leap year");
    assert!(is_leap_year(2024), "2024 is a leap year");
    assert!(!is_leap_year(2100), "2100 is not leap year");
}

#[test]
fn test_iso_with_seconds() {
    let time = DateTime::new("2022-02-06T23:59:59");
    assert_eq!(
        time.as_iso(),
        "2022-02-06T23:59:59",
        "as_iso() does not return the correct string"
    );
    assert_eq!(
        format!("{}", time),
        "06.02.2022_23:59:59",
        "format!() does not return the correct string"
    );
}

#[test]
fn test_iso_without_seconds() {
    let time = DateTime::new("2022-02-06T23:59");
    assert_eq!(
        time.as_iso(),
        "2022-02-06T23:59:00",
        "as_iso() does not return the correct string"
    );
    assert_eq!(
        format!("{}", time),
        "06.02.2022_23:59",
        "format!() does not return the correct string"
    );
}

#[test]
fn test_iso_with_earliest() {
    let time = DateTime::Earliest;
    assert_eq!(
        time.as_iso(),
        "EARLIEST",
        "as_iso() does not return the correct string"
    );
    assert_eq!(
        format!("{}", time),
        "Earliest",
        "format!() does not return the correct string"
    );
}

#[test]
fn test_iso_with_latest() {
    let time = DateTime::Latest;
    assert_eq!(
        time.as_iso(),
        "LATEST",
        "as_iso() does not return the correct string"
    );
    assert_eq!(
        format!("{}", time),
        "Latest",
        "format!() does not return the correct string"
    );
}

#[test]
fn sum_up_duration() {
    let dur1 = Duration::new("5000:40:31");
    let dur2 = Duration::new("00:46:30");
    let sum = Duration::new("5001:27:01");
    assert!(
        dur1 + dur2 == sum,
        "Duration does not sum up correctly. dur1: {} + dur2: {} is {}; but should be {}",
        dur1,
        dur2,
        dur1 + dur2,
        sum
    );
}

#[test]
fn add_duration_to_time_no_leap_year() {
    let time = DateTime::new("1999-2-28T23:40:59");
    let dur = Duration::new("48:46:01");
    let sum = DateTime::new("1999-3-3T00:27");
    assert!(
        time + dur == sum,
        "Duration does not sum up correctly. time: {} + dur: {} is {}; but should be {}",
        time,
        dur,
        time + dur,
        sum
    );
}

#[test]
fn add_duration_to_time_leap_year() {
    let time = DateTime::new("2000-02-28T23:40");
    let dur = Duration::new("48:46:03");
    let sum = DateTime::new("2000-3-2T00:26:03");
    assert!(
        time + dur == sum,
        "Duration does not sum up correctly. time: {} + dur: {} is {}; but should be {}",
        time,
        dur,
        time + dur,
        sum
    );
}

#[test]
fn add_long_duration_to_time() {
    let time = DateTime::new("1-01-01T00:00"); // jesus just got one year old ;)
    let dur = Duration::new("10000000:00:00");
    let sum = DateTime::new("1141-10-18T16:00");
    assert!(
        time + dur == sum,
        "Duration does not sum up correctly. time: {} + dur: {} is {}; but should be {}",
        time,
        dur,
        time + dur,
        sum
    );
}
#[test]
fn add_duration_to_earliest_latest() {
    {
        let earliest = DateTime::Earliest;
        let dur = Duration::new("50:00");
        assert!(earliest + dur == DateTime::Earliest, "Duration does not sum up correctly. time: {} + dur: {} is {}; but should be Time::Earliest", earliest, dur, earliest + dur);
    }
    {
        let latest = DateTime::Latest;
        let dur = Duration::new("50:00");
        assert!(latest + dur == DateTime::Latest, "Duration does not sum up correctly. time: {} + dur: {} is {}; but should be Time::Latest", latest, dur, latest + dur);
    }
}
#[test]
fn add_infinity_to_time() {
    {
        let time = DateTime::new("1-01-01T00:00");
        let dur = Duration::Infinity;
        assert!(time + dur == DateTime::Latest, "Duration does not sum up correctly. time: {} + dur: {} is {}; but should be Time::Latest", time, dur, time + dur);
    }
    {
        let earliest = DateTime::Earliest;
        let dur = Duration::Infinity;
        assert!(earliest + dur == DateTime::Latest, "Duration does not sum up correctly. time: {} + dur: {} is {}; but should be Time::Earliest", earliest, dur, earliest + dur);
    }
}

#[test]
fn test_difference_of_two_times() {
    {
        let earlier = DateTime::new("2022-02-06T16:32:45");
        let later = DateTime::new("2022-02-06T16:32:45");
        let duration = Duration::new("0:00:00");
        assert!(
            later - earlier == duration,
            "Subtracting {} from {} gives {} but should give {}",
            earlier,
            later,
            later - earlier,
            duration
        );
        assert!(
            earlier + (later - earlier) == later,
            "Adding (later - earlier) to earlier should give later; earlier: {}, later: {}",
            earlier,
            later
        );
    }
    {
        let earlier = DateTime::new("2022-02-06T16:32:45");
        let later = DateTime::new("2022-02-06T17:32:44");
        let duration = Duration::new("0:59:59");
        assert!(
            later - earlier == duration,
            "Subtracting {} from {} gives {} but should give {}",
            earlier,
            later,
            later - earlier,
            duration
        );
        assert!(
            earlier + (later - earlier) == later,
            "Adding (later - earlier) to earlier should give later; earlier: {}, later: {}",
            earlier,
            later
        );
    }
    {
        let earlier = DateTime::new("1989-10-01T02:25");
        let later = DateTime::new("2022-02-06T17:31");
        let duration = Duration::new("283599:06:00");
        assert!(
            later - earlier == duration,
            "Subtracting {} from {} gives {} but should give {}",
            earlier,
            later,
            later - earlier,
            duration
        );
        assert!(
            earlier + (later - earlier) == later,
            "Adding (later - earlier) to earlier should give later; earlier: {}, later: {}",
            earlier,
            later
        );
    }
    {
        let earlier = DateTime::new("2000-01-01T23:59:59");
        let later = DateTime::new("2000-01-02T00:00:00");
        let duration = Duration::new("0:00:01");
        assert!(
            later - earlier == duration,
            "Subtracting {} from {} gives {} but should give {}",
            earlier,
            later,
            later - earlier,
            duration
        );
        assert!(
            earlier + (later - earlier) == later,
            "Adding (later - earlier) to earlier should give later; earlier: {}, later: {}",
            earlier,
            later
        );
    }
}

#[test]
fn test_difference_of_latest_and_earliest() {
    {
        let earliest = DateTime::Earliest;
        let later = DateTime::new("2022-02-06T17:31");
        let duration = Duration::Infinity;
        assert!(
            later - earliest == duration,
            "Subtracting {} from {} gives {} but should give {}",
            earliest,
            later,
            later - earliest,
            duration
        );
    }
    {
        let earlier = DateTime::new("2022-02-06T16:32");
        let latest = DateTime::Latest;
        let duration = Duration::Infinity;
        assert!(
            latest - earlier == duration,
            "Subtracting {} from {} gives {} but should give {}",
            earlier,
            latest,
            latest - earlier,
            duration
        );
    }
    {
        let earliest = DateTime::Earliest;
        let latest = DateTime::Latest;
        let duration = Duration::Infinity;
        assert!(
            latest - earliest == duration,
            "Subtracting {} from {} gives {} but should give {}",
            earliest,
            latest,
            latest - earliest,
            duration
        );
        assert!(
            earliest + (latest - earliest) == latest,
            "Adding (later - earlier) to earlier should give later; earlier: {}, later: {}",
            earliest,
            latest
        );
    }
}

#[test]
fn test_subtracting_duration_from_time() {
    {
        let later = DateTime::new("2022-02-06T16:32");
        let duration = Duration::new("0:00:00");
        let earlier = DateTime::new("2022-02-06T16:32");
        assert!(
            later - duration == earlier,
            "Subtracting {} from {} gives {} but should give {}",
            duration,
            later,
            later - duration,
            earlier
        );
        assert!(
            later - (later - earlier) == earlier,
            "Subtracting (later - earlier) from later should give earlier; earlier: {}, later: {}",
            earlier,
            later
        );
    }
    {
        let later = DateTime::new("2022-02-06T17:31:10");
        let duration = Duration::new("0:59:59");
        let earlier = DateTime::new("2022-02-06T16:31:11");
        assert!(
            later - duration == earlier,
            "Subtracting {} from {} gives {} but should give {}",
            duration,
            later,
            later - duration,
            earlier
        );
        assert!(
            later - (later - earlier) == earlier,
            "Subtracting (later - earlier) from later should give earlier; earlier: {}, later: {}",
            earlier,
            later
        );
    }
    {
        let later = DateTime::new("2022-02-06T17:31:00");
        let duration = Duration::new("283599:06:01");
        let earlier = DateTime::new("1989-10-01T02:24:59");
        assert!(
            later - duration == earlier,
            "Subtracting {} from {} gives {} but should give {}",
            duration,
            later,
            later - duration,
            earlier,
        );
        assert!(
            later - (later - earlier) == earlier,
            "Subtracting (later - earlier) from later should give earlier; earlier: {}, later: {}",
            earlier,
            later
        );
    }
}
