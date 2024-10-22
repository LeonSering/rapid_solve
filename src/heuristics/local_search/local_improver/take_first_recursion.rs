//! [`TakeFirstRecursion`] takes the first improving solution according to the
//! neighborhood iterator. If no improvement is found, it takes the best neighbors into recursion.
use super::super::Neighborhood;
use super::LocalImprover;
use crate::objective::EvaluatedSolution;
use crate::objective::{Objective, ObjectiveValue};
use std::sync::Arc;

/// Takes the first improving solution according to the neighborhood iterator.
/// If no improvement is found, it takes the best neighbors into recursion.
/// * Works the same as [`TakeFirst`][super::take_first::TakeFirst] but with recursion.
/// * Repeats recursion `recursion_depth` often.
/// * Only the best `recursion_width`-many solution are considered for recursion. (Dublicates
/// accordings to the objective value are removed.)
/// * The diversification for recursion is probably low.
/// * As there is no parallelization this improver is fully deterministic.
pub struct TakeFirstRecursion<S> {
    recursion_depth: u8,
    recursion_width: u8,
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
}

impl<S> TakeFirstRecursion<S> {
    /// Creates a new instance of [`TakeFirstRecursion`]. In addition to the [`Neighborhood`]
    /// and the [`Objective`] the following parameters are needed:
    /// * `recursion_depth` is the number of recursions to be done.
    /// * `recursion_width` is the number of solutions to be taken to recursion.
    pub fn new(
        recursion_depth: u8,
        recursion_width: u8,
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
    ) -> TakeFirstRecursion<S> {
        TakeFirstRecursion {
            recursion_depth,
            recursion_width,
            neighborhood,
            objective,
        }
    }
}

impl<S: Clone> LocalImprover<S> for TakeFirstRecursion<S> {
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>> {
        let old_objective_value = solution.objective_value();
        self.improve_recursion(
            vec![solution.clone()],
            old_objective_value,
            self.recursion_depth,
        )
    }
}

impl<S: Clone> TakeFirstRecursion<S> {
    /// Returns the first improving solution in the neighborhood of the given solutions.
    /// If no improvement is found, None is returned.
    fn improve_recursion(
        &self,
        solutions: Vec<EvaluatedSolution<S>>,
        objective_to_beat: &ObjectiveValue,
        remaining_recursion: u8,
    ) -> Option<EvaluatedSolution<S>> {
        let neighboorhood_union = solutions
            .iter()
            .flat_map(|sol| self.neighborhood.neighbors_of(sol.solution()));

        let mut counter = 0;
        let mut solutions_for_recursion: Vec<EvaluatedSolution<S>> = Vec::new();

        let result = neighboorhood_union
            .map(|neighbor| {
                counter += 1;
                self.objective.evaluate(neighbor)
            })
            .find(|neighbor| {
                if remaining_recursion > 0 {
                    solutions_for_recursion.push(neighbor.clone());
                    solutions_for_recursion
                        .sort_unstable_by(|a, b| a.objective_value().cmp(b.objective_value()));
                    solutions_for_recursion
                        .dedup_by(|a, b| a.objective_value() == b.objective_value());
                    let width = (self.recursion_width as usize).min(solutions_for_recursion.len());
                    solutions_for_recursion.truncate(width);
                }
                neighbor.objective_value() < objective_to_beat
            });

        if result.is_none() {
            println!("No improvement found after {} swaps.", counter);

            if remaining_recursion > 0 {
                println!(
                    "Going into recursion. Remaining depth: {}. Schedule-count: {}",
                    remaining_recursion,
                    solutions_for_recursion.len()
                );

                self.improve_recursion(
                    solutions_for_recursion,
                    objective_to_beat,
                    remaining_recursion - 1,
                )
            } else {
                println!("No recursion-depth left.");
                None
            }
        } else {
            println!("Improvement found after {} swaps.", counter);
            result
        }
    }
}
