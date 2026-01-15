//! # Enums, traits and structs used by the various find methods
use crate::parse::{parse_function1, parse_function2};
use crate::query_processor::Part::{Matching, NonMatching};
use crate::DshApiError;
use itertools::Itertools;
use regex::Regex;
use std::fmt::{Display, Formatter};

/// # Represents a part of a matched query.
#[derive(Debug, PartialEq)]
pub enum Part {
  /// Represents a part of a string that did match the query.
  Matching(String),
  /// Represents a part of a string that did not match the query.
  NonMatching(String),
}

#[derive(Debug, PartialEq)]
pub enum Match {
  /// Matching expression
  /// - Kind of expression,
  /// - First parameter,
  /// - Optional second parameter.
  Expression(String, String, Option<String>),
  /// Matching and non-matching parts
  Parts(Vec<Part>),
  /// Simple match
  Simple,
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

  /// # Applies generic query to string
  ///
  /// # Parameters
  /// * `haystack` - `String` that will be matched against the expression query.
  ///
  /// # Returns
  /// * `Ok(Match::Expression)` - When the `haystack` matches the expression query.
  /// * `Ok(Match::Parts)` - When the `haystack` contains one or more parts that match the query.
  /// * `Ok(Match::Simple)` - When the `haystack` matches the query.
  /// * `None` - When the `haystack` did not match the query.
  fn matching(&self, haystack: &str) -> Option<Match>;

  /// # Applies expression query to string
  ///
  /// # Parameters
  /// * `haystack` - `String` that will be matched against the expression query.
  ///
  /// # Returns
  /// * `Ok((String, Option<String>))` - When the `haystack` matches the expression query.
  /// * `None` - When the `haystack` did not match the query.
  fn matching_expression(&self, haystack: &str) -> Option<(String, String, Option<String>)>;

  /// # Applies parts query to string
  ///
  /// # Parameters
  /// * `haystack` - `String` that will be matched against the parts query.
  ///
  /// # Returns
  /// * `Ok(Vec<Part>)` - When the `haystack` contains one or more parts that match the query.
  /// * `Ok(Simple)` - When the `haystack` matches the query.
  /// * `None` - When the `haystack` did not match the query.
  fn matching_parts(&self, haystack: &str) -> Option<Vec<Part>>;

  /// # Applies simple query to string
  ///
  /// # Parameters
  /// * `haystack` - `String` that will be matched against the simple query.
  ///
  /// # Returns
  /// * `true` - When the `haystack` matches the query.
  /// * `false` - When the `haystack` did not match the query.
  fn matching_simple(&self, haystack: &str) -> bool;
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
/// # use dsh_api::query_processor::{ExactMatchQueryProcessor, Match, Part, QueryProcessor};
/// let exact_match_query_processor = ExactMatchQueryProcessor::new("exact");
/// let matching = exact_match_query_processor.matching("exact").unwrap();
/// assert_eq!(matching, Match::simple());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ExactMatchQueryProcessor {
  exact_match: String,
}

impl ExactMatchQueryProcessor {
  pub fn create<T: Into<String>>(exact_match: T) -> Result<Self, DshApiError> {
    Ok(Self { exact_match: exact_match.into() })
  }

  pub fn new<T: Into<String>>(exact_match: T) -> Self {
    Self { exact_match: exact_match.into() }
  }
}

impl QueryProcessor for ExactMatchQueryProcessor {
  fn describe(&self) -> String {
    format!("match the string \"{}\"", self.exact_match)
  }

  fn matching(&self, haystack: &str) -> Option<Match> {
    if self.exact_match == haystack {
      Some(Match::Simple)
    } else {
      None
    }
  }

  fn matching_expression(&self, _haystack: &str) -> Option<(String, String, Option<String>)> {
    None
  }

  fn matching_parts(&self, haystack: &str) -> Option<Vec<Part>> {
    if self.exact_match == haystack {
      Some(vec![Part::matching(haystack.to_string())])
    } else {
      None
    }
  }

  fn matching_simple(&self, haystack: &str) -> bool {
    self.exact_match == haystack
  }
}

/// # Query processor implementation for string matches
///
/// # Parameters
/// * `string` - The pattern that will be matched against.
/// * `match_substring` - Whether matches on substring (`true`) or on the full string (`false`).
/// * `ignore_case` - Whether matching is case-sensitive.
///
/// # Examples
/// This example will demonstrate how to create and use a `StringQueryProcessor` that will
/// perform a case-insensitive substring match on the `haystack` string.
/// ```
/// # use dsh_api::query_processor::{Match, Part, QueryProcessor, StringQueryProcessor};
/// let string_query_processor = StringQueryProcessor::new("SUB", true, true);
/// let parts = string_query_processor.matching("contains substring").unwrap();
/// assert_eq!(
///   parts,
///   Match::parts(vec![
///     Part::non_matching("contains "),
///     Part::matching("sub"),
///     Part::non_matching("string")
///   ])
/// );
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct StringQueryProcessor {
  string: String,
  match_substring: bool,
  ignore_case: bool,
}

impl StringQueryProcessor {
  pub fn create<T: Into<String>>(string: T, match_substring: bool, ignore_case: bool) -> Result<Self, DshApiError> {
    Ok(Self { string: string.into(), match_substring, ignore_case })
  }

  pub fn new<T: Into<String>>(string: T, match_substring: bool, ignore_case: bool) -> Self {
    Self { string: string.into(), match_substring, ignore_case }
  }
}

impl QueryProcessor for StringQueryProcessor {
  fn describe(&self) -> String {
    if self.match_substring {
      if self.ignore_case {
        format!("match substring \"{}\", case-insensitive", self.string,)
      } else {
        format!("match substring \"{}\", case-sensitive", self.string,)
      }
    } else if self.ignore_case {
      format!("match full string \"{}\", case-insensitive", self.string,)
    } else {
      format!("match full string \"{}\", case-sensitive", self.string,)
    }
  }

  fn matching(&self, haystack: &str) -> Option<Match> {
    self.matching_parts(haystack).map(Match::parts)
  }

  fn matching_expression(&self, _haystack: &str) -> Option<(String, String, Option<String>)> {
    None
  }

  fn matching_parts(&self, haystack: &str) -> Option<Vec<Part>> {
    fn find_ignore_case(ignore_case: bool, haystack: &str, pattern: &str) -> Option<usize> {
      if ignore_case {
        haystack.to_lowercase().find(pattern.to_lowercase().as_str())
      } else {
        haystack.find(pattern)
      }
    }

    fn strip_prefix_ignore_case<'a>(ignore_case: bool, haystack: &'a str, prefix: &str) -> Option<(&'a str, &'a str)> {
      if ignore_case {
        if haystack.to_lowercase().starts_with(prefix.to_lowercase().as_str()) {
          Some((&haystack[0..prefix.len()], &haystack[prefix.len()..]))
        } else {
          None
        }
      } else if let Some(stripped_prefix) = haystack.strip_prefix(prefix) {
        Some((&haystack[0..prefix.len()], stripped_prefix))
      } else {
        None
      }
    }

    if self.match_substring {
      let mut parts: Vec<Part> = vec![];
      let mut leftover = haystack;
      let mut match_found = false;
      while !leftover.is_empty() {
        match strip_prefix_ignore_case(self.ignore_case, leftover, &self.string) {
          Some((prefix, rest)) => {
            match_found = true;
            parts.push(Part::matching(prefix.to_string()));
            leftover = rest;
          }
          None => match find_ignore_case(self.ignore_case, leftover, &self.string) {
            Some(index) => {
              parts.push(Part::non_matching(leftover[0..index].to_string()));
              leftover = &leftover[index..];
            }
            None => {
              parts.push(Part::non_matching(leftover.to_string()));
              leftover = "";
            }
          },
        }
      }
      if match_found {
        Some(parts)
      } else {
        None
      }
    } else if self.ignore_case {
      if self.string.eq_ignore_ascii_case(haystack) {
        Some(vec![Matching(haystack.to_string())])
      } else {
        None
      }
    } else if self.string == haystack {
      Some(vec![Matching(haystack.to_string())])
    } else {
      None
    }
  }

  fn matching_simple(&self, haystack: &str) -> bool {
    self.matching_parts(haystack).is_some()
  }
}

/// # Query processor implementation for substring matches
///
/// # Examples
/// This example will demonstrate how to create and use a `QueryProcessor` that will perform a
/// substring match on the `haystack` string.
/// ```
/// # use dsh_api::query_processor::{Match, Part, QueryProcessor, SubstringQueryProcessor};
/// let substring_query_processor = SubstringQueryProcessor::new("sub");
/// let parts = substring_query_processor.matching("contains substring").unwrap();
/// assert_eq!(
///   parts,
///   Match::parts(vec![
///     Part::non_matching("contains "),
///     Part::matching("sub"),
///     Part::non_matching("string")
///   ])
/// );
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SubstringQueryProcessor {
  substring: String,
}

impl SubstringQueryProcessor {
  pub fn create<T: Into<String>>(substring: T) -> Result<Self, DshApiError> {
    Ok(Self { substring: substring.into() })
  }

  pub fn new<T: Into<String>>(substring: T) -> Self {
    Self { substring: substring.into() }
  }
}

impl QueryProcessor for SubstringQueryProcessor {
  fn describe(&self) -> String {
    format!("match substring \"{}\"", self.substring)
  }

  fn matching(&self, haystack: &str) -> Option<Match> {
    self.matching_parts(haystack).map(Match::parts)
  }

  fn matching_expression(&self, _haystack: &str) -> Option<(String, String, Option<String>)> {
    None
  }

  fn matching_parts(&self, haystack: &str) -> Option<Vec<Part>> {
    let mut parts: Vec<Part> = vec![];
    let mut leftover = haystack;
    let mut match_found = false;
    while !leftover.is_empty() {
      match leftover.strip_prefix(&self.substring) {
        Some(stripped) => {
          match_found = true;
          parts.push(Part::matching(self.substring.to_string()));
          leftover = stripped;
        }
        None => match leftover.find(&self.substring) {
          Some(index) => {
            parts.push(Part::non_matching(leftover[0..index].to_string()));
            leftover = &leftover[index..];
          }
          None => {
            parts.push(Part::non_matching(leftover.to_string()));
            leftover = "";
          }
        },
      }
    }
    if match_found {
      Some(parts)
    } else {
      None
    }
  }

  fn matching_simple(&self, haystack: &str) -> bool {
    self.matching_parts(haystack).is_some()
  }
}

/// # Query processor implementation based on regular expressions
///
/// # Examples
/// ```
/// # use dsh_api::query_processor::{Match, Part, QueryProcessor, RegexQueryProcessor};
/// let regex_query_processor = RegexQueryProcessor::create(r#"a+"#).unwrap();
/// let parts = regex_query_processor.matching("bbabbbaab").unwrap();
/// assert_eq!(
///   parts,
///   Match::parts(vec![
///     Part::non_matching("bb"),
///     Part::matching("a"),
///     Part::non_matching("bbb"),
///     Part::matching("aa"),
///     Part::non_matching("b"),
///   ])
/// );
/// ```
#[derive(Clone, Debug)]
pub struct RegexQueryProcessor {
  regex: Regex,
}

impl RegexQueryProcessor {
  pub fn new<T: Into<Regex>>(regex: Regex) -> Self {
    Self { regex }
  }

  pub fn create<T: TryInto<Regex>>(pattern: T) -> Result<Self, DshApiError> {
    match pattern.try_into() {
      Ok(regex) => Ok(Self { regex }),
      Err(_) => Err(DshApiError::Configuration("illegal regular expression".to_string())),
    }
  }
}

impl QueryProcessor for RegexQueryProcessor {
  fn describe(&self) -> String {
    format!("match against regular expression \"{}\"", self.regex.as_str())
  }

  fn matching(&self, haystack: &str) -> Option<Match> {
    self.matching_parts(haystack).map(Match::parts)
  }

  fn matching_expression(&self, _haystack: &str) -> Option<(String, String, Option<String>)> {
    None
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

  fn matching_simple(&self, haystack: &str) -> bool {
    self.matching_parts(haystack).is_some()
  }
}

/// # Query processor for DSH expression
///
/// # Examples
/// ```
/// # use dsh_api::query_processor::{Match, Part, QueryProcessor, ExpressionQueryProcessor};
/// let expression_query_processor = ExpressionQueryProcessor::new("vhost");
/// let matching = expression_query_processor.matching("{ vhost('par') }").unwrap();
/// assert_eq!(matching, Match::expression("vhost", "par", None::<String>));
/// let matching = expression_query_processor.matching("{ vhost('par1', 'par2') }").unwrap();
/// assert_eq!(matching, Match::expression("vhost", "par1", Some("par2")));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionQueryProcessor {
  kind: String,
}

impl ExpressionQueryProcessor {
  pub fn new<T: Into<String>>(kind: T) -> Self {
    Self { kind: kind.into() }
  }

  pub fn create<T: Into<String>>(kind: T) -> Result<Self, DshApiError> {
    Ok(Self { kind: kind.into() })
  }
}

impl QueryProcessor for ExpressionQueryProcessor {
  fn describe(&self) -> String {
    format!("match against dsh expression {}()", self.kind)
  }

  fn matching(&self, haystack: &str) -> Option<Match> {
    self
      .matching_expression(haystack)
      .map(|(name, first, second)| Match::expression(name, first, second))
  }

  fn matching_expression(&self, haystack: &str) -> Option<(String, String, Option<String>)> {
    match parse_function2(haystack, &self.kind) {
      Ok((parameter1, parameter2)) => Some((self.kind.to_string(), parameter1.to_string(), Some(parameter2.to_string()))),
      Err(_) => match parse_function1(haystack, &self.kind) {
        Ok(parameter) => Some((self.kind.to_string(), parameter.to_string(), None)),
        Err(_) => None,
      },
    }
  }

  fn matching_parts(&self, haystack: &str) -> Option<Vec<Part>> {
    self.matching_expression(haystack).map(|(name, first, second)| match second {
      Some(second_parameter) => vec![Part::matching(name), Part::matching(first), Part::matching(second_parameter)],
      None => vec![Part::matching(name), Part::matching(first)],
    })
  }

  fn matching_simple(&self, haystack: &str) -> bool {
    self.matching_expression(haystack).is_some()
  }
}

/// # Dummy query processor implementation
///
/// This dummy query processor always returns the literal `haystack` as a
/// single non-matching `Part`.
/// This can be useful when you want to apply a function that expects a query processor,
/// without actually applying the query.
#[derive(Clone, Debug, PartialEq)]
pub struct DummyQueryProcessor {}

impl DummyQueryProcessor {
  pub fn create() -> Result<Self, DshApiError> {
    Ok(Self {})
  }
}

impl QueryProcessor for DummyQueryProcessor {
  fn describe(&self) -> String {
    "accept all input".to_string()
  }

  fn matching(&self, haystack: &str) -> Option<Match> {
    self.matching_parts(haystack).map(Match::parts)
  }

  fn matching_expression(&self, _haystack: &str) -> Option<(String, String, Option<String>)> {
    None
  }

  fn matching_parts(&self, haystack: &str) -> Option<Vec<Part>> {
    Some(vec![Part::non_matching(haystack)])
  }

  fn matching_simple(&self, _haystack: &str) -> bool {
    true
  }
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
      Matching(part) => write!(f, "{}", part),
      NonMatching(part) => write!(f, "{}", part),
    }
  }
}

impl Match {
  pub fn expression<T, U, V>(kind: T, first_parameter: U, second_parameter: Option<V>) -> Self
  where
    T: Into<String>,
    U: Into<String>,
    V: Into<String>,
  {
    Match::Expression(kind.into(), first_parameter.into(), second_parameter.map(|sp| sp.into()))
  }

  pub fn parts(parts: Vec<Part>) -> Self {
    Match::Parts(parts)
  }

  pub fn simple() -> Self {
    Match::Simple
  }
}

impl Display for Match {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Match::Expression(kind, first_parameter, second_parameter) => match second_parameter {
        Some(second) => write!(f, "{{ {}('{}', '{}') }}", kind, first_parameter, second),
        None => write!(f, "{{ {}('{}') }}", kind, first_parameter),
      },
      Match::Parts(parts) => write!(f, "{}", parts.iter().join("")),
      Match::Simple => write!(f, "match"),
    }
  }
}
