use std::{ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign,
    Shr, ShrAssign, Sub,}, fmt::{UpperHex, LowerHex, Octal, Binary}};

use crate::{flag_iter, Blong, FlagLs, B128, B32, B64, FlagLsError};

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A list of flags/bitfield up to the size of a pointer
/// 
/// This is only really useful if you need to deal with `usize`s for some other reason
pub struct Bsize {
    inner: usize,
    len: usize,
}
impl Bsize {
    fn lower_mask(point: usize) -> usize {
        if point > Self::MAX_LENGTH {
            panic!(
                "Cannot create mask more than {} bits for Bsize",
                Self::MAX_LENGTH
            )
        } else {
            (1 << point) - 1
        }
    }
    const fn inner(&self) -> usize {
        self.inner
    }
    /// Converts the bitfield into its integer representation, a usize, consuming it
    /// # Examples
    /// The most common use case would be doing bitwise operations with a non-FlagLs item
    /// Here we bitwisexor the inner state with an arbitrary usze number and then rebuild the flaglist into a result
    /// ```
    /// use packed_flags::Bsize;
    /// use packed_flags::FlagLs;
    /// use std::ops::BitXor;
    /// 
    /// let bitflags= Bsize::from_iter(vec![false,true,false,true,false,true,false,true]);
    /// let other: usize = 3198; //presumably this other value would come from some external source
    /// let updated = bitflags.as_inner().bitxor(&other);
    /// let res = Bsize::initialize(updated,8);
    ///
    /// ```
    #[must_use]
    pub const fn as_inner(self) -> usize {
        self.inner
    }
    fn uper_mask(point: usize) -> usize {
        if point > Self::MAX_LENGTH {
            panic!("Cannot mask above the end of the list");
        } else {
            usize::MAX - (1 << point) + 1
        }
    }
    /// Create a new blank empty list of flags
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    /// Create a new `Bsize` from a `usize` and a length
    /// 
    /// Will truncate len to `MAX_LENGTH` and will truncate inner to len bits
    /// # Examples
    /// See [`as_inner`][Bsize::as_inner]
    pub fn initialize(inner: usize, len: usize) -> Self {
        let len =len.min(Self::MAX_LENGTH);
        let mut out=Self { inner, len};
        out.set_len(len);
        out
    }
}
impl Index<usize> for Bsize {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "Cannot access out of bounds index");
        if (self.inner >> index) & 1 == 1 {
            &true
        } else {
            &false
        }
    }
}
impl FlagLs for Bsize {
    const MAX_LENGTH: usize = usize::BITS as usize;

    fn len(&self) -> usize {
        self.len
    }

    fn set_len(&mut self, new_len: usize) {
        assert!(new_len <= Self::MAX_LENGTH, "Cannot set length to a length larger than {} for Bsize", Self::MAX_LENGTH);
        self.len = new_len;
        self.inner &= (1 << new_len) - 1;
    }

    fn insert(&mut self, index: usize, flag: bool) {
        if index > self.len {
            panic!("Cannot insert out of bounds");
        } else if self.len == Self::MAX_LENGTH {
            panic!("cannot insert to full list of flags")
        } else {
            let uper = self.inner & Self::uper_mask(index);
            let lower = self.inner & Self::lower_mask(index);
            self.inner = (uper << 1) + (usize::from(flag) << index) + lower;
            self.len += 1;
        }
    }

    fn remove(&mut self, index: usize) -> bool {
        if index >= self.len {
            panic!("Cannot remove out of bounds");
        } else {
            let uper = self.inner & Self::uper_mask(index + 1);
            let lower = self.inner & Self::lower_mask(index);
            let out = (self.inner >> index) & 1;
            self.inner = (uper >> 1) + lower;
            self.len -= 1;
            out == 1
        }
    }

    fn clear(&mut self) {
        self.inner = 0;
        self.len = 0;
    }

    fn get(&self, index: usize) -> Option<bool> {
        if index<self.len{
            Some((self.inner >> index) & 1 == 1)   
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, flag: bool) {
        if index < self.len() {
            let uper = self.inner & Self::uper_mask(index + 1);
            let lower = self.inner & Self::lower_mask(index);
            self.inner = uper + ((usize::from(flag)) << index) + lower;
        } else {
            panic!("Cannot set out of bounds")
        }
    }

    fn iter(&self) -> flag_iter::Iter<Self> {
        flag_iter::Iter::new(self)
    }
}
impl BitAnd<Self> for Bsize {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self{inner: self.inner() & rhs.inner(),len:self.len().max(rhs.len())}
    }
}
impl BitAndAssign<Self> for Bsize {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner &= rhs.inner();
        self.len = self.len.max(rhs.len());
    }
}
impl BitOr<Self> for Bsize {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self{inner: self.inner() | rhs.inner(),len:self.len().max(rhs.len())}
    }
}
impl BitOrAssign<Self> for Bsize {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner |= rhs.inner();
        self.len = self.len.max(rhs.len());
    }
}
impl BitXor<Self> for Bsize {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self{inner:self.inner().bitxor(rhs.inner()),len:self.len().max(rhs.len())}
    }
}
impl BitXorAssign<Self> for Bsize {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner());
        self.len = self.len.max(rhs.len());
    }
}
impl Shl<usize> for Bsize {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        Self{inner:self.inner<<rhs, len:(self.len + rhs).min(Self::MAX_LENGTH)}
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<usize> for Bsize {
    fn shl_assign(&mut self, rhs: usize) {
        self.inner.shl_assign(rhs);
        self.len = (self.len + rhs).min(Self::MAX_LENGTH);
    }
}
impl Shr<usize> for Bsize {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        let new_len = if self.len > rhs { self.len - rhs } else { 0 };
        Self{inner:self.inner>>rhs,len:new_len}
    }
}
impl ShrAssign<usize> for Bsize {
    fn shr_assign(&mut self, rhs: usize) {
        self.inner.shr_assign(rhs);
        self.len = if self.len > rhs { self.len - rhs } else { 0 };
    }
}
impl Not for Bsize {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self{inner: (!self.inner) & Self::lower_mask(self.len),len:self.len}
    }
}
impl Sub<Self> for Bsize {
    type Output = Self;
    ///note subtraction is set difference
    fn sub(self, rhs: Self) -> Self::Output {
        Self{inner:self.inner&(!rhs.inner),len:self.len}
    }
}
impl TryFrom<B32> for Bsize{
    type Error = FlagLsError;
    fn try_from(value: B32) -> Result<Self, Self::Error> {
        let len = value.len();
        match len.cmp(&Self::MAX_LENGTH){
            std::cmp::Ordering::Greater=> Err(FlagLsError::MaximumLengthExceeded { mx_len: Self::MAX_LENGTH, attempt_len: len }),
            _=>Ok(Self { inner: value.as_inner().try_into().expect("Infalible"), len })
        }
    }
}
impl TryFrom<B64> for Bsize{
    type Error = FlagLsError;
    fn try_from(value: B64) -> Result<Self, Self::Error> {
        let len = value.len();
        match len.cmp(&Self::MAX_LENGTH){
            std::cmp::Ordering::Greater=> Err(FlagLsError::MaximumLengthExceeded { mx_len: Self::MAX_LENGTH, attempt_len: len }),
            _=>Ok(Self { inner: value.as_inner().try_into().expect("Infalible"), len })
        }
    }
}
impl TryFrom<B128> for Bsize{
    type Error = FlagLsError;
    fn try_from(value: B128) -> Result<Self, Self::Error> {
        let len = value.len();
        match len.cmp(&Self::MAX_LENGTH){
            std::cmp::Ordering::Greater=> Err(FlagLsError::MaximumLengthExceeded { mx_len: Self::MAX_LENGTH, attempt_len: len }),
            _=>Ok(Self { inner: value.as_inner().try_into().expect("Unreachable"), len })
        }
    }
}
impl TryFrom<Blong> for Bsize{
    type Error = FlagLsError;
    fn try_from(value: Blong) -> Result<Self, Self::Error> {
        let len = value.len();
        match len.cmp(&Self::MAX_LENGTH){
            std::cmp::Ordering::Greater=> Err(FlagLsError::MaximumLengthExceeded { mx_len: Self::MAX_LENGTH, attempt_len: len }),
            _=>Ok(Self { inner: *value.as_inner().first().unwrap_or(&0), len })
        }
    }
}
impl UpperHex for Bsize{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:X}",self.inner)
    }
}
impl LowerHex for Bsize{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:x}",self.inner)
    }
}
impl Octal for Bsize{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:o}",self.inner)
    }
}
impl Binary for Bsize{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:b}",self.inner)
    }
}