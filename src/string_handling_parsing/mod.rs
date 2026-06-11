#![feature(iter_intersperse)]
#![allow(unused)]

use std::{collections::HashMap, fmt::format, num::ParseIntError};

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

fn is_v(v: char) -> bool {
    "aeiou".contains(v)
}

pub fn count_vowels(s: &str) -> usize {
    s.to_lowercase()
        .chars()
        .fold(0, |acc, ch| if is_v(ch) { acc + 1 } else { acc })
}

pub fn is_palindrome(s: &str) -> bool {
    let mine: String = s.chars().rev().collect();
    mine == s
}

pub fn title_case(s: &str) -> String {
    s.split(' ')
        .map(|w| {
            w.chars()
                .enumerate()
                .map(|(pos, ch)| {
                    if pos == 0 {
                        ch.to_ascii_uppercase()
                    } else {
                        ch
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn truncate(s: &str, n: usize) -> &str {
    if n > s.len() { s } else { &s[0..n] }
}

pub fn capitalize_helper(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, ch)| if i == 0 { ch.to_ascii_uppercase() } else { ch })
        .collect()
}

pub fn capitalize_first(s: &str) -> String {
    match s.split_once(' ') {
        None => capitalize_helper(s),
        Some((hd, tl)) => {
            let new_head = capitalize_helper(hd);
            format!("{new_head} {tl}")
        }
    }
}

pub fn count_lines(s: &str) -> usize {
    s.split('\n').count()
}

pub fn longest_word(s: &str) -> Option<&str> {
    match s.split_once(" ") {
        None => None,
        Some((hd, tl)) => {
            let v = tl.split(" ").fold(
                hd,
                |acc, curr| {
                    if curr.len() > acc.len() { curr } else { acc }
                },
            );

            Some(v)
        }
    }
}

pub fn replace_all(s: &str, from: &str, to: &str) -> String {
    s.split(" ")
        .map(|w| if w == from { to } else { w })
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn indent(s: &str, spaces: usize) -> String {
    s.split('\n')
        .map(|el| format!("{}{}", " ".repeat(spaces), el))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn tc(s: &str) -> String {
    if s.is_empty() {
        return "".into();
    }

    let (chd, ctl) = s.split_at(1);
    format!("{}{}", chd.to_ascii_uppercase(), ctl.to_ascii_lowercase())
}

pub fn new_title_case(s: &str) -> String {
    s.split(" ").map(tc).collect::<Vec<String>>().join(" ")
}

pub fn truncate_with_ellips(s: &str, max_len: usize) -> String {
    if max_len > s.len() {
        return s.to_string();
    }

    let sl = &s[0..max_len];
    format!("{sl}...")
}

#[derive(PartialEq)]
pub enum PadDir {
    Left,
    Right,
}

pub fn pad_l_r(s: &str, total_width: usize, pad_char: char, dir: PadDir) -> String {
    if s.len() >= total_width {
        return s.to_string();
    }

    let n_times = total_width - s.len();
    let pad_str = pad_char.to_string().repeat(n_times);

    if dir == PadDir::Left {
        format!("{pad_str}{s}")
    } else {
        format!("{s}{pad_str}")
    }
}

pub fn strip_prefix_if<'a>(s: &'a str, prefix: &str) -> &'a str {
    if !s.starts_with(prefix) {
        return s;
    }

    let idx = prefix.len();

    &s[idx..]
}

pub fn repeat_with_sep(s: &str, n: usize, sep: &str) -> String {
    let with_sep = format!("{}{}", s, sep);
    with_sep.repeat(3)
}

pub fn file_extension(path: &str) -> Option<&str> {
    match path.split_once('.') {
        None => None,
        Some((_, tl)) => Some(tl),
    }
}

pub fn replace_nth_char(s: &str, n: usize, replacement: char) -> String {
    s.chars()
        .enumerate()
        .map(|(m, ch)| if (m == n) { replacement } else { ch })
        .collect()
}
