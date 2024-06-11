//! This module contains the implementation of several 3-opt local search algorithms.
//! In the [neighborhood] module the [`ThreeOptNeighborhood`][`neighborhood::ThreeOptNeighborhood`] for a given tour is defined.
pub mod neighborhood;

pub mod basic_three_opt_local_search;
pub mod take_first_three_opt_local_search;
