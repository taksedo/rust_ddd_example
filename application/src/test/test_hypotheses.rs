use std::mem;
use std::mem::Discriminant;

enum Foo {
    A(&'static str),
    B(i32),
    C(i32),
}

#[test]
fn test() {
    let _d: Discriminant<Foo> = mem::discriminant(&Foo::A("bar"));
    assert_eq!(
        mem::discriminant(&Foo::A("bar")),
        mem::discriminant(&Foo::A("baz"))
    );
    assert_eq!(mem::discriminant(&Foo::B(1)), mem::discriminant(&Foo::B(2)));
    assert_ne!(mem::discriminant(&Foo::B(3)), mem::discriminant(&Foo::C(3)));
}
