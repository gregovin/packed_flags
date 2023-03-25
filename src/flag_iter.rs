use crate::FlagLs;
/// An Iterator for Iterating over lists of flags
pub struct Iter<'a, T: FlagLs> {
    inner: &'a T,
    front: usize,
    back: usize,
}
impl<T> Iter<'_, T>
where
    T: FlagLs,
{
    /// Create a new Iterator referencing a flag list
    pub fn new(ls: &T) -> Iter<T> {
        Iter { inner: ls, front: 0,back: ls.len() }
    }
}
impl<T> Iterator for Iter<'_, T>
where
    T: FlagLs,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front>=self.back{
            None
        } else {
            let out = self.inner.get(self.front);
            self.front+=1;
            out
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem =self.back-self.front;
        (rem,Some(rem))
    }
}
impl<T> DoubleEndedIterator for Iter<'_,T>
where
    T: FlagLs
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front>=self.back{
            None
        } else {
            let out = self.inner.get(self.back-1);
            self.back-=1;
            out
        }
    }
}
impl<T: FlagLs> ExactSizeIterator for Iter<'_,T>
{

}
