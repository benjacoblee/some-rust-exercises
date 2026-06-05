#![allow(unused)]

use std::collections::{HashMap, HashSet};

pub fn as_str_slice(strings: &[String]) -> Vec<&str> {
    strings.iter().map(|s| s.as_str()).collect()
}

pub fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub fn clone_vec<T: Clone>(v: &[T]) -> Vec<T> {
    v.iter().map(|t| t.to_owned()).collect()
}

pub fn take_first<T>(v: Vec<T>) -> Option<T> {
    v.into_iter().next()
}

pub fn swap_ints(a: &mut i32, b: &mut i32) {
    // let temp = *a;
    // *a = *b;
    // *b = temp;

    std::mem::swap(&mut *a, &mut *b);
}

pub fn sort_in_place<T: Ord>(v: &mut [T]) {
    v.sort();
}

pub fn first_n<T>(v: &[T], n: usize) -> &[T] {
    &v[0..n]
}

pub fn unique_ordered<T: Eq + std::hash::Hash + Clone>(v: Vec<T>) -> Vec<T> {
    let mut map: HashMap<&T, bool> = HashMap::new();
    let mut ret: Vec<T> = vec![];

    v.iter().fold(map, |mut acc, cur| {
        let seen = *acc.entry(cur).or_default();

        if !seen {
            acc.insert(cur, true);
            ret.push(cur.to_owned());
        }

        acc
    });

    ret
}

pub fn add_unique(v: &mut Vec<i32>, item: i32) {
    if !v.contains(&item) {
        v.push(item)
    }
}

pub fn drain_all<T>(v: &mut Vec<T>) {
    v.drain(0..);
}
