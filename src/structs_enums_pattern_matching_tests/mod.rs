#![allow(unused)]

use crate::structs_enums_pattern_matching::*;
use std::{collections::HashMap, ops::Deref};

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

#[test]
fn it_sorta_parses_ip_addresses() {
    let res = parse_ipv4("192.168.0.1");
    let expected: [u8; 4] = [192, 168, 0, 1];
    assert_eq!(res, Ok(expected));
    let res = parse_ipv4("192.168.9");
    assert_eq!(res, Err("parts_err"));
    let res = parse_ipv4("192.168.9.asd");
    assert_eq!(res, Err("parse_err"));
}

#[test]
fn it_deconstructs_a_point() {
    let p = into_tuple(Point { x: 3, y: 3 });
    let t = (3, 3);
    assert_eq!(p, t);
}

#[test]
fn it_doubles_int_value() {
    let res = double_int(Value::Int(3));
    assert!(res == Value::Int(6));
}

#[test]
fn it_finds_item_and_returns_index() {
    let res = find_index(&[1, 2, 3, 4, 5], &2);
    assert!(res == Some(1));
}

#[test]
fn it_implements_display_for_person() {
    let person = Person {
        name: "Dan".into(),
        age: 32,
    };

    println!("{}", person)
}

#[test]
fn it_implements_try_from_for_non_zero() {
    let res: Result<NonZero, &str> = 32.try_into();
    assert_eq!(res, Ok(NonZero(32)));
}

#[test]
fn it_impls_config_with_builder_pattern() {
    let cfg = Config::default().with_prefix("some prefix");
    assert_eq!(cfg.prefix, "some prefix");
}

#[test]
fn it_increments_counter() {
    let counter = Counter::default(0).increment(1).increment(1).increment(1);
    assert_eq!(counter.v, 3);
}

#[test]
fn it_extracts_n_from_a_packet() {
    let p = Packet::Number(4);
    assert_eq!(extract_number(p), Some(4));
}

#[test]
fn it_extracts_text_packets_only() {
    let v = vec![
        Packet::Number(2),
        Packet::Null,
        Packet::Text("something".to_string()),
    ];
    let r = texts_only(v);
    assert!(r == vec!["something".to_string()])
}

#[test]
fn it_sorts_people_by_age() {
    let people = vec![
        Person {
            name: "alice".to_string(),
            age: 54,
        },
        Person {
            name: "bob".to_string(),
            age: 99,
        },
        Person {
            name: "john".to_string(),
            age: 1,
        },
    ];

    let sorted = sort_by_age(people);
    assert_eq!(sorted.first().unwrap().name, "john".to_string())
}

#[test]
fn it_impls_partial_ord_for_score() {
    let s1 = Score {
        name: "".to_string(),
        value: 44,
    };

    let s2 = Score {
        name: "".to_string(),
        value: 67,
    };

    assert!(s2 > s1);
}
