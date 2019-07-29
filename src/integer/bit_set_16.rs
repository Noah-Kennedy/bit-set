use crate::integer::{IntegerBitSet, OutOfBoundsError};
use std::convert::{TryFrom, TryInto};

/// An implementation of an unsigned integer BitSet with a capacity of 16.
/// This can store the numbers in the range [0, 15]
///
/// TODO add tests
#[repr(transparent)]
#[derive(Default, Clone, Eq, PartialEq, Debug)]
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
    pub fn contains_const(&self, entry: u64) -> bool {
        let mask = Self::get_mask(entry);
        (self.data & mask) != 0
    }

    #[inline]
    fn get_mask(entry: u64) -> u16 {
        match (1 as u16).checked_shl(entry as u32) {
            Some(n) => n,
            None => 0
        }
    }
}

impl From<u16> for IntegerBitSet16 {
    #[inline]
    fn from(data: u16) -> Self {
        Self::with_existing(data)
    }
}

impl Into<u16> for IntegerBitSet16 {
    #[inline]
    fn into(self) -> u16 {
        self.data
    }
}

impl TryFrom<u64> for IntegerBitSet16 {
    type Error = OutOfBoundsError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > 15 {
            Err(OutOfBoundsError { value, capacity: 16 })
        } else {
            Ok(Self::from(value as u16))
        }
    }
}

impl TryInto<u64> for IntegerBitSet16 {
    type Error = OutOfBoundsError;

    fn try_into(self) -> Result<u64, Self::Error> {
        Ok(u64::from(self.data))
    }
}

impl IntegerBitSet<u16> for IntegerBitSet16 {
    #[inline]
    fn insert_checked(&mut self, value: u64) -> Result<bool, OutOfBoundsError> {
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
    fn remove_checked(&mut self, value: u64) -> Result<bool, OutOfBoundsError> {
        if value < 15 {
            let mask = Self::get_mask(value);
            let result = self.data & mask != 0;
            self.data ^= mask;
            Ok(result)
        } else {
            Err(OutOfBoundsError {
                capacity: 15,
                value
            })
        }
    }

    #[inline]
    fn contains_checked(&self, value: u64) -> Result<bool, OutOfBoundsError> {
        if value < 16 {
            Ok(self.contains_unchecked(value))
        } else {
            Err(OutOfBoundsError {
                capacity: 16,
                value
            })
        }
    }

    #[inline]
    fn insert_unchecked(&mut self, value: u64) {
        let mask = Self::get_mask(value);
        self.data |= mask;
    }

    #[inline]
    fn remove_unchecked(&mut self, value: u64) {
        let mask = Self::get_mask(value);
        self.data ^= mask;
    }

    #[inline]
    fn contains_unchecked(&self, value: u64) -> bool {
        self.contains_const(value)
    }

    #[inline]
    fn in_bounds(&self, value: u64) -> bool {
        value < 16
    }
}