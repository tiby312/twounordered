//! A crate that provides the user with two fast "vec-like" vecs that are backed by
//! a single vec. The caveat is that the operations like push and truncate
//! may rearrange the order of the other vec in an unspecified way.
//! Also provides a `retain_mut_unordered` function to both the regular `Vec` as well as
//! the two "vec-like" vecs provided by this crate.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

impl<'a, T: VecTrait> core::ops::Deref for FirstVec<'a, T> {
    type Target = [T::T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T: VecTrait> core::ops::DerefMut for FirstVec<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

#[derive(Debug)]
pub struct FirstVec<'a, T> {
    foo: &'a mut TwoUnorderedVecs<T>,
}
impl<'a, T: VecTrait> RetainMutUnordered<T::T> for FirstVec<'a, T> {
    #[inline(always)]
    fn truncate(&mut self, num: usize) {
        FirstVec::truncate(self, num);
    }

    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [T::T] {
        FirstVec::as_slice_mut(self)
    }
}

impl<'a, T: VecTrait> FirstVec<'a, T> {
    #[inline(always)]
    pub fn truncate(&mut self, num: usize) {
        let first_length = self.foo.first_length;

        let num = if num > first_length {
            first_length
        } else {
            num
        };

        let diff = first_length - num;

        let total_len = self.foo.inner.borrow().len();

        let (_keep, rest) = self.foo.inner.borrow_mut().split_at_mut(num);
        let (slice_to_remove, rest) = rest.split_at_mut(diff);

        if rest.len() > slice_to_remove.len() {
            let (_rest, slice_to_move) = rest.split_at_mut(rest.len() - slice_to_remove.len());
            slice_to_remove.swap_with_slice(slice_to_move);
        } else {
            let (slice_to_move, _dont_bother_moving) = slice_to_remove.split_at_mut(rest.len());
            slice_to_move.swap_with_slice(rest);
        }

        self.foo.inner.borrow_mut().truncate(total_len - diff);

        self.foo.first_length = num;
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T::T] {
        &self.foo.inner.borrow()[..self.foo.first_length]
    }
    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T::T] {
        &mut self.foo.inner.borrow_mut()[..self.foo.first_length]
    }

    #[inline(always)]
    pub fn push(&mut self, a: T::T) {
        let total_len = self.foo.inner.borrow().len();

        self.foo.inner.borrow_mut().push(a);

        //now len is actually one less than current length.
        self.foo
            .inner
            .borrow_mut()
            .swap(self.foo.first_length, total_len);

        self.foo.first_length += 1;
    }
}

#[derive(Debug)]
pub struct SecondVec<'a, T> {
    foo: &'a mut TwoUnorderedVecs<T>,
}
impl<'a, T: VecTrait> RetainMutUnordered<T::T> for SecondVec<'a, T> {
    #[inline(always)]
    fn truncate(&mut self, num: usize) {
        SecondVec::truncate(self, num);
    }

    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [T::T] {
        SecondVec::as_slice_mut(self)
    }
}

impl<'a, T: VecTrait> core::ops::Deref for SecondVec<'a, T> {
    type Target = [T::T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T: VecTrait> core::ops::DerefMut for SecondVec<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<'a, T: VecTrait> SecondVec<'a, T> {
    #[inline(always)]
    pub fn as_slice(&self) -> &[T::T] {
        &self.foo.inner.borrow()[self.foo.first_length..]
    }
    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T::T] {
        &mut self.foo.inner.borrow_mut()[self.foo.first_length..]
    }

    #[inline(always)]
    pub fn push(&mut self, b: T::T) {
        self.foo.inner.borrow_mut().push(b);
    }

    #[inline(always)]
    pub fn truncate(&mut self, num: usize) {
        self.foo
            .inner
            .borrow_mut()
            .truncate(self.foo.first_length + num);
    }
}

///
/// Abstract over a Vec<T> and a &mut Vec<T>
///
pub trait VecTrait {
    type T;
    fn borrow_mut(&mut self) -> &mut Vec<Self::T>;
    fn borrow(&self) -> &Vec<Self::T>;
}

impl<T> VecTrait for Vec<T> {
    type T = T;
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut Vec<Self::T> {
        self
    }

    #[inline(always)]
    fn borrow(&self) -> &Vec<Self::T> {
        self
    }
}

impl<T> VecTrait for &mut Vec<T> {
    type T = T;
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut Vec<Self::T> {
        self
    }
    #[inline(always)]
    fn borrow(&self) -> &Vec<Self::T> {
        self
    }
}

#[derive(Debug)]
pub struct TwoUnorderedVecs<T> {
    inner: T,
    first_length: usize,
}

impl<T> Default for TwoUnorderedVecs<Vec<T>> {
    #[inline(always)]
    fn default() -> Self {
        TwoUnorderedVecs::new()
    }
}

impl<T: VecTrait> TwoUnorderedVecs<T> {
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
        self.inner.borrow_mut().clear();
    }

    #[inline(always)]
    pub fn as_vec(&self) -> &Vec<T::T> {
        self.inner.borrow()
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> (&mut [T::T], &mut [T::T]) {
        self.inner.borrow_mut().split_at_mut(self.first_length)
    }

    #[inline(always)]
    pub fn as_slice(&self) -> (&[T::T], &[T::T]) {
        self.inner.borrow().split_at(self.first_length)
    }
}

impl<'a, T> TwoUnorderedVecs<&'a mut Vec<T>> {
    #[inline(always)]
    fn from_mut(inner: &'a mut Vec<T>) -> TwoUnorderedVecs<&'a mut Vec<T>> {
        let first_length = inner.len();
        TwoUnorderedVecs {
            inner,
            first_length,
        }
    }
}

impl<T> TwoUnorderedVecs<Vec<T>> {
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
    fn from_vec(inner: Vec<T>) -> Self {
        let first_length = inner.len();
        TwoUnorderedVecs {
            inner,
            first_length,
        }
    }
}

impl<'a, T> From<&'a mut Vec<T>> for TwoUnorderedVecs<&'a mut Vec<T>> {
    #[inline(always)]
    fn from(a: &'a mut Vec<T>) -> TwoUnorderedVecs<&'a mut Vec<T>> {
        TwoUnorderedVecs::from_mut(a)
    }
}

impl<T> From<Vec<T>> for TwoUnorderedVecs<Vec<T>> {
    #[inline(always)]
    fn from(a: Vec<T>) -> TwoUnorderedVecs<Vec<T>> {
        TwoUnorderedVecs::from_vec(a)
    }
}

impl<T> From<TwoUnorderedVecs<Vec<T>>> for Vec<T> {
    #[inline(always)]
    fn from(a: TwoUnorderedVecs<Vec<T>>) -> Vec<T> {
        a.inner
    }
}

impl<'a, T> From<TwoUnorderedVecs<&'a mut Vec<T>>> for &'a mut Vec<T> {
    #[inline(always)]
    fn from(a: TwoUnorderedVecs<&'a mut Vec<T>>) -> &'a mut Vec<T> {
        a.inner
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
