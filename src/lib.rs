//! A crate that provides the user with two fast "vec-like" vecs that are backed by
//! a single vec. The caveat is that the operations like push and truncate
//! may rearrange the order of the other vec in an unspecified way.
//! Also provides a `retain_mut_unordered` function to both the regular `Vec` as well as
//! the two "vec-like" vecs provided by this crate.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

impl<'a, T> core::ops::Deref for FirstVec<'a, T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T> core::ops::DerefMut for FirstVec<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

#[derive(Debug)]
pub struct FirstVec<'a, T> {
    foo: &'a mut TwoUnorderedVecs<T>,
}
impl<'a, T> RetainMutUnordered<T> for FirstVec<'a, T> {
    #[inline(always)]
    fn truncate(&mut self, num: usize) {
        FirstVec::truncate(self, num);
    }

    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [T] {
        FirstVec::as_slice_mut(self)
    }
}

impl<'a, T> FirstVec<'a, T> {
    #[inline(always)]
    pub fn truncate(&mut self, num: usize) {
        let first_length = self.foo.first_length;

        //If user tries to truncate more elements than are in the slice,
        //just assume they want to truncate everything.
        let num = if num > first_length {
            first_length
        } else {
            num
        };

        let diff = first_length - num;

        let total_len = self.foo.inner.len();

        let (_keep, rest) = self.foo.inner.split_at_mut(num);
        let (slice_to_remove, rest) = rest.split_at_mut(diff);

        if rest.len() > slice_to_remove.len() {
            let (_rest, slice_to_move) = rest.split_at_mut(rest.len() - slice_to_remove.len());
            slice_to_remove.swap_with_slice(slice_to_move);
        } else {
            let (slice_to_move, _dont_bother_moving) = slice_to_remove.split_at_mut(rest.len());
            slice_to_move.swap_with_slice(rest);
        }

        self.foo.inner.truncate(total_len - diff);

        self.foo.first_length = num;
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        &self.foo.inner[..self.foo.first_length]
    }
    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.foo.inner[..self.foo.first_length]
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

#[derive(Debug)]
pub struct SecondVec<'a, T> {
    foo: &'a mut TwoUnorderedVecs<T>,
}
impl<'a, T> RetainMutUnordered<T> for SecondVec<'a, T> {
    #[inline(always)]
    fn truncate(&mut self, num: usize) {
        SecondVec::truncate(self, num);
    }

    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [T] {
        SecondVec::as_slice_mut(self)
    }
}

impl<'a, T> core::ops::Deref for SecondVec<'a, T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T> core::ops::DerefMut for SecondVec<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<'a, T> SecondVec<'a, T> {
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        &self.foo.inner[self.foo.first_length..]
    }
    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
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

impl<T> From<TwoUnorderedVecs<T>> for Vec<T> {
    #[inline(always)]
    fn from(a: TwoUnorderedVecs<T>) -> Vec<T> {
        a.inner
    }
}
#[derive(Debug)]
pub struct TwoUnorderedVecs<T> {
    inner: Vec<T>,
    first_length: usize,
}

impl<T> Default for TwoUnorderedVecs<T>{
    #[inline(always)]
    fn default()->Self{
        TwoUnorderedVecs::new()
    }
}
impl<T> TwoUnorderedVecs<T> {
    #[inline(always)]
    pub fn with_capacity(num: usize) -> Self {
        TwoUnorderedVecs {
            inner: Vec::with_capacity(num),
            first_length: 0,
        }
    }
    #[inline(always)]
    pub fn new() -> Self {
        TwoUnorderedVecs {
            inner: Vec::new(),
            first_length: 0,
        }
    }

    ///The first vec will have all the elements of
    ///the specified vec.
    ///The second vec will be empty.
    #[inline(always)]
    pub fn from_vec(inner: Vec<T>) -> Self {
        let first_length = inner.len();
        TwoUnorderedVecs {
            inner,
            first_length,
        }
    }

    #[inline(always)]
    pub fn first(&mut self) -> FirstVec<T> {
        FirstVec { foo: self }
    }

    #[inline(always)]
    pub fn second(&mut self) -> SecondVec<T> {
        SecondVec { foo: self }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.first_length = 0;
        self.inner.clear();
    }

    #[inline(always)]
    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    ///Uses the specified vec as the underlying vec.
    ///The original vec is returned.
    ///The first vec is set to the size of the specified vec.
    ///The second vec will be empty.
    #[inline(always)]
    pub fn replace_inner(&mut self, mut a: Vec<T>) -> (Vec<T>, usize) {
        let curr_len = self.first_length;
        self.first_length = a.len();
        core::mem::swap(&mut a, &mut self.inner);
        (a, curr_len)
    }

    #[inline(always)]
    pub fn as_vec(&self) -> &Vec<T> {
        &self.inner
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> (&mut [T], &mut [T]) {
        self.inner.split_at_mut(self.first_length)
    }

    #[inline(always)]
    pub fn as_slice(&self) -> (&[T], &[T]) {
        self.inner.split_at(self.first_length)
    }

    ///Cast this container into another provided `X` has the same
    ///size and alignment as `T`. Panics if they do not.
    ///
    /// ### Unsafety
    ///
    /// The destructors of T won't get called, and X may
    /// not be properly initialized. 
    ///
    pub unsafe fn convert<X>(mut self) -> TwoUnorderedVecs<X> {
        assert_eq!(core::mem::size_of::<X>(), core::mem::size_of::<T>());
        assert_eq!(core::mem::align_of::<X>(), core::mem::align_of::<T>());

        let ptr = self.inner.as_mut_ptr();
        let len = self.inner.len();
        let cap = self.inner.capacity();
        let first_length = self.first_length;
        core::mem::forget(self);
        let inner = Vec::from_raw_parts(ptr as *mut _, len, cap);

        TwoUnorderedVecs {
            inner,
            first_length,
        }
    }
}

impl<T> RetainMutUnordered<T> for Vec<T> {
    #[inline(always)]
    fn truncate(&mut self, val: usize) {
        Vec::truncate(self, val);
    }

    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [T] {
        self
    }
}

/// Retain only elements that satisfy the predicate.
/// May rearrange elements in the process.
pub trait RetainMutUnordered<T> {
    fn truncate(&mut self, val: usize);
    fn as_slice_mut(&mut self) -> &mut [T];

    #[inline(always)]
    fn retain_mut_unordered<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let len = self.as_slice_mut().len();
        let mut del = 0;
        {
            let v = self.as_slice_mut();
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

    //TODO extract/insert functions
    #[test]
    fn test_x() {
        let mut k = TwoUnorderedVecs::new();
        for _ in 0u64..1000 {
            k.second().push(4);
        }
        for _ in 0u64..1000 {
            k.first().push(10);
        }

        let mut flip = false;
        k.first().retain_mut_unordered(|a| {
            *a += 3;
            flip = !flip;
            flip
        });
        flip = false;
        k.second().retain_mut_unordered(|a| {
            *a += 3;
            flip = !flip;
            flip
        });
        assert_eq!(k.first().len(), 500);
        assert_eq!(k.second().len(), 500);
        for a in k.first().iter() {
            assert_eq!(*a, 13);
        }
        for a in k.second().iter() {
            assert_eq!(*a, 7);
        }
    }
    #[test]
    fn test_foo() {
        let mut k = TwoUnorderedVecs::new();
        k.first().push(0);
        k.first().push(1);
        k.first().push(2);

        let (inner, m) = k.replace_inner(Vec::new());
        assert_eq!(m, 3);
        assert_eq!(inner.len(), 3);

        let (inner, m) = k.replace_inner(inner);
        assert_eq!(m, 0);
        assert_eq!(inner.len(), 0);
        assert_eq!(inner.capacity(), 0);

        assert_eq!(k.first().len(), 3);
        assert_eq!(k.second().len(), 0);
    }
    use super::*;

    #[test]
    fn test_truncate_zero() {
        let mut k: TwoUnorderedVecs<u32> = TwoUnorderedVecs::new();

        k.first().push(5);
        k.first().push(5);
        k.first().truncate(3);
        assert_eq!(k.first().len(), 2);
        assert_eq!(k.second().len(), 0);
        k.clear();

        k.second().push(4);
        k.second().push(4);
        k.first().truncate(4);
        assert_eq!(k.first().len(), 0);
        assert_eq!(k.second().len(), 2);

        k.clear();

        k.first().push(5);
        k.first().push(6);
        k.second().push(7);
        k.second().push(8);
        k.first().truncate(2);
        k.second().truncate(2);
        assert_eq!(k.first().len(), 2);
        assert_eq!(k.second().len(), 2);
    }
    #[test]
    fn test_truncate() {
        let mut k = TwoUnorderedVecs::new();
        k.first().push(0);
        k.first().push(1);
        k.first().push(2);
        k.second().push(3);
        k.second().push(4);
        k.second().push(5);
        k.second().push(6);
        slices_tuple_eq(k.as_slice(), (&[0, 1, 2], &[3, 4, 5, 6]));

        k.first().truncate(2);
        slices_tuple_eq(k.as_slice(), (&[0, 1], &[6, 3, 4, 5]));
    }

    #[test]
    fn test_truncate2() {
        let mut k = TwoUnorderedVecs::new();
        k.first().push(0);
        k.first().push(1);
        k.first().push(2);
        k.first().push(3);
        k.first().push(4);
        k.second().push(5);

        k.first().truncate(3);

        slices_tuple_eq(k.as_slice(), (&[0, 1, 2], &[5]));
        assert_eq!(k.first_length, 3);
        assert_eq!(k.inner.len(), 4);
    }

    #[test]
    fn test_trunk() {
        let mut k = TwoUnorderedVecs::new();
        k.first().push(0);
        k.first().push(1);
        k.first().push(2);
        k.first().push(3);
        k.second().push(4);

        k.first().truncate(2);

        k.second().truncate(2);

        slices_match(&k.first(), &[0, 1]);
        slices_match(&k.second(), &[4]);
    }

    #[test]
    fn test_other() {
        let mut k = TwoUnorderedVecs::new();
        k.second().push(6);
        k.first().push(5);
        k.second().push(6);
        k.first().push(5);
        k.second().push(6);
        k.first().push(5);
        k.second().push(6);
        k.first().push(5);

        k.first().truncate(2);

        k.second().truncate(2);

        slices_match(&k.first(), &[5, 5]);
        slices_match(&k.second(), &[6, 6]);
    }

    #[test]
    fn test_push() {
        let mut k = TwoUnorderedVecs::new();
        k.first().push(9);
        k.second().push(0);
        k.first().push(3);

        k.first().push(6);
        k.second().push(8);
        k.first().push(5);

        slices_match(&k.first(), &[9, 3, 6, 5]);
        slices_match(&k.second(), &[0, 8]);

        assert_eq!(k.first_length, 4);

        k.first().truncate(2);
        k.second().truncate(1);

        slices_match(&k.first(), &[3, 9]);
        slices_match(&k.second(), &[8]);

        assert_eq!(k.first().len(), 2);
        assert_eq!(k.second().len(), 1);
        assert_eq!(k.first_length, 2);

        k.first().push(4);
        k.first().push(6);
        k.first().push(7);
        k.first().push(8);

        k.second().push(7);
        k.second().push(3);
        k.second().push(2);
        k.second().push(4);

        k.first().retain_mut_unordered(|&mut a| a % 2 == 1);
        k.second().retain_mut_unordered(|&mut a| a % 2 == 0);

        slices_match(&k.first(), &[9, 3, 7]);
        slices_match(&k.second(), &[8, 2, 4]);

        k.second().push(7);
        k.second().push(3);
        k.second().push(2);
        k.second().push(4);
    }

    fn slices_tuple_eq<T: Eq + core::fmt::Debug>(arr: (&[T], &[T]), arr2: (&[T], &[T])) {
        assert_eq!(arr, arr2);
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
