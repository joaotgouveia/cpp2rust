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
pub struct Point {
    #[offset(0)]
    pub x: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_0 {
    #[offset(0)]
    pub val: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_1 {
    #[offset(0)]
    #[byte_size(4)]
    pub val: Point,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Deduced_2 {
    #[offset(0)]
    pub val: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Deduced_3 {
    #[offset(0)]
    #[byte_size(4)]
    pub val: Point,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i: Value<Box_0> = Rc::new(RefCell::new(Box_0 { val: 3 }));
    (if (({ Box_0Impl::twice(&i.as_pointer()) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"i.twice() == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let p: Value<Box_1> = Rc::new(RefCell::new(Box_1 {
        val: Point { x: 4 },
    }));
    (if ({ ({ Box_1Impl::get(&p.as_pointer()) }).x } == 4) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"p.get().x == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let d: Value<Deduced_2> = Rc::new(RefCell::new(Deduced_2 { val: 5 }));
    (if (({ Deduced_2Impl::twice(&d.as_pointer()) }) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"d.twice() == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let dp: Value<Deduced_3> = Rc::new(RefCell::new(Deduced_3 {
        val: Point { x: 6 },
    }));
    (if ({ ({ Deduced_3Impl::get(&dp.as_pointer()) }).x } == 6) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"dp.get().x == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_4(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait Box_0Impl {
    fn get(&self) -> i32 {
        unimplemented!()
    }
    fn twice(&self) -> i32;
}
impl Box_0Impl for Ptr<Box_0> {
    fn twice(&self) -> i32 {
        return ((*self).with(|__s| __s.val) + (*self).with(|__s| __s.val));
    }
}
pub trait Box_1Impl {
    fn get(&self) -> Point;
    fn twice(&self) -> Point {
        unimplemented!()
    }
}
impl Box_1Impl for Ptr<Box_1> {
    fn get(&self) -> Point {
        return (*self).with(|__s| __s.val.clone());
    }
}
pub trait Deduced_2Impl {
    fn twice(&self) -> i32;
}
impl Deduced_2Impl for Ptr<Deduced_2> {
    fn twice(&self) -> i32 {
        return ((*self).with(|__s| __s.val) + (*self).with(|__s| __s.val));
    }
}
pub trait Deduced_3Impl {
    fn get(&self) -> Point;
}
impl Deduced_3Impl for Ptr<Deduced_3> {
    fn get(&self) -> Point {
        return (*self).with(|__s| __s.val.clone());
    }
}
pub fn __cpp2rust_init_globals() {}
