use crate::prelude::*;

pub trait GridLayer<T: GridParam> {
    type MutableReturn<'a>
    where Self: 'a;

    /// Create a new grid layer with the given size (clone the given value).
    fn new_clone(size: impl Size2d, value: T) -> Self
    where T: Clone;

    /// Clone a region of the grid onto a new grid.
    fn blit_clone(&mut self, to: impl GridPoint, source: &Self, from: impl GridPoint, size: impl Size2d)
    where T: Clone;

    /// Create a new grid with the given size (copy the given value).
    fn new_copy(size: impl Size2d, value: T) -> Self
    where T: Copy;

    /// Copy a region from another grid into this grid.
    fn blit_copy(&mut self, to: impl GridPoint, source: &Self, from: impl GridPoint, size: impl Size2d)
    where T: Copy;

    /// Create a new grid with the given size and default values.
    fn new_default(size: impl Size2d) -> Self
    where T: Default;

    fn new_fn(size: impl Size2d, f: impl Fn(IVec2) -> T) -> Self;

    ///////////////////////////////////////////////////////////////////////////
    // Utility Functionality
    ///////////////////////////////////////////////////////////////////////////
    /// Returns the size of the grid.
    fn width(&self) -> u32;

    /// Returns the height of the grid.
    fn height(&self) -> u32;

    /// Returns the size of the grid as a 2D vector.
    fn size(&self) -> UVec2;

    /// Returns the number of elements in the vector, also referred to as its 'length'.
    fn len(&self) -> usize;

    /// Returns true if the vector contains no elements
    fn is_empty(&self) -> bool;

    /// Tests whether a point is in bounds.
    fn in_bounds(&self, pos: impl GridPoint) -> bool;

    ///////////////////////////////////////////////////////////////////////////
    // Getter/Setter Functionality
    ///////////////////////////////////////////////////////////////////////////
    /// Try Gets the `GridPoint` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    fn get_idx(&self, pos: impl GridPoint) -> Option<usize>;

    /// Gets the index corresponding to a coordinate, which is row-wise.
    ///
    /// # Panic
    ///
    /// Panics if the coordinate is out of bounds.
    fn get_idx_unchecked(&self, point: impl GridPoint) -> usize;

    /// Try Gets the `GridPoint` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    fn index_to_pt(&self, idx: usize) -> Option<IVec2>;

    /// Gets the `GridPoint` corresponding to an index
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index_to_pt_unchecked(&self, idx: usize) -> IVec2;

    /// Gets a reference to the element at the given index.
    fn get(&self, pos: impl GridPoint) -> Option<&T>;

    /// Gets a mutable reference to the element at the given index.
    fn get_mut(&mut self, pos: impl GridPoint) -> Option<Self::MutableReturn<'_>>;

    /// Gets a reference to the element at the given index (unchecked).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn get_unchecked(&self, pos: impl GridPoint) -> &T;

    /// Gets a mutable reference to the element at the given index (unchecked).
    fn get_mut_unchecked(&mut self, pos: impl GridPoint) -> Self::MutableReturn<'_>;

    /// Sets the value of the element at the given index.
    fn set(&mut self, pos: impl GridPoint, value: T) -> Option<T>;

    /// Sets the value of the element at the given index (unchecked).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn set_unchecked(&mut self, pos: impl GridPoint, value: T) -> T;
}
