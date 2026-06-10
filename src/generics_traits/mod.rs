#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    hash::Hash,
    ops::{Add, AddAssign, Deref, Mul},
    str::FromStr,
};

pub fn max_el<T: Ord>(v: &[T]) -> Option<&T> {
    if v.is_empty() {
        return None;
    }

    let head = v.first()?;

    let max = v
        .iter()
        .fold(head, |acc, curr| if curr > head { curr } else { acc });

    Some(max)
}

pub fn sum_nums<T: Add + Default + Copy + AddAssign>(v: &[T]) -> T {
    let mut d = T::default();

    v.iter().fold(d, |mut acc, cur| {
        acc += *cur;
        acc
    })
}

pub fn to_set<T: Eq + Hash>(v: Vec<T>) -> HashSet<T> {
    v.into_iter().collect()
}

pub struct M<K, V> {
    pub hm: HashMap<K, V>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> From<Vec<(K, V)>> for M<K, V> {
    fn from(value: Vec<(K, V)>) -> Self {
        let m: HashMap<K, V> = HashMap::new();

        let hm = value.into_iter().fold(m, |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        });

        Self { hm }
    }
}

pub fn collect_to_vec<I: IntoIterator>(iter: I) -> Vec<I::Item> {
    iter.into_iter().collect()
}

pub trait Doubler<T: Mul> {
    fn double(x: T) -> T;
}

impl Doubler<i32> for i32 {
    fn double(x: i32) -> i32 {
        x * 2
    }
}

pub fn parse_or_default<T: FromStr>(s: &str, default: T) -> T {
    let r = s.parse::<T>();

    match r {
        Ok(v) => v,
        _ => default,
    }
}

pub fn merge_maps<K: Copy + std::cmp::Eq + std::hash::Hash, V: Copy + Add + AddAssign + Default>(
    a: &HashMap<K, V>,
    b: &HashMap<K, V>,
) -> HashMap<K, V> {
    let mut new: HashMap<K, V> = HashMap::new();

    let init = a.iter().fold(new, |mut acc, (&k, &v)| {
        acc.insert(k, v);
        acc
    });

    b.iter().fold(init, |mut acc, (&k, &v)| {
        *acc.entry(k).or_insert(Default::default()) += v;
        acc
    })
}

pub struct EvenRange {
    current: u32,
    end: u32,
}

impl EvenRange {
    pub fn new(current: u32, end: u32) -> Self {
        if current % 2 == 1 || end % 2 == 1 {
            panic!("current and end must be even")
        }

        if end < current {
            panic!("end cannot be less than current")
        }

        Self { current, end }
    }
}

impl Iterator for EvenRange {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current;
        self.current += 2;
        if ret <= self.end { Some(ret) } else { None }
    }
}

pub fn my_for_each<T>(v: &[T], mut f: impl Fn(&T)) {
    v.iter().for_each(f);
}

pub trait Named {
    fn name(&self) -> &str;
}

pub struct Dog {
    pub name: &'static str,
}

impl Named for Dog {
    fn name(&self) -> &str {
        self.name
    }
}

pub struct Human {
    pub name: &'static str,
}

impl Named for Human {
    fn name(&self) -> &str {
        self.name
    }
}

pub fn get_first_name(v: &[Box<dyn Named>]) -> Option<&str> {
    v.first().map(|b| b.name())
}

pub fn safe_zip_with<A, B, C, F>(a: Option<A>, b: Option<B>, f: F) -> Option<C>
where
    F: Fn(A, B) -> C,
{
    let inner_a = a?;
    let inner_b = b?;
    Some(f(inner_a, inner_b))
}

impl From<Dog> for String {
    fn from(value: Dog) -> Self {
        value.name.to_string()
    }
}

pub fn make_string<T: Into<String>>(val: T) -> String {
    val.into()
}

pub struct MyVec<T>(pub Vec<T>);

impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
