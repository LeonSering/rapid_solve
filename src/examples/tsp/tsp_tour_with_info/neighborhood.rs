//! This module contains the [`RotatedThreeOptNeighborhood`] for a [`TspTourWithInfo`] wrapper.

use std::sync::Arc;

use crate::{examples::tsp::tsp_instance::TspInstance, heuristics::common::Neighborhood};

use super::TspTourWithInfo;

/// Given a [`TspTourWithInfo`], this [`Neighborhood`] generates all tours that can be obtained by
/// applying a 3-opt move. It starts with the first index at `last_i + 1` in order to avoid back and forth
/// moves in threshold accepting or simulated annealing.
pub struct RotatedThreeOptNeighborhood {
    tsp_instance: Arc<TspInstance>,
}

impl RotatedThreeOptNeighborhood {
    /// Creates a new [`RotatedThreeOptNeighborhood`] for the given [`TspInstance`].
    pub fn new(tsp_instance: Arc<TspInstance>) -> Self {
        Self { tsp_instance }
    }
}

impl Neighborhood<TspTourWithInfo> for RotatedThreeOptNeighborhood {
    fn neighbors_of<'a>(
        &'a self,
        tour_with_info: &'a TspTourWithInfo,
    ) -> Box<dyn Iterator<Item = TspTourWithInfo> + Send + Sync + 'a> {
        let num_nodes = self.tsp_instance.get_number_of_nodes();
        let start_i = tour_with_info.last_i + 1 % num_nodes;

        Box::new(
            (start_i..num_nodes - 2)
                .chain(0..start_i)
                .flat_map(move |i| {
                    (i + 1..num_nodes - 1).flat_map(move |j| {
                        (j + 1..num_nodes).map(move |k| TspTourWithInfo {
                            tour: tour_with_info.tour.three_opt_swap(i, j, k),
                            last_i: i,
                        })
                    })
                }),
        )
    }
}
