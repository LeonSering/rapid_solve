use std::fmt;

use super::{base_value::BaseValue, coefficient::Coefficient, indicator::Indicator};

/// A linear combination of indicators. Forms a level of the objective.
pub struct LinearCombination<S> {
    // valueType must be multiplyable with Coefficient
    summands: Vec<(Coefficient, Box<dyn Indicator<S>>)>,
}

impl<S> LinearCombination<S> {
    pub fn evaluate(&self, solution: &S) -> BaseValue {
        self.summands
            .iter()
            .map(|(coefficient, indicator)| coefficient * indicator.evaluate(solution))
            .sum()
    }

    pub fn new(summands: Vec<(Coefficient, Box<dyn Indicator<S>>)>) -> LinearCombination<S> {
        LinearCombination { summands }
    }
}

impl<S> fmt::Display for LinearCombination<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.summands
                .iter()
                .map(|(coefficient, indicator)| {
                    if coefficient.is_one() {
                        indicator.name().to_string()
                    } else {
                        format!("{}*{}", coefficient, indicator.name())
                    }
                })
                .collect::<Vec<String>>()
                .join(" + ")
        )
    }
}
