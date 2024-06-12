//! This module contains the [`DateTime`] type and the [`TimePoint`] struct.
use std::fmt;
use std::ops::Add;
use std::ops::Sub;

use super::converters::days_of_month;
use super::converters::from_days_seconds_to_yyyy_mm_dd_hh_mm_ss;
use super::converters::from_yyyy_mm_dd_hh_mm_ss_to_days_seconds;

use super::{duration::DurationLength, Duration};

// Important: Leap year are integrated. But no daylight-saving.

/// Represents a point in time.
/// * The smallest unit is seconds.
/// * Leap years are integrated but no daylight-saving.
/// * In addition to an actual point in time, it can also be [`Earliest`][`DateTime::Earliest`] or [`Latest`][`DateTime::Latest`], which is the
/// smallest and largest element in the Ordering.
/// * A [`Durations`][`Duration`] can be added or subtracted to obtain a new [`DateTime`].
/// * Two [`DateTime`] can be subtracted to obtain a [`Duration`]. (Note that the left operand must
/// be later than the right operand, as negative durations are not allowed.)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)] // care the ordering of the variants is important
pub enum DateTime {
    /// The earliest possible point in time. (Smaller than all other [`DateTimes`][`DateTime`].)
    Earliest,
    /// An actual point in time.
    Point(TimePoint),
    /// The latest possible point in time. (Larger than all other [`DateTimes`][`DateTime`].)
    Latest,
}

/// An actual point in time.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)] // care the ordering of attributes is important
pub struct TimePoint {
    days: u64,    // days since 1.1. in year 0
    seconds: u32, // seconds since midnight
}

impl DateTime {
    /// Creates a new [`DateTime`] from a string. The string must be in the format
    /// "2009-06-15T13:45:13" or "2009-4-15T12:10".
    pub fn new(string: &str) -> DateTime {
        //"2009-06-15T13:45:13" or "2009-4-15T12:10"
        let shortened = string.replace('Z', "");
        let splitted: Vec<&str> = shortened.split(&['T', '-', ' ', ':'][..]).collect();
        let len = splitted.len();
        assert!((5..=6).contains(&len), "Wrong time format.");

        let year: u32 = splitted[0].parse().expect("Error at year.");
        let month: u8 = splitted[1].parse().expect("Error at month.");
        assert!((1..=12).contains(&month), "Wrong month format.");
        let day: u8 = splitted[2].parse().expect("Error at day.");
        assert!(
            day <= days_of_month(year, month) && day >= 1,
            "Wrong day format."
        );
        let hour: u8 = splitted[3].parse().expect("Error at hour.");
        assert!(hour <= 24, "Wrong hour format.");
        let minute: u8 = splitted[4].parse().expect("Error at minute.");
        assert!(minute < 60, "Wrong minute format.");
        let second: u8 = if len == 6 {
            splitted[5].parse().expect("Error at second.")
        } else {
            0
        };
        let (days, seconds) =
            from_yyyy_mm_dd_hh_mm_ss_to_days_seconds(year, month, day, hour, minute, second);

        DateTime::Point(TimePoint { days, seconds })
    }
}

impl DateTime {
    /// Returns the [`DateTime`] as a string in the format "2009-06-15T13:45:13".
    pub fn as_iso(&self) -> String {
        match self {
            DateTime::Earliest => String::from("EARLIEST"),
            DateTime::Point(t) => {
                let (year, month, day, hour, minute, second) =
                    from_days_seconds_to_yyyy_mm_dd_hh_mm_ss(t.days, t.seconds);
                format!(
                    "{:#04}-{:#02}-{:#02}T{:#02}:{:#02}:{:#02}",
                    year, month, day, hour, minute, second
                )
            }
            DateTime::Latest => String::from("LATEST"),
        }
    }
}

impl Add<Duration> for DateTime {
    type Output = Self;

    fn add(self, other: Duration) -> Self {
        match other {
            Duration::Infinity => DateTime::Latest, //note that Earliest + Infinity = Latest
            Duration::Length(l) => match self {
                DateTime::Earliest => DateTime::Earliest,
                DateTime::Point(t) => DateTime::Point(t + l),
                DateTime::Latest => DateTime::Latest,
            },
        }
    }
}

impl Sub<Duration> for DateTime {
    type Output = DateTime;

    fn sub(self, other: Duration) -> DateTime {
        match self {
            DateTime::Earliest => DateTime::Earliest,
            DateTime::Latest => {
                if other == Duration::Infinity {
                    panic!("Cannot subtract Infinity from Latest");
                } else {
                    DateTime::Latest
                }
            }
            DateTime::Point(t) => match other {
                Duration::Infinity => DateTime::Earliest,
                Duration::Length(d) => DateTime::Point(t - d),
            },
        }
    }
}

impl Sub for DateTime {
    type Output = Duration;

    fn sub(self, other: Self) -> Duration {
        assert!(other <= self, "Cannot subtract {} from {}, as it is a later point in time (no negative durations allowed)", other, self);
        match self {
            DateTime::Earliest => {
                Duration::Length(DurationLength { seconds: 0 }) // Earliest - Earliest
            }
            DateTime::Latest => {
                if other == DateTime::Latest {
                    Duration::Length(DurationLength { seconds: 0 }) // Latest - Latest
                } else {
                    Duration::Infinity // Latest - (something not Latest)
                }
            }
            DateTime::Point(l1) => {
                match other {
                    DateTime::Earliest => Duration::Infinity, // Length - Earliest
                    DateTime::Point(l2) => l1 - l2,
                    _ => panic!("This should never be reached"),
                }
            }
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateTime::Earliest => write!(f, "Earliest"),
            DateTime::Point(t) => write!(f, "{}", t),
            DateTime::Latest => write!(f, "Latest"),
        }
    }
}

////////////////////////////////////////////////////////////////////
////////////////////////// TimePoint ///////////////////////////////
////////////////////////////////////////////////////////////////////

impl Sub for TimePoint {
    type Output = Duration;

    fn sub(self, other: Self) -> Duration {
        let self_seconds = self.days * 86400 + self.seconds as u64;
        let other_seconds = other.days * 86400 + other.seconds as u64;
        assert!(self_seconds >= other_seconds, "Cannot subtract {} from {}, as it is a later point in time (no negative durations allowed)", other, self);

        Duration::from_seconds(self_seconds - other_seconds)
    }
}

impl Add<DurationLength> for TimePoint {
    type Output = Self;

    fn add(self, other: DurationLength) -> Self {
        let seconds = self.seconds as u64 + other.seconds;
        let days = self.days + (seconds / 86400);

        TimePoint {
            days,
            seconds: (seconds % 86400) as u32,
        }
    }
}

impl Sub<DurationLength> for TimePoint {
    type Output = TimePoint;

    fn sub(self, other: DurationLength) -> Self {
        let self_seconds = self.days * 86400 + self.seconds as u64;
        assert!(
            self_seconds >= other.seconds,
            "Cannot subtract {} from {}, as this would be before the date 1.1. in year 0)",
            Duration::from_seconds(other.seconds),
            self
        );
        let seconds = self_seconds - other.seconds;
        TimePoint {
            days: seconds / 86400,
            seconds: (seconds % 86400) as u32,
        }
    }
}

impl fmt::Display for TimePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (year, month, day, hour, minute, second) =
            from_days_seconds_to_yyyy_mm_dd_hh_mm_ss(self.days, self.seconds);
        if second > 0 {
            write!(
                f,
                "{:02}.{:02}.{}_{:02}:{:02}:{:02}",
                day, month, year, hour, minute, second
            )
        } else {
            write!(
                f,
                "{:02}.{:02}.{}_{:02}:{:02}",
                day, month, year, hour, minute
            )
        }
    }
}
