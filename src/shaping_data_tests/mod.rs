#![allow(unused)]

use std::{collections::HashMap, vec};

use crate::shaping_data::*;

#[cfg(test)]
#[test]
fn it_maps_node_relationship_to_hashmap() {
    let nodes = vec![
        Node {
            id: 1,
            parent_id: None,
        },
        Node {
            id: 2,
            parent_id: Some(1),
        },
        Node {
            id: 3,
            parent_id: Some(1),
        },
        Node {
            id: 4,
            parent_id: Some(2),
        },
    ];
    let r = build_children_map(&nodes);
    assert_eq!(r[&1], vec![2, 3]);
    assert_eq!(r[&2], vec![4]);
    assert!(!r.contains_key(&3));
    assert!(!r.contains_key(&4));
}

#[test]
fn it_dedups_duplicate_products() {
    let v = vec![
        Product {
            id: 1,
            name: "washing machine",
        },
        Product {
            id: 1,
            name: "sth else",
        },
        Product {
            id: 2,
            name: "dryer",
        },
    ];

    let r = dedup_by_id(v);

    println!("{:?}", r);

    assert!(r.len() == 2);
}

#[test]
fn it_flattens_map_of_vecs() {
    let mut m = HashMap::new();
    m.insert("a".to_string(), vec![1, 2]);
    m.insert("b".to_string(), vec![3, 4]);
    let mut r = flatten_map_of_vecs(&m);
    r.sort_by_key(|v| v.1);
    assert_eq!(
        r,
        vec![
            ("a".to_string(), 1),
            ("a".to_string(), 2),
            ("b".to_string(), 3),
            ("b".to_string(), 4)
        ]
    )
}

#[test]
fn it_parses_logs() {
    let lines = &["[INFO] some message", "[ERROR] some error"];
    let r = parse_log(lines);
    let mut iter = r.iter();
    let cur = iter.next();
    assert_eq!(
        cur,
        Some(&LogEntry {
            level: "INFO".to_string(),
            message: "some message".to_string()
        })
    );
    let cur = iter.next();
    assert_eq!(
        cur,
        Some(&LogEntry {
            level: "ERROR".to_string(),
            message: "some error".to_string()
        })
    );
}
