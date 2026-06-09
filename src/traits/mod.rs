#![allow(unused)]

use std::fmt::format;
use std::hash::Hash;
use std::{collections::HashSet, num::ParseIntError, ops::Add};

pub trait Appendable {
    type Item;
    fn append_(&mut self, item: Self::Item);
}

impl<T> Appendable for Vec<T> {
    type Item = T;
    fn append_(&mut self, item: Self::Item) {
        Vec::push(self, item);
    }
}

impl Appendable for String {
    type Item = char;
    fn append_(&mut self, ch: Self::Item) {
        self.push(ch);
    }
}

#[derive(PartialEq, Eq)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub timeout_secs: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "localhost".to_string(),
            timeout_secs: 30,
        }
    }
}

#[derive(PartialEq)]
pub struct UserId(pub u64);

impl From<u64> for UserId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<i32> for UserId {
    fn from(value: i32) -> Self {
        Self(value as u64)
    }
}

impl TryFrom<&str> for UserId {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(value.parse::<u64>()?))
    }
}

pub fn get_user(id: impl Into<UserId>) -> UserId {
    id.into()
}

pub fn get_user_maybe<T, E>(id: T) -> Result<UserId, E>
where
    T: TryInto<UserId, Error = E>,
{
    id.try_into()
}

#[derive(PartialEq)]
pub struct Meters(pub f64);

impl Add for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

pub struct UniqueVec<T> {
    pub items: Vec<T>,
}

impl<T: PartialEq> Extend<T> for UniqueVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let n = iter
            .into_iter()
            .filter(|item| !self.items.contains(item))
            .collect::<Vec<T>>();

        self.items.extend(n)
    }
}

impl<T: Eq + Hash> FromIterator<T> for UniqueVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret: Vec<T> = vec![];

        let mut items = iter.into_iter().fold(ret, |mut acc, cur| {
            if !acc.contains(&cur) {
                acc.push(cur);
            }

            acc
        });

        Self { items }
    }
}

pub trait Describable {
    fn describe(&self) -> String;
    fn short_description(&self) -> String {
        self.describe().chars().take(30).collect()
    }
}

impl Describable for i32 {
    fn describe(&self) -> String {
        format!("i32: {self}")
    }

    fn short_description(&self) -> String {
        format!("{self}")
    }
}

impl Describable for String {
    fn describe(&self) -> String {
        format!("String: {self}")
    }

    fn short_description(&self) -> String {
        self.to_string()
    }
}

impl<T: Describable + Clone> Describable for Vec<T> {
    fn describe(&self) -> String {
        let s = self.short_description();
        format!("Vec: {s}")
    }

    fn short_description(&self) -> String {
        let descriptions = self
            .iter()
            .map(|item| item.describe())
            .collect::<Vec<String>>();
        descriptions.join(", ")
    }
}
