use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Index;

use crate::{flag_iter, FlagLsError};
/// A trait that represents a list of flags.
///
/// Mostly the same as things that would be implemented by `Vec<bool>` with a few omisions
pub trait FlagLs: Sized + PartialEq + Eq + Default + Clone + Debug + Hash + Index<usize> 
{
    /// The max length a given flag list can store
    /// # Example
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let b=B64::all_false(64);
    /// 
    /// assert_eq!(B64::MAX_LENGTH,64);
    /// assert!(b.len()<=B64::MAX_LENGTH);
    /// ```
    const MAX_LENGTH: usize;
    /// Returns number of flags in the flags list
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let flag_ls=B64::from_iter(vec![true,false,false]);
    /// assert_eq!(flag_ls.len(),3);
    /// ```
    fn len(&self) -> usize;
    /// Sets the length of the flag ls, zeroing any flags removed
    /// # Panics
    /// Panics if the new length is larger than `MAX_LENGTH`
    /// # Examples
    /// This can be used to extend the length of a list of flags with false
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let mut flag_ls=B64::all_true(4);
    /// flag_ls.set_len(6);
    /// assert_eq!(flag_ls,B64::from_iter(vec![true,true,true,true,false,false]));
    /// ```
    fn set_len(&mut self, new_len: usize);
    /// Inserts a new flag at the position given by index(ie so `flag_ls[index]=flag`)
    /// # Panics
    /// Panics if the index is out of bounds or the new element would make the list longer than `MAX_LENGTH`
    /// # Examples  
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::default();
    /// flag_ls.insert(0,true);
    /// flag_ls.insert(0,false);
    /// flag_ls.insert(2,false);
    /// assert_eq!(flag_ls,B64::from_iter(vec![false,true,false]));
    /// ```
    fn insert(&mut self, index: usize, flag: bool);
    /// Attempts to insert at index, returning Ok(()) on success
    /// # Errors
    /// Errors if the index is out of bounds(note idx=self.len() is considered in bounds for insert) or if the insert would bring the list above it's maximum allowed length
    /// # Examples  
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::default();
    /// assert_eq!(flag_ls.try_insert(0,true),Ok(()));
    /// assert!(flag_ls.try_insert(2,false).is_err());
    /// assert_eq!(flag_ls,B64::from_iter(vec![true]));
    /// ```
    fn try_insert(&mut self,index: usize,flag: bool)->Result<(),FlagLsError>{
        if index>self.len(){
            Err(FlagLsError::IndexOutOfBounds { idx: index, len: self.len() })
        } else if self.len()>=Self::MAX_LENGTH{
            Err(FlagLsError::MaximumLengthExceeded {mx_len: Self::MAX_LENGTH, attempt_len: self.len().checked_add(1).unwrap_or_else(|| self.len())})
        } else {
            self.insert(index, flag);
            Ok(())
        }
    }
    /// Removes the flag at the position given by index, and returns it
    /// # Panics
    /// Panics if the index is out of bounds
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::from_iter(vec![false,true,false]);
    /// assert!(flag_ls.remove(1));
    /// assert_eq!(flag_ls,B64::from_iter(vec![false,false]));
    /// ```
    fn remove(&mut self, index: usize) -> bool;
    /// Returns the specified flag, if it exists
    /// 
    /// If the index is out of bounds returns none
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::from_iter(vec![false,true,false]);
    /// assert_eq!(flag_ls.try_remove(1),Some(true));
    /// assert_eq!(flag_ls.try_remove(2),None);
    /// assert_eq!(flag_ls,B64::from_iter(vec![false,false]));
    /// ```
    fn try_remove(&mut self,index: usize)->Option<bool>{
        if index<self.len(){
            Some(self.remove(index))
        } else {
            None
        }
    }
    /// Clears the list, setting the internal state and length to 0
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::from_iter(vec![false,true,false]);
    /// 
    /// flag_ls.clear();
    /// assert_eq!(flag_ls,B64::default());
    /// flag_ls.set_len(1);
    /// assert_eq!(flag_ls[0],false);
    /// ```
    fn clear(&mut self);

    /// Truncate the flags to len. Does nothing if `len>=self.len`
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let mut flag_ls=B64::from_iter(vec![false,true,false,true]);
    /// flag_ls.truncate(2);
    /// assert_eq!(flag_ls,B64::from_iter(vec![false,true]));
    /// flag_ls.truncate(3);
    /// assert_eq!(flag_ls,B64::from_iter(vec![false,true]));
    /// ```
    fn truncate(&mut self, len: usize) {
        if len < self.len() {
            self.set_len(len);
        }
    }
    /// Attempts to push a new flag to the end of the list
    /// # Errors
    /// Errors if this would make the flag list larger than `MAX_LENGTH`
    /// # Examples
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::default();
    /// flag_ls.try_push(false)?;
    /// flag_ls.try_push(true)?;
    /// flag_ls.try_push(false)?;
    /// assert_eq!(flag_ls,B64::from_iter(vec![false,true,false]));
    /// 
    /// use packed_flags::B32;
    /// 
    /// let mut flag_ls2= B32::all_true(32);
    /// assert!(flag_ls2.try_push(false).is_err());
    /// assert_eq!(flag_ls2,B32::all_true(32));
    /// # Ok(())
    /// # }
    /// ```
    fn try_push(&mut self,flag:bool)->Result<(),FlagLsError>{
        if self.len()>=Self::MAX_LENGTH{
            Err(FlagLsError::MaximumLengthExceeded { mx_len: Self::MAX_LENGTH, attempt_len: self.len().checked_add(1).unwrap_or_else(|| self.len())})
        } else {
            self.push(flag);
            Ok(())
        }
    }
    /// pushes a new flag to the end of the list
    /// # Panics
    /// Panics if the resulting list is too big
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::default();
    /// flag_ls.push(false);
    /// flag_ls.push(true);
    /// flag_ls.push(false);
    /// assert_eq!(flag_ls,B64::from_iter(vec![false,true,false]));
    /// ```
    fn push(&mut self, flag: bool) {
        self.insert(self.len(), flag);
    }
    /// removes and returns the last flag, or none if the list is empty
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::from_iter(vec![false,true]);
    /// assert_eq!(flag_ls.pop(),Some(true));
    /// assert_eq!(flag_ls.pop(),Some(false));
    /// assert_eq!(flag_ls.pop(),None);
    /// ```
    fn pop(&mut self) -> Option<bool> {
        if self.len() > 0 {
            Some(self.remove(self.len() - 1))
        } else {
            None
        }
    }
    /// Get the flag at a specified index, if it exists, otherwise returns None
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let flag_ls=B64::from_iter(vec![false,true]);
    /// assert_eq!(flag_ls.get(1),Some(true));
    /// assert_eq!(flag_ls.get(2),None);
    /// ```
    fn get(&self, index: usize) -> Option<bool>;
    /// Set the flag at a specified index
    /// # Panics
    /// If the index is out of bounds
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::from_iter(vec![false,true,false]);
    /// flag_ls.set(2,true);
    /// assert_eq!(flag_ls,B64::from_iter(vec![false,true,true]));
    /// ```
    fn set(&mut self, index: usize, flag: bool);
    /// Attempts to sets flag at the specified index, returning the flag that was there if the index is in bounds
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let mut flag_ls=B64::from_iter(vec![false,true,false]);
    /// assert_eq!(flag_ls.try_set(2,true),Some(false));
    /// assert_eq!(flag_ls,B64::from_iter(vec![false,true,true]));
    /// assert_eq!(flag_ls.try_set(3,false),None);
    /// ```
    fn try_set(&mut self, index: usize, flag: bool) -> Option<bool> {
        self.get(index).map(|b|{
            self.set(index,flag);
            b
        })
    }
    /// get an iterator over all flags in the list
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let flag_ls=B64::from_iter(vec![false,true]);
    /// let mut itr=flag_ls.iter();
    /// assert_eq!(itr.next(),Some(false));
    /// assert_eq!(itr.next(),Some(true));
    /// assert_eq!(itr.next(),None);
    /// ```
    fn iter(&self) -> flag_iter::Iter<Self>;
    /// build a compact list of flags from an iterator-like of flags, consuming it
    /// 
    /// If you are trying to convert an `InitialFlagLs` to a `DesiredFlagLs`, and `DesiredFlagLs` implements `From<InitialFlagLs>` or `TryFrom<InitialFlagLs>` then the assosiated conversion method is preferable to `DesiredFlags::from_iter(thing.iter())`
    /// # Panics
    /// Panics when v is longer than `MAX_LENGTH`
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let flag_ls=B64::from_iter(vec![false,true]);
    /// assert_eq!(flag_ls.len(),2);
    /// assert_eq!(flag_ls.get(0),Some(false));
    /// assert_eq!(flag_ls.get(1),Some(true));
    /// ```
    #[must_use]
    fn from_iter<I: IntoIterator<Item = bool>>(v: I) -> Self {
        Self::try_from_iter(v).expect("Iterator was longer than the maximum allowable length for chosen Flag List")
    }
    /// Attempt to build a compact list of flags from an iterator-like of flags, consuming it
    /// 
    /// If you are trying to convert an `InitialFlagLs` to a `DesiredFlagLs`, and `DesiredFlagLs` implements `From<InitialFlagLs>` or `TryFrom<InitialFlagLs>` then the assosiated conversion method is preferable
    /// # Errors
    /// Errors when v is longer than `MAX_LENGTH`
    /// # Examples
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let flag_ls=B64::try_from_iter(vec![false,true])?;
    /// assert_eq!(flag_ls.len(),2);
    /// assert_eq!(flag_ls.get(0),Some(false));
    /// assert_eq!(flag_ls.get(1),Some(true));
    /// # Ok(())
    /// # }
    /// ```
    fn try_from_iter<I: IntoIterator<Item=bool>>(v: I)->Result<Self,FlagLsError>{
        let mut out = Self::default();
        for item in v {
            out.try_push(item)?;
        }
        Ok(out)
    }
    /// build a list of flags of the specified length which is all true
    /// # Panics
    /// Panics when `len>MAX_LENGTH`
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let flag_ls=B64::all_true(10);
    /// for flag in flag_ls.iter(){
    ///     assert!(flag);
    /// }
    /// ```
    #[must_use]
    fn all_true(len: usize) -> Self {
        let mut out = Self::default();
        for _i in 0..len {
            out.push(true);
        }
        out
    }
    /// build a list of flags which is all false
    /// # Panics
    /// Panics when `len>MAX_LENGTH`
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let flag_ls=B64::all_false(10);
    /// for flag in flag_ls.iter(){
    ///     assert!(!flag);
    /// }
    /// ```
    #[must_use]
    fn all_false(len: usize) -> Self {
        let mut out = Self::default();
        for _i in 0..len {
            out.push(false);
        }
        out
    }
    /// Returns true when there are no flags in the list
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    ///
    /// let l1=B64::default();
    /// let l2=B64::all_true(1);
    ///
    /// assert!(l1.is_empty());
    /// assert!(!l2.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}