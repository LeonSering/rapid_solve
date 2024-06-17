//! Contains the [`ObjectiveValue`] struct, which represents the hierarchical objective value of a
//! solution.
use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
    slice::Iter,
};

use super::{base_value::BaseValue, Coefficient};

// TODO: Implement Copy
/// The hierarchical objective value of a solution, which is a vector of
/// [`BaseValues`][`BaseValue`].
#[derive(Clone, Debug)]
pub struct ObjectiveValue {
    objective_vector: Vec<BaseValue>,
}

impl ObjectiveValue {
    /// Creates a new objective value. This is usally done by the [evaluate][super::Objective::evaluate] method of an Objective.
    pub fn new(objective_vector: Vec<BaseValue>) -> ObjectiveValue {
        ObjectiveValue { objective_vector }
    }

    /// Returns the entries of the objective vector.
    pub fn iter(&self) -> Iter<BaseValue> {
        self.objective_vector.iter()
    }

    /// Returns the entries of the objective vector as a vector.
    pub fn as_vec(&self) -> &Vec<BaseValue> {
        &self.objective_vector
    }
}

impl Ord for ObjectiveValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.objective_vector
            .iter()
            .zip(other.objective_vector.iter())
            .fold(Ordering::Equal, |acc, (value, other_value)| {
                acc.then_with(|| value.partial_cmp(other_value).unwrap())
            })
    }
}

impl PartialOrd for ObjectiveValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ObjectiveValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for ObjectiveValue {}

impl Add for ObjectiveValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ObjectiveValue::new(
            self.objective_vector
                .into_iter()
                .zip(rhs.objective_vector)
                .map(|(value, other_value)| value + other_value)
                .collect(),
        )
    }
}

impl Sub for ObjectiveValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ObjectiveValue::new(
            self.objective_vector
                .into_iter()
                .zip(rhs.objective_vector)
                .map(|(value, other_value)| value - other_value)
                .collect(),
        )
    }
}

impl Mul<f32> for ObjectiveValue {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let coefficient = Coefficient::Float(rhs);
        ObjectiveValue::new(
            self.objective_vector
                .into_iter()
                .map(|value| coefficient * value)
                .collect(),
        )
    }
}

impl Mul<i32> for ObjectiveValue {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        let coefficient = Coefficient::Integer(rhs);
        ObjectiveValue::new(
            self.objective_vector
                .into_iter()
                .map(|value| coefficient * value)
                .collect(),
        )
    }
}
