#![allow(unused)]

use std::{
    collections::HashMap,
    fmt::{Display, write},
};

#[derive(PartialEq, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Color {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &value.to_ascii_lowercase() {
            value if value == "red" => Ok(Color::Red),
            value if value == "green" => Ok(Color::Green),
            value if value == "blue" => Ok(Color::Blue),
            _ => Err("Error: invalid input for color"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NonEmptyString(pub &'static str);

impl TryFrom<&'static str> for NonEmptyString {
    type Error = &'static str;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Error: empty string");
        }

        Ok(Self(value))
    }
}

pub fn ints_to_floats(v: Vec<i32>) -> Vec<f64> {
    v.into_iter().map(|i| i.into()).collect()
}

pub fn shout<S: AsRef<str>>(s: S) -> String {
    format!("{}!", s.as_ref())
}

pub struct Meters(pub f64);

impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

#[derive(PartialEq, Debug)]
pub struct Scoreboard {
    pub scores: Vec<(String, i32)>,
}

impl From<HashMap<String, i32>> for Scoreboard {
    fn from(v: HashMap<String, i32>) -> Self {
        let mut scores = v.into_iter().collect::<Vec<(String, i32)>>();
        scores.sort_by_key(|b| std::cmp::Reverse(b.1));
        Self { scores }
    }
}
