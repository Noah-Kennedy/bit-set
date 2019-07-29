use crate::numeric::{IntegerBitSet, OutOfBoundsError};

/// An implementation of an unsigned integer BitSet with a capacity of 16.
/// This can store the numbers in the range [0, 15]
///
/// TODO add tests
/// TODO cleanup API
#[repr(transparent)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct IntegerBitSet16 {
    data: u16
}

impl IntegerBitSet16 {
    /// Creates a new, empty BitSet.
    #[inline]
    pub const fn new() -> Self {
        Self {
            data: 0x00
        }
    }

    /// Creates a new BitSet with a specified initial state.
    #[inline]
    pub const fn with_existing(data: u16) -> Self {
        Self { data }
    }

    /// The same as contains_unchecked, but const.
    #[inline]
    pub const fn contains_const(&self, entry: usize) -> bool {
        let mask = Self::get_mask(entry);
        (self.data & mask) != 0
    }

    #[inline]
    const fn get_mask(entry: usize) -> u16 {
        ((1 as usize) << entry) as u16
    }
}

impl From<u16> for IntegerBitSet16 {
    #[inline]
    fn from(data: u16) -> Self {
        Self::with_existing(data)
    }
}

impl IntegerBitSet for IntegerBitSet16 {
    #[inline]
    fn insert_checked(&mut self, value: usize) -> Result<bool, OutOfBoundsError> {
        if value < 15 {
            let mask = Self::get_mask(value);
            let result = self.data & mask == 0;
            self.data |= mask;
            Ok(result)
        } else {
            Err(OutOfBoundsError {
                capacity: 15,
                value
            })
        }
    }

    #[inline]
    fn remove_checked(&mut self, value: usize) -> Result<bool, OutOfBoundsError> {
        unimplemented!()
    }

    #[inline]
    fn contains_checked(&mut self, value: usize) -> Result<bool, OutOfBoundsError> {
        if value < 15 {
            Ok(self.contains_unchecked(value))
        } else {
            Err(OutOfBoundsError {
                capacity: 15,
                value
            })
        }
    }

    #[inline]
    fn insert_unchecked(&mut self, value: usize) {
        let mask = Self::get_mask(value);
        self.data |= mask;
    }

    #[inline]
    fn remove_unchecked(&mut self, value: usize) {
        let mask = Self::get_mask(value);
        self.data ^= mask;
    }

    #[inline]
    fn contains_unchecked(&mut self, value: usize) -> bool {
        self.contains_const(value)
    }
}