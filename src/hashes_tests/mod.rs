#![allow(unused)]

use crate::hashes::*;
use std::collections::HashMap;

#[cfg(test)]
#[test]
fn it_prefers_second_map_when_merging_two_maps() {
    let map_a: HashMap<String, i32> = vec![
        ("a".to_string(), 2),
        ("b".to_string(), 3),
        ("c".to_string(), 4),
    ]
    .into_iter()
    .collect();
    let map_b: HashMap<String, i32> = vec![("a".to_string(), 100), ("b".to_string(), 200)]
        .into_iter()
        .collect();
    let expected: HashMap<String, i32> = vec![
        ("a".to_string(), 100),
        ("b".to_string(), 200),
        ("c".to_string(), 4),
    ]
    .into_iter()
    .collect();
    let r = prefer_snd(map_a, map_b);
    assert_eq!(r, expected);
}

#[test]
fn it_finds_repeated_values() {
    let m: HashMap<String, i32> = vec![
        ("a".to_string(), 2),
        ("b".to_string(), 2),
        ("c".to_string(), 1),
        ("d".to_string(), 1),
        ("e".to_string(), 4),
    ]
    .into_iter()
    .collect();

    let mut r = find_repeated_values(&m);
    r.sort();
    assert_eq!(r, &[1, 2]);
}

#[test]
fn it_inverts_a_map() {
    let mut m = HashMap::new();
    m.insert("a".to_string(), "b".to_string());
    let r = invert_map(m);
    assert!(r.contains_key("b"));
}

#[test]
fn it_removes_values_less_than_min() {
    let mut m = HashMap::new();
    m.insert("a".to_string(), 3);
    m.insert("b".to_string(), -2);
    let r = filter_map_values(m, 1);
    assert_eq!(r.values().len(), 1);
}

#[test]
fn test_overlapping_keys() {
    let a = HashMap::from([("x".into(), 1), ("y".into(), 2)]);
    let b = HashMap::from([("x".into(), 10), ("y".into(), 20)]);
    let result = add_maps(&a, &b);
    assert_eq!(result["x"], 11);
    assert_eq!(result["y"], 22);
}

#[test]
fn test_group_by_first_char() {
    let words = vec![
        "apple".to_string(),
        "avocado".to_string(),
        "banana".to_string(),
        "blueberry".to_string(),
        "c".to_string(),
        "".to_string(),
        "".to_string(),
    ];
    let r = group_by_first_char(words);
    assert_eq!(r[&'a'], vec!["apple", "avocado"]);
    assert_eq!(r[&'b'], vec!["banana", "blueberry"]);
    assert_eq!(r[&'c'], vec!["c"]);
}
