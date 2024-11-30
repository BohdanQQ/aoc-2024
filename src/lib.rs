pub mod template;
use regex::Regex;

// Use this file to add helper functions and additional modules.

pub fn matches(pattern: &str, haystack: &str) -> bool {
   Regex::new(pattern).unwrap().is_match(haystack)
}

pub fn capture_groups<'h>(pattern: &str, haystack: &'h str) -> Option<regex::Captures<'h>> {
  Regex::new(pattern).unwrap().captures(haystack)
}