extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_type_0() {
    let ii1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ii2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ii4: Value<()> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
    let cii: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((*ii1.borrow()) == (*ii2.borrow()));
    assert!((*ii1.borrow()) == (*ii4.borrow()));
    assert!((*ii1.borrow()) == (*cii.borrow()));
    assert!(!((*ii1.borrow()) != (*ii2.borrow())));
    assert!(!((*ii1.borrow()) != (*cii.borrow())));
    let v: Value<i8> = Rc::new(RefCell::new(0_i8));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        (*ii1.borrow()).partial_cmp(&(*ii2.borrow())).unwrap(),
    ));
    assert!((*r1.borrow()) == std::cmp::Ordering::Equal);
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!((*r2.borrow()) == std::cmp::Ordering::Equal);
}
pub fn test_type_1() {
    let ii1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ii2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ii4: Value<()> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
    let cii: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((*ii1.borrow()) == (*ii2.borrow()));
    assert!((*ii1.borrow()) == (*ii4.borrow()));
    assert!((*ii1.borrow()) == (*cii.borrow()));
    assert!(!((*ii1.borrow()) != (*ii2.borrow())));
    assert!(!((*ii1.borrow()) != (*cii.borrow())));
    let v: Value<i32> = Rc::new(RefCell::new(0_i32));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        (*ii1.borrow()).partial_cmp(&(*ii2.borrow())).unwrap(),
    ));
    assert!((*r1.borrow()) == std::cmp::Ordering::Equal);
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!((*r2.borrow()) == std::cmp::Ordering::Equal);
}
pub fn test_type_2() {
    let ii1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ii2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ii4: Value<()> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
    let cii: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((*ii1.borrow()) == (*ii2.borrow()));
    assert!((*ii1.borrow()) == (*ii4.borrow()));
    assert!((*ii1.borrow()) == (*cii.borrow()));
    assert!(!((*ii1.borrow()) != (*ii2.borrow())));
    assert!(!((*ii1.borrow()) != (*cii.borrow())));
    let v: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    let r1: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
        (*ii1.borrow()).partial_cmp(&(*ii2.borrow())).unwrap(),
    ));
    assert!((*r1.borrow()) == std::cmp::Ordering::Equal);
    let r2: Value<std::cmp::Ordering> = Rc::new(RefCell::new(unimplemented!()));
    assert!((*r2.borrow()) == std::cmp::Ordering::Equal);
}
pub fn test_3() -> bool {
    ({ test_type_0() });
    ({ test_type_1() });
    ({ test_type_2() });
    return true;
}
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
    ({ test_3() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
