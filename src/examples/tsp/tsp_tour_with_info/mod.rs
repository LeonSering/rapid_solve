//! In order to avoid back and forth moves in threshold accepting or simulated annealing, the
//! [`TspTourWithInfo`] struct equips a [`TspTour`] with the first index of the last accpeted
//! 3-opt move. The [`RotatedThreeOptNeighborhood`][`neighborhood::RotatedThreeOptNeighborhood`] generates the same neighbors as the
//! [`ThreeOptNeighborhood`][`super::neighborhood::ThreeOptNeighborhood`], but starts with the first index at `last_i + 1`.

use super::tsp_tour::TspTour;
pub mod neighborhood;
pub mod objective;

/// This struct labels are [`TspTour`] with the last index of the first node of the 3-opt move.
#[derive(Clone)]
pub struct TspTourWithInfo {
    tour: TspTour,
    last_i: usize,
}

impl TspTourWithInfo {
    /// Creates a new [`TspTourWithInfo`] from a [`TspTour`] and the first index of the last
    /// 3-opt move.
    pub fn new(tour: TspTour, last_i: usize) -> Self {
        Self { tour, last_i }
    }

    /// Returns the [`TspTour`] of this [`TspTourWithInfo`].
    pub fn get_tour(&self) -> &TspTour {
        &self.tour
    }

    /// Returns the first index of the last 3-opt move.
    pub fn get_last_i(&self) -> usize {
        self.last_i
    }

    /// Unwraps the [`TspTour`] from this [`TspTourWithInfo`].
    pub fn unwrap(self) -> TspTour {
        self.tour
    }
}
