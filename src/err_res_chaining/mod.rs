#![allow(unused)]

use std::{collections::HashMap, error::Error, fmt::format, num::ParseIntError};

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

pub fn is_valid_email(s: &str) -> bool {
    let r = s.split_once("@");

    match r {
        None => false,
        Some((_, rest)) => rest.contains("."),
    }
}

pub fn first_even_double(v: &[i32]) -> Option<i32> {
    v.iter().fold(None, |acc, curr| {
        if acc.is_some() {
            return acc;
        }

        if curr % 2 == 0 { Some(curr * 2) } else { None }
    })
}

pub fn parse_all_ints(v: &[&str]) -> Result<Vec<i32>, String> {
    v.iter()
        .map(|i| i.parse::<i32>().map_err(|_| "Some issue".to_string()))
        .collect()
}

pub fn get_nested(outer: Option<Option<i32>>) -> i32 {
    outer.unwrap_or_default().unwrap_or_default()
}

pub fn double_oks(v: Vec<Result<i32, String>>) -> Vec<Result<i32, String>> {
    v.into_iter().map(|r| r.map(|i| i * 2)).collect()
}

pub fn split_results<T, E>(v: Vec<Result<T, E>>) -> (Vec<T>, Vec<E>) {
    v.into_iter().fold((vec![], vec![]), |mut acc, curr| {
        match curr {
            Err(e) => acc.1.push(e),
            Ok(v) => acc.0.push(v),
        }

        acc
    })
}

pub fn first_ok<T: Clone, E>(v: &[Result<T, E>]) -> Option<T> {
    let oks = v
        .iter()
        .map(|item| item.as_ref().ok())
        .collect::<Vec<Option<&T>>>();
    oks.iter().find(|&i| i.is_some())?.cloned()
}

pub fn validate_empty(x: &str) -> Result<(), String> {
    if x.is_empty() {
        Err("Cannot be empty".to_string())
    } else {
        Ok(())
    }
}

pub fn validate_len(x: &str) -> Result<(), String> {
    if x.len() >= 3 {
        Ok(())
    } else {
        Err("Cannot be less than 3 chars".to_string())
    }
}

pub fn validate_password(s: &str) -> Result<(), Vec<String>> {
    let rules = &[validate_empty, validate_len];

    let errs = rules
        .iter()
        .map(|f| f(s))
        .filter_map(|item| item.err())
        .collect::<Vec<String>>();

    if errs.is_empty() { Ok(()) } else { Err(errs) }
}

pub fn option_to_result<T>(o: Option<T>, err: &'static str) -> Result<T, &'static str> {
    o.ok_or(err)
}

pub fn result_to_option<T, E>(r: Result<T, E>) -> Option<T> {
    r.ok()
}

pub fn lift2<A, B, C, F>(a: Option<A>, b: Option<B>, f: F) -> Option<C>
where
    F: Fn(A, B) -> C,
{
    match (a, b) {
        (Some(x), Some(y)) => Some(f(x, y)),
        (_, _) => None,
    }
}

pub fn or_else_compute(o: Option<i32>, f: impl FnOnce() -> i32) -> i32 {
    o.unwrap_or_else(f)
}

pub fn sequence_options<T: Clone>(v: Vec<Option<T>>) -> Option<Vec<T>> {
    let o_len = v.len();
    let opts = v.into_iter().flatten().collect::<Vec<T>>();

    if opts.len() == o_len {
        Some(opts)
    } else {
        None
    }
}

pub fn parse_positive(s: &str) -> Result<u32, String> {
    s.parse::<u32>().map_err(|e| format!("Error: {e}"))
}

pub fn flatten_result<T, E>(r: Result<Result<T, E>, E>) -> Result<T, E> {
    r.flatten()
}

pub fn first_ok_or_err<T: Clone, E: Clone>(v: &[Result<T, E>]) -> Result<T, String> {
    let opt = v.iter().find(|v| v.is_ok());

    match opt {
        Some(Ok(v)) => Ok(v.clone()),
        _ => Err("bad".to_string()),
    }
}
