//! The [`Neighborhood`] defines for every solution (in this case a tour) an iterator over all neighbors.
//! The [`ThreeOptNeighborhood`] generates all tours that can be obtained by applying a 3-opt move.
use std::sync::Arc;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour},
    heuristics::common::{Neighborhood, ParallelNeighborhood},
};

/// Given a [`TspTour`], this [`Neighborhood`] generates all tours that can be obtained by applying a
/// 3-opt move (deleting three arcs and reconnecting the tour by adding three new arcs).
pub struct ThreeOptNeighborhood {
    tsp_instance: Arc<TspInstance>,
}

impl ThreeOptNeighborhood {
    /// Creates a new [`ThreeOptNeighborhood`] for the given [`TspInstance`].
    pub fn new(tsp_instance: Arc<TspInstance>) -> Self {
        Self { tsp_instance }
    }
}

impl Neighborhood<TspTour> for ThreeOptNeighborhood {
    fn neighbors_of<'a>(
        &'a self,
        tour: &'a TspTour,
    ) -> Box<dyn Iterator<Item = TspTour> + Send + Sync + 'a> {
        let num_nodes = self.tsp_instance.get_number_of_nodes();
        Box::new((0..num_nodes - 2).flat_map(move |i| {
            (i + 1..num_nodes - 1)
                .flat_map(move |j| (j + 1..num_nodes).map(move |k| tour.three_opt_swap(i, j, k)))
        }))
    }
}

/// Given a [`TspTour`], this [`ParallelNeighborhood`] generates all tours that can be obtained by
/// applying a
/// 3-opt move (deleting three arcs and reconnecting the tour by adding three new arcs).
/// The parallel version of the [`ThreeOptNeighborhood`] as it uses the parallel iterator.
pub struct ParallelThreeOptNeighborhood {
    tsp_instance: Arc<TspInstance>,
}

impl ParallelThreeOptNeighborhood {
    /// Creates a new [`ParallelThreeOptNeighborhood`] for the given [`TspInstance`].
    pub fn new(tsp_instance: Arc<TspInstance>) -> Self {
        Self { tsp_instance }
    }
}

impl ParallelNeighborhood<TspTour> for ParallelThreeOptNeighborhood {
    fn neighbors_of<'a>(&'a self, tour: &'a TspTour) -> impl ParallelIterator<Item = TspTour> + 'a {
        let num_nodes = self.tsp_instance.get_number_of_nodes();
        (0..num_nodes - 2)
            .into_par_iter()
            .flat_map(move |i| (i + 1..num_nodes - 1).into_par_iter().map(move |j| (i, j)))
            .flat_map(move |(i, j)| {
                (j + 1..num_nodes)
                    .into_par_iter()
                    .map(move |k| tour.three_opt_swap(i, j, k))
            })
    }
}
