#![allow(unused)]

use crate::ownership::*;

#[cfg(test)]
#[test]

fn it_converts_owned_to_str_slice() {
    let binding = &["abc".into(), "something else".into()];
    let strs = as_str_slice(binding);
    assert!(strs == vec!["abc", "something else"]);
}

#[test]
fn it_returns_longer_string() {
    assert_eq!(longest_str("asd", "asdd"), "asdd");
}

#[test]
fn it_clones_a_vector() {
    let r = clone_vec(&["".to_string()]);
    assert!(r == vec!["".to_string()]);
}

#[test]
fn it_returns_the_first_value() {
    let r = take_first(vec!["".to_string()]);
    assert_eq!(r, Some("".to_string()));
}

#[test]
fn it_swaps_ints() {
    let mut a = 6;
    let mut b = 7;

    swap_ints(&mut a, &mut b);

    assert_eq!(a, 7);
    assert_eq!(b, 6)
}

#[test]
fn it_sorts_in_place() {
    let mut v = vec![100, 5, 3, 26];
    sort_in_place(&mut v);
    assert_eq!(v, vec![3, 5, 26, 100])
}

#[test]
fn it_gets_first_n() {
    let v = &[1, 2, 3, 4, 5];
    let first_few = first_n(v, 3);
    assert_eq!(first_few, &[1, 2, 3]);
}

#[test]
fn it_gets_unique_ordered() {
    let v = vec![1, 1, 1, 5, 5, 10];
    let res = unique_ordered(v);
    assert_eq!(res, vec![1, 5, 10]);
}

#[test]
fn it_adds_if_not_present() {
    let mut v = vec![1, 2, 3, 4];
    add_unique(&mut v, 5);
    assert!(v.len() == 5);
}

#[test]
fn it_drains_a_vec() {
    let mut v = vec![1, 2, 3, 4, 5];
    drain_all(&mut v);
    assert!(v.is_empty());
}
