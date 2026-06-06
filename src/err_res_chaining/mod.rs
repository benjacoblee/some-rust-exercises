#![allow(unused)]

use std::{collections::HashMap, error::Error, num::ParseIntError};

pub fn parse_and_double(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().map(|el| el * 2)
}

pub fn transpose_res_opt<T, E>(r: Result<Option<T>, E>) -> Option<Result<T, E>> {
    // let o = r.ok()??;
    // Some(Ok(o))

    r.transpose()
}

pub fn collect_results<T, E>(v: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    v.into_iter().collect()
}

pub fn parse(s: &str) -> Result<i32, String> {
    let r = s.parse::<i32>().map_err(|_| "parse_err".to_string())?;
    if r > 0 {
        Ok(r)
    } else {
        Err("non_positive_err".to_string())
    }
}

pub fn get_required(m: &HashMap<String, String>, key: &str) -> Result<String, String> {
    m.get(key).cloned().ok_or("".to_string())
}

pub fn combine_results<T, U, V, E, F>(r1: Result<T, E>, r2: Result<U, E>, f: F) -> Result<V, E>
where
    F: Fn(T, U) -> V,
{
    match (r1, r2) {
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
        (Ok(x), Ok(y)) => Ok(f(x, y)),
    }
}

pub fn stringify_error(r: Result<i32, ParseIntError>) -> Result<i32, String> {
    r.map_err(|_| "my_err".to_string())
}

#[derive(Debug, Eq, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn validate_name(name: &str) -> Result<bool, String> {
    if name.is_empty() {
        Err("Name cannot be empty".into())
    } else {
        Ok(true)
    }
}

pub fn validate_age(age: i32) -> Result<bool, String> {
    if age <= 0 {
        Err("Age cannot be less than or equal to zero".into())
    } else {
        Ok(true)
    }
}

pub fn validate_person(name: &str, age: i32) -> Result<Person, Vec<String>> {
    let ops = &[validate_name(name), validate_age(age)];

    let v: Vec<String> = ops
        .iter()
        .filter_map(|o| match o {
            Err(e) => Some(e.clone()),
            _ => None,
        })
        .collect();

    match v.is_empty() {
        false => Err(v),
        _ => Ok(Person {
            name: name.to_string(),
            age: age as u32,
        }),
    }
}
