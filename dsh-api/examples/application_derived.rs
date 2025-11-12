#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::bucket::BucketInjection;
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::query_processor::Part::{Matching, NonMatching};
use dsh_api::query_processor::{Match, Part, RegexQueryProcessor};
use dsh_api::secret::SecretInjection;
use dsh_api::types::{AllocationStatus, Application};
use dsh_api::DependantApplication;
use itertools::Itertools;

const BUCKET: &str = "flink-cluster-bucket";
const QUERY: &str = "greenbox-dev$";
const SECRET: &str = "boss-account-ids";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("client.find_applications(needs_token)");
  let predicate = |application: &Application| application.needs_token;
  let found_applications: Vec<(String, Application)> = client.applications_filtered(&predicate).await?;
  println!("{} applications need token", found_applications.len());
  for (application_id, _) in found_applications {
    println!("{}", application_id);
  }

  print_header("client.find_applications_that_use_env_value(query)");
  let query_processor = RegexQueryProcessor::create(QUERY).unwrap();
  let applications_that_use_env_value: Vec<(String, Application, Vec<(String, Match)>)> = client.applications_that_use_env_value(&query_processor).await?;
  for (application_id, _, matching_env_vars) in applications_that_use_env_value {
    println!("{}", application_id);
    for (key, matching_env_var) in &matching_env_vars {
      if let Match::Parts(parts) = matching_env_var {
        println!("  {} -> {}", key, parts_to_ansi_formatted_string(parts));
      }
    }
  }

  print_header(&format!("client.applications_dependant_on_bucket('{}')", BUCKET));
  let dependant_applications: Vec<DependantApplication<BucketInjection>> = client.applications_dependant_on_bucket(BUCKET).await?;
  println!("{} applications have injections for bucket '{}'", dependant_applications.len(), BUCKET);
  for dependant_application in dependant_applications {
    println!(
      "{} -> {}",
      dependant_application.application_id,
      dependant_application.injections.iter().map(|inj| inj.to_string()).collect_vec().join(", ")
    )
  }

  print_header(&format!("client.applications_dependant_on_secret('{}')", SECRET));
  let dependant_applications: Vec<DependantApplication<SecretInjection>> = client.applications_dependant_on_secret(SECRET).await?;
  println!("{} applications have injections for secret '{}'", dependant_applications.len(), SECRET);
  for dependant_application in dependant_applications {
    println!(
      "{} -> {}",
      dependant_application.application_id,
      dependant_application.injections.iter().map(|inj| inj.to_string()).collect_vec().join(", ")
    )
  }

  print_header("client.list_application_allocation_statuses()");
  let application_allocation_statuses: Vec<(String, AllocationStatus)> = client.application_ids_with_allocation_statuses().await?;
  println!("{}", application_allocation_statuses.len());
  for (application_id, allocation_status) in application_allocation_statuses {
    println!("{} -> {}", application_id, allocation_status);
  }

  print_header("client.list_application_ids()");
  let application_ids: Vec<String> = client.application_ids().await?;
  println!("{}", application_ids.len());
  for application_id in application_ids {
    println!("{}", application_id);
  }

  print_header("client.list_applications()");
  let applications_list: Vec<(String, Application)> = client.applications().await?;
  println!("{}", applications_list.len());
  for (application_id, application) in applications_list {
    println!("{} -> {}", application_id, application);
  }

  Ok(())
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
