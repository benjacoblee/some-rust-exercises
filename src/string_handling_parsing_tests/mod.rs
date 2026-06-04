#![allow(unused)]

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
