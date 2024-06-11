use super::ObjectiveValue;

/// A solution that has been evaluated by an [`Objective`](crate::objective::Objective).
#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct EvaluatedSolution<S> {
    objective_value: ObjectiveValue,
    solution: S,
}

impl<S> EvaluatedSolution<S> {
    /// Create a [`EvaluatedSolution`]. Usually done by the
    /// [`evaluate`](crate::objective::Objective::evaluate) method of an
    /// [`Objective`](crate::objective::Objective) instance.
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
