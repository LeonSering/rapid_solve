use super::super::{objective::build_tsp_objective, tsp_instance::TspInstance, tsp_tour::TspTour};
use super::neighborhood::ThreeOptNeighborhood;
use crate::heuristics::local_search::local_improver::TakeAnyParallelRecursion;
use crate::{heuristics::local_search::LocalSearchSolver, objective::Objective};
use std::sync::Arc;

/// In stead of looking at all neighbors and pick the best one, we take the first improving
/// neighbor. As the neighborhood is searched in parallel this solver is not deterministic.
pub fn build(tsp_instance: Arc<TspInstance>) -> LocalSearchSolver<TspTour> {
    let objective: Arc<Objective<TspTour>> = Arc::new(build_tsp_objective());
    let neighborhood = Arc::new(ThreeOptNeighborhood::new(tsp_instance));
    let take_any = Box::new(TakeAnyParallelRecursion::new(
        0,
        Some(0),
        neighborhood.clone(),
        objective.clone(),
    ));
    LocalSearchSolver::with_local_improver_and_function(
        neighborhood,
        objective,
        Some(take_any),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::build;
    use crate::examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour};
    use std::sync::Arc;

    #[test]
    fn test_take_first_three_opt_local_search() {
        let tsp_instance = Arc::new(TspInstance::new(
            4,
            vec![
                vec![0.0, 10.0, 15.0, 20.0],
                vec![10.0, 0.0, 35.0, 25.0],
                vec![15.0, 35.0, 0.0, 30.0],
                vec![20.0, 25.0, 30.0, 0.0],
            ],
        ));

        let tour = TspTour::new(vec![0, 1, 2, 3], tsp_instance.clone());

        let solver = build(tsp_instance.clone());

        solver.solve(tour);
    }

    #[test]
    fn test_take_any_three_opt_local_search_52_nodes() {
        let tsp_instance = Arc::new(
            TspInstance::from_tsplib_file("resources/tsp_test_instances/berlin52.tsp").unwrap(),
        );
        let tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());

        let solver = build(tsp_instance.clone());

        solver.solve(tour);
    }

    #[test] // about xx iteration within 10 minutes
    fn test_take_any_three_opt_local_search_150_nodes() {
        let tsp_instance = Arc::new(
            TspInstance::from_tsplib_file("resources/tsp_test_instances/ch150.tsp").unwrap(),
        );
        let tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());

        let solver = build(tsp_instance.clone());

        solver.solve(tour);
    }
}
