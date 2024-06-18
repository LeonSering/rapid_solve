//! TODO
use std::{collections::VecDeque, sync::Arc};

use crate::{
    examples::tsp::{
        objective::build_tsp_objective, tsp_instance::TspInstance, tsp_tour::TspTour, NodeIdx,
    },
    heuristics::tabu_search::{TabuNeighborhood, TabuSearchSolver},
    objective::Objective,
};

/// A tabu consisits of a directed arc between two nodes.
#[derive(Debug)]
pub struct Tabu {
    start: NodeIdx,
    end: NodeIdx,
}

impl Tabu {
    /// TODO
    pub fn is_tabu(&self, i: usize, j: usize, k: usize, tour: &TspTour) -> bool {
        let n = tour.get_nodes().len();
        (self.start == *tour.get_nodes().get(i).unwrap()
            && self.end == *tour.get_nodes().get(j + 1).unwrap())
            || (self.start == *tour.get_nodes().get(j).unwrap()
                && self.end == *tour.get_nodes().get((k + 1) % n).unwrap())
            || (self.start == *tour.get_nodes().get(k).unwrap()
                && self.end == *tour.get_nodes().get(i + 1).unwrap())
    }

    /// TODO
    pub fn create_tabus(i: usize, j: usize, k: usize, tour: &TspTour) -> Vec<Tabu> {
        let n = tour.get_nodes().len();
        vec![
            Tabu {
                start: *tour.get_nodes().get(i).unwrap(),
                end: *tour.get_nodes().get(i + 1).unwrap(),
            },
            Tabu {
                start: *tour.get_nodes().get(j).unwrap(),
                end: *tour.get_nodes().get(j + 1).unwrap(),
            },
            Tabu {
                start: *tour.get_nodes().get(k).unwrap(),
                end: *tour.get_nodes().get((k + 1) % n).unwrap(),
            },
        ]
    }
}

/// TODO
pub struct ThreeOptTabuNeighborhood {
    tsp_instance: Arc<TspInstance>,
}

impl ThreeOptTabuNeighborhood {
    /// TODO
    pub fn new(tsp_instance: Arc<TspInstance>) -> Self {
        Self { tsp_instance }
    }
}

impl TabuNeighborhood<TspTour, Tabu> for ThreeOptTabuNeighborhood {
    fn neighbors_of<'a>(
        &'a self,
        tour: &'a TspTour,
        tabu_list: &'a VecDeque<Tabu>,
    ) -> Box<dyn Iterator<Item = (TspTour, Vec<Tabu>)> + Send + Sync + 'a> {
        let num_nodes = self.tsp_instance.get_number_of_nodes();
        Box::new(
            (0..num_nodes - 2)
                .flat_map(move |i| {
                    (i + 1..num_nodes - 1)
                        .flat_map(move |j| (j + 1..num_nodes).map(move |k| (i, j, k)))
                })
                .filter_map(move |(i, j, k)| {
                    if tabu_list.iter().any(|tabu| tabu.is_tabu(i, j, k, tour)) {
                        return None;
                    }
                    Some((
                        tour.three_opt_swap(i, j, k),
                        Tabu::create_tabus(i, j, k, tour),
                    ))
                }),
        )
    }
}

/// TODO
pub fn build(tsp_instance: Arc<TspInstance>) -> TabuSearchSolver<TspTour, Tabu> {
    let objective: Arc<Objective<TspTour>> = Arc::new(build_tsp_objective());
    let neighborhood = Arc::new(ThreeOptTabuNeighborhood::new(tsp_instance.clone()));
    let tabu_list_size = 30;
    let iteration_without_global_improvement_limit = 100;
    TabuSearchSolver::initialize(
        neighborhood,
        objective,
        tabu_list_size,
        iteration_without_global_improvement_limit,
    )
}
