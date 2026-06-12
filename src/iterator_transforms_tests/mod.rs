#![allow(unused)]

use std::vec;

use crate::iterator_transforms::*;

#[cfg(test)]
#[test]
fn it_tracks_running_max() {
    let v = &[1, 2, 100, 1, 1, 1000];
    let r = scan_running_max(v);
    assert_eq!(r, vec![1, 2, 100, 100, 100, 1000]);
}

#[test]
fn it_gets_last_n() {
    let (v, n) = (&[1, 2, 3, 4, 5, 6], 3);
    let r = take_last(v, n);
    assert_eq!(r, vec![4, 5, 6]);
    let (v, n) = (&[1, 2, 3], 3);
    let r = take_last(v, n);
    assert_eq!(r, v);
    let (v, n) = (&[1, 2, 3], 4);
    let r = take_last(v, n);
    assert_eq!(r, v);
    let (v, n) = (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 11], 2);
    let r = take_last(v, n);
    assert_eq!(r, vec![9, 11]);
}

#[test]
fn it_sums_every_nth() {
    let v = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    let r = step_by_sum(v, 3);
    assert_eq!(r, 18);
    let v = &[1, 2, 3, 4];
    assert_eq!(step_by_sum(v, 2), 6);
    assert_eq!(step_by_sum(v, 2), step_by_sum2(v, 2));
}
