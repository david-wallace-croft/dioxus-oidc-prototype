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
  L016,
  L017,
  L018,
  L019,
  L020,
  L021,
  L022,
  L023,
  L024,
  L025,
  L026,
  L027,
  L028,
  L029,
  L030,
  L031,
  // L032,
  L033,
  L034,
  L035,
  L036,
  // L037,
  // L038,
  // L039,
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
