extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut x: i32) -> i32 {
    return x;
}
pub fn foo_1(mut x: Ptr<i32>) -> i32 {
    return (x.read());
}
pub fn foo_2(mut x: Ptr<i32>, mut y: Ptr<i32>) -> i32 {
    return ({ (x.read()) } + { (y.read()) });
}
pub fn foo_3(mut x: Ptr<i32>, mut y: Ptr<i32>, z: Ptr<i32>) -> i32 {
    return ({ ({ (x.read()) } + { (y.read()) }) } + { (z.read()) });
}
pub fn bar_4(x: Ptr<i32>) -> i32 {
    return (x.read());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Foo {}
pub fn func_5(mut x: i32) -> i32 {
    return 1;
}
pub fn func_6(mut x: Ptr<i32>) -> i32 {
    return 1;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let mut out: i32 = 0;
    out += ({ foo_0(0) });
    out += ({ foo_1((x.as_pointer())) });
    out += ({ bar_4(x.as_pointer()) });
    out += ({
        let _x: Ptr<i32> = (x.as_pointer());
        let _y: Ptr<i32> = (x.as_pointer());
        let _z: Ptr<i32> = x.as_pointer();
        foo_3(_x, _y, _z)
    });
    out += ({
        let _x: Ptr<i32> = (x.as_pointer());
        let _y: Ptr<i32> = (x.as_pointer());
        foo_2(_x, _y)
    });
    let mut bar: i32 = 5;
    out += ((bar + ({ foo_0(0) })) + ({ foo_1((x.as_pointer())) }));
    let foo1: Value<Foo> = Rc::new(RefCell::new(<Foo>::default()));
    let foo2: Value<Foo> = Rc::new(RefCell::new(<Foo>::default()));
    ({ FooImpl::foo_2(&foo1.as_pointer()) });
    ({ FooImpl::method_3(&foo1.as_pointer(), 1) });
    ({ FooImpl::foo_1(&foo2.as_pointer()) });
    ({ FooImpl::method_4(&foo2.as_pointer(), 2) });
    (if (out == 13) {
        &(0);
    } else {
        ({
            __assert_fail_7(
                Ptr::<i8>::from_string_literal(b"out == 13"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_7(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait FooImpl {
    fn foo_1(&self);
    fn foo_2(&self);
    fn method_3(&self, x: i32);
    fn method_4(&self, x: i32);
    fn method2_5(&self, x: i32, y: i32);
    fn method2_6(&self, x: f64, y: f64);
}
impl FooImpl for Ptr<Foo> {
    fn foo_1(&self) {}
    fn foo_2(&self) {}
    fn method_3(&self, mut x: i32) {}
    fn method_4(&self, mut x: i32) {}
    fn method2_5(&self, mut x: i32, mut y: i32) {}
    fn method2_6(&self, mut x: f64, mut y: f64) {}
}
pub fn __cpp2rust_init_globals() {}
