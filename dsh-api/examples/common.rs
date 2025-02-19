#[allow(dead_code)]
fn main() {}

#[allow(unused)]
pub fn initialize_logger() {
  env_logger::builder().format_target(false).format_timestamp(None).init();
}

#[allow(unused)]
pub fn print_header(header: &str) {
  let bar = (0..header.len()).map(|_| "-").collect::<String>();
  println!("\n{}\n{}\n{}", bar, header, bar);
}
