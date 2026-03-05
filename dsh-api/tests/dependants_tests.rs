use dsh_api::{strip_trifonius_prefix, CertificateSecretKind, Dependant, DependantTrifonius};
use itertools::Itertools;

#[test]
fn test_strip_trifonius_prefix() {
  assert_eq!(strip_trifonius_prefix("f-df87c48f-n-5104548e-p-sleep").unwrap(), "sleep");
  assert_eq!(strip_trifonius_prefix("p-6aythn-v-a6hrkl-n-7bvetd-metadata-manager").unwrap(), "metadata-manager");
  assert!(strip_trifonius_prefix("metadata-manager").is_none());
}

#[test]
fn test_dependant_service() {
  let trifonius = Dependant::<String>::service("f-df87c48f-n-5104548e-p-sleep", 1, vec![]);
  assert!(matches!(trifonius, Dependant::Trifonius { .. }));
  assert_eq!(trifonius.id(), "sleep");
  assert_eq!(trifonius.to_string(), "tr:sleep");
  let trifonius_old = Dependant::<String>::service("p-6aythn-v-a6hrkl-n-7bvetd-metadata-manager", 1, vec![]);
  assert!(matches!(trifonius_old, Dependant::Trifonius { .. }));
  assert_eq!(trifonius_old.id(), "metadata-manager");
  assert_eq!(trifonius_old.to_string(), "tr:metadata-manager");
  let non_trifonius = Dependant::<String>::service("sleep", 1, vec![]);
  assert!(matches!(non_trifonius, Dependant::Application { .. }));
  assert_eq!(non_trifonius.id(), "sleep");
  assert_eq!(non_trifonius.to_string(), "sleep");
}

#[test]
fn test_dependant_application_trifonius_new() {
  let trifonius: DependantTrifonius<String> = DependantTrifonius::try_new("f-df87c48f-n-5104548e-p-sleep", 1, vec![]).unwrap();
  assert_eq!(trifonius.trifonius_id, "sleep");
  assert_eq!(trifonius.to_string(), "tr:sleep");
  let non_trifonius: DependantTrifonius<String> = DependantTrifonius::new("sleep".to_string(), 1, vec![]);
  assert_eq!(non_trifonius.trifonius_id, "sleep");
  assert_eq!(non_trifonius.to_string(), "tr:sleep");
}

#[test]
fn test_dependant_ordering() {
  let mut reversed_list: [Dependant<String>; 5] = [
    Dependant::app("id5".to_string(), vec![]),
    Dependant::application("id4".to_string(), 1, vec![]),
    Dependant::certificate("id3".to_string(), CertificateSecretKind::CertChainSecret),
    Dependant::proxy("id2".to_string(), 1),
    Dependant::trifonius("id1".to_string(), 1, vec![]),
  ];
  reversed_list.sort();
  let sorted_ids = reversed_list.iter().map(|dependant| dependant.id()).collect_vec();
  assert_eq!(sorted_ids, vec!["id1", "id2", "id3", "id4", "id5"]);
}
