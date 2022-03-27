use twounordered::*;

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
fn test_x_borrowed() {
    let mut backed_vec = Vec::new();
    let mut k = TwoUnorderedVecs::from(&mut backed_vec);
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
fn test_truncate_zero() {
    let mut k: TwoUnorderedVecs<Vec<u32>> = TwoUnorderedVecs::new();

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
    assert_eq!(k.first().len(), 3);
    assert_eq!(k.as_vec().len(), 4);
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

    assert_eq!(k.first().len(), 4);

    k.first().truncate(2);
    k.second().truncate(1);

    slices_match(&k.first(), &[3, 9]);
    slices_match(&k.second(), &[8]);

    assert_eq!(k.first().len(), 2);
    assert_eq!(k.second().len(), 1);

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
