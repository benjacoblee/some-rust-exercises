#![allow(unused)]

use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;

// pub fn invert_map<K, V>(m: &mut HashMap<K, V>) -> HashMap<V, K>
// where
//     K: Eq + Hash,
//     V: Eq + Hash,
// {
//     m.drain().map(|(k, v)| (v, k)).collect()
// }

pub fn invert_map<K, V>(mut m: HashMap<K, V>) -> HashMap<V, K>
where
    K: Copy + Eq + Hash,
    V: Copy + Eq + Hash,
{
    m.drain().map(|(k, v)| (v, k)).collect()
}

pub fn find_duplicates<V: Hash + Eq + Copy>(v: &[V]) -> Vec<V> {
    let mut m: HashMap<V, u32> = HashMap::new();

    let counts = v.iter().fold(m, |mut acc, &cur| {
        *acc.entry(cur).or_insert(0) += 1;
        acc
    });

    counts
        .iter()
        .filter_map(|(&k, &v)| if v > 1 { Some(k) } else { None })
        .collect()
}

struct Acc<T> {
    prev: T,
    ret: Option<T>,
}

pub fn missing_number(v: &[u32]) -> Option<u32> {
    if v.is_empty() {
        return None;
    }

    if v.len() == 1 {
        return None;
    }

    let hd = *v.first()?;

    let ret = Acc {
        prev: hd,
        ret: None,
    };

    v[1..]
        .iter()
        .fold(ret, |acc, &cur| {
            if cur - 1 != acc.prev {
                Acc {
                    ret: Some(cur - 1),
                    ..acc
                }
            } else {
                Acc { prev: cur, ..acc }
            }
        })
        .ret
}

pub fn rotate_left<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    if n > v.len() {
        return v.to_vec();
    }

    let mut ret: Vec<T> = vec![];
    let right = &v[n..];
    let left = &v[0..n];

    for item in right.iter() {
        ret.push(item.to_owned())
    }

    for item in left.iter() {
        ret.push(item.to_owned())
    }

    ret
}

pub fn product(v: &[i32]) -> i32 {
    v.iter().fold(1, |mut acc, cur| {
        acc *= cur;
        acc
    })
}

pub fn all_some<T>(v: &[Option<T>]) -> bool {
    v.iter().all(|el| el.is_some())
}

pub fn count_nones<T>(v: &[Option<T>]) -> usize {
    v.iter()
        .fold(0, |acc, curr| if curr.is_none() { acc + 1 } else { acc })
}

pub fn last<T: Clone>(v: &[T]) -> Option<T> {
    match v.is_empty() {
        true => None,
        _ => v.last().to_owned().cloned(),
    }
}

pub fn zip_vecs<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter().cloned().zip(b.iter().cloned()).collect()
}

pub fn repeat(s: &str, n: usize) -> String {
    let mut ret = "".to_string();

    for _ in 0..n {
        ret.push_str(s);
    }

    ret
}

pub fn min_max(v: &[i32]) -> Option<(i32, i32)> {
    if v.is_empty() {
        return None;
    }

    if v.len() == 1 {
        let &hd = v.first()?;
        return Some((hd, hd));
    }

    let r = v.iter().fold((i32::MAX, i32::MIN), |acc, &cur| {
        let new_min = if cur < acc.0 { cur } else { acc.0 };
        let new_max = if cur > acc.1 { cur } else { acc.1 };
        (new_min, new_max)
    });

    Some(r)
}

pub fn all_eq<T: PartialEq + std::default::Default>(v: &[T]) -> bool {
    match v.split_first() {
        None => true,
        Some((hd, tl)) => tl.iter().all(|it| it == hd),
    }
}

pub fn clamp_all(v: &[i32], lo: i32, hi: i32) -> Vec<i32> {
    v.iter()
        .map(|&i| {
            if i < lo {
                return lo;
            }

            if i > hi { hi } else { i }
        })
        .collect()
}

pub fn count_pairs(s: &str, open: char, close: char) -> usize {
    let (l, r) = s.chars().fold((0, 0), |acc, ch| match ch {
        v if v == open => (acc.0 + 1, acc.1),
        v if v == close => (acc.0, acc.1 + 1),
        _ => acc,
    });

    let (larger, smaller) = if l >= r { (l, r) } else { (r, l) };
    smaller as usize
}

pub fn csv_to_table(rows: &[&str]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|&r| r.split(",").map(|v| v.to_string()).collect::<Vec<String>>())
        .collect()
}

fn try_parse_u64(o: Option<&str>) -> Result<u64, String> {
    let a_err = "No more values in iterator";
    let b_err = "Error parsing int";

    let v = o
        .ok_or(a_err.to_string())?
        .parse::<u64>()
        .map_err(|_| b_err.to_string())?;

    Ok(v)
}

pub fn hms_to_s(s: &str) -> Result<u64, String> {
    let mut vals = s.split(":");

    let h = try_parse_u64(vals.next())?;
    let m = try_parse_u64(vals.next())?;
    let s = try_parse_u64(vals.next())?;

    let hh = h * 60 * 60;
    let mm = m * 60;

    Ok(hh + mm + s)
}

pub fn abbr_helper(s: &str) -> String {
    let (f, _) = s.split_at(1);
    format!("{f}.")
}

pub fn abbreviate(full_name: &str) -> String {
    let parts = full_name.split(" ").collect::<Vec<&str>>();

    match parts.split_last() {
        None => "".to_string(),
        Some((last, others)) => {
            let abbreviated = others
                .iter()
                .map(|s| abbr_helper(s))
                .collect::<Vec<String>>()
                .join(" ");
            format!("{abbreviated} {last}")
        }
    }
}

pub fn rollup(entries: &[(&str, f64)]) -> HashMap<String, f64> {
    entries.iter().fold(HashMap::new(), |mut acc, (cat, amt)| {
        *acc.entry(cat.to_string()).or_default() += amt;
        acc
    })
}

pub fn converge<T: Clone + PartialEq>(val: T, f: impl Fn(T) -> T) -> T {
    let next = f(val.clone());
    if next == val { next } else { converge(next, f) }
}

pub struct Stack<T> {
    v: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    pub fn push(&mut self, v: T) {
        self.v.push(v);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.v.last()
    }

    pub fn empty(self) -> bool {
        self.v.is_empty()
    }
}

pub struct Queue<T> {
    s1: Stack<T>,
    s2: Stack<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            s1: Stack::new(),
            s2: Stack::new(),
        }
    }

    pub fn enqueue(&mut self, v: T) {
        while let Some(o) = self.s1.pop() {
            self.s2.push(o);
        }

        self.s1.push(v);

        while let Some(o) = self.s2.pop() {
            self.s1.push(o);
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.s1.pop()
    }

    pub fn front(&self) -> Option<&T> {
        self.s1.peek()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Nested<T: Clone + PartialEq> {
    Val(T),
    Values(Vec<Nested<T>>),
}

pub fn flatten_nested<T: Clone + PartialEq>(n: Nested<T>, depth: usize) -> Vec<Nested<T>> {
    if depth == 0 {
        return match n {
            Nested::Val(v) => vec![Nested::Val(v)],
            Nested::Values(v) => v,
        };
    }

    match n {
        Nested::Val(v) => vec![Nested::Val(v)],
        Nested::Values(v) => match v.split_first() {
            None => vec![],
            Some((hd, tl)) => {
                let mut ret = flatten_nested(hd.clone(), depth - 1);

                let items: Vec<Nested<T>> = tl
                    .iter()
                    .flat_map(|item| flatten_nested(item.clone(), depth - 1))
                    .collect();

                ret.extend(items);

                ret
            }
        },
    }
}

pub fn map_in_place(v: &mut [i32], f: impl Fn(i32) -> i32) {
    v.iter_mut().for_each(|v| {
        *v = f(*v);
    });
}

pub fn find_all_idx<T>(v: &[T], pred: impl Fn(&T) -> bool) -> Vec<usize> {
    v.iter()
        .enumerate()
        .filter_map(|(pos, elm)| if pred(elm) { Some(pos) } else { None })
        .collect()
}

pub fn map_average(m: &HashMap<String, f64>) -> Option<f64> {
    if m.is_empty() {
        None
    } else {
        let sum = m.iter().fold(0.0, |acc, (_, v)| acc + v);
        Some(sum / (m.len() as f64))
    }
}

pub struct Pipeline<T> {
    v: T,
}

impl<T> Pipeline<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }

    pub fn pipe<F>(self, f: F) -> Self
    where
        F: Fn(&T) -> T,
    {
        Self { v: f(&self.v) }
    }

    pub fn tap<F>(self, f: F) -> Self
    where
        F: Fn(&T),
    {
        f(&self.v);
        self
    }

    pub fn finish(self) -> T {
        self.v
    }
}

pub struct Row {
    target_len: usize,
    initial_num: usize,
    start: usize,
}

impl Row {
    pub fn new(initial_num: usize, target_len: usize) -> Self {
        Self {
            target_len,
            initial_num,
            start: 0,
        }
    }

    pub fn make(&self) -> Vec<usize> {
        // fn go(s: &Row, inner_prev: usize, accum: &mut Vec<usize>) -> Vec<usize> {
        //     if accum.len() == s.target_len {
        //         return accum.to_vec();
        //     }

        //     let new_prev = inner_prev + s.initial_num;

        //     accum.push(new_prev);

        //     go(s, new_prev, accum)
        // }

        // go(self, self.start, &mut vec![])

        // let mut ret: Vec<usize> = Vec::with_capacity(self.target_len);

        // for i in 0..self.target_len {
        //     let it = (i + 1) * self.initial_num;
        //     ret.push(it);
        // }

        // ret

        (1..=self.target_len)
            .map(|i| i * self.initial_num)
            .collect()
    }
}

pub fn mult_table(n: usize) -> Vec<Vec<usize>> {
    let initial = Row::new(1, n).make();

    initial.iter().fold(vec![], |mut acc, curr| {
        let row = Row::new(*curr, n).make();
        acc.push(row);
        acc
    })
}
