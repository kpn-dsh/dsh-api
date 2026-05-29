use dsh_api::types::Notification;
use std::collections::HashMap;

#[test]
fn test_notification_render_message() {
  let args = HashMap::from([
    ("key1".to_string(), "value1".to_string()),
    ("key2".to_string(), "value2".to_string()),
    ("urn1".to_string(), "allocation/m-tenant/secret/urnvalue1".to_string()),
    ("urn2".to_string(), "allocation/urnvalue2".to_string()),
  ]);
  let notification_with_template = Notification::new(false, "${urn:urn1}|${key1}|${urn:urn2}|${key2}", args.clone());
  assert_eq!(notification_with_template.render_message(), "urnvalue1|value1|urnvalue2|value2");
  assert_eq!(format!("{}", notification_with_template), "creation/update");
  assert_eq!(format!("{:#}", notification_with_template), "creation/update: urnvalue1|value1|urnvalue2|value2");

  let notification_with_template_list = Notification::new(false, "${urn_list:urn1}|${key1}|${urn_list:urn2}|${key2}", args);
  assert_eq!(notification_with_template_list.render_message(), "urnvalue1|value1|urnvalue2|value2");
  assert_eq!(format!("{}", notification_with_template_list), "creation/update");
  assert_eq!(
    format!("{:#}", notification_with_template_list),
    "creation/update: urnvalue1|value1|urnvalue2|value2"
  );

  let notification_with_string = Notification::new(true, "message without placeholders", HashMap::new());
  assert_eq!(notification_with_string.render_message(), "message without placeholders");
  assert_eq!(format!("{}", notification_with_string), "remove");
  assert_eq!(format!("{:#}", notification_with_string), "remove: message without placeholders");
}
