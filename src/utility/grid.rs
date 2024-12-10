use std::collections::HashSet;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::rc::Rc;

/// A 2D position.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos2D {
    x: usize,
    y: usize,
}

impl Pos2D {
    /// Creates a new [`Pos2D`].
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Converts the given linear index into a 2D position using the grid's size.
    ///
    /// This will return `None` if the position is outside of the grid's size.
    const fn from_index(size: Size2D, index: usize) -> Option<Self> {
        let this = Self::new(index % size.w().get(), index / size.w().get());

        if this.is_contained_within(size) { Some(this) } else { None }
    }

    /// Converts the given 2D position into a linear index using the grid's size.
    ///
    /// This will return `None` if the position is outside of the grid's size.
    const fn into_index(self, size: Size2D) -> Option<usize> {
        if self.is_contained_within(size) { Some(self.x() + (self.y() * size.w().get())) } else { None }
    }

    /// Returns this position's X value.
    pub const fn x(self) -> usize {
        self.x
    }

    /// Returns this position's Y value.
    pub const fn y(self) -> usize {
        self.y
    }

    /// Returns whether the given size contains this position.
    pub const fn is_contained_within(self, size: Size2D) -> bool {
        size.contains_position(self)
    }

    /// Returns a new position with the given X value.
    pub const fn with_x(self, x: usize) -> Self {
        Self::new(x, self.y())
    }

    /// Returns a new position with the given Y value.
    pub const fn with_y(self, y: usize) -> Self {
        Self::new(self.x(), y)
    }

    /// Offsets this position in the given direction.
    ///
    /// This will return `None` if either value addition would panic.
    pub const fn offset(self, offset: Offset2D) -> Option<Self> {
        self.offset_by(offset.x(), offset.y())
    }

    /// Offsets this position in the given direction.
    ///
    /// This will return `None` if either value addition would panic.
    pub const fn offset_by(self, x: isize, y: isize) -> Option<Self> {
        let x = match self.x().checked_add_signed(x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y().checked_add_signed(y) {
            Some(v) => v,
            None => return None,
        };

        Some(Self::new(x, y))
    }
}

/// A 2D position.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset2D {
    x: isize,
    y: isize,
}

impl Offset2D {
    /// Creates a new [`Offset2D`].
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Returns this offset's X value.
    pub const fn x(self) -> isize {
        self.x
    }

    /// Returns this offset's Y value.
    pub const fn y(self) -> isize {
        self.y
    }

    /// Returns a new offset with the given X value.
    pub const fn with_x(self, x: isize) -> Self {
        Self::new(x, self.y())
    }

    /// Returns a new offset with the given Y value.
    pub const fn with_y(self, y: isize) -> Self {
        Self::new(self.x(), y)
    }

    /// Combines two offsets together.
    pub const fn combine(self, other: Self) -> Self {
        Self::new(self.x() + other.x(), self.y() + other.y())
    }

    /// Scales up the offset by the given amount.
    pub const fn multiply(self, by: isize) -> Self {
        Self::new(self.x() * by, self.y() * by)
    }

    /// Scales down the offset by the given amount.
    pub const fn divide(self, by: isize) -> Self {
        Self::new(self.x() / by, self.y() / by)
    }

    /// Returns an iterator over each direction offset.
    pub fn directions() -> impl Iterator<Item = Self> {
        (-1 ..= 1).flat_map(|y| (-1 ..= 1).map(move |x| Self::new(x, y)))
    }
}

/// A 2D grid size.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size2D {
    w: NonZeroUsize,
    h: NonZeroUsize,
}

impl Size2D {
    /// Creates a new square [`Size2D`].
    pub const fn square(d: NonZeroUsize) -> Self {
        Self::new(d, d)
    }

    /// Creates a new [`Size2D`].
    pub const fn new(w: NonZeroUsize, h: NonZeroUsize) -> Self {
        Self { w, h }
    }

    /// Creates a new [`Size2D`].
    ///
    /// This will return [`None`] if the given dimension is zero.
    pub const fn try_square(d: usize) -> Option<Self> {
        Self::try_new(d, d)
    }

    /// Creates a new [`Size2D`].
    ///
    /// This will return [`None`] if either of the given `w` or `h` is zero.
    pub const fn try_new(w: usize, h: usize) -> Option<Self> {
        let w = match NonZeroUsize::new(w) {
            Some(v) => v,
            None => return None,
        };
        let h = match NonZeroUsize::new(h) {
            Some(v) => v,
            None => return None,
        };

        Some(Self::new(w, h))
    }

    /// Returns this position's width.
    pub const fn w(self) -> NonZeroUsize {
        self.w
    }

    /// Returns this position's height.
    pub const fn h(self) -> NonZeroUsize {
        self.h
    }

    /// Returns whether this grid size contains the given position.
    pub const fn contains_position(self, pos: Pos2D) -> bool {
        pos.x() < self.w().get() && pos.y() < self.h().get()
    }
}

/// A 2D grid containing values of type `T`.
#[derive(Clone, Debug)]
pub struct Grid2D<T> {
    /// The grid's size.
    size: Size2D,
    /// The grid's inner cells.
    cells: Box<[Option<T>]>,
}

impl<T> Grid2D<T> {
    /// Creates a new [`Grid2D<T>`].
    pub fn new(size: Size2D) -> Self {
        let capacity = size.w().get() * size.h().get();
        let cells = std::iter::repeat_with(|| None).take(capacity).collect();

        Self { size, cells }
    }

    /// Returns a reference to the value at the given position.
    pub fn get(&self, pos: Pos2D) -> Option<&T> {
        self.cells[pos.into_index(self.size)?].as_ref()
    }

    /// Sets the value at the given position.
    pub fn set(&mut self, pos: Pos2D, value: T) {
        let Some(index) = pos.into_index(self.size) else { return };

        _ = self.cells[index].insert(value);
    }

    /// Removes the value from the given position.
    pub fn remove(&mut self, pos: Pos2D) {
        let Some(index) = pos.into_index(self.size) else { return };

        _ = self.cells[index].take();
    }

    /// Returns the total number of cells in the grid.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns whether this grid is empty.
    pub fn is_empty(&self) -> bool {
        self.cells().all(Option::is_none)
    }

    /// Returns an iterator over references to the cells of this grid.
    pub fn cells(&self) -> impl Iterator<Item = &Option<T>> {
        self.cells.iter()
    }

    /// Returns an iterator over references to the cells of this grid.
    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut Option<T>> {
        self.cells.iter_mut()
    }

    /// Returns an iterator over the cells of this grid.
    pub fn into_cells(self) -> impl Iterator<Item = Option<T>> {
        self.cells.into_iter()
    }

    /// Returns an iterator over references to the cells of this grid and their positions.
    pub fn iter(&self) -> impl Iterator<Item = (Pos2D, &Option<T>)> {
        self.cells().enumerate().map(|(i, v)| (Pos2D::from_index(self.size, i).unwrap(), v))
    }

    /// Returns an iterator over references to the cells of this grid and their positions.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Pos2D, &mut Option<T>)> {
        let size = self.size;

        self.cells_mut().enumerate().map(move |(i, v)| (Pos2D::from_index(size, i).unwrap(), v))
    }

    /// Returns an iterator over references to the cells of this grid and their positions.
    pub fn into_iter(self) -> impl Iterator<Item = (Pos2D, Option<T>)> {
        let size = self.size;

        self.into_cells().enumerate().map(move |(i, v)| (Pos2D::from_index(size, i).unwrap(), v))
    }
}

/// A 2D grid containing values of type `T`.
///
/// This type attempts to save memory by avoiding storing duplicate values, instead storing
/// reference counted values in each slot.
#[derive(Clone, Debug)]
pub struct MappedGrid2D<T> {
    /// The grid's size.
    size: Size2D,
    /// The grid's inner cells.
    cells: Box<[Option<Rc<T>>]>,
    /// Stores the grid's unique values.
    values: HashSet<Rc<T>>,
}

impl<T> MappedGrid2D<T>
where
    T: Eq + Hash,
{
    /// Creates a new [`MappedGrid2D<T>`].
    pub fn new(size: Size2D) -> Self {
        let capacity = size.w().get() * size.h().get();
        let cells = std::iter::repeat(None).take(capacity).collect();

        Self { size, cells, values: HashSet::new() }
    }

    /// Returns a reference to the value at the given position.
    pub fn get(&self, pos: Pos2D) -> Option<&Rc<T>> {
        self.cells[pos.into_index(self.size)?].as_ref()
    }

    /// Sets the value at the given position.
    pub fn set(&mut self, pos: Pos2D, value: T) {
        let value = Rc::clone(self.values.get_or_insert(Rc::new(value)));
        let Some(index) = pos.into_index(self.size) else { return };

        _ = self.cells[index].insert(value);

        self.values.retain(|value| self.cells.iter().any(|v| v.as_ref().is_some_and(|v| v == value)));
    }

    /// Removes the value from the given position.
    pub fn remove(&mut self, pos: Pos2D) {
        let Some(index) = pos.into_index(self.size) else { return };

        _ = self.cells[index].take();

        self.values.retain(|value| self.cells.iter().any(|v| v.as_ref().is_some_and(|v| v == value)));
    }

    /// Returns the total number of cells in the grid.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns whether this grid is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns an iterator over references to the cells of this grid.
    pub fn cells(&self) -> impl Iterator<Item = &Option<Rc<T>>> {
        self.cells.iter()
    }

    /// Returns an iterator over references to the cells of this grid.
    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut Option<Rc<T>>> {
        self.cells.iter_mut()
    }

    /// Returns an iterator over the cells of this grid.
    pub fn into_cells(self) -> impl Iterator<Item = Option<Rc<T>>> {
        self.cells.into_iter()
    }

    /// Returns an iterator over references to the cells of this grid and their positions.
    pub fn iter(&self) -> impl Iterator<Item = (Pos2D, &Option<Rc<T>>)> {
        self.cells().enumerate().map(|(i, v)| (Pos2D::from_index(self.size, i).unwrap(), v))
    }

    /// Returns an iterator over references to the cells of this grid and their positions.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Pos2D, &mut Option<Rc<T>>)> {
        let size = self.size;

        self.cells_mut().enumerate().map(move |(i, v)| (Pos2D::from_index(size, i).unwrap(), v))
    }

    /// Returns an iterator over references to the cells of this grid and their positions.
    pub fn into_iter(self) -> impl Iterator<Item = (Pos2D, Option<Rc<T>>)> {
        let size = self.size;

        self.into_cells().enumerate().map(move |(i, v)| (Pos2D::from_index(size, i).unwrap(), v))
    }
}
