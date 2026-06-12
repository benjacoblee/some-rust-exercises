#![allow(unused)]

use std::collections::HashMap;

use crate::conversions::*;

#[cfg(test)]
#[test]
fn it_impls_try_from_for_str_to_color() {
    let r: Result<Color, &str> = "red".try_into();
    assert_eq!(r, Ok(Color::Red));
    let r: Result<Color, &str> = "".try_into();
    assert!(r.is_err());
}

#[test]
fn it_impls_try_from_for_str_to_ne_string() {
    let r: Result<NonEmptyString, &str> = "ne".try_into();
    assert_eq!(r, Ok(NonEmptyString("ne")));
    let r: Result<NonEmptyString, &str> = "".try_into();
    assert!(r.is_err())
}

#[test]
fn it_converts_i32_to_f64() {
    let v = vec![1, 2, 3];
    let r = ints_to_floats(v);
    assert_eq!(r, vec![1.0, 2.0, 3.0]);
}

#[test]
fn it_converts_str_to_string_and_shouts() {
    let s = "str";
    let r = shout(s);
    assert_eq!(r, "str!");
}

#[test]
fn it_impls_display_for_meters() {
    let v = Meters(1.5);
    let s = v.to_string();
    assert_eq!(s, "1.5m");
}

#[test]
fn it_converts_hashmap_to_scoreboard() {
    let mut m = HashMap::new();
    m.insert("alice".to_string(), 50);
    m.insert("bob".to_string(), 100);
    m.insert("charlie".to_string(), 75);

    let r = Scoreboard::from(m);
    let scores = r.scores.iter().map(|(k, v)| *v).collect::<Vec<i32>>();
    assert_eq!(scores, vec![100, 75, 50]);
}
