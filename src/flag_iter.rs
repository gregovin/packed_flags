use crate::FlagLs;

pub struct FlagIter<'a,T: FlagLs>{
    inner: &'a T,
    pos: usize
}
impl<T> FlagIter<'_,T>
where T:FlagLs
{
    pub fn new(ls: &T)->FlagIter<T>{
        FlagIter { inner: ls, pos: 0 }
    }
}
impl<T> Iterator for FlagIter<'_,T>
where T:FlagLs
{
    type Item=bool;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.inner.get(self.pos);
        if out.is_some(){
            self.pos+=1;
        }
        out
    }
}