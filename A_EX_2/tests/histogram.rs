use a_ex_2::*;
use std::collections::BTreeMap;

const NO_DATA: &str = "Brak danych";

#[test]
fn build_histogram_counts_each_letter() {
    let counts = build_histogram(&['a', 'b', 'a', 'c', 'a', 'b']);
    assert_eq!(counts.get(&'a'), Some(&3));
    assert_eq!(counts.get(&'b'), Some(&2));
    assert_eq!(counts.get(&'c'), Some(&1));
    assert_eq!(counts.len(), 3);
}

#[test]
fn normalize_letters_errors_when_no_letters_present() {
    assert_eq!(normalize_letters("123 !?\n"), Err(NO_DATA.to_string()));
    assert_eq!(normalize_letters(""), Err(NO_DATA.to_string()));
}

#[test]
fn format_histogram_returns_sorted_lines() {
    let mut counts = BTreeMap::new();
    counts.insert('c', 1);
    counts.insert('a', 2);
    counts.insert('b', 3);
    assert_eq!(format_histogram(&counts), vec!["a: 2", "b: 3", "c: 1"]);
}

#[test]
fn normalize_letters_removes_non_ascii_and_lowercases() {
    assert_eq!(normalize_letters("Ab-C 1d!"), Ok(vec!['a', 'b', 'c', 'd']));
}

#[test]
fn run_from_str_trims_and_ignores_case() {
    assert_eq!(
        run_from_str("  AaB \n"),
        Ok(vec!["a: 2".to_string(), "b: 1".to_string()])
    );
}

#[test]
fn run_from_str_errors_when_letters_absent() {
    assert_eq!(run_from_str("  42 \n"), Err(NO_DATA.to_string()));
}

#[test]
fn run_from_str_handles_mixed_input() {
    let expected: Vec<String> = ["d: 1", "e: 1", "h: 1", "l: 3", "o: 2", "r: 1", "w: 1"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(run_from_str("Hello, World 123!"), Ok(expected));
}

#[test]
fn normalize_letters_skips_non_ascii_letters() {
    assert_eq!(normalize_letters("zażółć"), Ok(vec!['z', 'a']));
    assert_eq!(normalize_letters("ąęł"), Err(NO_DATA.to_string()));
}
