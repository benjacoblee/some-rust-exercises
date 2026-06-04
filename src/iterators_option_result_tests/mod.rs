#![allow(unused)]

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
