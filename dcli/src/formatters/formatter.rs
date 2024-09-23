use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use tabled::settings::peaker::PriorityMax;
use tabled::settings::Reverse;
use tabled::settings::Rotate;
use tabled::settings::{Padding, Width};
use tabled::{builder::Builder as TabledBuilder, settings::Style};
use termion::terminal_size;

use crate::DcliContext;

pub trait Label: Eq + Hash + PartialEq {
  fn label_list(&self) -> &str {
    self.label_show()
  }

  fn label_show(&self) -> &str;
}

pub trait SubjectFormatter<L: Label> {
  fn value(&self, label: &L, target_id: &str) -> String;

  fn target_id(&self) -> Option<String> {
    None
  }

  #[allow(dead_code)]
  fn target_label(&self) -> Option<L>;
}

pub struct TableBuilder<'a, L: Label, V: SubjectFormatter<L>> {
  list: bool,
  labels: &'a [L],
  context: &'a DcliContext,
  tabled_builder: TabledBuilder,
  phantom: PhantomData<&'a V>,
}

impl<'a, L, V> TableBuilder<'a, L, V>
where
  L: Label,
  V: SubjectFormatter<L>,
{
  pub fn list(labels: &'a [L], context: &'a DcliContext) -> Self {
    let mut tabled_builder = TabledBuilder::default();
    if context.show_headers() {
      tabled_builder.push_record(labels.iter().map(|label| label.label_list()));
    }
    Self { list: true, labels, context, tabled_builder, phantom: PhantomData }
  }

  pub fn show(labels: &'a [L], context: &'a DcliContext) -> Self {
    let mut tabled_builder = TabledBuilder::default();
    if context.show_headers() {
      tabled_builder.push_record(labels.iter().map(|label| label.label_show()));
    }
    Self { list: false, labels, context, tabled_builder, phantom: PhantomData }
  }

  pub fn values(&mut self, values: &[(String, V)]) -> &Self {
    for (target_id, value) in values {
      self.tabled_builder.push_record(self.labels.iter().map(|label| value.value(label, target_id)));
    }
    self
  }

  pub fn value(&mut self, target_id: String, value: &V) -> &Self {
    self
      .tabled_builder
      .push_record(self.labels.iter().map(|label| value.value(label, target_id.as_str())));
    self
  }

  pub fn _vecs(&mut self, vecs: &Vec<Vec<String>>) -> &Self {
    for vec in vecs {
      self.tabled_builder.push_record(vec);
    }
    self
  }

  pub fn _vec(&mut self, vec: &Vec<String>) -> &Self {
    self.tabled_builder.push_record(vec);
    self
  }

  pub fn _map(&mut self, map: &HashMap<String, V>) -> &Self {
    let mut target_ids = map.keys().collect::<Vec<&String>>();
    target_ids.sort();
    for target_id in target_ids {
      self
        .tabled_builder
        .push_record(self.labels.iter().map(|label| map.get(target_id).unwrap().value(label, target_id)));
    }
    self
  }

  pub fn rows(&mut self, rows: &[V]) -> &Self {
    for row in rows {
      self
        .tabled_builder
        .push_record(self.labels.iter().map(|label| row.value(label, row.target_id().unwrap_or_default().as_str())));
    }
    self
  }

  pub fn row(&mut self, row: &V) -> &Self {
    self
      .tabled_builder
      .push_record(self.labels.iter().map(|label| row.value(label, row.target_id().unwrap_or_default().as_str())));
    self
  }

  pub fn print(self) {
    if self.list {
      let mut table = self.tabled_builder.build();
      if let Ok((columns, _rows)) = terminal_size() {
        table.with(Width::truncate(columns as usize).priority(PriorityMax).suffix("..."));
      }
      if self.context.border {
        table.with(Padding::new(1, 1, 0, 0));
        table.with(Style::sharp());
      } else {
        table.with(Padding::new(0, 2, 0, 0));
        table.with(Style::empty());
      }
      println!("{}", table);
    } else {
      let mut table = self.tabled_builder.build();
      if self.context.border {
        table.with(Padding::new(1, 1, 0, 0));
        table.with(Style::sharp());
      } else {
        table.with(Padding::new(0, 2, 0, 0));
        table.with(Style::empty());
      }
      table.with(Rotate::Left);
      table.with(Reverse::default());
      println!("{}", table);
    }
  }
}

impl<L> SubjectFormatter<L> for HashMap<&L, String>
where
  L: Label,
{
  fn value(&self, label: &L, _target_id: &str) -> String {
    self.get(label).map(|s| s.to_string()).unwrap_or_default()
  }

  fn target_label(&self) -> Option<L> {
    None
  }
}

pub struct StringTableBuilder<'a> {
  context: &'a DcliContext,
  tabled_builder: TabledBuilder,
}

impl<'a> StringTableBuilder<'a> {
  pub fn new(labels: &'a [&'a str], context: &'a DcliContext) -> Self {
    let mut tabled_builder = TabledBuilder::default();
    if context.show_headers() {
      tabled_builder.push_record(labels.iter().map(|label| label.to_string()));
    }
    Self { context, tabled_builder }
  }

  pub fn _vecs(&mut self, vecs: &Vec<Vec<String>>) -> &Self {
    for vec in vecs {
      self.tabled_builder.push_record(vec);
    }
    self
  }

  pub fn vec(&mut self, vec: &Vec<String>) -> &Self {
    self.tabled_builder.push_record(vec);
    self
  }

  pub fn print_list(self) {
    let mut table = self.tabled_builder.build();
    if let Ok((columns, _rows)) = terminal_size() {
      table.with(Width::truncate(columns as usize).priority(PriorityMax).suffix("..."));
    }
    if self.context.border {
      table.with(Padding::new(1, 1, 0, 0));
      table.with(Style::sharp());
    } else {
      table.with(Padding::new(0, 2, 0, 0));
      table.with(Style::empty());
    }
    println!("{}", table);
  }

  pub fn print_show(self) {
    let mut table = self.tabled_builder.build();
    if let Ok((columns, _rows)) = terminal_size() {
      table.with(Width::truncate(columns as usize).suffix("..."));
    }
    if self.context.border {
      table.with(Padding::new(1, 1, 0, 0));
      table.with(Style::sharp());
    } else {
      table.with(Padding::new(0, 2, 0, 0));
      table.with(Style::empty());
    }
    table.with(Rotate::Left);
    table.with(Reverse::default());
    println!("{}", table);
  }
}

pub fn print_ids(target_id: String, ids: Vec<String>, context: &DcliContext) {
  let mut tabled_builder = TabledBuilder::default();
  tabled_builder.push_record(vec![target_id]);
  for id in ids {
    tabled_builder.push_record(vec![id]);
  }
  let mut table = tabled_builder.build();
  if let Ok((columns, _rows)) = terminal_size() {
    table.with(Width::truncate(columns as usize).priority(PriorityMax).suffix("..."));
  }
  if context.border {
    table.with(Padding::new(1, 1, 0, 0));
    table.with(Style::sharp());
  } else {
    table.with(Padding::new(0, 2, 0, 0));
    table.with(Style::empty());
  }
  println!("{}", table);
}