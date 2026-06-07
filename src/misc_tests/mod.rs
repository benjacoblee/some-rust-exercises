#![allow(unused)]

use std::collections::HashMap;

use crate::misc::*;

#[cfg(test)]
#[test]
pub fn it_swaps_keys() {
    use crate::misc::invert_map;

    let mut m: HashMap<char, char> = HashMap::new();
    m.insert('a', 'b');
    let r = invert_map(m);
    assert_eq!(r.get(&'b'), Some(&'a'));
}

#[test]
pub fn it_finds_duplicates() {
    let v = &[1, 1, 1, 2, 3, 4, 5];
    let r = find_duplicates(v);
    assert!(r.len() == 1);
    assert!(r.first() == Some(&1));
}

#[test]
pub fn it_finds_missing_nums() {
    assert!(missing_number(&[]).is_none());
    assert!(missing_number(&[1]).is_none());
    assert!(missing_number(&[1, 2, 3]).is_none());
    assert!(missing_number(&[1, 2, 3, 5]).is_some());
    assert!(missing_number(&[1, 2, 3, 5]) == Some(4));
}

#[test]
pub fn it_rotates_slice() {
    let sl = &[1, 2, 3, 4, 5];
    let res = rotate_left(sl, 3);
    assert_eq!(res, vec![4, 5, 1, 2, 3]);
    let sl = &["a".to_string(), "b".to_string()];
    let res = rotate_left(sl, 1);
    assert_eq!(res, vec!["b".to_string(), "a".to_string()])
}

#[test]
pub fn it_gets_product() {
    let sl = &[1, 2, 3, 4, 5];
    assert_eq!(product(sl), 120);
}

#[test]
pub fn it_checks_if_all_are_some_correctly() {
    let somes = &[Some(1), Some(2), Some(3)];
    assert!(all_some(somes));
    let somes: &[Option<bool>; 1] = &[None];
    assert!(!all_some(somes))
}

#[test]
pub fn it_counts_nones() {
    let sl = &[Some(2), None, None];
    assert!(count_nones(sl) == 2);
}

#[test]
pub fn it_gets_last() {
    let sl = &["", "last"];
    assert_eq!(last(sl), Some("last"));
}

#[test]
pub fn it_zips_vecs() {
    let a = &[1, 2, 3];
    let b = &['a', 'b'];
    assert!(zip_vecs(a, b) == vec![(1, 'a'), (2, 'b')]);
}

#[test]
pub fn it_repeats_a_str() {
    assert!(repeat("foo", 3) == "foofoofoo")
}

#[test]
pub fn it_gets_min_max_for_empty() {
    assert!(min_max(&[]).is_none())
}

#[test]
pub fn it_gets_same_min_max_for_single_item_slice() {
    assert!(min_max(&[1]) == Some((1, 1)));
}

#[test]
pub fn it_gets_min_max_correctly() {
    let sl = &[1, 3, 1000, -44, -3];
    let r = min_max(sl);
    assert_eq!(r, Some((-44, 1000)));
}

#[test]
pub fn it_returns_eq_true_on_empty_list() {
    assert!(all_eq::<u32>(&[]));
}

#[test]
pub fn it_returns_eq_true_on_good_list() {
    let l = &[1, 1, 1, 1, 1];
    assert!(all_eq(l));
}

#[test]
pub fn it_returns_eq_false_on_bad_list() {
    let l = &[1, 2];
    assert!(!all_eq(l));
}

#[test]
pub fn it_clamps_i32s() {
    let v = &[1, 2, 3, 100, 44];
    let lo = 3;
    let hi = 50;
    let expected = &[3, 3, 3, 50, 44];
    let res = clamp_all(v, lo, hi);
    assert_eq!(&res, expected);
}
