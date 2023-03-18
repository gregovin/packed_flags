//!Provides various packed lists of flags(ie equivalent to Vec<bool>)
//!Useful anywhere you are tempted to use Vec<bool> or \[bool\], but want some amount of memory efficiency
use std::ops::Index;
use std::fmt::Debug;
use std::hash::Hash;
mod bit64;
mod bit32;
mod bit128;
mod flag_iter;
pub use crate::bit64::b64;
pub use crate::bit32::b32;
pub use crate::bit128::b128;
pub use crate::flag_iter::FlagIter;

/// A trat that represents the functionality we want
/// Mostly similar to Vec<bool> with a few changes
pub trait FlagLs:Sized+ PartialEq+Eq+Default+Clone+Debug+Hash+Index<usize>{
    /// Should be the max length a given flag ls can store
    const MAX_LENGTH: usize;
    /// Should get the number of flags in the flags
    fn len(&self)->usize;
    /// should set the length of the flag ls, may or may not zero the left over flags
    /// # Panics
    /// Should panic if the new length is larger than MAX_LENGTH
    fn set_len(&mut self, new_len:usize);
    /// should insert a flag at position index
    /// # Panics
    /// Should panic if the new length is larger than MAX_LENGTH
    fn insert(&mut self,index: usize,flag:bool);
    /// should remove a flag at position index
    /// # Panics
    /// Should panic if the index is out of bounds
    fn remove(&mut self,index: usize)->bool;
    /// should clear the flags and set the number to 0
    fn clear(&mut self);

    /// Truncate the flags to len. Does nothing if len>self.len()
    fn truncate(&mut self,len: usize){
        if len<self.len(){
            self.set_len(len);
        }
    }
    /// pushes a new flag to the end of the list
    /// # Panics
    /// Panics if the resulting list is too big
    fn push(&mut self,flag:bool){
        self.insert(self.len(),flag);
    }
    /// removes the last flag, if it exists
    fn pop(&mut self)->Option<bool>{
        if self.len()>0{
            Some(self.remove(self.len()-1))
        } else {
            None
        }
    }
    /// get the flag at a specified index
    fn get(&self, index: usize)->Option<bool>;
    /// set the flag at a specified index
    fn set(&mut self,index:usize,flag:bool);
    /// get an iterator over all flags
    fn iter(&self)->FlagIter<Self>;
    /// build a list of flags from a vector of flags
    /// # Panics
    /// Panics when v.len()>MAX_LENGTH
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
    /// build a list of flags which is all true
    /// # Panics
    /// Panics when len>MAX_LENGTH
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
    fn all_false(len: usize)->Self{
        let mut out=Self::default();
        for _i in 0..len{
            out.push(false);
        }
        out
    }
    /// Returns true when there are no flags
    fn is_empty(&self)->bool{
        self.len()==0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    
}
