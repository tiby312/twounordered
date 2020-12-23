

///Two unordered vecs backed by one vec.
///Pushing and retaining from the first cec,
///can change the ordering of the second vec.
///Assume both vecs ordering can change at any time.
#[derive(Debug)]
pub struct TwoUnorderedVecs<'a, T> {
    inner: &'a mut Vec<T>,
    first_length: usize,
}

impl<'a, T> TwoUnorderedVecs<'a, T> {
    #[inline(always)]
    pub fn new(inner: &'a mut Vec<T>) -> Self {
        TwoUnorderedVecs {
            inner,
            first_length: 0,
        }
    }
    #[inline(always)]
    pub fn get_both_mut(&mut self) -> (&mut [T],&mut [T]) {
        self.inner.split_at_mut(self.first_length)
    }


    #[inline(always)]
    pub fn get_first_mut(&mut self) -> &mut [T] {
        &mut self.inner[..self.first_length]
    }

    #[inline(always)]
    pub fn get_second_mut(&mut self) -> &mut [T] {
        &mut self.inner[self.first_length..]
    }

    #[inline(always)]
    pub fn push_first(&mut self, a: T) {
        let total_len = self.inner.len();

        self.inner.push(a);

        //now len is actually one less than current length.
        self.inner.swap(self.first_length, total_len);

        self.first_length += 1;
    }

    #[inline(always)]
    pub fn push_second(&mut self, b: T) {
        self.inner.push(b);
    }

    //   [-------xxx][==============]
    //   [-----xxxxx][===]
    #[inline(always)]
    pub fn truncate_first(&mut self, num: usize) {
        let total_len = self.inner.len();

        //the number to be removed
        let diff = self.first_length - num ;

        
        let target_ptr=&mut self.inner[num] as *mut _;
        let source_ptr=unsafe{self.inner.as_mut_ptr().offset( (total_len-diff.min(total_len-self.first_length)) as isize )};
        unsafe{
            core::ptr::drop_in_place(target_ptr);
            core::ptr::copy(source_ptr,target_ptr,diff);
        }

        self.first_length = num;

        unsafe{
            self.inner.set_len(total_len-diff);
    
        }
    }

    #[inline(always)]
    pub fn truncate_second(&mut self, num: usize) {
        self.inner.truncate(self.first_length + num);
    }

    #[inline(always)]
    pub fn retain_first_mut_unordered(&mut self, mut func: impl FnMut(&mut T) -> bool) {
        let len = self.get_first_mut().len();
        let mut del = 0;
        {
            //let v = &mut **self;
            let v = self.get_first_mut();

            let mut cursor = 0;
            for _ in 0..len {
                if !func(&mut v[cursor]) {
                    v.swap(cursor, len - 1 - del);
                    del += 1;
                } else {
                    cursor += 1;
                }
            }
        }
        if del > 0 {
            self.truncate_first(len - del);
        }
    }

    #[inline(always)]
    pub fn retain_second_mut_unordered(&mut self, mut func: impl FnMut(&mut T) -> bool) {
        let len = self.get_second_mut().len();
        let mut del = 0;
        {
            //let v = &mut **self;
            let v = self.get_second_mut();

            let mut cursor = 0;
            for _ in 0..len {
                if !func(&mut v[cursor]) {
                    v.swap(cursor, len - 1 - del);
                    del += 1;
                } else {
                    cursor += 1;
                }
            }
        }
        if del > 0 {
            self.truncate_second(len - del);
        }
    }
}




    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_truncate() {
            
            let mut vec = Vec::new();
            let mut k = TwoUnorderedVecs::new(&mut vec);
            k.push_first(0);
            k.push_first(1);
            k.push_first(2);
            k.push_second(3);
            k.push_second(4);
            k.push_second(5);
            k.push_second(6);
            assert_eq!(k.get_both_mut(),(&mut [0,1,2] as &mut [_],&mut [3,4,5,6] as &mut [_]));
            
            k.truncate_first(2);
            assert_eq!(k.get_both_mut(),(&mut [0,1] as &mut [_],&mut [6,3,4,5] as &mut [_]));
            
        }
        #[test]
        fn test_truncate2(){
            let mut vec = Vec::new();
            let mut k = TwoUnorderedVecs::new(&mut vec);
            k.push_first(0);
            k.push_first(1);
            k.push_first(2);
            k.push_first(3);
            k.push_first(4);
            k.push_second(5);
            k.truncate_first(3);
            assert_eq!(k.get_both_mut(),(&mut [0,1,2] as &mut [_],&mut [5] as &mut [_]));
            assert_eq!(k.first_length,3);
            assert_eq!(k.inner.len(),4);
        }
        #[test]
        fn test_other() {
            let mut vec = Vec::new();
            let mut k = TwoUnorderedVecs::new(&mut vec);
            k.push_second(5);
            k.push_first(6);
            k.push_second(5);
            k.push_first(6);
            k.push_second(5);
            k.push_first(6);
            k.push_second(5);
            k.push_first(6);
            k.truncate_first(2);
            k.truncate_second(2);
            slices_match(k.get_first_mut(), &mut [6, 6]);
            slices_match(k.get_second_mut(), &mut [5, 5]);
        }

        //TODO put in module
        #[test]
        fn test_push() {
            let mut vec = Vec::new();
            let mut k = TwoUnorderedVecs::new(&mut vec);
            k.push_first(9);
            k.push_second(0);
            k.push_first(3);

            k.push_first(6);
            k.push_second(8);
            k.push_first(5);

            slices_match(k.get_first_mut(), &mut [9, 3, 6, 5]);
            slices_match(k.get_second_mut(), &mut [0, 8]);

            assert_eq!(k.first_length, 4);

            k.truncate_first(2);
            k.truncate_second(1);

            slices_match(k.get_first_mut(), &mut [3, 9]);
            slices_match(k.get_second_mut(), &mut [8]);

            assert_eq!(k.get_first_mut().len(), 2);
            assert_eq!(k.get_second_mut().len(), 1);
            assert_eq!(k.first_length, 2);

            k.push_first(4);
            k.push_first(6);
            k.push_first(7);
            k.push_first(8);

            k.push_second(7);
            k.push_second(3);
            k.push_second(2);
            k.push_second(4);

            k.retain_first_mut_unordered(|&mut a| a % 2 == 1);
            k.retain_second_mut_unordered(|&mut a| a % 2 == 0);

            slices_match(k.get_first_mut(), &mut [9, 3, 7]);
            slices_match(k.get_second_mut(), &mut [8, 2, 4]);
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
