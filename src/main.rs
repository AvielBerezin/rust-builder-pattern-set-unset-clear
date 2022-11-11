use crate::generic_3d_builder::Params;

mod generic_3d_builder;
mod target_builder;
mod linked_list;

fn main() {
  // set demonstration
  let builder1 = Params::new()
    .set_a(12)
    .set_b("asd")
    .set_c(12.3)
    .set_a(10)
    .set_b("no");

  println!("{:?}", builder1.build_target()); // Target { a: 10, b: "no", c: 12.3 }

  // unset demonstration part 1
  let builder2 = builder1
    .unset_a()
    .unset_b();

  println!("{:?}", builder2.build_target()); // Target { a: 12, b: "asd", c: 12.3 }

  // unset demonstration part 2
  let builder3 = builder2.unset_c();

  // builder3.build_target(); // does not compile because parameter 'c' is unset completely
  // builder3.unset_c();      // does not compile because parameter 'c' is unset completely

  // clear demonstration
  let builder4 = builder3
    .set_c(1.23)
    .set_a(132)
    .set_a(13444)
    .set_a(6453)
    .clear_a();

  // let target = builder4.build_target(); // does not compile because parameter 'a' is cleared (unset completely)
}
