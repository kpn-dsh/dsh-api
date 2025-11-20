use dsh_api::version::Version;
use serde::Deserialize;
use std::str::FromStr;

#[test]
fn test_correct_representations() {
  let correct_representations: Vec<(Vec<&str>, Version)> = vec![
    (vec!["0", "0.0", "0.0.0"], Version::new(0, 0, 0, None)),
    (vec!["0-beta", "0.0-beta", "0.0.0-beta"], Version::new(0, 0, 0, Some("beta".to_string()))),
    (vec!["1", "1.0", "1.0.0"], Version::new(1, 0, 0, None)),
    (vec!["1-beta", "1.0-beta", "1.0.0-beta"], Version::new(1, 0, 0, Some("beta".to_string()))),
    (vec!["1.2", "1.2.0"], Version::new(1, 2, 0, None)),
    (vec!["1.2-beta", "1.2.0-beta"], Version::new(1, 2, 0, Some("beta".to_string()))),
    (vec!["1.2.3"], Version::new(1, 2, 3, None)),
    (vec!["1.2.3-beta"], Version::new(1, 2, 3, Some("beta".to_string()))),
  ];
  for (representations, version) in correct_representations {
    for representation in representations {
      assert_eq!(Version::from_str(representation).unwrap(), version);
    }
  }
}

#[test]
fn test_incorrect_representations() {
  const INCORRECT_REPRESENTATIONS: [&str; 10] = ["", " ", ".", "0.", ".0", "0..0", "a", "0beta", "1.2beta", "1.2.3beta"];
  for representation in INCORRECT_REPRESENTATIONS {
    assert!(Version::from_str(representation).is_err());
  }
}

#[test]
fn test_partial_ordering() {
  let mut ordered_versions = vec![
    Version::new(1, 1, 1, Some("alpha".to_string())),
    Version::new(1, 1, 1, Some("beta".to_string())),
    Version::new(1, 1, 1, None),
    Version::new(1, 1, 2, Some("alpha".to_string())),
    Version::new(1, 1, 2, Some("beta".to_string())),
    Version::new(1, 1, 2, None),
    Version::new(1, 2, 1, Some("alpha".to_string())),
    Version::new(1, 2, 1, Some("beta".to_string())),
    Version::new(1, 2, 1, None),
    Version::new(1, 2, 2, Some("alpha".to_string())),
    Version::new(1, 2, 2, Some("beta".to_string())),
    Version::new(1, 2, 2, None),
    Version::new(2, 1, 1, Some("alpha".to_string())),
    Version::new(2, 1, 1, Some("beta".to_string())),
    Version::new(2, 1, 1, None),
  ]
  .into_iter();
  let mut less = ordered_versions.next().unwrap();
  for greater in ordered_versions {
    assert!(less < greater);
    assert_ne!(less, greater);
    less = greater;
  }
}

#[test]
fn test_deserialize_version_field() {
  #[derive(Deserialize)]
  struct StructContainingVersion {
    version: Version,
  }
  assert_eq!(
    serde_json::from_str::<StructContainingVersion>("{\"version\": \"1.2.3\"}").unwrap().version,
    Version::new(1, 2, 3, None)
  );
}
