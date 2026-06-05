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
    s.split(' ').filter(|el| el.starts_with("#")).collect()
}

pub fn parse_k_v(s: &str) -> HashMap<String, String> {
    let parts = s.split(',');

    let filtered: Vec<(&str, &str)> = parts
        .filter_map(|word| match word.split_once("=") {
            Some((a, b)) => Some((a, b)),
            _ => None,
        })
        .collect();

    filtered.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.insert(k.to_string(), v.to_string());
        acc
    })
}

pub fn alphanumeric_only(s: &str) -> String {
    s.chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect::<String>()
}

#[derive(Debug, PartialEq)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

fn is_yearish(y: &str) -> bool {
    y.len() == 4
}

fn is_month_dayish(m: &str, d: &str) -> bool {
    m.len() == 2 && d.len() == 2
}

pub fn parse_date(s: &str) -> Result<Date, &'static str> {
    let parts: Vec<&str> = s.split('-').collect();
    let err: &'static str = "Error parsing date";

    match parts[..] {
        [y, m, d] => match is_yearish(y) && is_month_dayish(m, d) {
            false => Err(err),
            true => {
                let maybe_y = y.parse::<u32>();
                let maybe_m = m.parse::<u32>();
                let maybe_d = d.parse::<u32>();

                match (maybe_y, maybe_m, maybe_d) {
                    (Ok(year), Ok(month), Ok(day)) => Ok(Date { year, month, day }),
                    _ => Err(err),
                }
            }
        },
        _ => Err(err),
    }
}

pub fn reverse_words(s: &str) -> String {
    let strs = s.split(' ').rev().collect::<Vec<&str>>();
    strs.join(" ")
}

pub fn normalize_whitespace(s: &str) -> String {
    s.trim()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}
