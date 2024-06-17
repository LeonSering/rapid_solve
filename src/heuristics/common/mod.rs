//! This module contains types, traits and algorithms that are used by multiple solvers.
pub mod function_between_steps;
pub mod neighborhood;
pub use function_between_steps::FunctionBetweenSteps;
pub use neighborhood::Neighborhood;
