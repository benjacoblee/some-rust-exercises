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

#[test]
fn it_increments_all_values_in_hashmap() {
    let mut m: HashMap<String, i32> = vec![("a".to_string(), 1), ("b".to_string(), 1)]
        .into_iter()
        .collect();
    increment_all(&mut m);
    let r = m.iter().map(|(_, &v)| v).collect::<Vec<i32>>();
    assert!(r.iter().all(|&v| v == 2))
}

#[test]
fn it_keeps_items_within_threshold() {
    let t = 32;
    let m: HashMap<String, i32> = vec![("a".to_string(), 42), ("b".to_string(), 1)]
        .into_iter()
        .collect();
    let r = filter_map_values(&m, t);
    assert!(r.values().len() == 1);
}

#[test]
fn it_sorts_freqmap_by_freq_desc() {
    let s = "abcabcaaaaaaaaaa!";
    let r = sorted_by_freq(s);
    assert!(r.first().unwrap().0 == 'a');
    assert!(r.last().unwrap().0 == '!');
}

#[test]
fn it_keeps_maxes() {
    let pairs = vec![
        ("a".to_string(), 5),
        ("b".to_string(), 3),
        ("a".to_string(), 8),
        ("b".to_string(), 2),
    ];

    let r = max_per_key(pairs);

    assert!(r.get("a") == Some(&8));
    assert!(r.get("b") == Some(&3));
}

#[test]
fn it_transposes_records() {
    let r1: HashMap<String, i32> = vec![("a".into(), 2), ("b".into(), 3)].into_iter().collect();
    let r2: HashMap<String, i32> = vec![("a".into(), 5)].into_iter().collect();

    let records = vec![r1, r2];
    let result = transpose_records(records);

    assert_eq!(result.get("a"), Some(&vec![2, 5]));
}

#[test]
fn it_flattens_options() {
    let v = vec![Some(2), Some(3), None];
    let r = flatten_options(v);
    assert_eq!(r, vec![2, 3]);
}
