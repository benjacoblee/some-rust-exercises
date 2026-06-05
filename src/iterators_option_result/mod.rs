#![allow(unused)]

use std::{collections::HashMap, vec};

pub fn sum_even_squares(v: &[i32]) -> i32 {
    v.iter()
        .filter_map(|&n| if n % 2 == 0 { Some(n * n) } else { None })
        .sum::<i32>()
}

pub fn first_some<T>(v: Vec<Option<T>>) -> Option<T> {
    v.into_iter().find(|e| e.is_some()).unwrap_or(None)
}

pub fn oks<T, E>(v: Vec<Result<T, E>>) -> Vec<T> {
    v.into_iter().filter_map(|opt| opt.ok()).collect()
}

pub fn flatten<T: Clone>(v: Vec<Vec<T>>) -> Vec<T> {
    v.concat()
}

pub fn longest<'a>(strings: &'a [&str]) -> Option<&'a str> {
    let v = strings.iter().fold(
        "",
        |acc, &curr| {
            if acc.len() < curr.len() { curr } else { acc }
        },
    );

    match v {
        "" => None,
        _ => Some(v),
    }
}

pub fn unwrap_or_empty(v: Result<Vec<i32>, String>) -> Vec<i32> {
    v.unwrap_or_default()
}

pub fn flatten_option<T>(opt: Option<Option<T>>) -> Option<T> {
    opt.unwrap_or_default()
}

pub fn sum_map_values(m: &HashMap<String, i32>) -> i32 {
    m.iter().fold(0, |mut acc, (_, val)| {
        acc += val;
        acc
    })
}

pub fn partition_evens_odds<F, T: Copy>(pred: F, vals: Vec<T>) -> (Vec<T>, Vec<T>)
where
    F: Fn(T) -> bool,
{
    // vals.iter().partition(|&&item| pred(item))

    let lefts: Vec<T> = vec![];
    let rights: Vec<T> = vec![];

    vals.iter().fold((lefts, rights), |mut acc, &curr| {
        match pred(curr) {
            true => acc.0.push(curr),
            _ => acc.1.push(curr),
        }

        acc
    })
}

pub fn map_result<T, U, E>(r: Result<T, E>, f: impl FnOnce(T) -> U) -> Result<U, E> {
    r.map(f)
}
