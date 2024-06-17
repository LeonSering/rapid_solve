// TODO: Doc strings
use std::sync::Arc;

use crate::{
    examples::tsp::{objective::build_tsp_objective, tsp_instance::TspInstance, tsp_tour::TspTour},
    heuristics::threshold_accepting::ThresholdAcceptingSolver,
    objective::{BaseValue, Objective, ObjectiveValue},
};

use super::neighborhood::ThreeOptNeighborhood;

pub fn build(tsp_instance: Arc<TspInstance>) -> ThresholdAcceptingSolver<TspTour> {
    let node_count = tsp_instance.get_number_of_nodes();
    let max_distance = (0..node_count)
        .flat_map(|i| (0..node_count).filter_map(move |j| if i != j { Some((i, j)) } else { None }))
        .map(|(i, j)| tsp_instance.get_distance(i, j))
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();
    let objective: Arc<Objective<TspTour>> = Arc::new(build_tsp_objective());
    let neighborhood = Arc::new(ThreeOptNeighborhood::new(tsp_instance));
    let initial_threshold = ObjectiveValue::new(vec![BaseValue::Float(max_distance)]);
    ThresholdAcceptingSolver::initialize(neighborhood, objective, initial_threshold, 0.9)
}
