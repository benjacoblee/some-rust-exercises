#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

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
