#![allow(unused)]

use std::fmt::Debug;

pub fn rotate_left<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    if v.is_empty() {
        return Vec::new();
    }

    if n > v.len() {
        return v.to_vec();
    }

    let mut left = v[n..].to_vec();
    let right = v[0..n].to_vec();
    left.extend(right);

    left
}

pub fn dedup_consecutive_lazy<T: PartialEq + Clone + Ord>(v: &[T]) -> Vec<T> {
    let mut mine = v.to_vec();
    mine.sort();
    mine.dedup();
    mine
}

pub fn group_conseq<T: PartialEq + Clone>(v: &[T]) -> Vec<&[T]> {
    v.chunk_by(|a, b| a == b).collect()
}

pub fn dedup_conseq<T: PartialEq + Clone + Ord + Debug>(v: &[T]) -> Vec<T> {
    group_conseq(v)
        .iter_mut()
        .flat_map(|ch| {
            let mut ret = ch.to_vec();
            ret.dedup();
            ret
        })
        .collect::<Vec<T>>()
}

pub fn zip3<A: Clone, B: Clone, C: Clone>(a: &[A], b: &[B], c: &[C]) -> Vec<(A, B, C)> {
    a.iter()
        .zip(b)
        .zip(c)
        .map(|((a_, b_), c_)| (a_.clone(), b_.clone(), c_.clone()))
        .collect()
}

pub fn cumulative_prod(v: &[i32]) -> Vec<i32> {
    v.iter()
        .scan(1, |prev, cur| {
            *prev *= *cur;
            Some(*prev)
        })
        .collect::<Vec<i32>>()
}

pub fn flatten_one<T: Clone>(v: &[Vec<T>]) -> Vec<T> {
    v.iter().flat_map(|it| it.to_vec()).collect()
}

pub fn take_while_lt(v: &[i32], threshold: i32) -> Vec<i32> {
    v.iter()
        .take_while(|&&it| it < threshold)
        .copied()
        .collect()
}
