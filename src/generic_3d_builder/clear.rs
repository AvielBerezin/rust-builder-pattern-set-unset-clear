use crate::generic_3d_builder::Params;
use crate::linked_list::N;

impl<As, Bs, Cs> Params<As, Bs, Cs> {
  pub(crate) fn clear_a(self) -> Params<N, Bs, Cs> {
    Params {
      a: N(),
      b: self.b,
      c: self.c,
    }
  }

  pub(crate) fn clear_b(self) -> Params<As, N, Cs> {
    Params {
      a: self.a,
      b: N(),
      c: self.c,
    }
  }

  pub(crate) fn clear_c(self) -> Params<As, Bs, N> {
    Params {
      a: self.a,
      b: self.b,
      c: N(),
    }
  }
}
