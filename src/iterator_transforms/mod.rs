#![allow(unused)]

pub fn scan_running_max(v: &[i32]) -> Vec<i32> {
    v.iter()
        .scan(i32::MIN, |state, v| {
            if v > state {
                *state = *v;
                Some(*state)
            } else {
                Some(*state)
            }
        })
        .collect()
}

pub fn take_last<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    if n >= v.len() {
        v.to_vec()
    } else {
        let i = v.len() - n;
        v[i..].to_vec()
    }
}

pub fn step_by_sum(v: &[i32], step: usize) -> i32 {
    v.iter().enumerate().fold(
        0,
        |acc, (i, cur)| {
            if (i + 1) % step == 0 { acc + cur } else { acc }
        },
    )
}

pub fn step_by_sum2(v: &[i32], step: usize) -> i32 {
    v.iter().skip(step - 1).step_by(step).sum()
}
