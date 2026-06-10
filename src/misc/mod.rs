#![allow(unused)]

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

// pub fn invert_map<K, V>(m: &mut HashMap<K, V>) -> HashMap<V, K>
// where
//     K: Eq + Hash,
//     V: Eq + Hash,
// {
//     m.drain().map(|(k, v)| (v, k)).collect()
// }

pub fn invert_map<K, V>(mut m: HashMap<K, V>) -> HashMap<V, K>
where
    K: Copy + Eq + Hash,
    V: Copy + Eq + Hash,
{
    m.drain().map(|(k, v)| (v, k)).collect()
}

pub fn find_duplicates<V: Hash + Eq + Copy>(v: &[V]) -> Vec<V> {
    let mut m: HashMap<V, u32> = HashMap::new();

    let counts = v.iter().fold(m, |mut acc, &cur| {
        *acc.entry(cur).or_insert(0) += 1;
        acc
    });

    counts
        .iter()
        .filter_map(|(&k, &v)| if v > 1 { Some(k) } else { None })
        .collect()
}

struct Acc<T> {
    prev: T,
    ret: Option<T>,
}

pub fn missing_number(v: &[u32]) -> Option<u32> {
    if v.is_empty() {
        return None;
    }

    if v.len() == 1 {
        return None;
    }

    let hd = *v.first()?;

    let ret = Acc {
        prev: hd,
        ret: None,
    };

    v[1..]
        .iter()
        .fold(ret, |acc, &cur| {
            if cur - 1 != acc.prev {
                Acc {
                    ret: Some(cur - 1),
                    ..acc
                }
            } else {
                Acc { prev: cur, ..acc }
            }
        })
        .ret
}

pub fn rotate_left<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    if n > v.len() {
        return v.to_vec();
    }

    let mut ret: Vec<T> = vec![];
    let right = &v[n..];
    let left = &v[0..n];

    for item in right.iter() {
        ret.push(item.to_owned())
    }

    for item in left.iter() {
        ret.push(item.to_owned())
    }

    ret
}

pub fn product(v: &[i32]) -> i32 {
    v.iter().fold(1, |mut acc, cur| {
        acc *= cur;
        acc
    })
}

pub fn all_some<T>(v: &[Option<T>]) -> bool {
    v.iter().all(|el| el.is_some())
}

pub fn count_nones<T>(v: &[Option<T>]) -> usize {
    v.iter()
        .fold(0, |acc, curr| if curr.is_none() { acc + 1 } else { acc })
}

pub fn last<T: Clone>(v: &[T]) -> Option<T> {
    match v.is_empty() {
        true => None,
        _ => v.last().to_owned().cloned(),
    }
}

pub fn zip_vecs<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter().cloned().zip(b.iter().cloned()).collect()
}

pub fn repeat(s: &str, n: usize) -> String {
    let mut ret = "".to_string();

    for _ in 0..n {
        ret.push_str(s);
    }

    ret
}

pub fn min_max(v: &[i32]) -> Option<(i32, i32)> {
    if v.is_empty() {
        return None;
    }

    if v.len() == 1 {
        let &hd = v.first()?;
        return Some((hd, hd));
    }

    let r = v.iter().fold((i32::MAX, i32::MIN), |acc, &cur| {
        let new_min = if cur < acc.0 { cur } else { acc.0 };
        let new_max = if cur > acc.1 { cur } else { acc.1 };
        (new_min, new_max)
    });

    Some(r)
}

pub fn all_eq<T: PartialEq + std::default::Default>(v: &[T]) -> bool {
    match v.split_first() {
        None => true,
        Some((hd, tl)) => tl.iter().all(|it| it == hd),
    }
}

pub fn clamp_all(v: &[i32], lo: i32, hi: i32) -> Vec<i32> {
    v.iter()
        .map(|&i| {
            if i < lo {
                return lo;
            }

            if i > hi { hi } else { i }
        })
        .collect()
}

pub fn count_pairs(s: &str, open: char, close: char) -> usize {
    let (l, r) = s.chars().fold((0, 0), |acc, ch| match ch {
        v if v == open => (acc.0 + 1, acc.1),
        v if v == close => (acc.0, acc.1 + 1),
        _ => acc,
    });

    let (larger, smaller) = if l >= r { (l, r) } else { (r, l) };
    smaller as usize
}
