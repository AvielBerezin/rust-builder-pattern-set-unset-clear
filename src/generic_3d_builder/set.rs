use crate::generic_3d_builder::Params;
use crate::linked_list::P;

impl<As, Bs, Cs> Params<As, Bs, Cs> {
  pub(crate) fn set_a<A>(self, a: A) -> Params<P<A, As>, Bs, Cs> {
    Params {
      a: P(a, self.a),
      b: self.b,
      c: self.c,
    }
  }

  pub(crate) fn set_b<B>(self, b: B) -> Params<As, P<B, Bs>, Cs> {
    Params {
      a: self.a,
      b: P(b, self.b),
      c: self.c,
    }
  }

  pub(crate) fn set_c<C>(self, c: C) -> Params<As, Bs, P<C, Cs>> {
    Params {
      a: self.a,
      b: self.b,
      c: P(c, self.c),
    }
  }
}
