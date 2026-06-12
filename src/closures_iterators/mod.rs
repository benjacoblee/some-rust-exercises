#![allow(unused)]

use std::{collections::HashMap, hash::Hash, ops::Index};

pub fn apply_all(v: i32, transforms: &[fn(i32) -> i32]) -> i32 {
    let mut ret = v;
    transforms.iter().for_each(|f| ret = f(ret));
    ret
}

pub fn squares_table(n: u32) -> HashMap<u32, u32> {
    let r = 1..=n;
    r.into_iter()
        .map(|i| (i, i * i))
        .collect::<HashMap<u32, u32>>()
}

pub fn all_words<'a>(sentences: &'a [&str]) -> Vec<&'a str> {
    sentences
        .iter()
        .flat_map(|&v| Some(v.split(" ")))
        .flatten()
        .collect()
}

pub fn take_while_positive(v: &[i32]) -> &[i32] {
    let pos = v.iter().position(|&i| i <= 0);
    match pos {
        None => v,
        Some(p) => &v[0..p],
    }
}

pub fn skip_leading_zeros(v: &[i32]) -> &[i32] {
    let pos = v.iter().position(|&i| i != 0);

    if let Some(p) = pos {
        return &v[p..];
    }

    v
}

pub fn running_product(v: &[i32]) -> Vec<i32> {
    v.iter()
        .scan(1, |state, x| {
            *state *= x;
            Some(*state)
        })
        .collect::<Vec<i32>>()
}

pub fn unzip<A, B>(v: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    v.into_iter().fold((vec![], vec![]), |mut acc, (a, b)| {
        acc.0.push(a);
        acc.1.push(b);
        acc
    })
}

pub fn min_index(v: &[i32]) -> Option<usize> {
    if v.is_empty() {
        return None;
    }

    let r = v.iter().enumerate().fold(
        0,
        |acc, (pos, &cur)| {
            if cur < acc { pos as i32 } else { acc }
        },
    );

    Some(r as usize)
}

pub fn concat_vecs<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().chain(b.iter()).cloned().collect::<Vec<T>>()
}

pub fn join_with(v: &[i32], sep: &str) -> String {
    v.iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}

pub fn every_nth<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    v.iter().step_by(n).cloned().collect::<Vec<T>>()
}

pub fn even_indexed_items<T: Clone>(v: &[T]) -> Vec<T> {
    v.iter()
        .enumerate()
        .filter_map(|(pos, item)| {
            if pos % 2 == 0 {
                Some(item.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn product(v: &[i64]) -> i64 {
    v.iter().product()
}

pub fn min_max(v: &[i32]) -> Option<(i32, i32)> {
    match v.split_first() {
        None => None,
        Some((&hd, tl)) => {
            let res = tl.iter().fold((hd, hd), |mut acc, &cur| {
                if cur > acc.0 && cur < acc.1 {
                    acc
                } else if cur < acc.0 {
                    acc.0 = cur;
                    acc
                } else {
                    acc.1 = cur;
                    acc
                }
            });

            Some(res)
        }
    }
}

pub fn expand_words(words: &[&str]) -> Vec<char> {
    words.iter().flat_map(|s| s.chars()).collect()
}

pub fn total_variation(v: &[i32]) -> i32 {
    v.windows(2).fold(0, |acc, window| {
        let (a, b) = (window[0], window[1]);
        let diff = (a - b).abs();
        acc + diff
    })
}

pub fn apply_twice<T>(f: impl Fn(T) -> T, x: T) -> T {
    f(f(x))
}

pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

pub fn map_if<T>(value: T, condition: bool, f: impl FnOnce(T) -> T) -> T {
    if condition { f(value) } else { value }
}

pub fn my_reduce<T: Clone, F>(v: &[T], f: &F) -> Option<T>
where
    F: Fn(T, &T) -> T,
{
    match v.split_first() {
        None => None,
        Some((hd, tl)) => {
            if (tl.is_empty()) {
                return Some(hd.clone());
            }

            let rest = my_reduce(tl, f);
            rest.map(|t| f(t, hd))
        }
    }
}

pub fn curried_add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Student {
    pub grade: u32,
}

pub fn sort_by_grade<K: Ord, F>(mut students: Vec<Student>, mut f: F) -> Vec<Student>
where
    F: FnMut(&Student) -> K,
{
    students.sort_by_key(f);
    students
}

pub fn apply_until<T: Clone, F, G>(val: T, f: F, done: G) -> T
where
    F: Fn(T) -> T,
    G: Fn(T) -> bool,
{
    let new_val = f(val);

    if done(new_val.clone()) {
        apply_until(new_val, f, done)
    } else {
        new_val
    }
}

pub fn three_way_partition<T, F, G>(v: Vec<T>, is_low: F, is_high: G) -> (Vec<T>, Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
    G: Fn(&T) -> bool,
{
    v.into_iter()
        .fold((vec![], vec![], vec![]), |mut acc, curr| {
            if !is_low(&curr) && !is_high(&curr) {
                acc.1.push(curr);
                return acc;
            }
            if is_low(&curr) {
                acc.0.push(curr);
                acc
            } else {
                acc.2.push(curr);
                acc
            }
        })
}

pub fn map_pair<T, U>(pair: (T, T), f: impl Fn(T) -> U) -> (U, U) {
    let (a, b) = pair;
    (f(a), f(b))
}
