//! Contains the [`EvaluatedSolution`] struct, which is a solution equipped with an
//! [`ObjectiveValue`].

use super::ObjectiveValue;

/// A solution-wrapper that equips a solution with an
/// [`ObjectiveValue`]. This is the result of
/// [evaluating][super::Objective::evaluate] a solution with an [`Objective`][super::Objective].
#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct EvaluatedSolution<S> {
    objective_value: ObjectiveValue,
    solution: S,
}

impl<S> EvaluatedSolution<S> {
    /// Create a [`EvaluatedSolution`]. Usually done by the
    /// [`evaluate`][`super::Objective::evaluate`] method of an
    /// [`Objective`][`super::Objective`] instance.
    pub fn new(solution: S, objective_value: ObjectiveValue) -> EvaluatedSolution<S> {
        EvaluatedSolution {
            solution,
            objective_value,
        }
    }

    /// Get a reference to the solution.
    pub fn solution(&self) -> &S {
        &self.solution
    }

    /// Get a reference to the [`ObjectiveValue`].
    pub fn objective_value(&self) -> &ObjectiveValue {
        &self.objective_value
    }
}
