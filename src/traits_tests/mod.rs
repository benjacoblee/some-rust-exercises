#![allow(unused)]

use crate::traits::Appendable;
use crate::traits::*;

#[cfg(test)]
#[test]

fn it_impls_append_for_vec() {
    let mut v = vec![1, 2, 3];
    v.append_(4);
    assert!(v == vec![1, 2, 3, 4])
}

#[test]
fn it_impls_append_for_string() {
    let mut s = "Hello".to_string();
    s.append_('!');
    assert!(s == "Hello!")
}

#[test]
fn it_impls_default_for_config() {
    let c = Config::default();

    let c2 = Config {
        port: 8080,
        host: "localhost".to_string(),
        timeout_secs: 30,
    };

    assert!(c == c2);
}

#[test]
fn it_allows_various_types_to_convert_into_userid() {
    let i: i32 = 32;
    let u: u64 = 64;
    let ok_str = "4";
    let bad_str = "bad";

    assert!(get_user(i) == UserId(32));
    assert!(get_user(u) == UserId(64));
    assert!(get_user_maybe(ok_str).is_ok());
    assert!(get_user_maybe(bad_str).is_err());
}

#[test]
fn it_allows_meter_structs_to_be_added() {
    let m1 = Meters(1.0);
    let m2 = Meters(2.0);
    assert!(m1 + m2 == Meters(3.0));
}

#[test]
fn it_impls_unique_extend() {
    let mut u = UniqueVec {
        items: vec![1, 2, 3],
    };

    u.extend(vec![1, 4, 5]);

    assert_eq!(u.items, vec![1, 2, 3, 4, 5])
}

#[test]
fn it_impls_from_iterator_for_unique_vec() {
    let uv: UniqueVec<u32> = [1, 2, 2, 3].into_iter().collect();
    println!("{:?}", uv.items);
    assert!(uv.items == [1, 2, 3])
}

#[test]
fn it_impls_describe_for_various_types() {
    let i: i32 = 32;
    println!("{}", i.describe());
    println!("{}", i.short_description());
    let s = "some".to_string();
    println!("{}", s.describe());
    println!("{}", s.short_description());
    let v = vec!["this".to_string(), "that".to_string()];
    println!("{}", v.describe());
    println!("{}", v.short_description());
}
