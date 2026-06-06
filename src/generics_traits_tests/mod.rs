#![allow(unused)]

use std::collections::HashMap;

use crate::generics_traits::*;

#[cfg(test)]
#[test]
pub fn it_gets_the_max_el() {
    let vals = &[38, 22, 41, 67];
    let res = max_el(vals);
    assert!(res == Some(&67))
}

#[test]
pub fn it_sums_nums() {
    let vals = &[1, 2, 3];
    let res = sum_nums(vals);
    assert!(res == 6);
}

#[test]
pub fn it_converts_vec_to_hashset() {
    let v = vec![1, 1, 1, 1, 1];
    let res = to_set(v);
    assert!(res.len() == 1);
}

#[test]
pub fn it_impls_from_for_custom_type() {
    let kvs = vec![("a", 1), ("b", 2), ("c", 3)];
    let normal_hashmap: HashMap<&str, i32> = kvs.clone().into_iter().collect();
    let m: M<&str, i32> = kvs.into();
    assert_eq!(m.hm, normal_hashmap);
}

#[test]
pub fn it_impls_double_method_for_i32() {
    let i: i32 = 32;
    let doubled = i32::double(i);
    assert!(doubled == (i * 2))
}

#[test]
pub fn it_provides_fallback_if_parsing_fails() {
    let r = parse_or_default("s", 67);
    assert!(r == 67);
}

#[test]
pub fn it_impls_iter_for_even_range() {
    let e = EvenRange::new(2, 200);
    let v: Vec<u32> = e.into_iter().collect();
    assert_eq!(v.first(), Some(&2));
    assert_eq!(v.last(), Some(&200));
}

#[test]
pub fn it_merges_maps() {
    let a: HashMap<char, u32> = vec![('a', 24), ('b', 36)].into_iter().collect();
    let b: HashMap<char, u32> = vec![('c', 1), ('a', 1)].into_iter().collect();
    let c = merge_maps(&a, &b);
    let get = c.get(&'a').copied().unwrap_or(0);
    assert_eq!(get, 25);
    let get = c.get(&'b').copied().unwrap_or(0);
    assert_eq!(get, 36);
}

#[test]
pub fn it_runs_some_for_each() {
    let items = &[1, 2, 3, 4, 5];
    my_for_each(items, |item| println!("{}", item));
}
