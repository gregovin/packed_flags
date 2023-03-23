use std::ops::{Index, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, Not};

use crate::{FlagLs, flag_iter::FlagIter};

#[derive(PartialEq,Eq,Default,Clone,Copy, Debug,Hash)]
#[allow(non_camel_case_types)]
/// A list of flags up to 128 flags long, or a 128 bit bitfield
pub struct b128{
    inner: u128,
    len: usize
}
impl b128{
    fn lower_mask(point: usize)->u128{
        if point>128{
            panic!("Cannot create mask more than 128 bits for b128")
        } else {
            (1<<point) -1
        }
    }
    fn inner(&self)->u128{
        self.inner
    }
    fn uper_mask(point:usize)->u128{
        if point>128{
            panic!("Cannot mask above the end of the list");
        } else {
            u128::MAX-(1<<point)+1
        }
    }
    pub fn new()->b128{
        b128::default()
    }
    fn init(inner:u128,len:usize)->b128{
        b128{inner,len}
    }
}
impl Index<usize> for b128{
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
impl FlagLs for b128{
    const MAX_LENGTH: usize=128;

    fn len(&self)->usize {
        self.len
    }

    fn set_len(&mut self, new_len:usize){
        if new_len>Self::MAX_LENGTH{
            panic!("Cannot set length to a length larger than 128 for b128")
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
            self.inner=(uper<<1)+((flag as u128)<<index)+lower;
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

    fn get(&self, index: usize)->Option<bool> {
        if index<self.len(){
            Some(self[index])
        } else {
            None
        }
    }

    fn set(&mut self,index:usize,flag:bool) {
        if index<self.len(){
            let uper = self.inner & Self::uper_mask(index+1);
            let lower=self.inner & Self::lower_mask(index);
            self.inner=uper+((flag as u128)<<index)+lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self)->FlagIter<b128> {
        FlagIter::new(self)
    }


}
impl BitAnd<b128> for b128{
    type Output=b128;

    fn bitand(self, rhs: b128) -> Self::Output {
        b128::init(self.inner() & rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitAndAssign<b128> for b128{
    fn bitand_assign(&mut self, rhs: b128) {
        self.inner&=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitOr<b128> for b128{
    type Output=b128;
    fn bitor(self, rhs: b128) -> Self::Output {
        b128::init(self.inner() | rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitOrAssign<b128> for b128{
    fn bitor_assign(&mut self, rhs: b128) {
        self.inner|=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitXor<b128> for b128{
    type Output = b128;
    fn bitxor(self, rhs: b128) -> Self::Output {
        b128::init(self.inner().bitxor(rhs.inner()), self.len().max(rhs.len()))
    }
}
impl BitXorAssign<b128> for b128{
    fn bitxor_assign(&mut self, rhs: b128) {
        self.inner.bitxor_assign(rhs.inner());
        self.len=self.len.max(rhs.len());
    }
}
impl Shl<usize> for b128{
    type Output = b128;
    fn shl(self, rhs: usize) -> Self::Output {
        b128::init(self.inner<<rhs, (self.len+rhs).min(Self::MAX_LENGTH))
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<usize> for b128{
    fn shl_assign(&mut self, rhs: usize) {
        self.inner.shl_assign(rhs);
        self.len=(self.len+rhs).min(Self::MAX_LENGTH);
    }
}
impl Shr<usize> for b128{
    type Output = b128;
    fn shr(self, rhs: usize) -> Self::Output {
        let new_len=if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
        b128::init(self.inner>>rhs, new_len)
    }
}
impl ShrAssign<usize> for b128{
    fn shr_assign(&mut self, rhs: usize) {
        self.inner.shr_assign(rhs);
        self.len = if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
    }
}
impl Not for b128{
    type Output = b128;
    fn not(self) -> Self::Output {
        b128::init((!self.inner)&Self::lower_mask(self.len) , self.len)
    }
}
impl Sub<b128> for b128{
    type Output = b128;
    ///note subtraction is set difference
    fn sub(self, rhs: b128) -> Self::Output {
        b128::init(self.inner & (!rhs.inner), self.len)
    }
}