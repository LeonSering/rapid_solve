//! This module contains the implementation of several 3-opt local search metaheuristics.
//! In the [neighborhood] module the [`ThreeOptNeighborhood`][`neighborhood::ThreeOptNeighborhood`] for a given tour is defined.
pub mod neighborhood;

//TODO remove three_opt from local_search name
pub mod basic_three_opt_local_search;
pub mod take_first_three_opt_local_search;
pub mod threshold_accepting;
