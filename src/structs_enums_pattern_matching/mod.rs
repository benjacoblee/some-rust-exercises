#![allow(unused)]

use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    ops::Add,
};

pub fn aggregate_tuples(v: Vec<(char, i32)>) -> HashMap<char, i32> {
    v.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.entry(*k).and_modify(|it| *it += v).or_insert(*v);
        acc
    })
}

pub fn top_scorer(scores: Vec<(&str, i32)>) -> Option<&str> {
    match scores[..] {
        [] => None,
        _ => {
            let n = scores
                .clone()
                .iter()
                .fold(("", i32::MIN), |acc, (k, v)| match *v > acc.1 {
                    true => (*k, *v),
                    false => acc,
                })
                .0;
            Some(n)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AgeBracket {
    Child,
    Teen,
    Adult,
}

fn get_age_bracket(age: u32) -> AgeBracket {
    match age {
        0..=12 => AgeBracket::Child,
        13..=18 => AgeBracket::Teen,
        _ => AgeBracket::Adult,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn age_bracket(people: Vec<Person>) -> HashMap<AgeBracket, Vec<Person>> {
    people.into_iter().fold(
        HashMap::new(),
        |mut acc: HashMap<AgeBracket, Vec<Person>>, p| {
            let ab = get_age_bracket(p.age);
            acc.entry(ab).or_default().push(p);
            acc
        },
    )
}

pub fn try_parse_u8(o: Option<&str>) -> Result<u8, &'static str> {
    let parts_err = "parts_err";
    let parse_err = "parse_err";
    o.ok_or(parts_err)?.parse::<u8>().map_err(|_| parse_err)
}

pub fn parse_ipv4(s: &str) -> Result<[u8; 4], &'static str> {
    let mut parts = s.split('.');
    let a1 = try_parse_u8(parts.next())?;
    let a2 = try_parse_u8(parts.next())?;
    let a3 = try_parse_u8(parts.next())?;
    let a4 = try_parse_u8(parts.next())?;

    Ok([a1, a2, a3, a4])
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn into_tuple(p: Point) -> (i32, i32) {
    let Point { x, y } = p;

    (x, y)
}

#[derive(PartialEq, Eq)]
pub enum Value<'a> {
    String(&'a str),
    Int(u32),
}

pub fn double_int(v: Value) -> Value {
    match v {
        Value::Int(u32) => Value::Int(u32 * 2),
        x => x,
    }
}

pub fn find_index<T: PartialEq>(v: &[T], item: &T) -> Option<usize> {
    v.iter().position(|elt| elt == item)
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} ({})", self.name, self.age)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NonZero(pub i32);

impl TryFrom<i32> for NonZero {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err("Cannot be less than zero")
        } else {
            Ok(NonZero(value))
        }
    }
}

pub struct Config {
    pub prefix: String,
    pub debug: bool,
    pub timeout: u32,
}

impl Config {
    pub fn default() -> Self {
        Self {
            prefix: "".to_string(),
            debug: false,
            timeout: 5000,
        }
    }

    pub fn with_prefix(&self, p: &str) -> Self {
        Self {
            prefix: p.to_string(),
            debug: self.debug,
            timeout: self.timeout,
        }
    }
}

pub struct Counter<T> {
    pub v: T,
}

impl<T: Add<Output = T> + Copy> Counter<T> {
    pub fn increment(&self, step: T) -> Self {
        Self { v: self.v + step }
    }

    pub fn default(v: T) -> Self {
        Self { v }
    }
}

pub enum Packet {
    Number(i64),
    Text(String),
    Binary(Vec<u8>),
    Null,
}

pub fn extract_number(p: Packet) -> Option<i64> {
    match p {
        Packet::Number(i) => Some(i),
        _ => None,
    }
}

pub fn texts_only(v: Vec<Packet>) -> Vec<String> {
    v.iter()
        .filter_map(|p| match p {
            Packet::Text(t) => Some(t.clone()),
            _ => None,
        })
        .collect()
}

pub fn sort_by_age(mut people: Vec<Person>) -> Vec<Person> {
    people.sort_by_key(|a| a.age);
    people
}

pub struct Score {
    pub value: i32,
    pub name: String,
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.name == other.name
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}
