#![allow(unused)]

use std::{collections::HashMap, num::ParseIntError};

pub fn parse_csv_ints(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s.split(',').map(|e| e.parse::<i32>()).collect()
}

pub fn join_comma(v: &[String]) -> String {
    v.join(",")
}

pub fn first_word(s: &str) -> &str {
    let words: Vec<&str> = s.split(' ').collect();

    match words.first() {
        Some(&s) => s,
        None => "",
    }
}

pub fn word_counts(s: &str) -> HashMap<String, usize> {
    s.split(' ').fold(HashMap::new(), |mut acc, curr| {
        let lc = curr.to_lowercase();
        acc.entry(lc).and_modify(|item| *item += 1).or_insert(1);

        acc
    })
}

pub fn extract_hash_tags(s: &str) -> Vec<&str> {
    s.split(' ')
        .filter_map(|el| match el.starts_with("#") {
            true => Some(el),
            false => None,
        })
        .collect()
}
