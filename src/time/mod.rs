//! This module contains the implementation of the [`DateTime`] and [`Duration`] types, which are
//! useful to model times in combinatorial optimization problems.
//! * The smallest unit is a second.
//! * In addtion to actual times, [`DateTime::Earliest`] and [`DateTime::Latest`] represents plus and
//! minus infinity, respectively.
//! * Besides finite durations, [`Duration::Infinity`] represents an infinite duration.
//! * [`Durations`][`Duration`] can be added or subtracted from [`DateTimes`][`DateTime`].
//! * [`Durations`][`Duration`] can be added or subtracted from each other and implement
//! [`Sum`][`std::iter::Sum`].
//! * [`DateTimes`][`DateTime`] can be subtracted from each other to produce a [`Duration`].
//! * [`Durations`][`Duration`] and [`DateTimes`][`DateTime`] are total ordered. (They implement
//! [`Ord`].)
//! * No negative durations are allowed.
//! * Both types are [`Copy`] and [`Clone`].
//!
//! # Usage
//!
//! * Basic Usage
//! ```rust
//! use rapid_solve::time::{DateTime, Duration};
//! let tour_start = DateTime::new("2024-02-28T08:00:00");
//! let tour_length = Duration::new("100:00:00");
//! let tour_end = DateTime::new("2024-03-03T12:00:00");
//! assert_eq!(tour_start + tour_length, tour_end);
//! assert_eq!(tour_end - tour_start, tour_length);
//! assert_eq!(tour_end - tour_length, tour_start);
//! // Note that 2024 is a leap year.
//! ```
//!
//! * [`Earliest`][`DateTime::Earliest`], [`Latest`][`DateTime::Latest`], and [`Infinity`][`Duration::Infinity`]
//! ```rust
//! # use rapid_solve::time::{DateTime, Duration};
//! assert_eq!(DateTime::Earliest + Duration::new("10000:00:00"), DateTime::Earliest);
//! assert_eq!(DateTime::new("0000-01-01T00:00:00") + Duration::Infinity, DateTime::Latest);
//! assert_eq!(DateTime::Latest - DateTime::Earliest, Duration::Infinity);
//! assert_eq!(DateTime::Earliest + Duration::Infinity, DateTime::Latest);
//! ```
//!
//! * More [`Duration`]
//! ```rust
//! # use rapid_solve::time::{DateTime, Duration};
//! assert_eq!(Duration::new("1:00:00") + Duration::from_seconds(120), Duration::new("1:02:00"));
//! assert_eq!(Duration::new("100:00:00").in_sec().unwrap(), 100 * 3600);
//! assert_eq!(Duration::from_iso("P10DT2H00M59S").in_min().unwrap(), 10 * 24 * 60 + 2 * 60);
//! // Duration::from_seconds(10) - Duration::from_seconds(20); // panics
//! ```
//!
//! * More [`DateTime`]
//! ```rust
//! # use rapid_solve::time::{DateTime, Duration};
//! assert_eq!(DateTime::new("2024-02-28T08:30").as_iso(), "2024-02-28T08:30:00");
//! // DateTime::new("2024-01-01T08:00") - DateTime::new("2024-01-01T09:00"); // panics
//! ```
//!
mod converters;
pub mod date_time;
pub mod duration;

pub use date_time::DateTime;
pub use duration::Duration;

#[cfg(test)]
mod tests;
