#![allow(unused)]

use std::collections::HashMap;

use crate::string_handling_parsing::*;

#[cfg(test)]
#[test]
fn it_parses_csv_strings_to_int() {
    let r = parse_csv_ints("1,2,3,4,5");
    assert_eq!(r, Ok(vec![1, 2, 3, 4, 5]));
}

#[test]
fn it_joins_commas() {
    let r = join_comma(&["abc".to_string(), "def".to_string()]);
    assert_eq!(r, "abc,def".to_string());
}

#[test]
fn it_gets_first_word() {
    assert_eq!(first_word("mary had"), "mary");
    assert_eq!(first_word(""), "");
}

#[test]
fn it_counts_words() {
    let my_map: HashMap<String, usize> = vec![("wow".to_string(), 3), ("banana".to_string(), 1)]
        .into_iter()
        .collect();
    let result = word_counts("wow WOW Wow banana");
    assert_eq!(my_map, result);
}

#[test]
fn it_extracts_hashtags() {
    let ms = "this that #wow #banana something";
    let result = extract_hash_tags(ms);
    assert_eq!(result, vec!["#wow", "#banana"]);
}

#[test]
fn it_parses_key_values() {
    let my_map: HashMap<String, String> = vec![
        ("wow".to_string(), "banana".to_string()),
        ("other".to_string(), "thing".to_string()),
    ]
    .into_iter()
    .collect();
    let ms = "wow=banana,other=thing";
    assert_eq!(parse_k_v(ms), my_map);
}

#[test]
fn it_gets_alphanumeric_only() {
    let res = alphanumeric_only("abc! def;");
    assert_eq!(res, "abcdef");
}

#[test]
fn it_parses_date_correctly() {
    let r = parse_date("2022-02-02");
    assert_eq!(
        r,
        Ok(Date {
            year: 2022,
            month: 2,
            day: 2
        })
    );

    assert!(parse_date("rubbish").is_err());
}

#[test]
fn it_reverses_words() {
    let r = reverse_words("one fine day");
    assert_eq!(r, "day fine one");
}

#[test]
fn it_trims_and_collapses_whitespace() {
    let r = normalize_whitespace("    a        person    ");
    assert_eq!(r, "a person")
}

#[test]
fn it_counts_vowels_correctly() {
    let r = count_vowels("bbbaeiou");
    assert!(r == 5);
}

#[test]
fn it_checks_palindromicity_correctly() {
    assert!(is_palindrome("s"));
    assert!(is_palindrome(""));
    assert!(is_palindrome("racecar"));
    assert!(!is_palindrome("bubba"))
}

#[test]
fn it_titlecases_strings() {
    assert!(title_case("wow dude") == "Wow Dude");
}

#[test]
fn it_truncates_strings() {
    assert!(truncate("somestring", 200) == "somestring");
    assert!(truncate("somestring", 5) == "somes")
}

#[test]
fn it_capitalizes_first_word() {
    let s = "she sells sea shells";
    assert!(capitalize_first(s) == "She sells sea shells")
}

#[test]
fn it_counts_newlines() {
    let s = "some thing
    another thing
    ";
    assert!(count_lines(s) == 3)
}

#[test]
fn it_gets_the_longest_word() {
    let s = "something other areallylongword or whatever.";
    assert_eq!(longest_word(s), Some("areallylongword"));
}

#[test]
fn it_replaces_words() {
    let s = "that is really something isnt it. something for sure";
    let expected = "that is really nothing isnt it. nothing for sure";
    let res = replace_all(s, "something", "nothing");
    assert_eq!(res, expected);
}

#[test]
fn it_adds_indent() {
    let s = "wow.\nreally amazing";
    let res = indent(s, 4);
    let mut cur = res.split('\n').map(|s| s.to_string()).peekable();
    let w = cur.peek();
    assert_eq!(Some("    wow."), w.map(|el| el.as_str()));
    cur.next();
    let w = cur.peek();
    assert_eq!(Some("    really amazing"), w.map(|el| el.as_str()));
}

#[test]
fn it_titlecases() {
    let s = "HELLO WORLD";
    assert_eq!(tc(s), "Hello world");
    assert_eq!(tc("something"), "Something");
    assert_eq!(tc("SOMETHING"), "Something");
    assert_eq!(tc(""), "");
}

#[test]
fn it_truncates_with_ellipsis() {
    let n = 4;
    let s = "hello world";
    let r = truncate_with_ellips(s, n);
    assert_eq!(r, "hell...".to_string());
}

#[test]
fn it_pads_left() {
    let w = 6;
    let s = "cow";
    let r = pad_l_r(s, w, '#', PadDir::Left);
    assert_eq!(r, "###cow");
    let r = pad_l_r(s, w, '#', PadDir::Right);
    assert_eq!(r, "cow###");
    let r = pad_l_r(s, 1, '#', PadDir::Left);
    assert_eq!(r, "cow");
}

#[test]
fn it_strips_prefix_if() {
    let prefix = "wha";
    assert_eq!(strip_prefix_if("sth", prefix), "sth");
    assert_eq!(strip_prefix_if("what", prefix), "t");
}

#[test]
fn it_repeats_with_sep() {
    let sep = "!";
    let s = "wow";
    let r = repeat_with_sep(s, 3, sep);
    assert_eq!(r, "wow!wow!wow!");
}

#[test]
fn it_gets_file_ext() {
    assert_eq!(file_extension("path"), None);
    assert_eq!(file_extension("mod.rs"), Some("rs"));
}

#[test]
fn it_replaces_nth_ch() {
    let s = "wow";
    let r = replace_nth_char(s, 1, '0');
    assert_eq!(r, "w0w");
}
