//! This module contains the [`Duration`] type and the [`DurationLength`] struct.

use std::fmt;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Sub;

use super::converters::from_d_hh_mm_ss_to_seconds;
use super::converters::from_h_mm_ss_to_seconds;
use super::converters::from_seconds_to_h_mm_ss;

/// Represents a duration of time.
/// * In addition to a finite duration, it can also represent an infinite duration. (E.g. if you
/// subtract [`DateTime::Latest`][`super::date_time::DateTime::Latest`] from some other DateTime, you get [`Duration::Infinity`].)
/// * The smallest unit of time is a second.
/// * Can be added or subtracted from each other.
/// * Can be added or subtracted from [`DateTimes`][`super::date_time::DateTime`].
/// * Negative durations are not allowed. (E.g. you cannot subtract a longer duration from a
/// shorter duration.)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)] // care the ordering of the variants is important
pub enum Duration {
    /// A time duration of finite length.
    Length(DurationLength),
    /// An infinite time duration. (Longer than all other [`Durations`][`Duration`].)
    Infinity,
}

/// An finite duration of time.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DurationLength {
    pub(super) seconds: u64,
}

////////////////////////////////////////////////////////////////////
////////////////////////// Duration ////////////////////////////////
////////////////////////////////////////////////////////////////////

impl Duration {
    /// Returns the duration in minutes (rounded down).
    pub fn in_min(&self) -> Result<u64, &str> {
        match self {
            Duration::Length(l) => Ok(l.seconds / 60),
            Duration::Infinity => Err("Cannot get minutes of Duration::Infinity."),
        }
    }

    /// Returns the duration in seconds.
    pub fn in_sec(&self) -> Result<u64, &str> {
        match self {
            Duration::Length(l) => Ok(l.seconds),
            Duration::Infinity => Err("Cannot get seconds of Duration::Infinity."),
        }
    }
}

impl Duration {
    /// The zero duration.
    pub const ZERO: Duration = Duration::Length(DurationLength { seconds: 0 });

    /// Creates a new [`Duration`] from a string. The string must be in the format "hh:mm" or
    /// "hh:mm:ss".
    pub fn new(string: &str) -> Duration {
        // "hh:mm" or "hh:mm:ss"
        let splitted: Vec<&str> = string.split(&[':'][..]).collect();
        assert!(
            splitted.len() <= 3 && splitted.len() >= 2,
            "Wrong duration format! string: {}",
            string
        );

        let hours: u64 = splitted[0].parse().expect("Error at hour.");
        let minutes: u8 = splitted[1].parse().expect("Error at minute.");
        let seconds: u8 = if splitted.len() == 2 {
            0
        } else {
            splitted[2].parse().expect("Error at second.")
        };
        assert!(minutes < 60, "Wrong minute format.");
        assert!(seconds < 60, "Wrong seconds format.");

        Duration::Length(DurationLength {
            seconds: from_h_mm_ss_to_seconds(hours, minutes, seconds),
        })
    }

    /// Creates a new [`Duration`] from a number of seconds.
    pub fn from_seconds(seconds: u64) -> Duration {
        Duration::Length(DurationLength { seconds })
    }

    /// Creates a new [`Duration`] from an ISO 8601 string. The string must be in the format
    /// "P10DT0H31M02S".
    pub fn from_iso(string: &str) -> Duration {
        //"P10DT0H31M02S"
        let splitted: Vec<&str> = string
            .split_inclusive(&['P', 'D', 'T', 'H', 'M', 'S'][..])
            .collect();
        assert!(
            splitted.len() <= 7,
            "Wrong duration format! string: {}",
            string
        );

        let mut days: u64 = 0;
        let mut hours: u8 = 0;
        let mut minutes: u8 = 0;
        let mut seconds: u8 = 0;

        for &s in splitted.iter() {
            match s.chars().last().unwrap() {
                'D' => days = s.replace('D', "").parse().expect("Error at days."),
                'H' => hours = s.replace('H', "").parse().expect("Error at hours."),
                'M' => minutes = s.replace('M', "").parse().expect("Error at minutes."),
                'S' => seconds = s.replace('S', "").parse().expect("Error at seconds."),
                _ => {}
            }
        }

        assert!(seconds < 60, "Wrong seconds format.");
        assert!(minutes < 60, "Wrong minute format.");
        assert!(hours < 24, "Wrong hours format.");

        Duration::Length(DurationLength {
            seconds: from_d_hh_mm_ss_to_seconds(days, hours, minutes, seconds),
        })
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Duration::Infinity => Duration::Infinity,
            Duration::Length(l1) => match other {
                Duration::Infinity => Duration::Infinity,
                Duration::Length(l2) => Duration::Length(l1 + l2),
            },
        }
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert!(
            self >= other,
            "Cannot subtract a longer duration ({}) from a shorter duration ({}).",
            other,
            self
        );
        match self {
            Duration::Infinity => Duration::Infinity,
            Duration::Length(l1) => match other {
                Duration::Infinity => panic!("Cannot subtract Infinity"),
                Duration::Length(l2) => Duration::Length(l1 - l2),
            },
        }
    }
}

impl Sum for Duration {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Duration::ZERO, |a, b| a + b)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Duration::Length(l) => {
                let (hours, minutes, seconds) = from_seconds_to_h_mm_ss(l.seconds);
                if seconds > 0 {
                    write!(f, "{:02}:{:02}:{:02}h", hours, minutes, seconds)
                } else {
                    write!(f, "{:02}:{:02}h", hours, minutes)
                }
            }
            Duration::Infinity => write!(f, "Inf"),
        }
    }
}

////////////////////////////////////////////////////////////////////
/////////////////////// DurationLength /////////////////////////////
////////////////////////////////////////////////////////////////////

impl Add for DurationLength {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        DurationLength {
            seconds: self.seconds + other.seconds,
        }
    }
}

impl Sub for DurationLength {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert!(
            self >= other,
            "Cannot subtract a longer duration from a shorter duration."
        );
        DurationLength {
            seconds: self.seconds - other.seconds,
        }
    }
}
