extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    let f: Value<std_bit_and_int_> = Rc::new(RefCell::new(<std_bit_and_int_>::default()));
    assert!(
        (({
            let ___x: Value<i32> = Rc::new(RefCell::new(60053));
            let ___y: Value<i32> = Rc::new(RefCell::new(60053));
            (*f.borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        }) == 60053)
    );
    assert!(
        (({
            let ___x: Value<i32> = Rc::new(RefCell::new(60053));
            let ___y: Value<i32> = Rc::new(RefCell::new(22739));
            (*f.borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        }) == 18577)
    );
    assert!(
        (({
            let ___x: Value<i32> = Rc::new(RefCell::new(22739));
            let ___y: Value<i32> = Rc::new(RefCell::new(60053));
            (*f.borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        }) == 18577)
    );
    assert!(
        (({
            let ___x: Value<i32> = Rc::new(RefCell::new(22739));
            let ___y: Value<i32> = Rc::new(RefCell::new(0));
            (*f.borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        }) == 0)
    );
    assert!(
        (({
            let ___x: Value<i32> = Rc::new(RefCell::new(65535));
            let ___y: Value<i32> = Rc::new(RefCell::new(22739));
            (*f.borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        }) == 22739)
    );
    let f2: Value<std_bit_and_void_> = Rc::new(RefCell::new(<std_bit_and_void_>::default()));
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(60053));
            let ___u: Value<i32> = Rc::new(RefCell::new(60053));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 60053)
    );
    assert!(
        (({
            let ___t: Value<i64> = Rc::new(RefCell::new(60053_i64));
            let ___u: Value<i32> = Rc::new(RefCell::new(60053));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 60053_i64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(60053));
            let ___u: Value<i64> = Rc::new(RefCell::new(60053_i64));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 60053_i64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(60053));
            let ___u: Value<i32> = Rc::new(RefCell::new(22739));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 18577)
    );
    assert!(
        (({
            let ___t: Value<i64> = Rc::new(RefCell::new(60053_i64));
            let ___u: Value<i32> = Rc::new(RefCell::new(22739));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 18577_i64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(60053));
            let ___u: Value<i64> = Rc::new(RefCell::new(22739_i64));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 18577_i64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(22739));
            let ___u: Value<i32> = Rc::new(RefCell::new(60053));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 18577)
    );
    assert!(
        (({
            let ___t: Value<i64> = Rc::new(RefCell::new(22739_i64));
            let ___u: Value<i32> = Rc::new(RefCell::new(60053));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 18577_i64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(22739));
            let ___u: Value<i64> = Rc::new(RefCell::new(60053_i64));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 18577_i64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(22739));
            let ___u: Value<i32> = Rc::new(RefCell::new(0));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 0)
    );
    assert!(
        (({
            let ___t: Value<i64> = Rc::new(RefCell::new(22739_i64));
            let ___u: Value<i32> = Rc::new(RefCell::new(0));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 0_i64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(22739));
            let ___u: Value<i64> = Rc::new(RefCell::new(0_i64));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 0_i64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(65535));
            let ___u: Value<i32> = Rc::new(RefCell::new(22739));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 22739)
    );
    assert!(
        (({
            let ___t: Value<i64> = Rc::new(RefCell::new(65535_i64));
            let ___u: Value<i32> = Rc::new(RefCell::new(22739));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 22739_i64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(65535));
            let ___u: Value<i64> = Rc::new(RefCell::new(22739_i64));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 22739_i64)
    );
    let mut foo: i32 = 18577;
    const _: () = assert!(true, "foo == 0x4891");;
    let mut bar: i32 = 18577;
    const _: () = assert!(true, "bar == 0x4891");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
