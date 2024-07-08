//! Contains the [`BaseValue`] enum, which represents a single scalar value.
use std::{
    cmp::Ordering,
    fmt,
    iter::Sum,
    ops::{Add, Sub},
};

use rapid_time::Duration;

const TOLERANCE: f64 = 0.0001;

/// A single value of an [`Indicator`][super::indicator::Indicator] or [`LinearCombination`][super::linear_combination::LinearCombination]. E.g., count of things, durations, costs.
/// * Supports integers (i64), floats (f64), durations (from the RapidTime crate).
/// * `Maximum` is larger (worse) than all other values.
/// * `Zero` is the neutral element for addition.
#[derive(Debug, Clone, Copy)]
pub enum BaseValue {
    /// An integer value.
    Integer(i64),
    /// A floating point value.
    Float(f64),
    /// A [`Duration`] value (from the RapidTime crate).
    Duration(Duration), // cannot handle negative durations
    /// Represents the maximum value.
    Maximum,
    /// Represents the neutral element for addition.
    Zero,
}

impl BaseValue {
    /// Unwraps [`BaseValue::Integer`].
    /// Panics if other variant.
    pub fn unwrap_integer(self) -> i64 {
        match self {
            BaseValue::Integer(i) => i,
            _ => panic!("Expected BaseValue::Integer, got {:?}", self),
        }
    }

    /// Unwraps [`BaseValue::Float`].
    /// Panics if other variant.
    pub fn unwrap_float(self) -> f64 {
        match self {
            BaseValue::Float(f) => f,
            _ => panic!("Expected BaseValue::Float, got {:?}", self),
        }
    }

    /// Unwraps [`BaseValue::Duration`].
    /// Panics if other variant.
    pub fn unwrap_duration(self) -> Duration {
        match self {
            BaseValue::Duration(d) => d,
            _ => panic!("Expected BaseValue::Duration, got {:?}", self),
        }
    }

    /// Prints the difference between two BaseValuesin green or red depending on the sign.
    pub fn print_difference(self, other: BaseValue) -> String {
        if self == other {
            return String::new();
        }
        match (self, other) {
            (BaseValue::Integer(a), BaseValue::Integer(b)) => {
                BaseValue::print_difference_in_value(a, b)
            }
            (BaseValue::Float(a), BaseValue::Float(b)) => {
                BaseValue::print_difference_in_value(a, b)
            }
            (BaseValue::Duration(a), BaseValue::Duration(b)) => {
                BaseValue::print_difference_in_value(a, b)
            }
            (BaseValue::Maximum, _) => String::new(),
            (_, BaseValue::Maximum) => String::new(),
            (new_value, BaseValue::Zero) => format!("(\x1b[0;31m+{:2.1}\x1b[0m)", new_value),
            (BaseValue::Zero, old_value) => format!("(\x1b[0;32m-{:2.1}\x1b[0m)", old_value),
            _ => panic!("Cannot subtract {:?} and {:?}", self, other),
        }
    }

    fn print_difference_in_value<V>(value: V, value_for_comparison: V) -> String
    where
        V: fmt::Display + PartialOrd + Sub,
        <V as Sub>::Output: fmt::Display,
    {
        if value > value_for_comparison {
            format!("(\x1b[0;31m+{:2.1}\x1b[0m)", value - value_for_comparison)
        } else if value < value_for_comparison {
            format!("(\x1b[0;32m-{:2.1}\x1b[0m)", value_for_comparison - value)
        } else {
            String::new()
        }
    }
}

impl Add for BaseValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (BaseValue::Integer(a), BaseValue::Integer(b)) => BaseValue::Integer(a + b),
            (BaseValue::Float(a), BaseValue::Float(b)) => BaseValue::Float(a + b),
            (BaseValue::Duration(a), BaseValue::Duration(b)) => BaseValue::Duration(a + b),
            (BaseValue::Maximum, _) => BaseValue::Maximum,
            (_, BaseValue::Maximum) => BaseValue::Maximum,
            (BaseValue::Zero, value) => value,
            (value, BaseValue::Zero) => value,
            _ => panic!("Cannot add {:?} and {:?}", self, other),
        }
    }
}

impl Sub for BaseValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (BaseValue::Integer(a), BaseValue::Integer(b)) => BaseValue::Integer(a - b),
            (BaseValue::Float(a), BaseValue::Float(b)) => BaseValue::Float(a - b),
            (BaseValue::Duration(a), BaseValue::Duration(b)) => BaseValue::Duration(a - b),
            (BaseValue::Maximum, _) => BaseValue::Maximum,
            (value, BaseValue::Zero) => value,
            (BaseValue::Zero, BaseValue::Integer(a)) => BaseValue::Integer(-a),
            (BaseValue::Zero, BaseValue::Float(a)) => BaseValue::Float(-a),
            _ => panic!("Cannot sub {:?} and {:?}", self, other),
        }
    }
}

impl Ord for BaseValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (BaseValue::Integer(a), BaseValue::Integer(b)) => a.cmp(b),
            (BaseValue::Float(a), BaseValue::Float(b)) => {
                if a - b > TOLERANCE {
                    Ordering::Greater
                } else if b - a > TOLERANCE {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
            (BaseValue::Duration(a), BaseValue::Duration(b)) => a.cmp(b),
            (BaseValue::Maximum, BaseValue::Maximum) => Ordering::Equal,
            (BaseValue::Zero, BaseValue::Zero) => Ordering::Equal,
            (BaseValue::Maximum, _) => Ordering::Greater,
            (_, BaseValue::Maximum) => Ordering::Less,
            (BaseValue::Zero, BaseValue::Integer(_)) => BaseValue::Integer(0).cmp(other),
            (BaseValue::Zero, BaseValue::Float(_)) => BaseValue::Float(0.0).cmp(other),
            (BaseValue::Zero, BaseValue::Duration(_)) => {
                BaseValue::Duration(Duration::ZERO).cmp(other)
            }
            (BaseValue::Integer(_), BaseValue::Zero) => self.cmp(&BaseValue::Integer(0)),
            (BaseValue::Float(_), BaseValue::Zero) => self.cmp(&BaseValue::Float(0.0)),
            (BaseValue::Duration(_), BaseValue::Zero) => {
                self.cmp(&BaseValue::Duration(Duration::ZERO))
            }
            _ => panic!("Cannot compare {:?} and {:?}", self, other),
        }
    }
}

impl PartialOrd for BaseValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BaseValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for BaseValue {}

impl Sum<Self> for BaseValue {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(BaseValue::Zero, |a, b| a + b)
    }
}

impl fmt::Display for BaseValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaseValue::Integer(i) => write!(f, "{}", i),
            BaseValue::Float(c) => write!(f, "{:0.2}", c),
            BaseValue::Duration(d) => write!(f, "{}", d),
            BaseValue::Maximum => write!(f, "MAX"),
            BaseValue::Zero => write!(f, "0"),
        }
    }
}
