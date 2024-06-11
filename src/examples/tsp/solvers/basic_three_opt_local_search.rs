//! For a basic [`LocalSearchSolver`], we need to define an [`Objective`] and for every tour the
//! [`Neighborhood`][`crate::heuristics::local_search::neighborhood::Neighborhood`].
//! This solver searches in each step the whole neighborhood of the current [`TspTour`] (in parallel) and
//! picks the best neighbor.
use super::super::{objective::build_tsp_objective, tsp_instance::TspInstance, tsp_tour::TspTour};
use super::neighborhood::ThreeOptNeighborhood;
use crate::{heuristics::local_search::LocalSearchSolver, objective::Objective};
use std::sync::Arc;

/// Builds a local search solver with the default
/// [`LocalImprover`](crate::heuristics::local_search::local_improver::LocalImprover)
/// [`Minimizer`](crate::heuristics::local_search::local_improver::minimizer::Minimizer).
pub fn build(tsp_instance: Arc<TspInstance>) -> LocalSearchSolver<TspTour> {
    let objective: Arc<Objective<TspTour>> = Arc::new(build_tsp_objective());
    let neighborhood = Arc::new(ThreeOptNeighborhood::new(tsp_instance));
    // let local_improver = Box::new(TakeFirst::new(neighborhood.clone(), objective.clone()));
    LocalSearchSolver::initialize(neighborhood, objective)
}

#[cfg(test)]
mod tests {
    use super::build;
    use crate::examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour};
    use std::sync::Arc;

    #[test]
    fn test_basic_three_opt_local_search() {
        let tsp_instance = Arc::new(TspInstance::new(vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ]));

        let tour = TspTour::new(vec![0, 1, 2, 3], tsp_instance.clone());

        let solver = build(tsp_instance.clone());

        let local_opt_tour = solver.solve(tour);

        assert_eq!(local_opt_tour.solution().get_nodes(), &vec![0, 2, 3, 1]);
    }

    #[test]
    fn test_basic_three_opt_local_search_large_instance() {
        let tsp_instance = Arc::new(
            TspInstance::from_tsplib_file("resources/tsp_test_instances/berlin52.tsp").unwrap(),
        );
        let tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());
        let solver = build(tsp_instance.clone());

        let local_opt_tour = solver.solve(tour);

        assert_eq!(
            local_opt_tour.solution().get_nodes(),
            &vec![
                0, 35, 38, 39, 36, 37, 47, 23, 4, 14, 5, 3, 24, 45, 43, 33, 34, 48, 31, 44, 18, 40,
                7, 8, 9, 42, 32, 50, 11, 27, 26, 12, 13, 51, 10, 25, 46, 28, 15, 49, 19, 22, 29, 1,
                6, 41, 20, 16, 2, 17, 30, 21
            ]
        );
    }
}
