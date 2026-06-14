#![allow(unused)]

use std::{collections::HashMap, vec};

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

#[test]
pub fn it_gets_first_named_box_item() {
    let d = Dog { name: "dango" };
    let d2 = Dog { name: "daisy" };
    let h = Human { name: "wtv" };
    let v: &[Box<dyn Named>] = &[Box::new(d), Box::new(d2), Box::new(h)];
    let r = get_first_name(v);
    assert_eq!(r, Some("dango"))
}

#[test]
pub fn it_safely_zips() {
    let f = |a, b| (a, b);
    let a = Some("a");
    let b = Some(1);
    let r = safe_zip_with(a, b, f);
    assert_eq!(r, Some(("a", 1)));
    assert_eq!(safe_zip_with(None, Some(2), f), None);
}

#[test]
pub fn it_converts_dog_to_string() {
    let d = Dog { name: "Dango" };
    let r = make_string(d);
    assert_eq!(r, "Dango".to_string())
}

#[test]
pub fn it_impls_deref() {
    let mv = MyVec(vec![1, 2, 3, 4]);
    assert_eq!(vec![1, 2, 3, 4], *mv)
}

#[test]
pub fn it_swaps() {
    let p = Pair { a: 1, b: '1' };
    let r = swap(p);
    assert_eq!(r.a, '1');
}

#[test]
pub fn it_impls_iter_for_counter() {
    let mut c = Counter {
        current: 1,
        max: 100,
    }
    .collect::<Vec<u32>>();

    assert_eq!(c.len(), 100);
}

#[test]
pub fn it_impls_map_for_stack() {
    let mut stk = Stack { v: vec![1, 2, 3] };
    stk.push(4);
    let new_stk = stk.map(|item| item.to_string());
    assert_eq!(*new_stk, vec!["1", "2", "3", "4"]);
}

#[test]
pub fn it_impls_display_for_season() {
    let s = Season::Winter;
    let r = s.to_string();
    assert_eq!(r, "Season(Winter)");
}

#[test]
pub fn it_impls_map_and_unwrap_for_mymaybe() {
    let m = MyMaybe::Nothing;
    assert_eq!(m.unwrap_or(0), 0);
    let m = MyMaybe::Just(2);
    let m = m.map(|i| i * 100);
    assert_eq!(m.unwrap_or(0), 200);
}

#[test]
pub fn it_impls_index_for_wrapper_type() {
    let m = Mrapper::new().add("this", "that");
    let v = m["this"];
    assert_eq!(v, "that");
}

#[test]
pub fn it_impls_from_iter_for_custom_struct() {
    let mut v = vec![100, 25, 28];
    let iter = v.into_iter();
    let r = SortedV::from_iter(iter);
    assert_eq!(r.0, vec![25, 28, 100]);
}

#[test]
pub fn it_pipes() {
    let r = pipe(pipe(2, |x| x + 1), |x| x * 100);
    assert_eq!(r, 300);
}
