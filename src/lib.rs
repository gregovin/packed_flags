#![warn(clippy::pedantic,clippy::nursery,clippy::unwrap_used,clippy::perf)]
//!Provides various packed lists of flags(ie equivalent to `Vec<bool>`).
//!Useful anywhere you are tempted to use `Vec<bool>` or `[bool]`, but want some amount of memory efficiency
mod bit128;
mod bit32;
mod bit64;
mod bitlong;
mod bitsize;
mod flagls;
pub mod flag_iter;
use std::error::Error;
use std::fmt::{Display};

pub use crate::bit128::B128;
pub use crate::bit32::B32;
pub use crate::bit64::B64;
pub use crate::bitlong::Blong;
pub use crate::bitsize::Bsize;
pub use crate::flagls::FlagLs;
#[derive(Clone,Copy,PartialEq, Eq,Hash,Debug)]
///Represents errors that can occur for a flag list
pub enum FlagLsError{
    IndexOutOfBounds{idx:usize,len:usize},
    MaximumLengthExceeded{mx_len:usize,attempt_len:usize}
}
impl Error for FlagLsError{}
impl Display for FlagLsError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self{
            Self::IndexOutOfBounds { idx, len }=>write!(f, "attempted to access out of bounds index {idx} of flag list of length {len}"),
            Self::MaximumLengthExceeded { mx_len, attempt_len }=>write!(f, "flag list has maximum length {mx_len}, attempted to increase this to {attempt_len}")
        }
    }
}
#[cfg(test)]
// runs all the doc test on the other ones
mod tests {
    use super::*;
    #[test]
    fn len() {
        let flag_ls1 = B32::from_iter(vec![true, false, false]);
        let flag_ls2 = B128::from_iter(vec![true, false, false]);
        let flag_ls3 = Bsize::from_iter(vec![true, false, false]);
        let flag_ls4 = Blong::from_iter(vec![true, false, false]);
        assert_eq!(flag_ls1.len(), 3);
        assert_eq!(flag_ls2.len(), 3);
        assert_eq!(flag_ls3.len(), 3);
        assert_eq!(flag_ls4.len(), 3);
    }
    #[test]
    fn insert() {
        let mut flag_ls1 = B32::default();
        flag_ls1.insert(0, true);
        flag_ls1.insert(0, false);
        flag_ls1.insert(2, false);
        assert_eq!(flag_ls1, B32::from_iter(vec![false, true, false]));

        let mut flag_ls2 = B128::default();
        flag_ls2.insert(0, true);
        flag_ls2.insert(0, false);
        flag_ls2.insert(2, false);
        assert_eq!(flag_ls2, B128::from_iter(vec![false, true, false]));

        let mut flag_ls3 = Bsize::default();
        flag_ls3.insert(0, true);
        flag_ls3.insert(0, false);
        flag_ls3.insert(2, false);
        assert_eq!(flag_ls3, Bsize::from_iter(vec![false, true, false]));

        let mut flag_ls4 = Blong::default();
        flag_ls4.insert(0, true);
        flag_ls4.insert(0, false);
        flag_ls4.insert(2, false);
        assert_eq!(flag_ls4, Blong::from_iter(vec![false, true, false]));
    }
    #[test]
    fn remove() {
        let mut flag_ls = B32::from_iter(vec![false, true, false]);
        assert!(flag_ls.remove(1));
        assert_eq!(flag_ls, B32::from_iter(vec![false, false]));

        let mut flag_ls = B128::from_iter(vec![false, true, false]);
        assert!(flag_ls.remove(1));
        assert_eq!(flag_ls, B128::from_iter(vec![false, false]));

        let mut flag_ls = Bsize::from_iter(vec![false, true, false]);
        assert!(flag_ls.remove(1));
        assert_eq!(flag_ls, Bsize::from_iter(vec![false, false]));

        let mut flag_ls = Blong::from_iter(vec![false, true, false]);
        assert!(flag_ls.remove(1));
        assert_eq!(flag_ls, Blong::from_iter(vec![false, false]));
    }
    #[test]
    fn clear() {
        let mut flag_ls = B32::from_iter(vec![false, true, false]);
        flag_ls.clear();
        assert_eq!(flag_ls, B32::default());
        flag_ls.set_len(1);
        assert_eq!(flag_ls[0], false);

        let mut flag_ls = B128::from_iter(vec![false, true, false]);
        flag_ls.clear();
        assert_eq!(flag_ls, B128::default());
        flag_ls.set_len(1);
        assert!(!flag_ls[0]);

        let mut flag_ls = Bsize::from_iter(vec![false, true, false]);
        flag_ls.clear();
        assert_eq!(flag_ls, Bsize::default());
        flag_ls.set_len(1);
        assert!(!flag_ls[0]);

        let mut flag_ls = Blong::from_iter(vec![false, true, false]);
        flag_ls.clear();
        assert_eq!(flag_ls, Blong::default());
        flag_ls.set_len(1);
        assert!(!flag_ls[0]);
    }
    #[test]
    fn push() {
        let mut flag_ls = B32::default();
        flag_ls.push(false);
        flag_ls.push(true);
        flag_ls.push(false);
        assert_eq!(flag_ls, B32::from_iter(vec![false, true, false]));

        let mut flag_ls = B128::default();
        flag_ls.push(false);
        flag_ls.push(true);
        flag_ls.push(false);
        assert_eq!(flag_ls, B128::from_iter(vec![false, true, false]));

        let mut flag_ls = Bsize::default();
        flag_ls.push(false);
        flag_ls.push(true);
        flag_ls.push(false);
        assert_eq!(flag_ls, Bsize::from_iter(vec![false, true, false]));

        let mut flag_ls = Blong::default();
        flag_ls.push(false);
        flag_ls.push(true);
        flag_ls.push(false);
        assert_eq!(flag_ls, Blong::from_iter(vec![false, true, false]));
    }
    #[test]
    fn pop() {
        let mut flag_ls = B32::from_iter(vec![false, true]);
        assert_eq!(flag_ls.pop(), Some(true));
        assert_eq!(flag_ls.pop(), Some(false));
        assert_eq!(flag_ls.pop(), None);

        let mut flag_ls = B128::from_iter(vec![false, true]);
        assert_eq!(flag_ls.pop(), Some(true));
        assert_eq!(flag_ls.pop(), Some(false));
        assert_eq!(flag_ls.pop(), None);

        let mut flag_ls = Bsize::from_iter(vec![false, true]);
        assert_eq!(flag_ls.pop(), Some(true));
        assert_eq!(flag_ls.pop(), Some(false));
        assert_eq!(flag_ls.pop(), None);

        let mut flag_ls = Blong::from_iter(vec![false, true]);
        assert_eq!(flag_ls.pop(), Some(true));
        assert_eq!(flag_ls.pop(), Some(false));
        assert_eq!(flag_ls.pop(), None);
    }
    #[test]
    fn get() {
        let flag_ls = B32::from_iter(vec![false, true]);
        assert_eq!(flag_ls.get(1), Some(true));
        assert_eq!(flag_ls.get(2), None);

        let flag_ls = B128::from_iter(vec![false, true]);
        assert_eq!(flag_ls.get(1), Some(true));
        assert_eq!(flag_ls.get(2), None);

        let flag_ls = Bsize::from_iter(vec![false, true]);
        assert_eq!(flag_ls.get(1), Some(true));
        assert_eq!(flag_ls.get(2), None);

        let flag_ls = Blong::from_iter(vec![false, true]);
        assert_eq!(flag_ls.get(1), Some(true));
        assert_eq!(flag_ls.get(2), None);
    }
    #[test]
    fn set() {
        let mut flag_ls = B32::from_iter(vec![false, true, false]);
        flag_ls.set(2, true);
        assert_eq!(flag_ls, B32::from_iter(vec![false, true, true]));

        let mut flag_ls = B128::from_iter(vec![false, true, false]);
        flag_ls.set(2, true);
        assert_eq!(flag_ls, B128::from_iter(vec![false, true, true]));

        let mut flag_ls = Bsize::from_iter(vec![false, true, false]);
        flag_ls.set(2, true);
        assert_eq!(flag_ls, Bsize::from_iter(vec![false, true, true]));

        let mut flag_ls = Blong::from_iter(vec![false, true, false]);
        flag_ls.set(2, true);
        assert_eq!(flag_ls, Blong::from_iter(vec![false, true, true]));
    }
    #[test]
    fn try_set() {
        let mut flag_ls = B32::from_iter(vec![false, true, false]);
        assert_eq!(flag_ls.try_set(2, true), Some(false));
        assert_eq!(flag_ls, B32::from_iter(vec![false, true, true]));
        assert_eq!(flag_ls.try_set(3, false), None);

        let mut flag_ls = B128::from_iter(vec![false, true, false]);
        assert_eq!(flag_ls.try_set(2, true), Some(false));
        assert_eq!(flag_ls, B128::from_iter(vec![false, true, true]));
        assert_eq!(flag_ls.try_set(3, false), None);

        let mut flag_ls = Bsize::from_iter(vec![false, true, false]);
        assert_eq!(flag_ls.try_set(2, true), Some(false));
        assert_eq!(flag_ls, Bsize::from_iter(vec![false, true, true]));
        assert_eq!(flag_ls.try_set(3, false), None);

        let mut flag_ls = Blong::from_iter(vec![false, true, false]);
        assert_eq!(flag_ls.try_set(2, true), Some(false));
        assert_eq!(flag_ls, Blong::from_iter(vec![false, true, true]));
        assert_eq!(flag_ls.try_set(3, false), None);
    }
    #[test]
    fn iter() {
        let flag_ls = B32::from_iter(vec![false, true]);
        let mut itr = flag_ls.iter();
        assert_eq!(itr.next(), Some(false));
        assert_eq!(itr.next(), Some(true));
        assert_eq!(itr.next(), None);

        let flag_ls = B128::from_iter(vec![false, true]);
        let mut itr = flag_ls.iter();
        assert_eq!(itr.next(), Some(false));
        assert_eq!(itr.next(), Some(true));
        assert_eq!(itr.next(), None);

        let flag_ls = Bsize::from_iter(vec![false, true]);
        let mut itr = flag_ls.iter();
        assert_eq!(itr.next(), Some(false));
        assert_eq!(itr.next(), Some(true));
        assert_eq!(itr.next(), None);

        let flag_ls = Blong::from_iter(vec![false, true]);
        let mut itr = flag_ls.iter();
        assert_eq!(itr.next(), Some(false));
        assert_eq!(itr.next(), Some(true));
        assert_eq!(itr.next(), None);
    }
    #[test]
    fn from_iter() {
        let flag_ls = B32::from_iter(vec![false, true]);
        assert_eq!(flag_ls.len(), 2);
        assert_eq!(flag_ls.get(0), Some(false));
        assert_eq!(flag_ls.get(1), Some(true));

        let flag_ls = B128::from_iter(vec![false, true]);
        assert_eq!(flag_ls.len(), 2);
        assert_eq!(flag_ls.get(0), Some(false));
        assert_eq!(flag_ls.get(1), Some(true));

        let flag_ls = Bsize::from_iter(vec![false, true]);
        assert_eq!(flag_ls.len(), 2);
        assert_eq!(flag_ls.get(0), Some(false));
        assert_eq!(flag_ls.get(1), Some(true));

        let flag_ls = Blong::from_iter(vec![false, true]);
        assert_eq!(flag_ls.len(), 2);
        assert_eq!(flag_ls.get(0), Some(false));
        assert_eq!(flag_ls.get(1), Some(true));
    }
    #[test]
    fn all_true() {
        let flag_ls = B32::all_true(10);
        for flag in flag_ls.iter() {
            assert!(flag);
        }

        let flag_ls = B128::all_true(10);
        for flag in flag_ls.iter() {
            assert!(flag);
        }

        let flag_ls = Bsize::all_true(10);
        for flag in flag_ls.iter() {
            assert!(flag);
        }

        let flag_ls = Blong::all_true(10);
        for flag in flag_ls.iter() {
            assert!(flag);
        }
    }
    #[test]
    fn all_false() {
        let flag_ls = B32::all_false(10);
        for flag in flag_ls.iter() {
            assert!(!flag);
        }

        let flag_ls = B128::all_false(10);
        for flag in flag_ls.iter() {
            assert!(!flag);
        }

        let flag_ls = Bsize::all_false(10);
        for flag in flag_ls.iter() {
            assert!(!flag);
        }

        let flag_ls = Blong::all_false(10);
        for flag in flag_ls.iter() {
            assert!(!flag);
        }
    }
    #[test]
    fn is_empty() {
        let l1 = B32::default();
        let l2 = B32::all_true(1);
        assert!(l1.is_empty());
        assert!(!l2.is_empty());

        let l1 = B128::default();
        let l2 = B128::all_true(1);
        assert!(l1.is_empty());
        assert!(!l2.is_empty());

        let l1 = Bsize::default();
        let l2 = Bsize::all_true(1);
        assert!(l1.is_empty());
        assert!(!l2.is_empty());

        let l1 = Blong::default();
        let l2 = Blong::all_true(1);
        assert!(l1.is_empty());
        assert!(!l2.is_empty());
    }
}
