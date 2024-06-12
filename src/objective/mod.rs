//! In this module, the hierarchical [`Objective`] of an optimization problem is defined.
//! * The objective is constant throughout the optimization and consists of several levels of
//! [`LinearCombinations`][`LinearCombination`] of [`Indicators`][`Indicator`] (each multiplied
//! with a [`Coefficient`]).
//! * With an [`Objective`] instance, each solution instance can be evaluated, which equips the
//! solution with an [`ObjectiveValue`] (a vector of [`BaseValues`][`BaseValue`], one per level) by wrapping
//! it into an [`EvaluatedSolution`].

pub mod base_value;
pub mod coefficient;
pub mod evaluated_solution;
pub mod indicator;
pub mod linear_combination;
pub mod objective_value;
#[cfg(test)]
mod tests;

pub use base_value::BaseValue;
pub use coefficient::Coefficient;
pub use evaluated_solution::EvaluatedSolution;
pub use indicator::Indicator;
pub use linear_combination::LinearCombination;
pub use objective_value::ObjectiveValue;

/// Defines the objective of an optimization problem, which is constant throughout the
/// optimization. Afterwards an objective instance can be used to evaluate every solution object.
///
/// It is a hierarchical objective, i.e., it consists of several levels of
/// [`LinearCombinations`][`LinearCombination`] of [`Indicators`][`Indicator`].
/// The objective is to be minimized with the most important level being the first entry of the
/// vector.
/// A solution is evaluated by using the [`evaluate`][`Objective::evaluate`] method, which consumes the solution, computes its
/// [`ObjectiveValue`] and returns both as [`EvaluatedSolution`].
///
/// `S`: the solution type for which the objective is defined.
pub struct Objective<S> {
    hierarchy_levels: Vec<LinearCombination<S>>,
}

// methods
impl<S> Objective<S> {
    /// Consumes the solution, computes its [`ObjectiveValue`], and returns both as [`EvaluatedSolution`].
    pub fn evaluate(&self, solution: S) -> EvaluatedSolution<S> {
        let objective_value_hierarchy: Vec<BaseValue> = self
            .hierarchy_levels
            .iter()
            .map(|level| level.evaluate(&solution))
            .collect();

        EvaluatedSolution::new(solution, ObjectiveValue::new(objective_value_hierarchy))
    }

    /// Returns the zero [`ObjectiveValue`] ([`BaseValue::Zero`] on each level).
    pub fn zero(&self) -> ObjectiveValue {
        ObjectiveValue::new(vec![BaseValue::Zero; self.hierarchy_levels.len()])
    }

    /// Returns the maximum [`ObjectiveValue`] ([`BaseValue::Maximum`] on each level).
    pub fn maximum(&self) -> ObjectiveValue {
        ObjectiveValue::new(vec![BaseValue::Maximum; self.hierarchy_levels.len()])
    }

    /// Prints the [`ObjectiveValue`].
    pub fn print_objective_value(&self, objective_value: &ObjectiveValue) {
        for (level, value) in self.hierarchy_levels.iter().zip(objective_value.iter()) {
            println!(" * {}: {}", level, value);
        }
    }

    /// Prints the [`ObjectiveValue`] with a comparison to another [`ObjectiveValue`].
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

    /// Converts an [`ObjectiveValue`] to a JSON object (using [`serde_json`]).
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
    /// Creates a new [`Objective`] with the given [`LinearCombinations`][`LinearCombination`] as hierarchy levels.
    /// The most important level is the first entry of the vector.
    pub fn new(hierarchy_levels: Vec<LinearCombination<S>>) -> Objective<S> {
        Objective { hierarchy_levels }
    }

    /// Creates a new [`Objective`] with a single [`LinearCombination`] as the only hierarchy level.
    pub fn new_single_level(linear_combination: LinearCombination<S>) -> Objective<S> {
        Objective::new(vec![linear_combination])
    }

    /// Creates a new [`Objective`] with a single [`Indicator`] as the only hierarchy level.
    pub fn new_single_indicator(indicator: Box<dyn Indicator<S>>) -> Objective<S> {
        Objective::new_single_level(LinearCombination::new(vec![(
            Coefficient::from(1),
            indicator,
        )]))
    }

    /// Creates a new [`Objective`] with a single [`Indicator`] per hierarchy level.
    /// The most important level is the first entry of the vector.
    pub fn new_single_indicator_per_level(indicators: Vec<Box<dyn Indicator<S>>>) -> Objective<S> {
        Objective::new(
            indicators
                .into_iter()
                .map(|indicator| LinearCombination::new(vec![(Coefficient::from(1), indicator)]))
                .collect(),
        )
    }
}
