use std::{collections::VecDeque, sync::Arc};

use crate::{
    heuristics::tabu_search::TabuNeighborhood,
    objective::{EvaluatedSolution, Objective},
};

use super::TabuImprover;

pub struct TabuMinimizer<S, T> {
    neighborhood: Arc<dyn TabuNeighborhood<S, T>>,
    objective: Arc<Objective<S>>,
}

impl<S, T> TabuMinimizer<S, T> {
    pub fn new(
        neighborhood: Arc<dyn TabuNeighborhood<S, T>>,
        objective: Arc<Objective<S>>,
    ) -> Self {
        Self {
            neighborhood,
            objective,
        }
    }
}

impl<S, T> TabuImprover<S, T> for TabuMinimizer<S, T> {
    fn improve(
        &self,
        solution: &EvaluatedSolution<S>,
        tabu_list: &VecDeque<T>,
    ) -> Option<(EvaluatedSolution<S>, Vec<T>)> {
        let best_neighbor_with_new_tabus = self
            .neighborhood
            .neighbors_of(solution.solution(), tabu_list)
            .map(|(neighbor, new_tabus)| (self.objective.evaluate(neighbor), new_tabus))
            .min_by(|(s1, _), (s2, _)| {
                s1.objective_value()
                    .partial_cmp(s2.objective_value())
                    .unwrap()
            });
        if best_neighbor_with_new_tabus.is_none() {
            println!("\x1b[31mwarning:\x1b[0m no swap possible.");
        }

        best_neighbor_with_new_tabus
    }
}
