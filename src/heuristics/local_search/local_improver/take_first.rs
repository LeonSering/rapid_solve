//! [`TakeFirst`] takes the first improving neighbor according to the order of the neighborhood
//! iterator.
use super::super::Neighborhood;
use super::LocalImprover;
use crate::objective::EvaluatedSolution;
use crate::objective::Objective;
use std::sync::Arc;

/// Takes the first improving neighbor according to the order of the neighborhood iterator.
/// * No parallelism is used.
/// * Works for every solution type `S`.
/// * Is fast if the computation and the evaluating of a neighbor is cheap.
/// * Each step is faster than the [`Minimizer`][super::Minimizer], but it might take more steps until a local optimum is
/// reached.
/// * Works best with 'smart' [`Neighborhoods`][`Neighborhood`], e.g., if the next neighborhood iterator continues at
/// the swaps of the last neighborhood iterator.
pub struct TakeFirst<S> {
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
}

impl<S> TakeFirst<S> {
    /// Creates a new [`TakeFirst`] with the given [`Neighborhood`] and [`Objective`].
    pub fn new(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
    ) -> TakeFirst<S> {
        TakeFirst {
            neighborhood,
            objective,
        }
    }
}

impl<S> LocalImprover<S> for TakeFirst<S> {
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>> {
        self.neighborhood
            .neighbors_of(solution.solution())
            .map(|neighbor| self.objective.evaluate(neighbor))
            .find(|neighbor| neighbor.objective_value() < solution.objective_value())
    }
}
