use std::ops::{Index, BitAndAssign, BitOrAssign, BitXorAssign, Not, SubAssign};

use crate::{FlagLs, flag_iter, B32, Bsize, B64, B128};
/// An arbitrarily long list of flags
/// 
/// You should use b32,b64, or b128 instead unless you really need a lot of flags
#[derive(PartialEq,Eq,Default,Clone, Debug,Hash)]
pub struct Blong{
    inner: Vec<usize>,
    len: usize
}
impl Blong{
    const INNER_SIZE: usize=usize::BITS as usize;
    fn lower_mask(inner_point: usize)->usize{
        if inner_point>Self::INNER_SIZE{
            panic!("Cannot mask more than pointer size for Blong")
        } else {
            (1<<inner_point)-1
        }
    }
    fn inner(&self)->&Vec<usize>{
        &self.inner
    }
    /// Converts the bitfield into its inner representation, a list of integers, consuming it
    pub fn as_inner(self)->Vec<usize>{
        self.inner
    }
    fn uper_mask(inner_point: usize)->usize{
        if inner_point>Self::INNER_SIZE{
            panic!("Cannot mask above the end of the list")
        } else {
            usize::MAX-(1<<inner_point)+1
        }
    }
    #[allow(dead_code)]
    /// Creates an empty list of flags
    pub fn new()->Blong{
        Blong { inner: vec![], len: 0 }
    }
}
impl Index<usize> for Blong{
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        if index>self.len{
            panic!("Index out of bounds")
        } else {
            let (t_index,m_index)=(index/Self::INNER_SIZE,index%Self::INNER_SIZE);
            if (self.inner[t_index]>>m_index) & 1==1{
                &true
            } else {
                &false
            }
        }
    }
}
impl FlagLs for Blong{
    const MAX_LENGTH: usize=usize::MAX;

    fn len(&self)->usize {
        self.len
    }

    fn set_len(&mut self, new_len:usize) {
        let t_len=(new_len/Self::INNER_SIZE)+1;
        match t_len.cmp(&self.inner.len()){
            std::cmp::Ordering::Greater=>{
                let mut new:Vec<usize> =vec![0;t_len-self.inner.len()];
                self.inner.append(&mut new);
            },
            std::cmp::Ordering::Less=>{
                let _=self.inner.drain(t_len..);
            }
            _=>{}
        }
        self.len=new_len;
    }

    fn insert(&mut self,index: usize,flag:bool) {
        if index>self.len{
            panic!("Index out of bounds");
        } else {
            let mut t_index=index/Self::INNER_SIZE;
            let m_index=index%Self::INNER_SIZE;
            if t_index==self.len{
                self.inner.push(flag as usize);
                self.len+=1;
            } else {
                let lower = self.inner[t_index] & Self::lower_mask(m_index);
                let upper=self.inner[t_index] & Self::uper_mask(m_index);
                let mut top=self.inner[t_index]>>(Self::INNER_SIZE-1);
                
                self.inner[t_index] = (upper<<1)+((flag as usize)<<m_index)+lower;
                t_index+=1;
                while t_index<self.inner.len(){
                    let old=top;
                    top=self.inner[t_index]>>(Self::INNER_SIZE-1);
                    self.inner[t_index]=(self.inner[t_index]<<1)+old;
                    t_index+=1;
                }
                if m_index==Self::INNER_SIZE-1{
                    self.inner.push(top);
                }
                self.len+=1;
            }
        }
    }

    fn remove(&mut self,index: usize)->bool {
        if index>=self.len{
            panic!("Index out of bounds")
        } else {
            let t_index=index/Self::INNER_SIZE;
            let m_indx=index%Self::INNER_SIZE;
            let mut top_idx=self.inner.len()-1;
            let mut bot: usize=0;

            while top_idx>t_index{
                let old=bot;
                bot=self.inner[top_idx]&1;
                self.inner[top_idx]=(self.inner[top_idx]>>1)+(old<<(Self::INNER_SIZE-1));
                top_idx-=1;
            }
            let upper=self.inner[t_index] &Self::uper_mask(m_indx+1);
            let lower=self.inner[t_index]&Self::lower_mask(m_indx);
            let out = (self.inner[t_index]>>m_indx)&1;
            self.inner[t_index]=lower+(upper>>1)+(bot<<(Self::INNER_SIZE-1));
            self.len-=1;
            out==1
        }
    }

    fn clear(&mut self) {
        self.inner.clear();
        self.len=0;
    }

    unsafe fn get_unchecked(&self, index: usize)->bool {
        let (t_index,m_index)=(index/Self::INNER_SIZE,index%Self::INNER_SIZE);
        (self.inner.get_unchecked(t_index)>>m_index) & 1==1
    }

    fn set(&mut self,index:usize,flag:bool) {
        if index<self.len{
            let (t_index,m_index)=(index/Self::INNER_SIZE,index%Self::INNER_SIZE);
            let upper = self.inner[t_index] & Self::uper_mask(m_index+1);
            let lower = self.inner[t_index] & Self::lower_mask(m_index);
            self.inner[t_index]=upper+((flag as usize)<<m_index)+lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self)->flag_iter::Iter<Self> {
        flag_iter::Iter::new(self)
    }
}
impl BitAndAssign<&Blong> for Blong{
    fn bitand_assign(&mut self, rhs: &Blong) {
        for i in 0..self.inner.len().min(rhs.inner().len()){
                self.inner[i].bitand_assign(rhs.inner()[i]);
        }
        for i in self.inner.len().min(rhs.inner().len())..self.inner.len(){
            self.inner[i]=0;
        }
        for _i in self.inner.len().min(rhs.inner().len()) .. rhs.inner().len(){
            self.inner.push(0);
        }
        self.len=self.len.max(rhs.len());
    }
}
impl BitOrAssign<&Blong> for Blong{
    fn bitor_assign(&mut self, rhs: &Blong) {
        for i in 0..self.inner.len().min(rhs.inner().len()){
            self.inner[i].bitor_assign(rhs.inner()[i]);
        }
        for i in self.inner.len().min(rhs.inner().len()) .. rhs.inner().len(){
         self.inner.push(rhs.inner()[i]);
        }   
        self.len=self.len.max(rhs.len());
    }
}
impl BitXorAssign<&Blong> for Blong{
    fn bitxor_assign(&mut self, rhs: &Blong) {
        for i in 0..self.inner.len().min(rhs.inner().len()){
            self.inner[i].bitand_assign(rhs.inner()[i]);
        }
        for _i in self.inner.len().min(rhs.inner().len()) .. rhs.inner().len(){
            self.inner.push(rhs.inner()[1]);
        }
        self.len=self.len.max(rhs.len());
    }
}
impl Not for Blong{
    type Output = Blong;
    fn not(mut self) -> Self::Output {
        let len = self.inner.len();
        for i in 0.. len{
            self.inner[i]= !self.inner[i];
        }
        self.inner[len-1] &= Self::lower_mask(self.len%Self::INNER_SIZE);
        self
    }
}
impl SubAssign<&Blong> for Blong{
    /// Subtration is set difference
    fn sub_assign(&mut self, rhs: &Blong) {
        for i in 0..self.inner.len(){
            self.inner[i] &= !rhs.inner()[i];
        }
    }
}