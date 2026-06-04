#![allow(unused)]

use std::collections::HashMap;

use crate::string_handling_parsing::*;

#[cfg(test)]
#[test]
fn it_parses_csv_strings_to_int() {
    let r = parse_csv_ints("1,2,3,4,5");
    assert_eq!(r, Ok(vec![1, 2, 3, 4, 5]));
}

#[test]
fn it_joins_commas() {
    let r = join_comma(&vec!["abc".to_string(), "def".to_string()]);
    assert_eq!(r, "abc,def".to_string());
}

#[test]
fn it_gets_first_word() {
    assert_eq!(first_word("mary had"), "mary");
    assert_eq!(first_word(""), "");
}

#[test]
fn it_counts_words() {
    let my_map: HashMap<String, usize> = vec![("wow".to_string(), 3), ("banana".to_string(), 1)]
        .into_iter()
        .collect();
    let result = word_counts("wow WOW Wow banana");
    assert_eq!(my_map, result);
}

#[test]
fn it_extracts_hashtags() {
    let ms = "this that #wow #banana something";
    let result = extract_hash_tags(&ms);
    assert_eq!(result, vec!["#wow", "#banana"]);
}
