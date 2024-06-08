pub(crate) fn from_seconds_to_h_mm_ss(seconds: u64) -> (u64, u8, u8) {
    let hours = seconds / 3600;
    let minutes = ((seconds % 3600) / 60) as u8;
    let seconds = (seconds % 60) as u8;
    (hours, minutes, seconds)
}

pub(crate) fn from_h_mm_ss_to_seconds(hours: u64, minutes: u8, seconds: u8) -> u64 {
    hours * 3600 + (minutes as u64) * 60 + (seconds as u64)
}

pub(crate) fn from_d_hh_mm_ss_to_seconds(days: u64, hours: u8, minutes: u8, seconds: u8) -> u64 {
    days * 24 * 3600 + (hours as u64) * 3600 + (minutes as u64) * 60 + seconds as u64
}

pub(crate) fn from_yyyy_mm_dd_hh_mm_ss_to_days_seconds(
    year: u32,
    month: u8,
    day: u8,
    hours: u8,
    minutes: u8,
    seconds: u8,
) -> (u64, u32) {
    let days = from_yyyy_mm_dd_to_days(year, month, day);
    let seconds = from_h_mm_ss_to_seconds(hours as u64, minutes, seconds);
    (days, seconds as u32)
}

pub(crate) fn from_days_seconds_to_yyyy_mm_dd_hh_mm_ss(
    days: u64,
    seconds: u32,
) -> (u32, u8, u8, u8, u8, u8) {
    let (year, month, day) = from_days_to_yyyy_mm_dd(days);
    let (hours, minutes, seconds) = from_seconds_to_h_mm_ss(seconds as u64);
    (year, month, day, hours as u8, minutes, seconds)
}

pub(crate) fn from_yyyy_mm_dd_to_days(year: u32, month: u8, day: u8) -> u64 {
    let mut days: u64 = 0;
    let mut years_counted: u32 = 0;
    let four_hundreds: u32 = year / 400;

    days += four_hundreds as u64 * 146097; // 400 years have 146097 days
    years_counted += four_hundreds * 400;

    for y in years_counted..year {
        days += get_days_in_year(y) as u64;
    }
    for m in 1..month {
        days += days_of_month(year, m) as u64;
    }
    days + day as u64 - 1 // as 0000-01-01 is day 0
}

pub(crate) fn from_days_to_yyyy_mm_dd(days: u64) -> (u32, u8, u8) {
    let mut year = 0;
    let mut month = 1;
    let mut days_remaining = days;

    while days_remaining >= 146097 {
        days_remaining -= 146097;
        year += 400;
    }

    while days_remaining >= get_days_in_year(year) as u64 {
        days_remaining -= get_days_in_year(year) as u64;
        year += 1;
    }
    while days_remaining >= days_of_month(year, month) as u64 {
        days_remaining -= days_of_month(year, month) as u64;
        month += 1;
    }
    let day = days_remaining as u8 + 1; // as 0000-01-01 is day 0
    (year, month, day)
}

pub(crate) fn days_of_month(year: u32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

pub(crate) fn get_days_in_year(year: u32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

pub(crate) fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
