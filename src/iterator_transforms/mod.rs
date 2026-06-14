#![allow(unused)]

use std::fmt::Display;

pub fn scan_running_max(v: &[i32]) -> Vec<i32> {
    v.iter()
        .scan(i32::MIN, |state, v| {
            if v > state {
                *state = *v;
                Some(*state)
            } else {
                Some(*state)
            }
        })
        .collect()
}

pub fn take_last<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    if n >= v.len() {
        v.to_vec()
    } else {
        let i = v.len() - n;
        v[i..].to_vec()
    }
}

pub fn step_by_sum(v: &[i32], step: usize) -> i32 {
    v.iter().enumerate().fold(
        0,
        |acc, (i, cur)| {
            if (i + 1) % step == 0 { acc + cur } else { acc }
        },
    )
}

pub fn step_by_sum2(v: &[i32], step: usize) -> i32 {
    v.iter().skip(step - 1).step_by(step).sum()
}

pub fn zip_sum(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b.iter()).map(|(i, j)| i + j).collect()
}

pub fn enumerate_filter<T, F>(v: &[T], pred: F) -> Vec<(usize, &T)>
where
    F: Fn(&T) -> bool,
{
    v.iter().enumerate().filter(|x| pred(x.1)).collect()
}

pub fn fold_strings(v: &[&str], sep: &str) -> String {
    match v.split_first() {
        None => "".to_string(),
        Some((hd, tl)) => tl.iter().fold(hd.to_string(), |acc, curr| {
            format!("{}{}{}", acc, sep, curr)
        }),
    }
}

pub fn map_alternating<T: Clone + Display>(v: &[T], f: impl Fn(T) -> T) -> Vec<T> {
    v.iter()
        .enumerate()
        .map(|(pos, elem)| {
            if pos % 2 == 0 {
                f(elem.clone())
            } else {
                elem.clone()
            }
        })
        .collect()
}
