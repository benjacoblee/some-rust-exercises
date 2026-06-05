#![allow(unused)]

use std::collections::HashMap;

use crate::collections::*;

#[cfg(test)]
#[test]
fn it_gets_intersection() {
    let res = intersection(&[1, 2, 3, 4, 5], &[4, 5, 6, 7, 8]);
    assert_eq!(res, vec![4, 5]);
}

#[test]
fn it_calculates_running_sum() {
    assert!(running_sum(&[]) == vec![]);
    let res = running_sum(&[1, 2, 3, 4]);
    assert!(res == vec![1, 3, 6, 10]);
    let res = running_sum(&[1, -1, 1, -1]);
    assert!(res == vec![1, 0, 1, 0]);
}

#[test]
fn it_interleaves() {
    let res = interleave(vec![1, 2, 3], vec![4, 5, 6]);
    assert!(res == vec![1, 4, 2, 5, 3, 6]);
}

#[test]
fn it_provides_fallback_for_none_values() {
    let res = fill_none(vec![Some("string"), None, Some("other")], "wow");
    assert!(res == vec!["string", "wow", "other"]);
}

#[test]
fn it_counts_occurrences() {
    let m: HashMap<char, usize> = vec![('a', 3), ('b', 2)].into_iter().collect();
    let s = char_counts("aaabb");
    assert!(m == s);
}

#[test]
fn it_finds_duplicates() {
    let res = first_duplicate(&[1, 2, 3, 1, 2]);
    assert!(res == Some(1));
    assert!(first_duplicate(&[1, 2, 3]).is_none());
}

#[test]
fn it_chunks_vecs() {
    let res = chunk(vec![1, 2, 3, 4, 5, 6], 2);
    let expected = vec![[1, 2], [3, 4], [5, 6]];
    assert!(res == expected);
}

#[test]
fn it_zips_with_fn() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let res = zip_with(a, b, |a, b| a + b);
    assert!(res == vec![5, 7, 9]);
}
