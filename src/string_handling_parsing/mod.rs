#![allow(unused)]

use std::num::ParseIntError;

pub fn parse_csv_ints(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s.split(',').map(|e| e.parse::<i32>()).collect()
}

pub fn join_comma(v: &[String]) -> String {
    v.join(",")
}
