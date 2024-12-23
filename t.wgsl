struct S { t : i32, }

  @group(1) @binding(1) var<storage, read> a: array<S>;

fn f() {
    let p = &a;
    let r = p[3];
}