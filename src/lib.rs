




impl<'a,T> core::ops::Deref for FirstVec<'a,T>{
    type Target=[T];
    fn deref(&self)->&Self::Target{
        self.get_slice()
    }
}

impl<'a,T> core::ops::DerefMut for FirstVec<'a,T>{
    fn deref_mut(&mut self)->&mut Self::Target{
        self.get_slice_mut()
    }
}

pub struct FirstVec<'a,T>{
    foo: &'a mut TwoUnorderedVecs<T>,
}
impl<'a,T> RetainMutUnordered<T> for FirstVec<'a,T>{
    fn truncate(&mut self,num:usize){
        FirstVec::truncate(self,num);
    }
    fn get_slice_mut(&mut self)->&mut [T]{
        FirstVec::get_slice_mut(self)
    }
}


impl<'a,T> FirstVec<'a,T>{
    #[inline(always)]
    pub fn get_slice(&self) -> &[T] {
        &self.foo.inner[..self.foo.first_length]
    }
    #[inline(always)]
    pub fn get_slice_mut(&mut self) -> &mut [T] {
        &mut self.foo.inner[..self.foo.first_length]
    }
    #[inline(always)]
    pub fn truncate(&mut self, num: usize) {
        let total_len = self.foo.inner.len();

        //the number to be removed
        let diff = self.foo.first_length - num ;

        
        let target_ptr=&mut self.foo.inner[num] as *mut _;
        let source_ptr=unsafe{self.foo.inner.as_mut_ptr().offset( (total_len-diff.min(total_len-self.foo.first_length)) as isize )};
        unsafe{
            core::ptr::drop_in_place(target_ptr);
            core::ptr::copy(source_ptr,target_ptr,diff);
        }

        self.foo.first_length = num;

        unsafe{
            self.foo.inner.set_len(total_len-diff);
        }
    }

    #[inline(always)]
    pub fn push(&mut self, a: T) {
        let total_len = self.foo.inner.len();

        self.foo.inner.push(a);

        //now len is actually one less than current length.
        self.foo.inner.swap(self.foo.first_length, total_len);

        self.foo.first_length += 1;
    }
}


pub struct SecondVec<'a,T>{
    foo: &'a mut TwoUnorderedVecs<T>,
}
impl<'a,T> RetainMutUnordered<T> for SecondVec<'a,T>{
    fn truncate(&mut self,num:usize){
        SecondVec::truncate(self,num);
    }
    fn get_slice_mut(&mut self)->&mut [T]{
        SecondVec::get_slice_mut(self)
    }
}

impl<'a,T> core::ops::Deref for SecondVec<'a,T>{
    type Target=[T];
    fn deref(&self)->&Self::Target{
        self.get_slice()
    }
}

impl<'a,T> core::ops::DerefMut for SecondVec<'a,T>{
    fn deref_mut(&mut self)->&mut Self::Target{
        self.get_slice_mut()
    }
}


impl<'a,T> SecondVec<'a,T>{
    #[inline(always)]
    pub fn get_slice(&self) -> &[T] {
        &self.foo.inner[self.foo.first_length..]
    }
    #[inline(always)]
    pub fn get_slice_mut(&mut self) -> &mut [T] {
        &mut self.foo.inner[self.foo.first_length..]
    }
 
    #[inline(always)]
    pub fn push(&mut self, b: T) {
        self.foo.inner.push(b);
    }

    #[inline(always)]
    pub fn truncate(&mut self, num: usize) {
        self.foo.inner.truncate(self.foo.first_length + num);
    }
}

///Two unordered vecs backed by one vec.
///Pushing and retaining from the first cec,
///can change the ordering of the second vec.
///Assume both vecs ordering can change at any time.
#[derive(Debug)]
pub struct TwoUnorderedVecs<T> {
    inner: Vec<T>,
    first_length: usize,
}

impl<T> TwoUnorderedVecs< T> {
    #[inline(always)]
    pub fn new() -> Self {
        TwoUnorderedVecs {
            inner:Vec::new(),
            first_length: 0,
        }
    }
    pub fn first_mut(&mut self)->FirstVec<T>{
        FirstVec{foo:self}
    }

    pub fn second_mut(&mut self)->SecondVec<T>{
        SecondVec{foo:self}
    }


    #[inline(always)]
    pub fn get_slice_mut(&mut self) -> (&mut [T],&mut [T]) {
        self.inner.split_at_mut(self.first_length)
    }

    #[inline(always)]
    pub fn get_slice(&self) -> (&[T],&[T]) {
        self.inner.split_at(self.first_length)
    }

    



}


impl<T> RetainMutUnordered<T> for Vec<T>{
    fn truncate(&mut self,val:usize){
        Vec::truncate(self,val);
    }
    fn get_slice_mut(&mut self)->&mut [T]{
        self
    }
}


pub trait RetainMutUnordered<T> {
    fn truncate(&mut self,val:usize);
    fn get_slice_mut(&mut self)->&mut [T];

    fn retain_mut_unordered<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool{
        let len = self.get_slice_mut().len();
        let mut del = 0;
        {
            let v = self.get_slice_mut();
            let mut cursor = 0;
            for _ in 0..len {
                if !f(&mut v[cursor]) {
                    v.swap(cursor, len - 1 - del);
                    del += 1;
                } else {
                    cursor += 1;
                }
            }
        }
        if del > 0 {
            self.truncate(len - del);
        }

    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_truncate() {
        
        let mut k = TwoUnorderedVecs::new();
        k.first_mut().push(0);
        k.first_mut().push(1);
        k.first_mut().push(2);
        k.second_mut().push(3);
        k.second_mut().push(4);
        k.second_mut().push(5);
        k.second_mut().push(6);
        slices_tuple_eq(k.get_slice(),(&[0,1,2],&[3,4,5,6]));

        k.first_mut().truncate(2);
        slices_tuple_eq(k.get_slice(),(&[0,1],&[6,3,4,5]));
    }

    
    #[test]
    fn test_truncate2(){
        let mut k = TwoUnorderedVecs::new();
        k.first_mut().push(0);
        k.first_mut().push(1);
        k.first_mut().push(2);
        k.first_mut().push(3);
        k.first_mut().push(4);
        k.second_mut().push(5);
        k.first_mut().truncate(3);
        slices_tuple_eq(k.get_slice(),(&[0,1,2],&[5]));
        assert_eq!(k.first_length,3);
        assert_eq!(k.inner.len(),4);
    }
    #[test]
    fn test_other() {
        let mut k = TwoUnorderedVecs::new();
        k.second_mut().push(5);
        k.first_mut().push(6);
        k.second_mut().push(5);
        k.first_mut().push(6);
        k.second_mut().push(5);
        k.first_mut().push(6);
        k.second_mut().push(5);
        k.first_mut().push(6);
        k.first_mut().truncate(2);
        k.second_mut().truncate(2);
        
        slices_match(&k.first_mut(), &[6, 6]);
        slices_match(&k.second_mut(), &[5, 5]);
    }
    
    //TODO put in module
    #[test]
    fn test_push() {
        let mut k = TwoUnorderedVecs::new();
        k.first_mut().push(9);
        k.second_mut().push(0);
        k.first_mut().push(3);

        k.first_mut().push(6);
        k.second_mut().push(8);
        k.first_mut().push(5);

        slices_match(&k.first_mut(), &[9, 3, 6, 5]);
        slices_match(&k.second_mut(), &[0, 8]);

        assert_eq!(k.first_length, 4);

        k.first_mut().truncate(2);
        k.second_mut().truncate(1);

        slices_match(&k.first_mut(), &[3, 9]);
        slices_match(&k.second_mut(), &[8]);

        assert_eq!(k.first_mut().len(), 2);
        assert_eq!(k.second_mut().len(), 1);
        assert_eq!(k.first_length, 2);

        k.first_mut().push(4);
        k.first_mut().push(6);
        k.first_mut().push(7);
        k.first_mut().push(8);

        k.second_mut().push(7);
        k.second_mut().push(3);
        k.second_mut().push(2);
        k.second_mut().push(4);

        k.first_mut().retain_mut_unordered(|&mut a| a % 2 == 1);
        k.second_mut().retain_mut_unordered(|&mut a| a % 2 == 0);

        slices_match(&k.first_mut(), &[9, 3, 7]);
        slices_match(&k.second_mut(), &[8, 2, 4]);
    }
    
    fn slices_tuple_eq<T:Eq+core::fmt::Debug>(arr:(&[T],&[T]),arr2:(&[T],&[T])){
        assert_eq!(arr,arr2);
    }
    fn slices_eq<T:Eq+core::fmt::Debug>(arr:&[T],arr2:&[T]){
        assert_eq!(arr,arr2);
    }
    fn slices_match<T: Eq>(arr1: &[T], arr2: &[T]) {
        for a in arr2.iter() {
            assert!(arr1.contains(a));
        }
        for b in arr1.iter() {
            assert!(arr2.contains(b));
        }
        assert_eq!(arr1.len(), arr2.len());
    }
}
