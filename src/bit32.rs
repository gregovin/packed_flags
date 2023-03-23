use std::ops::{Index, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, Not};

use crate::{FlagLs, flag_iter::FlagIter};

#[derive(PartialEq,Eq,Default,Clone,Copy, Debug,Hash)]
/// A list of flags up to 32 flags long, or a 32 bit bitfield
pub struct B32{
    inner: u32,
    len: usize
}
impl B32{
    fn lower_mask(point: usize)->u32{
        if point>32{
            panic!("Cannot create mask more than 32 bits for B32")
        } else {
            (1<<point) -1
        }
    }
    fn inner(&self)->u32{
        self.inner
    }
    fn uper_mask(point:usize)->u32{
        if point>32{
            panic!("Cannot mask above the end of the list");
        } else {
            u32::MAX-(1<<point)+1
        }
    }
    pub fn new()->B32{
        B32::default()
    }
    fn init(inner:u32,len:usize)->B32{
        B32{inner,len}
    }
}
impl Index<usize> for B32{
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        if index>self.len{
            panic!("Cannot access out of bounds index");
        }
        if (self.inner>>index) & 1==1{
            &true
        } else {
            &false
        }
    }
}
impl FlagLs for B32{
    const MAX_LENGTH: usize=32;

    fn len(&self)->usize {
        self.len
    }

    fn set_len(&mut self, new_len:usize){
        if new_len>Self::MAX_LENGTH{
            panic!("Cannot set length to a length larger than 32 for B32")
        }
        self.len=new_len;
        self.inner &= (1<<new_len) -1;
    }

    fn insert(&mut self,index: usize,flag:bool) {
        if index>self.len{
            panic!("Cannot insert out of bounds");
        } else if self.len ==Self::MAX_LENGTH{
            panic!("cannot insert to full list of flags")
        }else {
            let uper = self.inner & Self::uper_mask(index);
            let lower = self.inner & Self::lower_mask(index);
            self.inner=(uper<<1)+((flag as u32)<<index)+lower;
            self.len+=1;
        }
    }

    fn remove(&mut self,index: usize)->bool {
        if index>=self.len{
            panic!("Cannot remove out of bounds");
        } else {
            let uper =self.inner & Self::uper_mask(index+1);
            let lower = self.inner & Self::lower_mask(index);
            let out = (self.inner >>index)&1;
            self.inner=(uper>>1)+lower;
            self.len-=1;
            out==1
        }
    }

    fn clear(&mut self) {
        self.inner=0;
        self.len=0;
    }

    unsafe fn get_unchecked(&self, index: usize)->bool {
        (self.inner>>index) & 1==1
    }

    fn set(&mut self,index:usize,flag:bool) {
        if index<self.len(){
            let uper = self.inner & Self::uper_mask(index+1);
            let lower=self.inner & Self::lower_mask(index);
            self.inner=uper+((flag as u32)<<index)+lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self)->FlagIter<B32> {
        FlagIter::new(self)
    }


}
impl BitAnd<B32> for B32{
    type Output=B32;

    fn bitand(self, rhs: B32) -> Self::Output {
        B32::init(self.inner() & rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitAndAssign<B32> for B32{
    fn bitand_assign(&mut self, rhs: B32) {
        self.inner&=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitOr<B32> for B32{
    type Output=B32;
    fn bitor(self, rhs: B32) -> Self::Output {
        B32::init(self.inner() | rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitOrAssign<B32> for B32{
    fn bitor_assign(&mut self, rhs: B32) {
        self.inner|=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitXor<B32> for B32{
    type Output = B32;
    fn bitxor(self, rhs: B32) -> Self::Output {
        B32::init(self.inner().bitxor(rhs.inner()), self.len().max(rhs.len()))
    }
}
impl BitXorAssign<B32> for B32{
    fn bitxor_assign(&mut self, rhs: B32) {
        self.inner.bitxor_assign(rhs.inner());
        self.len=self.len.max(rhs.len());
    }
}
impl Shl<usize> for B32{
    type Output = B32;
    fn shl(self, rhs: usize) -> Self::Output {
        B32::init(self.inner<<rhs, (self.len+rhs).min(Self::MAX_LENGTH))
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<usize> for B32{
    fn shl_assign(&mut self, rhs: usize) {
        self.inner.shl_assign(rhs);
        self.len=(self.len+rhs).min(Self::MAX_LENGTH);
    }
}
impl Shr<usize> for B32{
    type Output = B32;
    fn shr(self, rhs: usize) -> Self::Output {
        let new_len=if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
        B32::init(self.inner>>rhs, new_len)
    }
}
impl ShrAssign<usize> for B32{
    fn shr_assign(&mut self, rhs: usize) {
        self.inner.shr_assign(rhs);
        self.len = if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
    }
}
impl Not for B32{
    type Output = B32;
    fn not(self) -> Self::Output {
        B32::init((!self.inner)&Self::lower_mask(self.len) , self.len)
    }
}
impl Sub<B32> for B32{
    type Output = B32;
    ///note subtraction is set difference
    fn sub(self, rhs: B32) -> Self::Output {
        B32::init(self.inner & (!rhs.inner), self.len)
    }
}