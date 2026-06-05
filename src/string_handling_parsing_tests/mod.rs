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
    let r = join_comma(&["abc".to_string(), "def".to_string()]);
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
    let result = extract_hash_tags(ms);
    assert_eq!(result, vec!["#wow", "#banana"]);
}

#[test]
fn it_parses_key_values() {
    let my_map: HashMap<String, String> = vec![
        ("wow".to_string(), "banana".to_string()),
        ("other".to_string(), "thing".to_string()),
    ]
    .into_iter()
    .collect();
    let ms = "wow=banana,other=thing";
    assert_eq!(parse_k_v(ms), my_map);
}

#[test]
fn it_gets_alphanumeric_only() {
    let res = alphanumeric_only("abc! def;");
    assert_eq!(res, "abcdef");
}

#[test]
fn it_parses_date_correctly() {
    let r = parse_date("2022-02-02");
    assert_eq!(
        r,
        Ok(Date {
            year: 2022,
            month: 2,
            day: 2
        })
    );

    assert!(parse_date("rubbish").is_err());
}

#[test]
fn it_reverses_words() {
    let r = reverse_words("one fine day");
    assert_eq!(r, "day fine one");
}

#[test]
fn it_trims_and_collapses_whitespace() {
    let r = normalize_whitespace("    a        person    ");
    assert_eq!(r, "a person")
}
