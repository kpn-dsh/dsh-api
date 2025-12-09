use dsh_api::query_processor::{
  ExactMatchQueryProcessor, ExpressionQueryProcessor, Match, Part, QueryProcessor, RegexQueryProcessor, StringQueryProcessor, SubstringQueryProcessor,
};

#[test]
fn test_exact_match_query_processor() {
  let haystacks: [(&str, &str, Option<Match>); 4] = [("aa", "", None), ("aa", "a", None), ("aa", "aa", Some(Match::simple())), ("aa", "aaa", None)];
  for (pattern, haystack, matching) in haystacks {
    let exact_match_query_processor = ExactMatchQueryProcessor::create(pattern).unwrap();
    assert_eq!(exact_match_query_processor.describe(), format!("match the string \"{}\"", pattern));
    assert_eq!(exact_match_query_processor.matching(haystack), matching);
  }
}

#[test]
fn test_string_query_processor() {
  let patterns_haystacks: [(&str, &str, Option<Match>); 14] = [
    ("A", "", None),
    ("A", "b", None),
    ("A", "a", Some(Match::parts(vec![Part::matching("a")]))),
    ("A", "aa", Some(Match::parts(vec![Part::matching("a"), Part::matching("a")]))),
    ("AA", "aa", Some(Match::parts(vec![Part::matching("aa")]))),
    ("AA", "aaa", Some(Match::parts(vec![Part::matching("aa"), Part::non_matching("a")]))),
    ("A", "aaa", Some(Match::parts(vec![Part::matching("a"), Part::matching("a"), Part::matching("a")]))),
    (
      "A",
      "aab",
      Some(Match::parts(vec![Part::matching("a"), Part::matching("a"), Part::non_matching("b")])),
    ),
    (
      "A",
      "aba",
      Some(Match::parts(vec![Part::matching("a"), Part::non_matching("b"), Part::matching("a")])),
    ),
    ("A", "abb", Some(Match::parts(vec![Part::matching("a"), Part::non_matching("bb")]))),
    (
      "A",
      "baa",
      Some(Match::parts(vec![Part::non_matching("b"), Part::matching("a"), Part::matching("a")])),
    ),
    (
      "A",
      "bab",
      Some(Match::parts(vec![Part::non_matching("b"), Part::matching("a"), Part::non_matching("b")])),
    ),
    ("A", "bba", Some(Match::parts(vec![Part::non_matching("bb"), Part::matching("a")]))),
    ("A", "bbb", None),
  ];
  for (pattern, haystack, matching) in patterns_haystacks {
    println!("{} -> {}", pattern, haystack);
    let string_query_processor = StringQueryProcessor::new(pattern, true, true);
    assert_eq!(string_query_processor.matching(haystack), matching);
  }
}

#[test]
fn test_substring_query_processor() {
  let patterns_haystacks: [(&str, &str, Option<Match>); 14] = [
    ("a", "", None),
    ("a", "b", None),
    ("a", "a", Some(Match::parts(vec![Part::matching("a")]))),
    ("a", "aa", Some(Match::parts(vec![Part::matching("a"), Part::matching("a")]))),
    ("aa", "aa", Some(Match::parts(vec![Part::matching("aa")]))),
    ("aa", "aaa", Some(Match::parts(vec![Part::matching("aa"), Part::non_matching("a")]))),
    ("a", "aaa", Some(Match::parts(vec![Part::matching("a"), Part::matching("a"), Part::matching("a")]))),
    (
      "a",
      "aab",
      Some(Match::parts(vec![Part::matching("a"), Part::matching("a"), Part::non_matching("b")])),
    ),
    (
      "a",
      "aba",
      Some(Match::parts(vec![Part::matching("a"), Part::non_matching("b"), Part::matching("a")])),
    ),
    ("a", "abb", Some(Match::parts(vec![Part::matching("a"), Part::non_matching("bb")]))),
    (
      "a",
      "baa",
      Some(Match::parts(vec![Part::non_matching("b"), Part::matching("a"), Part::matching("a")])),
    ),
    (
      "a",
      "bab",
      Some(Match::parts(vec![Part::non_matching("b"), Part::matching("a"), Part::non_matching("b")])),
    ),
    ("a", "bba", Some(Match::parts(vec![Part::non_matching("bb"), Part::matching("a")]))),
    ("a", "bbb", None),
  ];
  for (pattern, haystack, matching) in patterns_haystacks {
    let substring_query_processor = SubstringQueryProcessor::create(pattern).unwrap();
    assert_eq!(substring_query_processor.describe(), format!("match substring \"{}\"", pattern));
    assert_eq!(substring_query_processor.matching(haystack), matching);
    let string_query_processor = StringQueryProcessor::create(pattern, true, false).unwrap();
    assert_eq!(string_query_processor.matching(haystack), matching);
  }
}

#[test]
fn test_regex_query_processor() {
  let haystacks: [(&str, &str, Option<Match>); 19] = [
    ("a+", "", None),
    ("a+", "b", None),
    ("a+", "a", Some(Match::parts(vec![Part::matching("a")]))),
    ("a+", "aaa", Some(Match::parts(vec![Part::matching("aaa")]))),
    (
      "a+",
      "bbabbbaab",
      Some(Match::parts(vec![
        Part::non_matching("bb"),
        Part::matching("a"),
        Part::non_matching("bbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
      ])),
    ),
    (
      "a+",
      "aaabbabbbaab",
      Some(Match::parts(vec![
        Part::matching("aaa"),
        Part::non_matching("bb"),
        Part::matching("a"),
        Part::non_matching("bbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
      ])),
    ),
    (
      "a+",
      "bbabbbaabaaa",
      Some(Match::parts(vec![
        Part::non_matching("bb"),
        Part::matching("a"),
        Part::non_matching("bbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
        Part::matching("aaa"),
      ])),
    ),
    (
      "a+",
      "aaabbabbbaabaaa",
      Some(Match::parts(vec![
        Part::matching("aaa"),
        Part::non_matching("bb"),
        Part::matching("a"),
        Part::non_matching("bbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
        Part::matching("aaa"),
      ])),
    ),
    ("aa", "", None),
    ("aa", "bbb", None),
    ("aa", "aa", Some(Match::parts(vec![Part::matching("aa")]))),
    ("aa", "aaa", Some(Match::parts(vec![Part::matching("aa"), Part::non_matching("a")]))),
    ("aa", "aaaa", Some(Match::parts(vec![Part::matching("aa"), Part::matching("aa")]))),
    (
      "aa",
      "aaaaa",
      Some(Match::parts(vec![Part::matching("aa"), Part::matching("aa"), Part::non_matching("a")])),
    ),
    ("aa", "aaabb", Some(Match::parts(vec![Part::matching("aa".to_string()), Part::non_matching("abb")]))),
    (
      "aa",
      "bbaaabbbaaab",
      Some(Match::parts(vec![
        Part::non_matching("bb"),
        Part::matching("aa"),
        Part::non_matching("abbb"),
        Part::matching("aa"),
        Part::non_matching("ab"),
      ])),
    ),
    (
      "aa",
      "aaabbabbbaab",
      Some(Match::parts(vec![
        Part::matching("aa"),
        Part::non_matching("abbabbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
      ])),
    ),
    (
      "aa",
      "bbabbbaabaaa",
      Some(Match::parts(vec![
        Part::non_matching("bbabbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
        Part::matching("aa"),
        Part::non_matching("a"),
      ])),
    ),
    (
      "aa",
      "aaabbabbbaabaaa",
      Some(Match::parts(vec![
        Part::matching("aa"),
        Part::non_matching("abbabbb"),
        Part::matching("aa"),
        Part::non_matching("b"),
        Part::matching("aa"),
        Part::non_matching("a"),
      ])),
    ),
  ];
  for (pattern, haystack, matching) in haystacks {
    let regex_query_processor = RegexQueryProcessor::create(pattern).unwrap();
    assert_eq!(regex_query_processor.describe(), format!("match against regular expression \"{}\"", pattern));
    assert_eq!(regex_query_processor.matching(haystack), matching);
  }
}

#[test]
fn test_expression_query_processor() {
  let expression_query_processor = ExpressionQueryProcessor::new("vhost");
  assert_eq!(expression_query_processor.describe(), "match against dsh expression vhost()");
  assert_eq!(
    expression_query_processor.matching("{ vhost('par') }").unwrap(),
    Match::expression("vhost", "par", None::<String>)
  );
  assert_eq!(
    expression_query_processor.matching("{ vhost('par1', 'par2') }").unwrap(),
    Match::expression("vhost", "par1", Some("par2".to_string()))
  );
}
