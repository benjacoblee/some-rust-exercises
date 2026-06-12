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

#[test]
fn it_chains_vecs() {
    let a = &["a", "b"];
    let b = &["c", "d"];
    let r = concat_vecs(a, b);
    assert_eq!(r, &["a", "b", "c", "d"]);
}

#[test]
fn it_joins_a_list_of_i32() {
    let v = &[1, 2, 3, 4, 5];
    let r = join_with(v, "!");
    assert_eq!(r, "1!2!3!4!5");
}

#[test]
fn it_takes_every_nth() {
    let v = &[1, 2, 3, 4, 5];
    let r = every_nth(v, 2);
    assert_eq!(r, &[1, 3, 5]);
}

#[test]
fn it_takes_every_even_indexed_item() {
    let v = &["a", "b", "c", "d"];
    let r = even_indexed_items(v);
    assert_eq!(r, &["a", "c"]);
}

#[test]
fn it_finds_min_max() {
    let v = &[1, 2, 3, 100, -500, 20_000];
    let r = min_max(v);
    assert_eq!(r, Some((-500, 20_000)))
}

#[test]
fn it_expands_a_vec_of_strs() {
    let v = &["hi", "bye"];
    assert_eq!(expand_words(v), &['h', 'i', 'b', 'y', 'e']);
}

#[test]
fn it_gets_variance() {
    let r = total_variation(&[1, 10, 2, 9]);
    assert_eq!(r, 24);
    assert_eq!(total_variation(&[5, 5, 5, 5]), 0);
}

#[test]
fn it_applies_twice() {
    let r = apply_twice(|x: i32| x + x, 1);
    assert_eq!(r, 4);
}

#[test]
fn it_composes() {
    let add2 = |x: i32| x + 2;
    let to_str = |x: i32| x.to_string();
    let r = compose(add2, to_str);
    assert_eq!(r(2), "4");
}

#[test]
fn it_maps_if() {
    let a = map_if(2, 2 == 2, |x| x + 2);
    assert_eq!(a, 4);
    let b = map_if(2, 2 != 2, |x| x + 2);
    assert_eq!(b, 2);
}

#[test]
fn it_reduces() {
    let v = &[1, 2, 3, 4, 5];
    let result: Option<i32> = my_reduce(v, &|acc, x| acc + x);
    assert_eq!(result, Some(15));
}

#[test]
fn it_curries_add() {
    let add5 = curried_add(5);
    assert_eq!(add5(5), 10);
}

#[test]
fn it_allows_sorting_by_key() {
    let mut v = vec![
        Student { grade: 20 },
        Student { grade: 14 },
        Student { grade: 55 },
    ];
    let r = sort_by_grade(v, |x| x.grade);
    assert!(r.is_sorted());
}

#[test]
fn it_applies_repeatedly() {
    let r = apply_until(1, |x| x + 1, |x| x == 10);
    assert_eq!(r, 10);
}

#[test]
fn it_does_three_way_partition() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (low, mid, high) = three_way_partition(v, |&x| x < 5, |&x| x > 5);
    assert_eq!(low, vec![1, 2, 3, 4]);
    assert_eq!(mid, vec![5]);
    assert_eq!(high, vec![6, 7, 8, 9])
}

#[test]
fn it_maps_tuple() {
    let t = (1, 1);
    let (a, _) = map_pair(t, |x| x.to_string());
    assert_eq!(a, "1");
}
