use std::ops::{Index, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, Not};

use crate::{FlagLs, flag_iter::FlagIter};

#[derive(PartialEq,Eq,Default,Clone,Copy, Debug,Hash)]
#[allow(non_camel_case_types)]
/// Up to 64 bit bitfields
pub struct b64{
    inner: u64,
    len: usize
}
impl b64{
    fn lower_mask(point: usize)->u64{
        if point>64{
            panic!("Cannot create mask more than 64 bits")
        } else {
            (1<<point) -1
        }
    }
    fn inner(&self)->u64{
        self.inner
    }
    fn uper_mask(point:usize)->u64{
        if point>64{
            panic!("Cannot mask above the end of the list");
        }
        u64::MAX-(1<<point)+1
    }
    pub fn new()->b64{
        b64::default()
    }
    fn init(inner:u64,len:usize)->b64{
        b64{inner,len}
    }
}
impl Index<usize> for b64{
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
impl FlagLs for b64{
    const MAX_LENGTH: usize=64;

    fn len(&self)->usize {
        self.len
    }

    fn set_len(&mut self, new_len:usize){
        if new_len>Self::MAX_LENGTH{
            panic!("Cannot set length to a length larger than 64 for b64")
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
            let uper = self.inner * Self::uper_mask(index);
            let lower = self.inner* Self::lower_mask(index);
            self.inner=(uper<<1)+((flag as u64)<<index)+lower;
            self.len+=1;
        }
    }

    fn remove(&mut self,index: usize)->bool {
        if index>=self.len{
            panic!("Cannot remove out of bounds");
        } else {
            let uper =self.inner *Self::uper_mask(index+1);
            let lower = self.inner *Self::lower_mask(index);
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
            let uper = self.inner *Self::uper_mask(index+1);
            let lower=self.inner* Self::lower_mask(index);
            self.inner=uper+((flag as u64)<<index)+lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self)->FlagIter<b64> {
        FlagIter::new(self)
    }


}
impl BitAnd<b64> for b64{
    type Output=b64;

    fn bitand(self, rhs: b64) -> Self::Output {
        b64::init(self.inner() & rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitAndAssign<b64> for b64{
    fn bitand_assign(&mut self, rhs: b64) {
        self.inner&=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitOr<b64> for b64{
    type Output=b64;
    fn bitor(self, rhs: b64) -> Self::Output {
        b64::init(self.inner() | rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitOrAssign<b64> for b64{
    fn bitor_assign(&mut self, rhs: b64) {
        self.inner|=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitXor<b64> for b64{
    type Output = b64;
    fn bitxor(self, rhs: b64) -> Self::Output {
        b64::init(self.inner().bitxor(rhs.inner()), self.len().max(rhs.len()))
    }
}
impl BitXorAssign<b64> for b64{
    fn bitxor_assign(&mut self, rhs: b64) {
        self.inner.bitxor_assign(rhs.inner());
        self.len=self.len.max(rhs.len());
    }
}
impl Shl<usize> for b64{
    type Output = b64;
    fn shl(self, rhs: usize) -> Self::Output {
        b64::init(self.inner<<rhs, (self.len+rhs).min(Self::MAX_LENGTH))
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<usize> for b64{
    fn shl_assign(&mut self, rhs: usize) {
        self.inner.shl_assign(rhs);
        self.len=(self.len+rhs).min(Self::MAX_LENGTH);
    }
}
impl Shr<usize> for b64{
    type Output = b64;
    fn shr(self, rhs: usize) -> Self::Output {
        let new_len=if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
        b64::init(self.inner>>rhs, new_len)
    }
}
impl ShrAssign<usize> for b64{
    fn shr_assign(&mut self, rhs: usize) {
        self.inner.shr_assign(rhs);
        self.len = if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
    }
}
impl Not for b64{
    type Output = b64;
    fn not(self) -> Self::Output {
        b64::init((!self.inner)&Self::lower_mask(self.len) , self.len)
    }
}
impl Sub<b64> for b64{
    type Output = b64;
    ///note subtraction is set difference
    fn sub(self, rhs: b64) -> Self::Output {
        b64::init(self.inner & (!rhs.inner), self.len)
    }
}