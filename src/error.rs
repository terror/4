use crate::common::*;

#[derive(Debug, Snafu)]
pub enum Error {
  #[snafu(display("Command not found."))]
  NotFound,
  #[snafu(display("Token out of bounds."))]
  TokenOutOfBounds,
  #[snafu(display("Stack underflow."))]
  StackUnderflow,
}
