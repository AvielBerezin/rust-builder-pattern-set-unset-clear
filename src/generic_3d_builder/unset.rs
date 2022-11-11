use crate::generic_3d_builder::Params;
use crate::linked_list::P;

impl<A, As, Bs, Cs> Params<P<A, As>, Bs, Cs> {
  pub(crate) fn unset_a(self) -> Params<As, Bs, Cs> {
    Params {
      a: self.a.1,
      b: self.b,
      c: self.c,
    }
  }
}

impl<As, B, Bs, Cs> Params<As, P<B, Bs>, Cs> {
  pub(crate) fn unset_b(self) -> Params<As, Bs, Cs> {
    Params {
      a: self.a,
      b: self.b.1,
      c: self.c,
    }
  }
}

impl<As, Bs, C, Cs> Params<As, Bs, P<C, Cs>> {
  pub(crate) fn unset_c(self) -> Params<As, Bs, Cs> {
    Params {
      a: self.a,
      b: self.b,
      c: self.c.1,
    }
  }
}
