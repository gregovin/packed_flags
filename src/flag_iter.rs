use crate::FlagLs;
/// An Iterator for Iterating over lists of flags
pub struct Iter<'a, T: FlagLs> {
    inner: &'a T,
    pos: usize,
}
impl<T> Iter<'_, T>
where
    T: FlagLs,
{
    /// Create a new Iterator referencing a flag list
    pub const fn new(ls: &T) -> Iter<T> {
        Iter { inner: ls, pos: 0 }
    }
}
impl<T> Iterator for Iter<'_, T>
where
    T: FlagLs,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.inner.get(self.pos);
        if out.is_some() {
            self.pos += 1;
        }
        out
    }
}
