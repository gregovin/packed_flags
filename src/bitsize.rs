use std::ops::{Index, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, Not};

use crate::{FlagLs, flag_iter::FlagIter};

#[derive(PartialEq,Eq,Default,Clone,Copy, Debug,Hash)]
#[allow(non_camel_case_types)]
/// Up bitfields the size of a pointer
pub struct bsize{
    inner: usize,
    len: usize
}
impl bsize{
    fn lower_mask(point: usize)->usize{
        if point>Self::MAX_LENGTH{
            panic!("Cannot create mask more than {} bits for bsize",bsize::MAX_LENGTH)
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
        }
        usize::MAX-(1<<point)+1
    }
    pub fn new()->bsize{
        bsize::default()
    }
    fn init(inner:usize,len:usize)->bsize{
        bsize{inner,len}
    }
}
impl Index<usize> for bsize{
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
impl FlagLs for bsize{
    const MAX_LENGTH: usize=usize::BITS as usize;

    fn len(&self)->usize {
        self.len
    }

    fn set_len(&mut self, new_len:usize){
        if new_len>Self::MAX_LENGTH{
            panic!("Cannot set length to a length larger than {} for bsize",Self::MAX_LENGTH)
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
            self.inner=(uper<<1)+((flag as usize)<<index)+lower;
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
            self.inner=uper+((flag as usize)<<index)+lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self)->FlagIter<bsize> {
        FlagIter::new(self)
    }


}
impl BitAnd<bsize> for bsize{
    type Output=bsize;

    fn bitand(self, rhs: bsize) -> Self::Output {
        bsize::init(self.inner() & rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitAndAssign<bsize> for bsize{
    fn bitand_assign(&mut self, rhs: bsize) {
        self.inner&=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitOr<bsize> for bsize{
    type Output=bsize;
    fn bitor(self, rhs: bsize) -> Self::Output {
        bsize::init(self.inner() | rhs.inner(), self.len().max(rhs.len()))
    }
}
impl BitOrAssign<bsize> for bsize{
    fn bitor_assign(&mut self, rhs: bsize) {
        self.inner|=rhs.inner();
        self.len=self.len.max(rhs.len());
    }
}
impl BitXor<bsize> for bsize{
    type Output = bsize;
    fn bitxor(self, rhs: bsize) -> Self::Output {
        bsize::init(self.inner().bitxor(rhs.inner()), self.len().max(rhs.len()))
    }
}
impl BitXorAssign<bsize> for bsize{
    fn bitxor_assign(&mut self, rhs: bsize) {
        self.inner.bitxor_assign(rhs.inner());
        self.len=self.len.max(rhs.len());
    }
}
impl Shl<usize> for bsize{
    type Output = bsize;
    fn shl(self, rhs: usize) -> Self::Output {
        bsize::init(self.inner<<rhs, (self.len+rhs).min(Self::MAX_LENGTH))
    }
}
#[allow(clippy::suspicious_op_assign_impl)] 
impl ShlAssign<usize> for bsize{
    fn shl_assign(&mut self, rhs: usize) {
        self.inner.shl_assign(rhs);
        self.len=(self.len+rhs).min(Self::MAX_LENGTH);
    }
}
impl Shr<usize> for bsize{
    type Output = bsize;
    fn shr(self, rhs: usize) -> Self::Output {
        let new_len=if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
        bsize::init(self.inner>>rhs, new_len)
    }
}
impl ShrAssign<usize> for bsize{
    fn shr_assign(&mut self, rhs: usize) {
        self.inner.shr_assign(rhs);
        self.len = if self.len>rhs{
            self.len-rhs
        } else {
            0
        };
    }
}
impl Not for bsize{
    type Output = bsize;
    fn not(self) -> Self::Output {
        bsize::init((!self.inner)&Self::lower_mask(self.len) , self.len)
    }
}
impl Sub<bsize> for bsize{
    type Output = bsize;
    ///note subtraction is set difference
    fn sub(self, rhs: bsize) -> Self::Output {
        bsize::init(self.inner & (!rhs.inner), self.len)
    }
}