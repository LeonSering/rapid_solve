pub mod base_value;
pub mod coefficient;
pub mod evaluated_solution;
pub mod indicator;
pub mod level;
pub mod objective_value;
#[cfg(test)]
mod tests;

pub use base_value::BaseValue;
pub use coefficient::Coefficient;
pub use evaluated_solution::EvaluatedSolution;
pub use indicator::Indicator;
pub use level::Level;
pub use objective_value::ObjectiveValue;

/// Defines the values of a schedule that form the objective.
/// It is constant throughout optimization.
/// It is a hierarchical objective, i.e., it consists of several levels.
/// Each level consists of a linear combination of indicators.
///
/// The objective is to be minimized with the most important level being the first entry of the
/// vector.
///
/// S: the solution type for which the objective is defined.
pub struct Objective<S> {
    hierarchy_levels: Vec<Level<S>>,
}

// methods
impl<S> Objective<S> {
    /// Consumes solution, computes objective value, and returns both, as EvaluatedSolution.
    pub fn evaluate(&self, solution: S) -> EvaluatedSolution<S> {
        let objective_value_hierarchy: Vec<BaseValue> = self
            .hierarchy_levels
            .iter()
            .map(|level| level.evaluate(&solution))
            .collect();

        EvaluatedSolution::new(solution, ObjectiveValue::new(objective_value_hierarchy))
    }

    pub fn zero(&self) -> ObjectiveValue {
        ObjectiveValue::new(vec![BaseValue::Zero; self.hierarchy_levels.len()])
    }

    pub fn maximum(&self) -> ObjectiveValue {
        ObjectiveValue::new(vec![BaseValue::Maximum; self.hierarchy_levels.len()])
    }

    pub fn print_objective_value(&self, objective_value: &ObjectiveValue) {
        for (level, value) in self.hierarchy_levels.iter().zip(objective_value.iter()) {
            println!(" * {}: {}", level, value);
        }
    }

    pub fn print_objective_value_with_comparison(
        &self,
        objective_value: &ObjectiveValue,
        comparison: &ObjectiveValue,
    ) {
        for ((level, value), comparison_value) in self
            .hierarchy_levels
            .iter()
            .zip(objective_value.iter())
            .zip(comparison.iter())
        {
            println!(
                " * {}: {} {}",
                level,
                value,
                value.print_difference(*comparison_value)
            );
        }
    }

    pub fn objective_value_to_json(&self, objective_value: &ObjectiveValue) -> serde_json::Value {
        let mut json_object = serde_json::json!({});
        for (level, base_value) in self.hierarchy_levels.iter().zip(objective_value.iter()) {
            match base_value {
                BaseValue::Integer(value) => {
                    json_object[level.to_string()] = serde_json::json!(value);
                }
                BaseValue::Float(value) => {
                    json_object[level.to_string()] = serde_json::json!(value);
                }
                BaseValue::Duration(value) => {
                    json_object[level.to_string()] = serde_json::json!(value.to_string());
                }
                BaseValue::Zero => {
                    json_object[level.to_string()] = serde_json::json!("Zero");
                }
                BaseValue::Maximum => {
                    json_object[level.to_string()] = serde_json::json!("Maximum");
                }
            }
        }
        json_object
    }
}

// static
impl<S> Objective<S> {
    pub fn new(hierarchy_levels: Vec<Level<S>>) -> Objective<S> {
        Objective { hierarchy_levels }
    }
}
