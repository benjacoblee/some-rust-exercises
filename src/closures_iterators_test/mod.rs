#![allow(unused)]

use crate::closures_iterators::*;

#[cfg(test)]
#[test]
fn it_applies_transforms_to_int() {
    let v = 1;
    let double = |x: i32| x * 2;
    let plus_100 = |x: i32| x + 100;
    let transforms = &[double, plus_100];
    let res = apply_all(v, transforms);
    assert!(res == 102);
}

#[test]
fn it_computes_sq_table() {
    let n = 5;
    let sq_tbl = &[(1, 1), (2, 4), (3, 9), (4, 16), (5, 25)];
    let r = squares_table(n);

    sq_tbl.iter().for_each(|(i, sq_i)| {
        assert!(r.contains_key(i));
        assert!(r.get(i) == Some(sq_i));
    });
}

#[test]
fn it_flattens_words_vec() {
    let w = &["hello world", "foo bar baz"];
    let expected = &["hello", "world", "foo", "bar", "baz"];
    assert_eq!(all_words(w), expected);
}

#[test]
fn it_gets_a_slice_of_positive_numbers() {
    let s = &[1, 2, 3, 4, 5, 0];
    let r = take_while_positive(s);
    assert_eq!(r, &[1, 2, 3, 4, 5])
}

#[test]
fn it_gets_a_slice_without_leading_zeroes() {
    let v = &[0, 0, 0, 1, 2, 3];
    let r = skip_leading_zeros(v);
    assert_eq!(r, &[1, 2, 3]);
}

#[test]
fn it_tracks_running_product() {
    let v = &[1, 2, 3, 4];
    let expected = &[1, 2, 6, 24];
    let r = running_product(v);
    assert_eq!(r, expected);
}

#[test]
fn it_unzips_a_vector_of_tuples() {
    let v = vec![(1, 'a'), (2, 'b')];
    let expected = (vec![1, 2], vec!['a', 'b']);
    let r = unzip(v);
    assert_eq!(r, expected)
}

#[test]
fn it_finds_min_idx() {
    let v = &[1, 2, -100_000, 4];
    let expected = 2;
    let r = min_index(v);
    assert!(r == Some(expected));
}
