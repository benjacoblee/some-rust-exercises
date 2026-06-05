#![allow(unused)]

use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
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
            acc.entry(ab).or_insert_with(Vec::new).push(p);
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
    let mut parts = s.split('.').into_iter();
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
