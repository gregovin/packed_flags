//!Provides various packed lists of flags(ie equivalent to `Vec<bool>`).
//!Useful anywhere you are tempted to use `Vec<bool>` or `[bool]`, but want some amount of memory efficiency
use std::ops::Index;
use std::fmt::Debug;
use std::hash::Hash;
mod bit64;
mod bit32;
mod bitsize;
mod bit128;
mod bitlong;
pub mod flag_iter;
pub use crate::bit64::B64;
pub use crate::bit32::B32;
pub use crate::bit128::B128;
pub use crate::bitsize::Bsize;
pub use crate::bitlong::Blong;

/// A trait that represents a list of flags.
/// 
/// Mostly the same as things that would be implemented by `Vec<bool>` with a few omisions
pub trait FlagLs:Sized+ PartialEq+Eq+Default+Clone+Debug+Hash+Index<usize>{
    /// The max length a given flag list can store
    const MAX_LENGTH: usize;
    /// Returns number of flags in the flags list
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let flag_ls=B64::from_vec(vec![true,false,false]);
    /// assert_eq!(flag_ls.len(),3);
    /// ```
    fn len(&self)->usize;
    /// Sets the length of the flag ls, zeroing any flags removed
    /// # Panics
    /// Panics if the new length is larger than MAX_LENGTH
    fn set_len(&mut self, new_len:usize);
    /// Inserts a new flag at the position given by index(ie so `flag_ls[index]=flag`)
    /// # Panics
    /// Panics if the index is out of bounds or the new element would make the list longer than MAX_LENGTH
    /// # Examples  
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let mut flag_ls=B64::default();
    /// flag_ls.insert(0,true);
    /// flag_ls.insert(0,false);
    /// flag_ls.insert(2,false);
    /// assert_eq!(flag_ls,B64::from_vec(vec![false,true,false]));
    /// ```
    fn insert(&mut self,index: usize,flag:bool);
    /// Removes the flag at the position given by index, and returns it
    /// # Panics
    /// Panics if the index is out of bounds
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let mut flag_ls=B64::from_vec(vec![false,true,false]);
    /// assert!(flag_ls.remove(1));
    /// assert_eq!(flag_ls,B64::from_vec(vec![false,false]));
    /// ```
    fn remove(&mut self,index: usize)->bool;
    /// Clears the list, setting the internal state and length to 0
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let mut flag_ls=B64::from_vec(vec![false,true,false]);
    /// flag_ls.clear();
    /// assert_eq!(flag_ls,B64::default());
    /// flag_ls.set_len(1);
    /// assert_eq!(flag_ls[0],false);
    /// ```
    fn clear(&mut self);

    /// Truncate the flags to len. Does nothing if len>self.len(
    fn truncate(&mut self,len: usize){
        if len<self.len(){
            self.set_len(len);
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
    /// assert_eq!(flag_ls,B64::from_vec(vec![false,true,false]));
    /// ```
    fn push(&mut self,flag:bool){
        self.insert(self.len(),flag);
    }
    /// removes and returns the last flag, or none if the list is empty
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let mut flag_ls=B64::from_vec(vec![false,true]);
    /// assert_eq!(flag_ls.pop(),Some(true));
    /// assert_eq!(flag_ls.pop(),Some(false));
    /// assert_eq!(flag_ls.pop(),None);
    /// ```
    fn pop(&mut self)->Option<bool>{
        if self.len()>0{
            Some(self.remove(self.len()-1))
        } else {
            None
        }
    }
    /// Returns the element at index without performing bounds checking
    /// 
    /// For a safe alternative see [get][FlagLs::get]
    /// # Safety
    /// Calling this function with an out of bounds index may result in undefined behavior, or unpredictible/garbage outputs
    unsafe fn get_unchecked(&self,index:usize)->bool;
    /// Get the flag at a specified index, if it exists, otherwise returns None
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let flag_ls=B64::from_vec(vec![false,true]);
    /// assert_eq!(flag_ls.get(1),Some(true));
    /// assert_eq!(flag_ls.get(2),None);
    /// ```
    fn get(&self, index: usize)->Option<bool>{
        if index<self.len(){
            unsafe{
                Some(self.get_unchecked(index))
            }
        } else {
            None
        }
    }
    /// Set the flag at a specified index
    /// # Panics
    /// If the index is out of bounds
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let mut flag_ls=B64::from_vec(vec![false,true,false]);
    /// flag_ls.set(2,true);
    /// assert_eq!(flag_ls,B64::from_vec(vec![false,true,true]));
    /// ```
    fn set(&mut self,index:usize,flag:bool);
    /// Sets the flag at the specified index, returning the flag that was there if the index is in bounds
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let mut flag_ls=B64::from_vec(vec![false,true,false]);
    /// assert_eq!(flag_ls.try_set(2,true),Some(false));
    /// assert_eq!(flag_ls,B64::from_vec(vec![false,true,true]));
    /// assert_eq!(flag_ls.try_set(3,false),None);
    /// ```
    fn try_set(&mut self,index: usize,flag:bool)->Option<bool>{
        match self.get(index){
            Some(b)=>{
                self.set(index,flag);
                Some(b)},
            None=>None
        }
    }
    /// get an iterator over all flags in the list
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let flag_ls=B64::from_vec(vec![false,true]);
    /// let mut itr=flag_ls.iter();
    /// assert_eq!(itr.next(),Some(false));
    /// assert_eq!(itr.next(),Some(true));
    /// assert_eq!(itr.next(),None);
    /// ```
    fn iter(&self)->flag_iter::Iter<Self>;
    /// build a compact list of flags from a vector of flags
    /// # Panics
    /// Panics when v.len()>MAX_LENGTH
    /// # Examples
    /// ```
    /// use packed_flags::B64;
    /// use packed_flags::FlagLs;
    /// 
    /// let flag_ls=B64::from_vec(vec![false,true]);
    /// assert_eq!(flag_ls.len(),2);
    /// assert_eq!(flag_ls.get(0),Some(false));
    /// assert_eq!(flag_ls.get(1),Some(true));
    /// ```
    fn from_vec(v: Vec<bool>)->Self{
        if v.len()>Self::MAX_LENGTH{
            panic!("Cannot pack more than {} flags into this struct",{Self::MAX_LENGTH})
        } else {
            let mut out=Self::default();
            for item in v{
                out.push(item);
            }
            out
        }
    }
    /// build a list of flags of the specified length which is all true
    /// # Panics
    /// Panics when len>MAX_LENGTH
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
    fn all_true(len: usize)->Self{
        let mut out=Self::default();
        for _i in 0..len{
            out.push(true);
        }
        out
    }
    /// build a list of flags which is all false
    /// # Panics
    /// Panics when len>MAX_LENGTH
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
    fn all_false(len: usize)->Self{
        let mut out=Self::default();
        for _i in 0..len{
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
    fn is_empty(&self)->bool{
        self.len()==0
    }
}
#[cfg(test)]
// runs all the doc test on the other ones
mod tests {
    use super::*;
    #[test]
    fn len(){
        let flag_ls1=B32::from_vec(vec![true,false,false]);
        let flag_ls2=B128::from_vec(vec![true,false,false]);
        let flag_ls3=Bsize::from_vec(vec![true,false,false]);
        let flag_ls4=Blong::from_vec(vec![true,false,false]);
        assert_eq!(flag_ls1.len(),3);
        assert_eq!(flag_ls2.len(),3);
        assert_eq!(flag_ls3.len(),3);
        assert_eq!(flag_ls4.len(),3);
    }
    #[test]
    fn insert(){
        let mut flag_ls1=B32::default();
        flag_ls1.insert(0,true);
        flag_ls1.insert(0,false);
        flag_ls1.insert(2,false);
        assert_eq!(flag_ls1,B32::from_vec(vec![false,true,false]));

        let mut flag_ls2=B128::default();
        flag_ls2.insert(0,true);
        flag_ls2.insert(0,false);
        flag_ls2.insert(2,false);
        assert_eq!(flag_ls2,B128::from_vec(vec![false,true,false]));

        let mut flag_ls3=Bsize::default();
        flag_ls3.insert(0,true);
        flag_ls3.insert(0,false);
        flag_ls3.insert(2,false);
        assert_eq!(flag_ls3,Bsize::from_vec(vec![false,true,false]));

        let mut flag_ls4=Blong::default();
        flag_ls4.insert(0,true);
        flag_ls4.insert(0,false);
        flag_ls4.insert(2,false);
        assert_eq!(flag_ls4,Blong::from_vec(vec![false,true,false]));
    }
    #[test]
    fn remove(){
        let mut flag_ls=B32::from_vec(vec![false,true,false]);
        assert!(flag_ls.remove(1));
        assert_eq!(flag_ls,B32::from_vec(vec![false,false]));

        let mut flag_ls=B128::from_vec(vec![false,true,false]);
        assert!(flag_ls.remove(1));
        assert_eq!(flag_ls,B128::from_vec(vec![false,false]));

        let mut flag_ls=Bsize::from_vec(vec![false,true,false]);
        assert!(flag_ls.remove(1));
        assert_eq!(flag_ls,Bsize::from_vec(vec![false,false]));

        let mut flag_ls=Blong::from_vec(vec![false,true,false]);
        assert!(flag_ls.remove(1));
        assert_eq!(flag_ls,Blong::from_vec(vec![false,false]));
    }
    #[test]
    fn clear(){
        let mut flag_ls=B32::from_vec(vec![false,true,false]);
        flag_ls.clear();
        assert_eq!(flag_ls,B32::default());
        flag_ls.set_len(1);
        assert_eq!(flag_ls[0],false);

        let mut flag_ls=B128::from_vec(vec![false,true,false]);
        flag_ls.clear();
        assert_eq!(flag_ls,B128::default());
        flag_ls.set_len(1);
        assert_eq!(flag_ls[0],false);

        let mut flag_ls=Bsize::from_vec(vec![false,true,false]);
        flag_ls.clear();
        assert_eq!(flag_ls,Bsize::default());
        flag_ls.set_len(1);
        assert_eq!(flag_ls[0],false);

        let mut flag_ls=Blong::from_vec(vec![false,true,false]);
        flag_ls.clear();
        assert_eq!(flag_ls,Blong::default());
        flag_ls.set_len(1);
        assert_eq!(flag_ls[0],false);
    }
    #[test]
    fn push(){
        let mut flag_ls=B32::default();
        flag_ls.push(false);
        flag_ls.push(true);
        flag_ls.push(false);
        assert_eq!(flag_ls,B32::from_vec(vec![false,true,false]));

        let mut flag_ls=B128::default();
        flag_ls.push(false);
        flag_ls.push(true);
        flag_ls.push(false);
        assert_eq!(flag_ls,B128::from_vec(vec![false,true,false]));

        let mut flag_ls=Bsize::default();
        flag_ls.push(false);
        flag_ls.push(true);
        flag_ls.push(false);
        assert_eq!(flag_ls,Bsize::from_vec(vec![false,true,false]));

        let mut flag_ls=Blong::default();
        flag_ls.push(false);
        flag_ls.push(true);
        flag_ls.push(false);
        assert_eq!(flag_ls,Blong::from_vec(vec![false,true,false]));
    }
    #[test]
    fn pop(){
        let mut flag_ls=B32::from_vec(vec![false,true]);
        assert_eq!(flag_ls.pop(),Some(true));
        assert_eq!(flag_ls.pop(),Some(false));
        assert_eq!(flag_ls.pop(),None);

        let mut flag_ls=B128::from_vec(vec![false,true]);
        assert_eq!(flag_ls.pop(),Some(true));
        assert_eq!(flag_ls.pop(),Some(false));
        assert_eq!(flag_ls.pop(),None);

        let mut flag_ls=Bsize::from_vec(vec![false,true]);
        assert_eq!(flag_ls.pop(),Some(true));
        assert_eq!(flag_ls.pop(),Some(false));
        assert_eq!(flag_ls.pop(),None);

        let mut flag_ls=Blong::from_vec(vec![false,true]);
        assert_eq!(flag_ls.pop(),Some(true));
        assert_eq!(flag_ls.pop(),Some(false));
        assert_eq!(flag_ls.pop(),None);
    }
    #[test]
    fn get(){
        let flag_ls=B32::from_vec(vec![false,true]);
        assert_eq!(flag_ls.get(1),Some(true));
        assert_eq!(flag_ls.get(2),None);

        let flag_ls=B128::from_vec(vec![false,true]);
        assert_eq!(flag_ls.get(1),Some(true));
        assert_eq!(flag_ls.get(2),None);

        let flag_ls=Bsize::from_vec(vec![false,true]);
        assert_eq!(flag_ls.get(1),Some(true));
        assert_eq!(flag_ls.get(2),None);

        let flag_ls=Blong::from_vec(vec![false,true]);
        assert_eq!(flag_ls.get(1),Some(true));
        assert_eq!(flag_ls.get(2),None);
    }
    #[test]
    fn set(){
        let mut flag_ls=B32::from_vec(vec![false,true,false]);
        flag_ls.set(2,true);
        assert_eq!(flag_ls,B32::from_vec(vec![false,true,true]));

        let mut flag_ls=B128::from_vec(vec![false,true,false]);
        flag_ls.set(2,true);
        assert_eq!(flag_ls,B128::from_vec(vec![false,true,true]));

        let mut flag_ls=Bsize::from_vec(vec![false,true,false]);
        flag_ls.set(2,true);
        assert_eq!(flag_ls,Bsize::from_vec(vec![false,true,true]));

        let mut flag_ls=Blong::from_vec(vec![false,true,false]);
        flag_ls.set(2,true);
        assert_eq!(flag_ls,Blong::from_vec(vec![false,true,true]));
    }
    #[test]
    fn try_set(){
        let mut flag_ls=B32::from_vec(vec![false,true,false]);
        assert_eq!(flag_ls.try_set(2,true),Some(false));
        assert_eq!(flag_ls,B32::from_vec(vec![false,true,true]));
        assert_eq!(flag_ls.try_set(3,false),None);

        let mut flag_ls=B128::from_vec(vec![false,true,false]);
        assert_eq!(flag_ls.try_set(2,true),Some(false));
        assert_eq!(flag_ls,B128::from_vec(vec![false,true,true]));
        assert_eq!(flag_ls.try_set(3,false),None);

        let mut flag_ls=Bsize::from_vec(vec![false,true,false]);
        assert_eq!(flag_ls.try_set(2,true),Some(false));
        assert_eq!(flag_ls,Bsize::from_vec(vec![false,true,true]));
        assert_eq!(flag_ls.try_set(3,false),None);

        let mut flag_ls=Blong::from_vec(vec![false,true,false]);
        assert_eq!(flag_ls.try_set(2,true),Some(false));
        assert_eq!(flag_ls,Blong::from_vec(vec![false,true,true]));
        assert_eq!(flag_ls.try_set(3,false),None);
    }
    #[test]
    fn iter(){
        let flag_ls=B32::from_vec(vec![false,true]);
        let mut itr=flag_ls.iter();
        assert_eq!(itr.next(),Some(false));
        assert_eq!(itr.next(),Some(true));
        assert_eq!(itr.next(),None);

        let flag_ls=B128::from_vec(vec![false,true]);
        let mut itr=flag_ls.iter();
        assert_eq!(itr.next(),Some(false));
        assert_eq!(itr.next(),Some(true));
        assert_eq!(itr.next(),None);

        let flag_ls=Bsize::from_vec(vec![false,true]);
        let mut itr=flag_ls.iter();
        assert_eq!(itr.next(),Some(false));
        assert_eq!(itr.next(),Some(true));
        assert_eq!(itr.next(),None);

        let flag_ls=Blong::from_vec(vec![false,true]);
        let mut itr=flag_ls.iter();
        assert_eq!(itr.next(),Some(false));
        assert_eq!(itr.next(),Some(true));
        assert_eq!(itr.next(),None);
    }
    #[test]
    fn from_vec(){
        let flag_ls=B32::from_vec(vec![false,true]);
        assert_eq!(flag_ls.len(),2);
        assert_eq!(flag_ls.get(0),Some(false));
        assert_eq!(flag_ls.get(1),Some(true));

        let flag_ls=B128::from_vec(vec![false,true]);
        assert_eq!(flag_ls.len(),2);
        assert_eq!(flag_ls.get(0),Some(false));
        assert_eq!(flag_ls.get(1),Some(true));

        let flag_ls=Bsize::from_vec(vec![false,true]);
        assert_eq!(flag_ls.len(),2);
        assert_eq!(flag_ls.get(0),Some(false));
        assert_eq!(flag_ls.get(1),Some(true));

        let flag_ls=Blong::from_vec(vec![false,true]);
        assert_eq!(flag_ls.len(),2);
        assert_eq!(flag_ls.get(0),Some(false));
        assert_eq!(flag_ls.get(1),Some(true));
    }
    #[test]
    fn all_true(){
        let flag_ls=B32::all_true(10);
        for flag in flag_ls.iter(){
            assert!(flag);
        }

        let flag_ls=B128::all_true(10);
        for flag in flag_ls.iter(){
            assert!(flag);
        }

        let flag_ls=Bsize::all_true(10);
        for flag in flag_ls.iter(){
            assert!(flag);
        }

        let flag_ls=Blong::all_true(10);
        for flag in flag_ls.iter(){
            assert!(flag);
        }
    }
    #[test]
    fn all_false(){
        let flag_ls=B32::all_false(10);
        for flag in flag_ls.iter(){
            assert!(!flag);
        }

        let flag_ls=B128::all_false(10);
        for flag in flag_ls.iter(){
            assert!(!flag);
        }

        let flag_ls=Bsize::all_false(10);
        for flag in flag_ls.iter(){
            assert!(!flag);
        }

        let flag_ls=Blong::all_false(10);
        for flag in flag_ls.iter(){
            assert!(!flag);
        }
    }
    #[test]
    fn is_empty(){
        let l1=B32::default();
        let l2=B32::all_true(1);
        assert!(l1.is_empty());
        assert!(!l2.is_empty());

        let l1=B128::default();
        let l2=B128::all_true(1);
        assert!(l1.is_empty());
        assert!(!l2.is_empty());

        let l1=Bsize::default();
        let l2=Bsize::all_true(1);
        assert!(l1.is_empty());
        assert!(!l2.is_empty());

        let l1=Blong::default();
        let l2=Blong::all_true(1);
        assert!(l1.is_empty());
        assert!(!l2.is_empty());
    }
}
