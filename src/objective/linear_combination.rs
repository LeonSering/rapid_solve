//! Contains the [`LinearCombination`] struct, which represents a linear combination of
//! [`Indicators`][`Indicator`].
use std::fmt;

use super::{base_value::BaseValue, coefficient::Coefficient, indicator::Indicator};

/// A linear combination of [`Indicators`][`Indicator`] (each equipped with an [`Coefficient`]). Forms a level of the [`Objective`][`super::Objective`].
pub struct LinearCombination<S> {
    // valueType must be multiplyable with Coefficient
    summands: Vec<(Coefficient, Box<dyn Indicator<S>>)>,
}

impl<S> LinearCombination<S> {
    /// Evaluate the linear combination for a given solution.
    pub fn evaluate(&self, solution: &S) -> BaseValue {
        self.summands
            .iter()
            .map(|(coefficient, indicator)| coefficient * indicator.evaluate(solution))
            .sum()
    }

    /// Creates a new linear combination from a list of summands.
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
