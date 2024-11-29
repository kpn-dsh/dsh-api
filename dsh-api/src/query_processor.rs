//! # Enums, traits and structs used by the various find methods.
use std::fmt::{Display, Formatter};

use crate::query_processor::Part::{Matching, NonMatching};
use crate::DshApiError;
use regex::Regex;

/// # Represents a part of a matched query.
#[derive(Debug, PartialEq)]
pub enum Part {
  /// Represents a part of a string that did match the query.
  Matching(String),
  /// Represents a part of a string that did not match the query.
  NonMatching(String),
}

/// # Defines the methods in the query processor
///
/// A `QueryProcessor` will query a `haystack` string for substrings that match a certain pattern.
/// If there is a match, the result will be a vector with alternating matching and
/// non-matching parts, represented by [`Part`] enums.
pub trait QueryProcessor: Send + Sync {
  /// # Returns a description of the query
  ///
  /// # Returns
  /// * a `String` describing the query processor
  fn describe(&self) -> String;

  /// # Applies query to string
  ///
  /// # Parameters
  /// * `haystack` - `String` that will be searched for parts that match the query
  ///
  /// # Returns
  /// * `Ok(Vec<Part>)` - when the `haystack` contains one or more parts that match the query
  /// * `None` - when the `haystack` did not match the query
  fn matching_parts(&self, haystack: &str) -> Option<Vec<Part>>;
}

impl Part {
  /// # Create a `Part::Matching`
  ///
  /// # Parameters
  /// `value` - the value of this `Part::Matching`
  ///
  /// # Returns
  /// The created instance.
  pub fn matching(value: impl Into<String>) -> Part {
    Matching(value.into())
  }

  /// # Create a `Part::NonMatching`
  ///
  /// # Parameters
  /// `value` - the value of this `Part::NonMatching`
  ///
  /// # Returns
  /// The created instance.
  pub fn non_matching(value: impl Into<String>) -> Part {
    NonMatching(value.into())
  }
}

impl Display for Part {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Matching(part) => write!(f, "\x1B[1m{}\x1B[0m", part),
      NonMatching(part) => write!(f, "{}", part),
    }
  }
}

/// # Generate string with ansi formatting from a `Part`
///
/// For a `NonMatching` part this method will return the literal inner `String`. For a `Matching`
/// part the returned `String` will be wrapped in an ANSI escape code for a bold type face.
///
/// # Parameters
/// `part` - The `Part` to generate the formatted string from
///
/// # Returns
/// String representation of this `Part`
///
/// # Examples
/// ```
/// use dsh_api::query_processor::{part_to_ansi_formatted_string, Part};
///
/// println!("part is {}", part_to_ansi_formatted_string(&Part::matching("MATCH")));
/// ```
/// This will print the string `"part is \x1B[1mMATCH\x1B[0m"` which,
/// on a terminal that supports ANSI escape sequences,
/// will be shown as `"part is `<code><b>MATCH</b></code>`"`.
pub fn part_to_ansi_formatted_string(part: &Part) -> String {
  match part {
    Matching(part) => format!("\x1B[1m{}\x1B[0m", part),
    NonMatching(part) => part.to_string(),
  }
}

/// # Generate string with ansi formatting from a slice of `Part`s
///
/// This method will generate a `String` representation from a `&[Part]` slice, where the
/// `Matching` parts will be wrapped in an ANSI escape code for a bold type face.
///
/// # Parameters
/// `parts` - The `Part`s to generate the formatted string from
///
/// # Returns
/// String representation of this `&[Part]` slice
/// # Examples
/// ```
/// use dsh_api::query_processor::{parts_to_ansi_formatted_string, Part};
///
/// let parts: [Part; 3] =
///   [Part::non_matching("prefix"), Part::matching("MATCH"), Part::non_matching("postfix")];
/// println!("parts are {}", parts_to_ansi_formatted_string(&parts));
/// ```
/// This will print the string `"parts are prefix\x1B[1mMATCH\x1B[0mpostfix"` which,
/// on a terminal that supports ANSI escape sequences,
/// will be shown as `"parts are prefix`<code><b>MATCH</b></code>`postfix"`.
pub fn parts_to_ansi_formatted_string(parts: &[Part]) -> String {
  parts.iter().map(part_to_ansi_formatted_string).collect::<Vec<_>>().join("")
}

/// # Query processor implementation for exact matches
///
/// # Examples
/// This example will demonstrate how to create and use a `QueryProcessor` that will performa an
/// exact match on the `haystack` string.
/// Note that the `matching_parts` method can only return `None` when no match was found,
/// or a `Some` which contains a `Vec` with exactly one `Part::Matching` element,
/// containing the entire `haystack`.
/// ```
/// use dsh_api::query_processor::{ExactMatchQueryProcessor, Part, QueryProcessor};
///
/// let exact_match_query_processor = ExactMatchQueryProcessor::create("exact").unwrap();
/// let parts = exact_match_query_processor.matching_parts("exact").unwrap();
/// assert_eq!(parts, vec![Part::matching("exact")]);
/// ```
pub struct ExactMatchQueryProcessor<'a> {
  pattern: &'a str,
}

impl<'a> ExactMatchQueryProcessor<'a> {
  pub fn create(pattern: &'a str) -> Result<Self, DshApiError> {
    Ok(Self { pattern })
  }
}

impl QueryProcessor for ExactMatchQueryProcessor<'_> {
  fn describe(&self) -> String {
    format!("match the pattern \"{}\"", self.pattern)
  }

  fn matching_parts(&self, haystack: &str) -> Option<Vec<Part>> {
    if self.pattern == haystack {
      Some(vec![Part::matching(haystack)])
    } else {
      None
    }
  }
}

/// # Query processor implementation based on regular expressions
///
/// # Examples
/// ```
/// use dsh_api::query_processor::{Part, QueryProcessor, RegexQueryProcessor};
///
/// let regex_query_processor = RegexQueryProcessor::create("a+").unwrap();
/// let parts = regex_query_processor.matching_parts("bbabbbaab").unwrap();
/// assert_eq!(parts, vec![
///   Part::non_matching("bb"),
///   Part::matching("a"),
///   Part::non_matching("bbb"),
///   Part::matching("aa"),
///   Part::non_matching("b"),
/// ]);
/// ```
pub struct RegexQueryProcessor {
  regex: Regex,
}

impl RegexQueryProcessor {
  pub fn create(pattern: &str) -> Result<Self, DshApiError> {
    match Regex::new(pattern) {
      Ok(regex) => Ok(Self { regex }),
      Err(error) => Err(DshApiError::Configuration(error.to_string())),
    }
  }
}

impl QueryProcessor for RegexQueryProcessor {
  fn describe(&self) -> String {
    format!("match against regular expression \"{}\"", self.regex.as_str())
  }

  fn matching_parts(&self, haystack: &str) -> Option<Vec<Part>> {
    let mut parts: Vec<Part> = vec![];
    let mut ptr: usize = 0;
    let mut match_found = false;
    for matching in self.regex.find_iter(haystack) {
      if matching.start() > ptr {
        parts.push(Part::non_matching(&haystack[ptr..matching.start()]))
      }
      match_found = true;
      parts.push(Part::matching(matching.as_str()));
      ptr = matching.end();
    }
    if haystack.len() > ptr {
      parts.push(Part::non_matching(&haystack[ptr..haystack.len()]));
    }
    if match_found {
      Some(parts)
    } else {
      None
    }
  }
}

#[test]
fn test_exact_match_query_processor() {
  let haystacks: [(&str, &str, Option<Vec<Part>>); 4] = [("aa", "", None), ("aa", "a", None), ("aa", "aa", Some(vec![Part::matching("aa")])), ("aa", "aaa", None)];
  for (pattern, haystack, parts) in haystacks {
    let exact_match_query_processor = ExactMatchQueryProcessor::create(pattern).unwrap();
    assert_eq!(exact_match_query_processor.describe(), format!("match the pattern \"{}\"", pattern));
    assert_eq!(exact_match_query_processor.matching_parts(haystack), parts);
  }
}

#[test]
fn test_regex_query_processor() {
  let haystacks: [(&str, &str, Option<Vec<Part>>); 19] = [
    ("a+", "", None),
    ("a+", "b", None),
    ("a+", "a", Some(vec![Part::matching("a")])),
    ("a+", "aaa", Some(vec![Part::matching("aaa")])),
    (
      "a+",
      "bbabbbaab",
      Some(vec![
        Part::non_matching("bb"),
        Part::matching("a"),
        Part::non_matching("bbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
      ]),
    ),
    (
      "a+",
      "aaabbabbbaab",
      Some(vec![
        Part::matching("aaa"),
        Part::non_matching("bb"),
        Part::matching("a"),
        Part::non_matching("bbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
      ]),
    ),
    (
      "a+",
      "bbabbbaabaaa",
      Some(vec![
        Part::non_matching("bb"),
        Part::matching("a"),
        Part::non_matching("bbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
        Part::matching("aaa"),
      ]),
    ),
    (
      "a+",
      "aaabbabbbaabaaa",
      Some(vec![
        Part::matching("aaa"),
        Part::non_matching("bb"),
        Part::matching("a"),
        Part::non_matching("bbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
        Part::matching("aaa"),
      ]),
    ),
    ("aa", "", None),
    ("aa", "bbb", None),
    ("aa", "aa", Some(vec![Part::matching("aa")])),
    ("aa", "aaa", Some(vec![Part::matching("aa"), Part::non_matching("a")])),
    ("aa", "aaaa", Some(vec![Part::matching("aa"), Part::matching("aa")])),
    ("aa", "aaaaa", Some(vec![Part::matching("aa"), Part::matching("aa"), Part::non_matching("a")])),
    ("aa", "aaabb", Some(vec![Part::matching("aa".to_string()), Part::non_matching("abb")])),
    (
      "aa",
      "bbaaabbbaaab",
      Some(vec![
        Part::non_matching("bb"),
        Part::matching("aa"),
        Part::non_matching("abbb"),
        Part::matching("aa"),
        Part::non_matching("ab"),
      ]),
    ),
    (
      "aa",
      "aaabbabbbaab",
      Some(vec![
        Part::matching("aa"),
        Part::non_matching("abbabbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
      ]),
    ),
    (
      "aa",
      "bbabbbaabaaa",
      Some(vec![
        Part::non_matching("bbabbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
        Part::matching("aa"),
        Part::non_matching("a"),
      ]),
    ),
    (
      "aa",
      "aaabbabbbaabaaa",
      Some(vec![
        Part::matching("aa"),
        Part::non_matching("abbabbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
        Part::matching("aa"),
        Part::non_matching("a"),
      ]),
    ),
  ];
  for (pattern, haystack, parts) in haystacks {
    let regex_query_processor = RegexQueryProcessor::create(pattern).unwrap();
    assert_eq!(regex_query_processor.describe(), format!("match against regular expression \"{}\"", pattern));
    assert_eq!(regex_query_processor.matching_parts(haystack), parts);
  }
}
