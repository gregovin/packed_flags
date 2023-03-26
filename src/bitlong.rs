use std::{ops::{BitAndAssign, BitOrAssign, BitXorAssign, Index, Not, SubAssign}, fmt::{UpperHex, LowerHex, Octal, Binary}};

use crate::{flag_iter, Bsize, FlagLs, B128, B32, B64};
/// An arbitrarily long list of flags
///
/// You should use b32,b64, or b128 instead unless you really need a lot of flags
#[derive(PartialEq, Eq, Default, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Blong {
    inner: Vec<usize>,
    len: usize,
}
impl Blong {
    const INNER_SIZE: usize = usize::BITS as usize;
    fn lower_mask(inner_point: usize) -> usize {
        if inner_point > Self::INNER_SIZE {
            panic!("Cannot mask more than pointer size for Blong")
        } else {
            (1 << inner_point) - 1
        }
    }
    const fn inner(&self) -> &Vec<usize> {
        &self.inner
    }
    /// Converts the bitfield into its integer representation, a `Vec<usize>`, consuming it
    /// 
    /// The first flag is at the least significant bit of the 0th value of the output
    /// # Examples
    /// This would keep the first and third flag, set the rest of the first `usize::BITS` flags to false(0), and then move the new flags into a result
    /// ```
    /// use packed_flags::Blong;
    /// use packed_flags::FlagLs;
    /// 
    /// let bitflags= Blong::all_true(100);
    /// 
    /// let len=bitflags.len();
    /// let mut inner=bitflags.as_inner();
    /// inner[0] &= 5;
    /// 
    /// let res =Blong::initialize(inner,len);
    /// ```
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_inner(self) -> Vec<usize> {
        self.inner
    }
    /// Initializes a `Blong` from a list of integers and a known length
    /// 
    /// Any flags beyond the length will be zeroed
    /// # Examples
    /// See [`as_inner`][Blong::as_inner]
    #[must_use]
    pub fn initialize(inner: Vec<usize>,len:usize)->Self{
        let mut out =Self{len: inner.len() * Self::INNER_SIZE,inner};
        out.set_len(len);
        out
    }
    fn uper_mask(inner_point: usize) -> usize {
        if inner_point > Self::INNER_SIZE {
            panic!("Cannot mask above the end of the list")
        } else {
            usize::MAX - (1 << inner_point) + 1
        }
    }
    #[allow(dead_code)]
    #[must_use]
    /// Creates an empty list of flags
    pub const fn new() -> Self {
        Self {
            inner: vec![],
            len: 0,
        }
    }
}
impl Index<usize> for Blong {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        if index > self.len {
            panic!("Index out of bounds")
        } else {
            let (t_index, m_index) = (index / Self::INNER_SIZE, index % Self::INNER_SIZE);
            if (self.inner[t_index] >> m_index) & 1 == 1 {
                &true
            } else {
                &false
            }
        }
    }
}
impl FlagLs for Blong {
    const MAX_LENGTH: usize = usize::MAX;

    fn len(&self) -> usize {
        self.len
    }

    fn set_len(&mut self, new_len: usize) {
        let (t_len,m_len) = ((new_len+Self::INNER_SIZE-1) / Self::INNER_SIZE,((new_len+Self::INNER_SIZE-1)%Self::INNER_SIZE)+1);
        println!("t: {t_len}, m: {m_len}");
        match t_len.cmp(&self.inner.len()) {
            std::cmp::Ordering::Greater => {
                let mut new: Vec<usize> = vec![0; t_len - self.inner.len()];
                self.inner.append(&mut new);
            }
            std::cmp::Ordering::Less => {
                let _ = self.inner.drain(t_len..);
            }
            std::cmp::Ordering::Equal => {}
        }
        if t_len>0{
            self.inner[t_len-1] &= (1 << m_len) - 1;
        }
        self.len = new_len;
    }

    fn insert(&mut self, index: usize, flag: bool) {
        if index > self.len {
            panic!("Index out of bounds");
        } else {
            let mut t_index = index / Self::INNER_SIZE;
            let m_index = index % Self::INNER_SIZE;
            if t_index == self.len {
                self.inner.push(usize::from(flag));
            } else {
                let lower = self.inner[t_index] & Self::lower_mask(m_index);
                let upper = self.inner[t_index] & Self::uper_mask(m_index);
                let mut top = self.inner[t_index] >> (Self::INNER_SIZE - 1);

                self.inner[t_index] = (upper << 1) + ((usize::from(flag)) << m_index) + lower;
                t_index += 1;
                while t_index < self.inner.len() {
                    let old = top;
                    top = self.inner[t_index] >> (Self::INNER_SIZE - 1);
                    self.inner[t_index] = (self.inner[t_index] << 1) + old;
                    t_index += 1;
                }
                if m_index == Self::INNER_SIZE - 1 {
                    self.inner.push(top);
                }
            }
            self.len += 1;
        }
    }

    fn remove(&mut self, index: usize) -> bool {
        if index >= self.len {
            panic!("Index out of bounds")
        } else {
            let t_index = index / Self::INNER_SIZE;
            let m_indx = index % Self::INNER_SIZE;
            let mut top_idx = self.inner.len() - 1;
            let mut bot: usize = 0;

            while top_idx > t_index {
                let old = bot;
                bot = self.inner[top_idx] & 1;
                self.inner[top_idx] = (self.inner[top_idx] >> 1) + (old << (Self::INNER_SIZE - 1));
                top_idx -= 1;
            }
            let upper = self.inner[t_index] & Self::uper_mask(m_indx + 1);
            let lower = self.inner[t_index] & Self::lower_mask(m_indx);
            let out = (self.inner[t_index] >> m_indx) & 1;
            self.inner[t_index] = lower + (upper >> 1) + (bot << (Self::INNER_SIZE - 1));
            self.len -= 1;
            out == 1
        }
    }

    fn clear(&mut self) {
        self.inner.clear();
        self.len = 0;
    }

    fn get(&self, index: usize) -> Option<bool> {
        if index<self.len{
            let (t_index, m_index) = (index / Self::INNER_SIZE, index % Self::INNER_SIZE);
            Some((self.inner[t_index] >> m_index) & 1 == 1)
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, flag: bool) {
        if index < self.len {
            let (t_index, m_index) = (index / Self::INNER_SIZE, index % Self::INNER_SIZE);
            let upper = self.inner[t_index] & Self::uper_mask(m_index + 1);
            let lower = self.inner[t_index] & Self::lower_mask(m_index);
            self.inner[t_index] = upper + ((usize::from(flag)) << m_index) + lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self) -> flag_iter::Iter<Self> {
        flag_iter::Iter::new(self)
    }
}
impl BitAndAssign<&Self> for Blong {
    fn bitand_assign(&mut self, rhs: &Self) {
        for i in 0..self.inner.len().min(rhs.inner().len()) {
            self.inner[i].bitand_assign(rhs.inner()[i]);
        }
        for i in self.inner.len().min(rhs.inner().len())..self.inner.len() {
            self.inner[i] = 0;
        }
        for _i in self.inner.len().min(rhs.inner().len())..rhs.inner().len() {
            self.inner.push(0);
        }
        self.len = self.len.max(rhs.len());
    }
}
impl BitOrAssign<&Self> for Blong {
    fn bitor_assign(&mut self, rhs: &Self) {
        for i in 0..self.inner.len().min(rhs.inner().len()) {
            self.inner[i].bitor_assign(rhs.inner()[i]);
        }
        for i in self.inner.len().min(rhs.inner().len())..rhs.inner().len() {
            self.inner.push(rhs.inner()[i]);
        }
        self.len = self.len.max(rhs.len());
    }
}
impl BitXorAssign<&Self> for Blong {
    fn bitxor_assign(&mut self, rhs: &Self) {
        for i in 0..self.inner.len().min(rhs.inner().len()) {
            self.inner[i].bitand_assign(rhs.inner()[i]);
        }
        for _i in self.inner.len().min(rhs.inner().len())..rhs.inner().len() {
            self.inner.push(rhs.inner()[1]);
        }
        self.len = self.len.max(rhs.len());
    }
}
impl Not for Blong {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        let len = self.inner.len();
        for i in 0..len {
            self.inner[i] = !self.inner[i];
        }
        self.inner[len - 1] &= Self::lower_mask(self.len % Self::INNER_SIZE);
        self
    }
}
impl SubAssign<&Self> for Blong {
    /// Subtration is set difference
    fn sub_assign(&mut self, rhs: &Self) {
        for i in 0..self.inner.len() {
            self.inner[i] &= !rhs.inner()[i];
        }
    }
}
#[allow(clippy::fallible_impl_from)]
impl From<B32> for Blong{
    fn from(value: B32) -> Self {
        let len=value.len();
        let mut val_inner=value.as_inner();
        let inner:Vec<usize> =usize::try_from(val_inner).map_or_else(|_|
            {
                let mut inner: Vec<usize> = vec![];
                let mut rem_len=len;
                while rem_len>0{
                    // If inner representation(as a u32) did not fit in a usize, then usize fits in u32
                    inner.push((val_inner & u32::try_from(usize::MAX).expect("Infalible")).try_into().expect("Infalible"));
                    // the number of bits in a usize will always fit into a 32 bit integer forever, so this is fine
                    val_inner=val_inner.checked_shr(Self::INNER_SIZE.try_into().expect("Infalible")).unwrap_or(0);
                    rem_len-=Self::INNER_SIZE;
                }
                inner
            }, |r| vec![r]);
        Self { inner, len}
    }
}
#[allow(clippy::fallible_impl_from)]
impl From<B64> for Blong{
    fn from(value: B64) -> Self {
        let len=value.len();
        let mut val_inner=value.as_inner();
        let inner:Vec<usize> =usize::try_from(val_inner).map_or_else(|_|
            {
                let mut inner: Vec<usize> = vec![];
                let mut rem_len=len;
                while rem_len>0{
                    // If inner representation(as a u64) did not fit in a usize, then usize fits in u64
                    inner.push((val_inner & u64::try_from(usize::MAX).expect("Infalible")).try_into().expect("Infalible"));
                    // the number of bits in a usize will always fit into a 32 bit integer forever, so this is fine
                    val_inner=val_inner.checked_shr(Self::INNER_SIZE.try_into().expect("Infalible")).unwrap_or(0);
                    rem_len-=Self::INNER_SIZE;
                }
                inner
            }, |r| vec![r]);
        Self { inner, len}
    }
}
#[allow(clippy::fallible_impl_from)]
impl From<B128> for Blong{
    fn from(value: B128) -> Self {
        let len=value.len();
        let mut val_inner=value.as_inner();
        let inner:Vec<usize> =usize::try_from(val_inner).map_or_else(|_|
            {
                let mut inner: Vec<usize> = vec![];
                let mut rem_len=len;
                while rem_len>0{
                    // If inner representation(as a u128) did not fit in a usize, then usize fits in u128
                    inner.push((val_inner & u128::try_from(usize::MAX).expect("Infalible")).try_into().expect("Infalible"));
                    // the number of bits in a usize will always fit into a 32 bit integer forever, so this is fine
                    val_inner=val_inner.checked_shr(Self::INNER_SIZE.try_into().expect("Infalible")).unwrap_or(0);
                    rem_len-=Self::INNER_SIZE;
                }
                inner
            }, |r| vec![r]);
        Self { inner, len}
    }
}
impl From<Bsize> for Blong{
    fn from(value: Bsize) -> Self {
        let len=value.len();
        Self { inner: vec![value.as_inner()], len}
    }
}
impl UpperHex for Blong{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out= String::new();
        for t in &self.inner{
            out+=&format!("{:X}",*t);
        }
        write!(f,"{out}")
    }
}
impl LowerHex for Blong{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out= String::new();
        for t in &self.inner{
            out+=&format!("{:x}",*t);
        }
        write!(f,"{out}")
    }
}
impl Octal for Blong{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out= String::new();
        for t in &self.inner{
            out+=&format!("{:o}",*t);
        }
        write!(f,"{out}")
    }
}
/// Probably a bad idea to ever use this
impl Binary for Blong{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out= String::new();
        for t in &self.inner{
            out+=&format!("{:b}",*t);
        }
        write!(f,"{out}")
    }
}