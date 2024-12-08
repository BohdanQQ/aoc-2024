pub mod template;
use regex::Regex;

// Use this file to add helper functions and additional modules.

pub fn matches(pattern: &str, haystack: &str) -> bool {
    Regex::new(pattern).unwrap().is_match(haystack)
}

pub fn capture_groups<'h>(pattern: &str, haystack: &'h str) -> Option<regex::Captures<'h>> {
    Regex::new(pattern).unwrap().captures(haystack)
}

pub fn parse_field<S, T>(input: &str, mut mapper: T) -> Vec<Vec<S>>
where
    T: FnMut(char, (usize, usize)) -> S,
{
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(idx, line)| {
            line.chars()
                .enumerate()
                .map(|(ci, c)| mapper(c, (idx, ci)))
                .collect::<Vec<_>>()
        })
        .collect()
}
