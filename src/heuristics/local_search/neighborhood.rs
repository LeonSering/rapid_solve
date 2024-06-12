//! This module provides the [`Neighborhood`] trait which is used to define a local search
//! neighborhood.

/// A local search neighborhood that provides for each solution an iterator over all neighbors.
/// The provided `current_solution`, as well as the [`Neighborhood`] instance must live as long as the iterator.
/// (Note that the iterator highly depends on the `current_solution` and that the [`Neighborhood`] may
/// have some attributes which goes into the iterator.)
pub trait Neighborhood<S>: Send + Sync {
    /// Returns an iterator over all neighbors of `current_solution`.
    fn neighbors_of<'a>(
        &'a self,
        current_solution: &'a S,
    ) -> Box<dyn Iterator<Item = S> + Send + Sync + 'a>;
}
