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

    /// Returns an iterator over all neighbors of `current_solution` where the first `rotation`
    /// neighbors are rotated to the end of the iterator. (12345 -> 34512)
    /// Note that [`neighbors_of`] is called twice, so this only works if the neighborhood is
    /// deterministic.
    fn neighbors_of_rotated<'a>(
        &'a self,
        current_solution: &'a S,
        rotation: usize,
    ) -> Box<dyn Iterator<Item = S> + Send + Sync + 'a> {
        let first = self.neighbors_of(current_solution);
        let second = self.neighbors_of(current_solution);

        Box::new(first.skip(rotation).chain(second.take(rotation)))
    }
}
