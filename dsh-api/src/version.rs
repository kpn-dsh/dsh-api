use lazy_static::lazy_static;
use regex::Regex;
use serde::{de, Deserializer};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
  postfix: Option<String>,
}

impl Version {
  pub fn new(major: u32, minor: u32, patch: u32, postfix: Option<String>) -> Version {
    Version { major, minor, patch, postfix }
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self.postfix {
      Some(ref postfix) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, postfix),
      None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
    }
  }
}

impl PartialOrd<Self> for Version {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Version {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch)) {
      Ordering::Less => Ordering::Less,
      Ordering::Equal => match (&self.postfix, &other.postfix) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(self_postfix), Some(other_postfix)) => self_postfix.cmp(other_postfix),
      },
      Ordering::Greater => Ordering::Greater,
    }
  }
}

lazy_static! {
  static ref VERSION_REGEX: Regex = Regex::new(r"^([0-9]+)(?:.([0-9]+))?(?:.([0-9]+))?(?:-([a-zA-Z][a-zA-Z0-9_-]*))?$").unwrap();
}

impl FromStr for Version {
  type Err = String;

  fn from_str(representation: &str) -> Result<Self, Self::Err> {
    match VERSION_REGEX.captures(representation) {
      Some(captures) => Ok(Version::new(
        captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        captures.get(2).map(|m| m.as_str().parse::<u32>().unwrap()).unwrap_or(0),
        captures.get(3).map(|m| m.as_str().parse::<u32>().unwrap()).unwrap_or(0),
        captures.get(4).map(|m| m.as_str().to_string()),
      )),
      None => Err(format!("invalid version representation {}", representation)),
    }
  }
}

impl<'de> Deserialize<'de> for Version {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    FromStr::from_str(&String::deserialize(deserializer)?).map_err(de::Error::custom)
  }
}
