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

#[test]
pub fn it_counts_pairs_correctly() {
    let open = '(';
    let close = ')';
    let s = "((()))";
    assert_eq!(count_pairs(s, open, close), 3);
    let s = "((())))";
    assert_eq!(count_pairs(s, open, close), 3);
    let s = "(((";
    assert_eq!(count_pairs(s, open, close), 0);
}

#[test]
pub fn it_converts_csv_to_table() {
    let v = &["alice,25,singapore", "bob,30,london", "charlie,22,tokyo"];
    let r = csv_to_table(v);
    let e = vec![
        "alice".to_string(),
        "25".to_string(),
        "singapore".to_string(),
    ];
    assert_eq!(r.first(), Some(e.as_ref()))
}

#[test]
pub fn it_parses_hms_to_string() {
    assert_eq!(hms_to_s("01:30:45"), Ok(5445));
}

#[test]
pub fn it_abbreviates_names() {
    assert_eq!(abbreviate("John Ronald Tolkien"), "J. R. Tolkien")
}

#[test]
pub fn it_aggregates_values_to_hashmap_from_vec() {
    let v = &[
        ("food", 10.5),
        ("transport", 5.0),
        ("food", 8.0),
        ("transport", 3.5),
    ];

    let r = rollup(v);

    assert_eq!(r.get("food"), Some(&18.5));
    assert_eq!(r.get("transport"), Some(&8.5));
}

#[test]
pub fn it_converges() {
    let f = |x: u32| if x > 0 { x - 1 } else { x };
    let v = 3;
    assert_eq!(converge(0, f), 0);
    assert_eq!(converge(3, f), 0);
}

#[test]
pub fn it_impls_stack() {
    let mut s = Stack::new();
    s.push(2);
    assert!(s.pop() == Some(2));
    assert!(s.empty())
}

#[test]
pub fn it_impls_queue() {
    let mut q = Queue::new();
    [1, 2, 3].iter().for_each(|&i| q.enqueue(i));
    assert!(q.front() == Some(&1));
    assert!(q.dequeue() == Some(1))
}

#[test]
pub fn it_flattens_nested_values() {
    let n = Nested::Values(vec![
        Nested::Val(1),
        Nested::Values(vec![Nested::Val(2), Nested::Val(3)]),
        Nested::Val(4),
    ]);

    let r = flatten_nested(n, 1);
    assert_eq!(
        r,
        vec![
            Nested::Val(1),
            Nested::Val(2),
            Nested::Val(3),
            Nested::Val(4)
        ]
    )
}

#[test]
pub fn it_maps_vecs_mutably() {
    let mut v = vec![1, 2, 3];
    map_in_place(&mut v, |x| x * 100);
    assert_eq!(v, vec![100, 200, 300]);
}

#[test]
pub fn it_finds_indices_that_match_pred() {
    let v = &[1, 2, 3, 4, 5];
    let pred = |&x: &i32| x > 2;
    let positions = find_all_idx(v, pred);
    assert_eq!(positions, vec![2, 3, 4]);
}

#[test]
pub fn it_averages_values_of_map() {
    let mut m: HashMap<String, f64> = HashMap::new();
    m.insert("a".into(), 1.0);
    m.insert("b".into(), 2.0);
    m.insert("c".into(), 3.0);
    let r = map_average(&m);
    assert_eq!(r, Some(2.0));
}

#[test]
pub fn it_impls_pipeline() {
    let p = Pipeline::new(1);
    let f = |x: &i32| println!("{x}");
    let v = p.pipe(|x| x * 2).tap(f).pipe(|x| x * 100).tap(f).finish();
    assert_eq!(v, 200);
}

#[test]
pub fn it_makes_row() {
    let v = Row::new(3, 5);
    assert_eq!(v.make(), &[3, 6, 9, 12, 15]);
}

#[test]
pub fn it_makes_table() {
    let r = mult_table(3);
    assert_eq!(r.len(), 3);
    assert_eq!(r, &[&[1, 2, 3], &[2, 4, 6], &[3, 6, 9]])
}
