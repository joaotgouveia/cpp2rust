extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Test {
    #[offset(0)]
    pub x: i32,
}
pub fn Update_0(mut t: Ptr<Test>) -> Ptr<Test> {
    let mut x: i32 = 1;
    let mut y: i32 = 2;
    x.prefix_inc();
    ({ TestImpl::update(&t, x, y) });
    x = t.with(|__s| (__s).x);
    y = t.with(|__s| __s.x);
    ({
        let _x: i32 = x;
        let _y: i32 = y;
        TestImpl::update(&(t), _x, _y)
    });
    return t;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t1: Value<Test> = Rc::new(RefCell::new(Test { x: 100 }));
    let mut t2: Ptr<Test> = ({ Update_0((t1.as_pointer())) });
    let mut t3: Ptr<Test> = Ptr::<Test>::null();
    t3 = (t2).clone();
    field!(t3, x).write(15);
    {
        ({ TestImpl::as_ptr(&t3) }).with_mut(|__v| *__v = *__v + 10)
    };
    (if (({ ({ t3.with(|__s| __s.x) } + { t2.with(|__s| __s.x) }) } + { { (*t1.borrow()).x } })
        == 75)
    {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"t3->x + t2->x + t1.x == 75"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait TestImpl {
    fn inc(&self);
    fn dec(&self);
    fn as_ptr(&self) -> Ptr<i32>;
    fn update(&self, x: i32, y: i32);
}
impl TestImpl for Ptr<Test> {
    fn inc(&self) {
        field!((*self), x).with_mut(|__v| __v.postfix_inc());
    }
    fn dec(&self) {
        field!((*self), x).with_mut(|__v| __v.postfix_dec());
    }
    fn as_ptr(&self) -> Ptr<i32> {
        return (field_ptr!((*self), x));
    }
    fn update(&self, mut x: i32, mut y: i32) {
        field!((*self), x).write((x + y));
    }
}
pub fn __cpp2rust_init_globals() {}
