use super::ObjectiveValue;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct EvaluatedSolution<S> {
    objective_value: ObjectiveValue,
    solution: S,
}

impl<S> EvaluatedSolution<S> {
    pub fn new(solution: S, objective_value: ObjectiveValue) -> EvaluatedSolution<S> {
        EvaluatedSolution {
            solution,
            objective_value,
        }
    }

    pub fn solution(&self) -> &S {
        &self.solution
    }

    pub fn objective_value(&self) -> &ObjectiveValue {
        &self.objective_value
    }
}
