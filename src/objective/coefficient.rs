//! Contains the [`Coefficient`] enum, which is used by
//! [`LinearCombinations`][`super::LinearCombination`].

use std::{fmt, ops::Mul};

use crate::time::Duration;

use super::base_value::BaseValue;

/// A coefficient of a [`Indicator`][`super::Indicator`]. A coefficents is either an
/// [`Integer`][`Coefficient::Integer`] or a [`Float`][`Coefficient::Float`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Coefficient {
    /// An integer coefficient.
    Integer(i32),
    /// A float coefficient.
    Float(f32),
}

impl Coefficient {
    /// Returns `True` if the coefficient is one.
    pub fn is_one(&self) -> bool {
        match self {
            Coefficient::Integer(i) => *i == 1,
            Coefficient::Float(f) => *f == 1.0,
        }
    }
}

impl From<i32> for Coefficient {
    fn from(i: i32) -> Self {
        Coefficient::Integer(i)
    }
}

impl From<f32> for Coefficient {
    fn from(f: f32) -> Self {
        Coefficient::Float(f)
    }
}

impl Mul<BaseValue> for Coefficient {
    type Output = BaseValue;

    fn mul(self, other: BaseValue) -> BaseValue {
        match self {
            Coefficient::Integer(c) => match other {
                BaseValue::Integer(b) => BaseValue::Integer(c as i64 * b),
                BaseValue::Float(b) => BaseValue::Float(c as f64 * b),
                BaseValue::Duration(b) => match b.in_sec() {
                    Ok(sec) => BaseValue::Duration(Duration::from_seconds(c as u64 * sec)),
                    Err(_) => BaseValue::Duration(Duration::Infinity),
                },
                BaseValue::Maximum => BaseValue::Maximum,
                BaseValue::Zero => BaseValue::Zero,
            },
            Coefficient::Float(c) => match other {
                BaseValue::Integer(b) => BaseValue::Integer((c * b as f32) as i64),
                BaseValue::Float(b) => BaseValue::Float(c as f64 * b),
                BaseValue::Duration(b) => match b.in_sec() {
                    Ok(sec) => BaseValue::Duration(Duration::from_seconds((c * sec as f32) as u64)),
                    Err(_) => BaseValue::Duration(Duration::Infinity),
                },
                BaseValue::Maximum => BaseValue::Maximum,
                BaseValue::Zero => BaseValue::Zero,
            },
        }
    }
}

// impl Mul<ObjBaseValue> for &ObjCoefficient, therefore we can use '*' even for references.
impl Mul<BaseValue> for &Coefficient {
    type Output = BaseValue;
    fn mul(self, other: BaseValue) -> BaseValue {
        (*self).mul(other)
    }
}

impl fmt::Display for Coefficient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Coefficient::Integer(i) => write!(f, "{}", i),
            Coefficient::Float(fl) => write!(f, "{}", fl),
        }
    }
}
