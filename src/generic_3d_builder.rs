mod set;
mod unset;
mod clear;

use crate::linked_list::{N, P};

pub(crate) struct Params<A, B, C> {
  pub(crate) a: A,
  pub(crate) b: B,
  pub(crate) c: C,
}

impl Params<N, N, N> {
  pub(crate) fn new() -> Self {
    Params {
      a: N(),
      b: N(),
      c: N(),
    }
  }
}
