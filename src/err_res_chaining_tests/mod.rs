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
