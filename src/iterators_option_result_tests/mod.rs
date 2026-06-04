#![allow(unused)]

use std::{collections::HashMap, num::ParseIntError, vec};

use crate::iterators_option_result::*;

#[cfg(test)]
#[test]
fn it_sums_and_squares_even_numbers() {
    let result = sum_even_squares(&vec![1, 2, 3, 4, 5]);
    assert_eq!(result, 20);
}

#[test]
fn it_gets_first_some() {
    let result = first_some(vec![None, None, None, Some(4), Some(3)]);
    assert_eq!(result, Some(4));
}

#[test]
fn it_gets_oks() {
    let result = oks(vec![Err(0), Err(0), Ok(3), Ok(4), Ok(5)]);
    assert_eq!(result, vec![3, 4, 5]);
}

#[test]
fn it_flattens() {
    let result = flatten(vec![vec![1, 2], vec![3, 4]]);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn it_gets_longest() {
    let binding = vec!["something", "other"];
    let result = longest(&binding);
    assert_eq!(result, Some("something"));
    let binding_1: Vec<&str> = vec![];
    assert_eq!(longest(&binding_1), None);
}

#[test]
fn it_returns_an_empty_vec_on_error() {
    assert_eq!(unwrap_or_empty(Err("".to_string())), vec![]);
}

#[test]
fn it_flattens_opt() {
    assert_eq!(flatten_option(Some(Some(2))), Some(2));
}

#[test]
fn it_sums_values() {
    //    pub fn sum_map_values(m: &HashMap<String, i32>) -> i32 {
    let vals = vec![("a".to_string(), 4), ("b".to_string(), 5)];
    let m: HashMap<String, i32> = vals.into_iter().collect();
    println!("{:?}", &m);
    assert_eq!(sum_map_values(&m), 9);
}

#[test]
fn it_partitions() {
    let partitioned = partition_evens_odds(|x| x % 2 == 0, vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(partitioned, (vec![2, 4, 6], vec![1, 3, 5]));
}

#[test]
fn it_maps_result() {
    let result = map_result(Ok::<i32, String>(2), |x| x * 100);
    assert_eq!(result, Ok(200));
}
