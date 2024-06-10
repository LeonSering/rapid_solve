use super::base_value::BaseValue;

/// An atomic aspect of the solution. E.g., "total distance" or "number of tours".
pub trait Indicator<S>: Send + Sync {
    fn evaluate(&self, solution: &S) -> BaseValue;
    fn name(&self) -> String;
}
