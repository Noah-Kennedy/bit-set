//use std::fmt::Debug;
//use crate::integer::{IntegerBitSet, OutOfBoundsError};
//use std::ops::{BitOrAssign, BitAndAssign, BitXorAssign, ShlAssign};
//use std::convert::{TryInto, TryFrom};
//
//#[repr(transparent)]
//#[derive(Debug, Default, Eq, PartialEq, Clone)]
//pub struct BitSetGeneric<T: Eq + Default + Debug + Clone> {
//    data: T
//}
//
//impl <T> Into<T> for BitSetGeneric<T> where T: Eq + Default + Debug + Clone + Sized {
//    fn into(self) -> T {
//        self.data
//    }
//}
//
//impl <T> From<T> for BitSetGeneric<T> where T: Eq + Default + Debug + Clone + Sized{
//    fn from(data: T) -> Self {
//        Self { data }
//    }
//}
//
//impl <T> TryInto<usize> for BitSetGeneric<T> where T: Eq + Default + Debug + Clone + Sized {
//    type Error = OutOfBoundsError;
//
//    fn try_into(self) -> Result<usize, Self::Error> {
//        unimplemented!()
//    }
//}
//
//impl <T> TryFrom<usize> for BitSetGeneric<T> where T: Eq + Default + Debug + Clone + Sized {
//    type Error = OutOfBoundsError;
//
//    fn try_from(value: usize) -> Result<Self, Self::Error> {
//        unimplemented!()
//    }
//}
//
//impl <T> IntegerBitSet<T> for BitSetGeneric<T>
//    where T: Eq + Default + Debug + Clone + BitOrAssign + BitAndAssign + BitXorAssign + ShlAssign + Sized {
//    fn insert_checked(&mut self, value: usize) -> Result<bool, OutOfBoundsError> {
//        unimplemented!()
//    }
//
//    fn remove_checked(&mut self, value: usize) -> Result<bool, OutOfBoundsError> {
//        unimplemented!()
//    }
//
//    fn contains_checked(&self, value: usize) -> Result<bool, OutOfBoundsError> {
//        unimplemented!()
//    }
//
//    fn insert_unchecked(&mut self, value: usize) {
//        unimplemented!()
//    }
//
//    fn remove_unchecked(&mut self, value: usize) {
//        unimplemented!()
//    }
//
//    fn contains_unchecked(&self, value: usize) -> bool {
//        unimplemented!()
//    }
//}