//! This module contains types, traits and algorithms that are used by multiple solvers.
//! In particular, it contains the [`Neighborhood`] trait, which is used to define the neighborhood
//! of a solution, and the [`FunctionBetweenSteps`] type, which is used to define the function
//! that is executed between steps of the solver.

mod function_between_steps;
mod neighborhood;
pub use function_between_steps::default_function_between_steps;
pub use function_between_steps::FunctionBetweenSteps;
pub use neighborhood::Neighborhood;
pub use neighborhood::ParallelNeighborhood;
