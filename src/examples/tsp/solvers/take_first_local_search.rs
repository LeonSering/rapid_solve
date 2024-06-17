//! The local search implementation takes the first improving neighbor, instead of looking at all
//! neighbors, see the [build] function for details.
//! ```rust
//! pub fn build(tsp_instance: Arc<TspInstance>) -> LocalSearchSolver<TspTour> {
//!     let objective: Arc<Objective<TspTour>> = Arc::new(build_tsp_objective());
//!     let neighborhood = Arc::new(ThreeOptNeighborhood::new(tsp_instance));
//!     let local_improver = Box::new(TakeFirstRecursion::new(
//!         2,
//!         5,
//!         neighborhood.clone(),
//!         objective.clone(),
//!     ));
//!     LocalSearchSolver::with_options(
//!         neighborhood,
//!         objective,
//!         Some(local_improver),
//!         None,
//!         Some(std::time::Duration::from_secs(600)),
//!         None,
//!     )
//! }
//! ```
use super::super::{objective::build_tsp_objective, tsp_instance::TspInstance, tsp_tour::TspTour};
use crate::examples::tsp::neighborhood::ThreeOptNeighborhood;
use crate::heuristics::local_search::local_improver::TakeFirstRecursion;
use crate::{heuristics::local_search::LocalSearchSolver, objective::Objective};
use std::sync::Arc;

/// Builds a [`LocalSearchSolver`] with [`TakeFirstRecursion`] as
/// [`LocalImprover`][`crate::heuristics::local_search::local_improver::LocalImprover`].
/// * The neighborhood is the 3-opt neighborhood, i.e., the neighborhood that consists of
/// all tours that can be obtained by applying the 3-opt operation.
/// * The local improver is set to [`TakeFirstRecursion`], which takes the first improving
/// neighbor.
/// * If no improving neighbor is found, the best 5 neighbors are considered in recursion.
/// * The recursion depth is set to 2. The time limit is set to 10 minutes. There is no iteration
/// limit.
/// * The time limit is set to 10 minutes. There is no iteration limit.
pub fn build(tsp_instance: Arc<TspInstance>) -> LocalSearchSolver<TspTour> {
    let objective: Arc<Objective<TspTour>> = Arc::new(build_tsp_objective());
    let neighborhood = Arc::new(ThreeOptNeighborhood::new(tsp_instance));
    let local_improver = Box::new(TakeFirstRecursion::new(
        2,
        5,
        neighborhood.clone(),
        objective.clone(),
    ));
    LocalSearchSolver::with_options(
        neighborhood,
        objective,
        Some(local_improver),
        None,
        Some(std::time::Duration::from_secs(600)),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::build;
    use crate::{
        examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour},
        heuristics::Solver,
    };
    use std::sync::Arc;

    #[test]
    fn test_take_first_local_search() {
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
    fn test_first_local_search_52_nodes() {
        let tsp_instance = Arc::new(
            TspInstance::from_tsplib_file("resources/tsp_test_instances/berlin52.tsp").unwrap(),
        );
        let tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());
        let solver = build(tsp_instance.clone());

        let local_opt_tour = solver.solve(tour);

        assert_eq!(
            local_opt_tour.solution().get_nodes(),
            &vec![
                0, 48, 31, 44, 18, 40, 7, 8, 9, 42, 32, 11, 27, 26, 25, 46, 12, 13, 51, 10, 50, 24,
                3, 5, 14, 4, 23, 47, 37, 36, 39, 38, 35, 34, 33, 43, 45, 15, 28, 49, 19, 22, 29, 1,
                6, 41, 20, 16, 2, 17, 30, 21
            ]
        );
    }
}
