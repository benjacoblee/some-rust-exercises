#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub id: u32,
    pub parent_id: Option<u32>,
}

pub fn build_children_map(nodes: &[Node]) -> HashMap<u32, Vec<u32>> {
    nodes
        .iter()
        .fold(HashMap::new(), |mut acc, cur| match cur.parent_id {
            None => acc,
            Some(par_id) => {
                acc.entry(par_id).or_default().push(cur.id);
                acc
            }
        })
}

#[derive(Debug)]
pub struct Product {
    pub id: u32,
    pub name: &'static str,
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn dedup_by_id(products: Vec<Product>) -> Vec<Product> {
    let mut ret = products;

    ret.sort_by_key(|p| p.id);
    ret.dedup();

    ret
}

pub fn flatten_map_of_vecs(m: &HashMap<String, Vec<i32>>) -> Vec<(String, i32)> {
    m.iter()
        .flat_map(|(key, values)| values.iter().map(|&i| (key.to_string(), i)))
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
}

pub fn parse_log(lines: &[&str]) -> Vec<LogEntry> {
    lines
        .iter()
        .flat_map(|line| {
            let parts = line.split_once(" ");

            parts.map(|(hd, tl)| {
                let good_hd = hd
                    .chars()
                    .filter(|&ch| ch != '[' && ch != ']')
                    .collect::<String>();

                LogEntry {
                    level: good_hd,
                    message: tl.to_string(),
                }
            })
        })
        .collect()
}
