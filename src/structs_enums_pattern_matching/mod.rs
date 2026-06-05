#![allow(unused)]

use std::collections::HashMap;

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
