use std::fmt::Display;

#[derive(Debug)]
pub enum LogId {
  L001,
  L002,
  L003,
  L004,
  L005,
  L006,
  L007,
  L008,
  L009,
  L010,
  L011,
  L012,
  L013,
  L014,
  L015,
}

impl Display for LogId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "[{self:?}]")
  }
}

// pub trait Logger {
//   fn info(
//     log_id: LogId,
//     message: &str,
//   );
// }
