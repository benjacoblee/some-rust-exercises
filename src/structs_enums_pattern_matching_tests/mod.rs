#![allow(unused)]

use crate::structs_enums_pattern_matching::*;
use std::collections::HashMap;

#[cfg(test)]
#[test]
fn it_aggregates_a_vec_of_tuples() {
    let data: Vec<(char, i32)> = vec![('a', 42), ('b', 56), ('c', 3), ('c', 400)];
    let expected = vec![('a', 42), ('b', 56), ('c', 403)].into_iter().collect();
    assert_eq!(aggregate_tuples(data), expected);
}

#[test]
fn it_gets_highest_scorer() {
    let v = vec![("john", 24), ("sam", 46), ("alex", 3)];
    let res = top_scorer(v);
    assert_eq!(res, Some("sam"));
}

#[test]
fn it_categorizes_by_age_bracket() {
    let larry = Person {
        name: "Larry".to_string(),
        age: 2,
    };

    let bob = Person {
        name: "Bob".to_string(),
        age: 44,
    };

    let cass = Person {
        name: "Cass".to_string(),
        age: 1,
    };

    let people = vec![larry.clone(), bob.clone(), cass.clone()];
    let res = age_bracket(people).get(&AgeBracket::Child).cloned();
    assert_eq!(res, Some(vec![larry, cass]));
}
