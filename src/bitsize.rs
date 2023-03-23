use std::ops::{Index, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, Not};

use crate::{FlagLs, flag_iter::FlagIter};

#[derive(PartialEq,Eq,Default,Clone,Copy, Debug,Hash)]
/// a list of flags/bitfield up to the size of a pointer
pub struct Bsize{
    inner: usize,
    len: usize
}
impl Bsize{
    fn lower_mask(point: usize)->usize{
        if point>Self::MAX_LENGTH{
            panic!("Cannot create mask more than {} bits for Bsize",Bsize::MAX_LENGTH)
        } else {
            (1<<point) -1
        }
    }
    fn inner(&self)->usize{
        self.inner
    }
    fn uper_mask(point:usize)->usize{
        if point>Self::MAX_LENGTH{
            panic!("Cannot mask above the end of the list");
        } else {
            usize::MAX-(1<<point)+1
        }
    }
    /// Create a new blank empty list of flags
    pub fn new()->Bsize{
        Bsize::default()
    }
    fn init(inner:usize,len:usize)->Bsize{
        Bsize{inner,len}
    }
}
impl Index<usize> for Bsize{
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
impl FlagLs for Bsize{
    const MAX_LENGTH: usize=usize::BITS as usize;

    fn len(&self)->usize {
        self.len
    }

    fn set_len(&mut self, new_len:usize){
        if new_len>Self::MAX_LENGTH{
            panic!("Cannot set length to a length larger than {} for Bsize",Self::MAX_LENGTH)
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
            self.inner=(uper<<1)+((flag as usize)<<index)+lower;
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
            self.inner=uper+((flag as usize)<<index)+lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self)->FlagIter<Bsize> {
        FlagIter::new(self)
    }


}
impl BitAnd<Bsize> for Bsize{
    type Output=Bsize;

    fn bitand(self, rhs: Bsize) -> Self::Output {
        Bsize::init(self.inner() & rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitAndAssign<Bsize> for Bsize{
    fn bitand_assign(&mut self, rhs: Bsize) {
        self.inner&=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitOr<Bsize> for Bsize{
    type Output=Bsize;
    fn bitor(self, rhs: Bsize) -> Self::Output {
        Bsize::init(self.inner() | rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitOrAssign<Bsize> for Bsize{
    fn bitor_assign(&mut self, rhs: Bsize) {
        self.inner|=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitXor<Bsize> for Bsize{
    type Output = Bsize;
    fn bitxor(self, rhs: Bsize) -> Self::Output {
        Bsize::init(self.inner().bitxor(rhs.inner()), self.len().max(rhs.len()))
    }
}
impl BitXorAssign<Bsize> for Bsize{
    fn bitxor_assign(&mut self, rhs: Bsize) {
        self.inner.bitxor_assign(rhs.inner());
        self.len=self.len.max(rhs.len());
    }
}
impl Shl<usize> for Bsize{
    type Output = Bsize;
    fn shl(self, rhs: usize) -> Self::Output {
        Bsize::init(self.inner<<rhs, (self.len+rhs).min(Self::MAX_LENGTH))
    }
}
#[allow(clippy::suspicious_op_assign_impl)] 
impl ShlAssign<usize> for Bsize{
    fn shl_assign(&mut self, rhs: usize) {
        self.inner.shl_assign(rhs);
        self.len=(self.len+rhs).min(Self::MAX_LENGTH);
    }
}
impl Shr<usize> for Bsize{
    type Output = Bsize;
    fn shr(self, rhs: usize) -> Self::Output {
        let new_len=if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
        Bsize::init(self.inner>>rhs, new_len)
    }
}
impl ShrAssign<usize> for Bsize{
    fn shr_assign(&mut self, rhs: usize) {
        self.inner.shr_assign(rhs);
        self.len = if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
    }
}
impl Not for Bsize{
    type Output = Bsize;
    fn not(self) -> Self::Output {
        Bsize::init((!self.inner)&Self::lower_mask(self.len) , self.len)
    }
}
impl Sub<Bsize> for Bsize{
    type Output = Bsize;
    ///note subtraction is set difference
    fn sub(self, rhs: Bsize) -> Self::Output {
        Bsize::init(self.inner & (!rhs.inner), self.len)
    }
}