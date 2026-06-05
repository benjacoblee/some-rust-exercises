#![allow(unused)]

use std::collections::HashMap;

pub fn intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().filter(|i| b.contains(i)).cloned().collect()
}

pub fn running_sum(v: &[i32]) -> Vec<i32> {
    let mut prev = 0;
    v.iter()
        .map(|el| {
            let ret = el + prev;
            prev += el;
            ret
        })
        .collect()
}

pub fn interleave<T: Copy>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.chunks(1)
        .zip(b.chunks(1))
        .flat_map(|(a, b)| a.iter().chain(b))
        .copied() // &f64 -> f64, optional
        .collect()
}

pub fn fill_none<T: Clone>(v: Vec<Option<T>>, default: T) -> Vec<T> {
    v.into_iter()
        .map(|elt| elt.unwrap_or(default.clone()))
        .collect()
}

pub fn char_counts(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    })
}

pub fn first_duplicate<T: Eq + std::hash::Hash + Clone>(v: &[T]) -> Option<T> {
    v.iter()
        .fold((HashMap::new(), None), |(mut acc, seen), cur| {
            let c = acc.get(cur).unwrap_or(&0) + 1;

            if c > 1 && seen.is_none() {
                acc.insert(cur, c + 1);
                (acc, Some(cur.clone()))
            } else {
                acc.insert(cur, c + 1);
                (acc, seen)
            }
        })
        .1
}

pub fn chunk<T: Clone>(v: Vec<T>, size: usize) -> Vec<Vec<T>> {
    v.chunks(size).map(|x| x.to_vec()).collect()
}

pub fn zip_with<A: Clone, B: Clone, C>(a: Vec<A>, b: Vec<B>, f: impl Fn(A, B) -> C) -> Vec<C> {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| f(a.to_owned(), b.to_owned()))
        .collect()
}
