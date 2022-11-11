use crate::linked_list::P;
use crate::generic_3d_builder::Params;

#[derive(Debug)]
pub(crate) struct Target<'a> {
  a: u32,
  b: &'a str,
  c: f64,
}

impl<As, Bs, Cs> Params<P<u32, As>, P<&str, Bs>, P<f64, Cs>> {
  pub(crate) fn build_target(&self) -> Target {
    Target {
      a: self.a.0,
      b: self.b.0,
      c: self.c.0,
    }
  }
}