//! Contains the [`Indicator`] trait, which is used to evaluate a specific quality of a solution.
use super::base_value::BaseValue;

/// An atomic quality of the solution. E.g., `total_distance` or `number_of_tours`.
pub trait Indicator<S>: Send + Sync {
    /// Evaluates the provided solution according to the desired quality.
    /// This defines the [`Indicator`].
    fn evaluate(&self, solution: &S) -> BaseValue;

    /// Returns the name of the indicator, which is used to display an
    /// [`ObjectiveValue`][`super::objective_value::ObjectiveValue`].
    fn name(&self) -> String;
}
