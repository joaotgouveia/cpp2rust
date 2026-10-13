extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, Destructor, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {
    pub fn new(mut init: i32) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self { v: init }));
        let this: Ptr<S> = __this.as_pointer();
        ({ SImpl::mut_method(&this) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(&this) }));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Point {
    pub fn new_1(mut x: i32, mut y: i32) -> Self {
        Self { x: x, y: y }
    }
    pub fn new_2(mut v: i32) -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Point::new_1({ v }, { (v + 1) })));
        let this: Ptr<Point> = __this.as_pointer();
        {
            field!(this, y).with_mut(|__v| *__v = *__v * 10)
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3() -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Point::new_2({ 4 })));
        let this: Ptr<Point> = __this.as_pointer();
        {
            field!(this, x).with_mut(|__v| *__v = *__v + 100)
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Point {
    fn default() -> Self {
        { Point::new_3() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(S::new({ 3 })));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
        (if ({ (*s.borrow()).v } == 4) {
            &(0);
        } else {
            ({
                __assert_fail_1(
                    Ptr::<i8>::from_string_literal(b"s.v == 4"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
        (if (total_0.with(|rc| *rc.borrow()) == 8) {
            &(0);
        } else {
            ({
                __assert_fail_1(
                    Ptr::<i8>::from_string_literal(b"total == 8"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    (if (total_0.with(|rc| *rc.borrow()) == 18) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"total == 18"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p: Point = Point::new_3();
    (if (p.x == 104) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"p.x == 104"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (p.y == 50) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"p.y == 50"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut q: Point = Point::new_2({ 7 });
    (if (q.x == 7) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"q.x == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (q.y == 80) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"q.y == 80"),
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
pub trait SImpl {
    fn const_method(&self) -> i32;
    fn mut_method(&self);
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn const_method(&self) -> i32 {
        return ((*self).with(|__s| __s.v) * 2);
    }
    fn mut_method(&self) {
        {
            field!((*self), v).with_mut(|__v| *__v = *__v + 1)
        };
    }
    fn destructor(&self) {
        ({ SImpl::mut_method(self) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(self) }));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = total_0.with(|_| ());
}
