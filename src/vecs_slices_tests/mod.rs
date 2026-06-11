#![allow(unused)]

use crate::vecs_slices::*;

#[cfg(test)]
#[test]
fn it_rotates_a_slice() {
    assert_eq!(rotate_left::<i32>(&[], 1), vec![]);
    assert_eq!(rotate_left(&[1], 1), vec![1]);
    assert_eq!(rotate_left(&[1], 2), vec![1]);
    assert_eq!(rotate_left(&[1, 2, 3], 1), vec![2, 3, 1]);
    assert_eq!(rotate_left(&[1, 2, 3], 2), vec![3, 1, 2]);
}

#[test]
fn it_dedups_conseq() {
    assert!(dedup_conseq(&[1, 2, 3, 4, 5]) == [1, 2, 3, 4, 5]);
    assert!(dedup_conseq(&[1, 1, 2, 2, 2, 2, 3, 4, 5]) == [1, 2, 3, 4, 5]);
    assert!(dedup_conseq(&[1, 1, 1, 1, 1, 2]) == [1, 2]);
}

#[test]
fn it_zips_3() {
    let a = vec![1, 2, 3];
    let b = vec!['a', 'b', 'c'];
    let c = vec![true, false, true];
    let r = zip3(&a, &b, &c);
    assert_eq!(r.first(), Some((1, 'a', true)).as_ref());
}

#[test]
fn it_cumulatively_multiplies() {
    assert_eq!(cumulative_prod(&[2, 3, 4, 5]), vec![2, 6, 24, 120]);
}

#[test]
fn it_flattens_one() {
    let v = &[vec![1, 2, 3]];
    let r = flatten_one(v);
    assert_eq!(r, &[1, 2, 3]);
}

#[test]
fn it_takes_while_less_than() {
    let threshold = 4;
    let v = &[1, 2, 3, 4, 5];
    assert_eq!(take_while_lt(v, threshold), [1, 2, 3]);
}
