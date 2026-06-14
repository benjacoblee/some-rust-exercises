#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, format},
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Index, Mul},
    str::FromStr,
    vec,
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

pub struct Pair<A, B> {
    pub a: A,
    pub b: B,
}

pub fn swap<A, B>(p: Pair<A, B>) -> Pair<B, A> {
    Pair { a: p.b, b: p.a }
}

pub struct Counter {
    pub current: u32,
    pub max: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.current;

        if cur <= self.max {
            self.current += 1;
            return Some(cur);
        }

        None
    }
}

#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    pub v: Vec<T>,
}

impl<T> DerefMut for Stack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

impl<T> Deref for Stack<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> Stack<T> {
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Stack<U> {
        let v = self.v.into_iter().map(f).collect::<Vec<U>>();
        Stack { v }
    }
}

pub enum Season {
    Winter,
    Spring,
    Autumn,
    Summer,
}

impl Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let season = match self {
            Season::Autumn => "Autumn",
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Winter => "Winter",
        };

        write!(f, "Season({})", season)
    }
}

pub enum MyMaybe<T> {
    Nothing,
    Just(T),
}

impl<T> MyMaybe<T> {
    pub fn map<F, U>(self, f: F) -> MyMaybe<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            MyMaybe::Nothing => MyMaybe::Nothing,
            MyMaybe::Just(v) => MyMaybe::Just(f(v)),
        }
    }

    pub fn unwrap_or(self, t: T) -> T {
        match self {
            MyMaybe::Nothing => t,
            MyMaybe::Just(v) => v,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Mrapper<T: Hash + Eq, U> {
    vals: HashMap<T, U>,
}

impl<T: Hash + Eq, U> Index<T> for Mrapper<T, U> {
    type Output = U;

    fn index(&self, idx: T) -> &Self::Output {
        &self.vals[&idx]
    }
}

impl<T: Hash + Eq, U> Mrapper<T, U> {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
        }
    }

    pub fn add(mut self, k: T, v: U) -> Self {
        self.vals.insert(k, v);
        self
    }
}

pub struct SortedV(pub Vec<i32>);

impl SortedV {
    fn new() -> Self {
        Self(vec![])
    }

    fn add(&mut self, v: i32) {
        self.0.push(v)
    }
}

impl FromIterator<i32> for SortedV {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut c = SortedV::new();

        for i in iter {
            c.add(i);
        }

        c.0.sort();

        c
    }
}

pub fn pipe<T, U>(value: T, f: impl FnOnce(T) -> U) -> U {
    f(value)
}
