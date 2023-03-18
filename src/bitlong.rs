use std::ops::{Index, BitAndAssign, BitOrAssign, BitXorAssign, Not, SubAssign};

use crate::{FlagLs, flag_iter::FlagIter};
#[derive(PartialEq,Eq,Default,Clone, Debug,Hash)]
#[allow(non_camel_case_types)]
pub struct blong{
    inner: Vec<usize>,
    len: usize
}
impl blong{
    const INNER_SIZE: usize=usize::BITS as usize;
    fn lower_mask(inner_point: usize)->usize{
        if inner_point>Self::INNER_SIZE{
            panic!("Cannot mask more than pointer size for blong")
        } else {
            (1<<inner_point)-1
        }
    }
    fn inner(&self)->&Vec<usize>{
        &self.inner
    }
    fn uper_mask(inner_point: usize)->usize{
        if inner_point>Self::INNER_SIZE{
            panic!("Cannot mask above the end of the list")
        } else {
            usize::MAX-(1<<inner_point)+1
        }
    }
    #[allow(dead_code)]
    pub fn new()->blong{
        blong { inner: vec![], len: 0 }
    }
}
impl Index<usize> for blong{
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
impl FlagLs for blong{
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
            let upper=self.inner[t_index] &Self::uper_mask(m_indx);
            let lower=self.inner[t_index]&Self::lower_mask(m_indx);
            let out = (self.inner[t_index]>>m_indx)&1;
            self.inner[t_index]=lower+(upper>>1)+(bot<<(Self::INNER_SIZE-1));
            out==1
        }
    }

    fn clear(&mut self) {
        self.inner.clear();
        self.len=0;
    }

    fn get(&self, index: usize)->Option<bool> {
        if index>=self.len{
            None
        } else {
            Some(self[index])
        }
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

    fn iter(&self)->FlagIter<Self> {
        FlagIter::new(self)
    }
}
impl BitAndAssign<&blong> for blong{
    fn bitand_assign(&mut self, rhs: &blong) {
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
impl BitOrAssign<&blong> for blong{
    fn bitor_assign(&mut self, rhs: &blong) {
        for i in 0..self.inner.len().min(rhs.inner().len()){
            self.inner[i].bitor_assign(rhs.inner()[i]);
        }
        for i in self.inner.len().min(rhs.inner().len()) .. rhs.inner().len(){
         self.inner.push(rhs.inner()[i]);
        }   
        self.len=self.len.max(rhs.len());
    }
}
impl BitXorAssign<&blong> for blong{
    fn bitxor_assign(&mut self, rhs: &blong) {
        for i in 0..self.inner.len().min(rhs.inner().len()){
            self.inner[i].bitand_assign(rhs.inner()[i]);
        }
        for _i in self.inner.len().min(rhs.inner().len()) .. rhs.inner().len(){
            self.inner.push(rhs.inner()[1]);
        }
        self.len=self.len.max(rhs.len());
    }
}
impl Not for blong{
    type Output = blong;
    fn not(mut self) -> Self::Output {
        let len = self.inner.len();
        for i in 0.. len{
            self.inner[i]= !self.inner[i];
        }
        self.inner[len-1] &= Self::lower_mask(self.len%Self::INNER_SIZE);
        self
    }
}
impl SubAssign<&blong> for blong{
    /// Subtration is set difference
    fn sub_assign(&mut self, rhs: &blong) {
        for i in 0..self.inner.len(){
            self.inner[i] &= !rhs.inner()[i];
        }
    }
}