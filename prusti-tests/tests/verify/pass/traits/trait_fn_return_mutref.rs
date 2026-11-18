trait Foo {
    fn to_mut<'a>(&self) -> &'a mut i32;
}

fn foo<T: Foo>(x: &T) {
    let z = x.to_mut();
    *z = 100;
}
