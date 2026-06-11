#![allow(unused)]

use std::{collections::HashMap, ops::Index};

pub fn prefer_snd(a: HashMap<String, i32>, b: HashMap<String, i32>) -> HashMap<String, i32> {
    b.into_iter().fold(a, |mut acc, (key, val)| {
        *acc.entry(key).or_default() = val;
        acc
    })
}

pub fn find_repeated_values(m: &HashMap<String, i32>) -> Vec<i32> {
    m.values()
        .fold(HashMap::new(), |mut acc, &curr| {
            *acc.entry(curr).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter_map(|(&k, &v)| if v > 1 { Some(k) } else { None })
        .collect()
}

pub fn invert_map(m: HashMap<String, String>) -> HashMap<String, String> {
    m.into_iter().map(|(k, v)| (v, k)).collect()
}

pub fn filter_map_values(m: HashMap<String, i32>, min_val: i32) -> HashMap<String, i32> {
    m.into_iter()
        .filter_map(|(k, v)| if v < min_val { None } else { Some((k, v)) })
        .collect()
}

pub fn add_maps(a: &HashMap<String, i32>, b: &HashMap<String, i32>) -> HashMap<String, i32> {
    a.iter().fold(b.clone(), |mut acc, (k, &v)| {
        *acc.entry(k.to_string()).or_default() += v;
        acc
    })
}

pub fn group_by_first_char(words: Vec<String>) -> HashMap<char, Vec<String>> {
    words.into_iter().fold(HashMap::new(), |mut acc, curr| {
        if curr.is_empty() {
            return acc;
        }

        let (hd, tl) = curr.split_at(1);
        let ch = hd.chars().next().unwrap();
        acc.entry(ch).or_default().push(curr);

        acc
    })
}
