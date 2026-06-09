#![allow(unused)]

use std::collections::HashMap;

use crate::err_res_chaining::*;

#[cfg(test)]
#[test]
pub fn it_parses_str_to_int_opt() {
    let s = "24";
    let r = parse_and_double(s);
    assert!(r == Some(48));
}

#[test]
pub fn it_transposes_result() {
    let v: Result<Option<i32>, ()> = Ok(Some(3));
    let r = transpose_res_opt(v);
    assert!(r == Some(Ok(3)));
}

#[test]
pub fn it_collects_results() {
    let v: Vec<Result<i32, String>> = vec![Ok(2), Ok(3), Err("Fail".to_string())];
    assert_eq!(collect_results(v), Err("Fail".to_string()));
    let v: Vec<Result<i32, String>> = vec![Ok(2), Ok(3)];
    assert_eq!(collect_results(v), Ok(vec![2, 3]));
}

#[test]
pub fn it_turns_a_get_option_to_res() {
    let m = HashMap::new();
    let r = get_required(&m, "some key");
    assert_eq!(r, Err("".to_string()));
}

#[test]
pub fn it_combines_results() {
    let r1: Result<i32, String> = Ok(3);
    let r2: Result<i32, String> = Ok(4);
    let res = combine_results(r1, r2, |a, b| a + b);
    assert!(res == Ok(7))
}

#[test]
pub fn it_maps_err() {
    let res: Result<i32, String> = stringify_error("".parse());
    assert_eq!(res, Err("my_err".to_string()))
}

#[test]
pub fn it_validates_email_poorly() {
    let valid = "sth@.";
    let invalid = "";
    let invalid1 = "@asd";
    assert!(is_valid_email(valid));
    assert!(!is_valid_email(invalid));
    assert!(!is_valid_email(invalid1));
}

#[test]
pub fn it_doubles_the_first_even_num() {
    let v = &[1, 1, 1, 1, 20, 1, 1];
    let r = first_even_double(v);
    assert_eq!(r, Some(40));
}

#[test]
pub fn it_parses_ints_all_or_nothing() {
    let v = &["1", "2", "3"];
    let r = parse_all_ints(v);
    assert_eq!(r, Ok(vec![1, 2, 3]));
    let v = &["1", "2", ""];
    let r = parse_all_ints(v);
    assert_eq!(r, Err("Some issue".to_string()))
}

#[test]
pub fn it_unwraps_number_option_properly() {
    let outer = Some(Some(5));
    assert_eq!(get_nested(outer), 5);
    let outer = None;
    assert_eq!(get_nested(outer), 0);
    let outer = Some(None);
    assert_eq!(get_nested(outer), 0)
}

#[test]
pub fn it_doubles_oks() {
    let v = vec![Ok(1), Ok(2), Err("Some err".to_string())];
    let expected = vec![Ok(2), Ok(4), Err("Some err".to_string())];
    let r = double_oks(v);
    assert_eq!(r, expected);
}

#[test]
pub fn it_splits_ok_errs() {
    let v = vec![Ok(1), Ok(2), Err("some")];
    let expected = (vec![1, 2], vec!["some"]);
    let r = split_results(v);
    assert_eq!(r, expected);
}

#[test]
pub fn it_converts_first_ok_to_some() {
    let v = &[Err(2), Err(3), Err(4), Ok(41), Ok(2)];
    let e = Some(41);
    let r = first_ok(v);
    assert_eq!(e, r);
}

#[test]
pub fn it_collects_errors_if_any() {
    let s = "okpassword";
    let r = validate_password(s);
    assert_eq!(r, Ok(()));
    let s = "";
    let r = validate_password(s);
    assert!(r.is_err_and(|inner| inner.len() == 2))
}
