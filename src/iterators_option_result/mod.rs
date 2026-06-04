#![allow(unused)]

pub fn sum_even_squares(v: &[i32]) -> i32 {
    v.iter()
        .filter_map(|&n| if n % 2 == 0 { Some(n * n) } else { None })
        .into_iter()
        .sum::<i32>()
}

pub fn first_some<T>(v: Vec<Option<T>>) -> Option<T> {
    v.into_iter().find(|e| e.is_some()).unwrap_or(None)
}

pub fn oks<T, E>(v: Vec<Result<T, E>>) -> Vec<T> {
    v.into_iter()
        .filter_map(|opt| if let Ok(o) = opt { Some(o) } else { None })
        .collect()
}

pub fn flatten<T: Clone>(v: Vec<Vec<T>>) -> Vec<T> {
    v.concat()
}

pub fn longest<'a>(strings: &'a [&str]) -> Option<&'a str> {
    let v = strings.iter().fold(
        "",
        |acc, &curr| {
            if acc.len() < curr.len() { curr } else { acc }
        },
    );

    match v {
        "" => None,
        _ => Some(v),
    }
}
