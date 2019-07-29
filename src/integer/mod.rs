use std::ops::IndexMut;

mod bit_set_16;

pub type BitSetResult<T> = Result<T, OutOfBoundsError>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OutOfBoundsError {
    /// The value which the caller attempted to use in the set
    pub value: usize,
    /// The capacity of the set. The range of storable values is [0, capacity - 1].
    pub capacity: usize
}

pub trait IntegerBitSet: Default + Clone + Eq + PartialEq + IndexMut<usize, Output=bool> {
    /// Inserts an item if it is within the bounds of the set.
    ///
    /// If the item is too large to fit within the bounds of our set, we return an error.
    ///
    /// Otherwise, we return true if the value was not already present (an insertion occurred)
    /// and false if it was present already (an insertion did not occur).
    fn insert_checked(&mut self, value: usize) -> BitSetResult<bool>;

    /// Removes an item if it is within the bounds of the set.
    ///
    /// If the item is too large to fit within the bounds of our set, we return an error.
    ///
    /// Otherwise, we return true if the value was present (a removal occurred)
    /// and false if it was not present (a removal did not occur).
    fn remove_checked(&mut self, value: usize) -> BitSetResult<bool>;

    /// Checks for an item if it is within the bounds of the set.
    ///
    /// If the item is too large to fit within the bounds of our set, we return an error.
    ///
    /// Otherwise, we return true if the value is present and false if it is not present.
    fn contains_checked(&mut self, value: usize) -> BitSetResult<bool>;

    /// Inserts an item if it is within the bounds of the set.
    ///
    /// If the item is too large to fit within the bounds of our set, nothing happens.
    fn insert_unchecked(&mut self, value: usize);

    /// Removes an item if it is within the bounds of the set.
    ///
    /// If the item is too large to fit within the bounds of our set, nothing happens.
    fn remove_unchecked(&mut self, value: usize);

    /// Checks for an item if it is within the bounds of the set.
    ///
    /// If the item is too large to fit within the bounds of our set, we return false.
    fn contains_unchecked(&mut self, value: usize) -> bool;
}