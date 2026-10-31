extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(f: (), mut expected: i32) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    assert!((({ (*f.borrow_mut())() }) == expected));
}
pub fn test_1(f: (), mut expected: i32) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    assert!((({ (*f.borrow_mut())() }) == expected));
}
pub fn test_2(f: (), mut expected: i32) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    assert!((({ (*f.borrow_mut())() }) == expected));
}
pub fn test_3(f: (), mut expected: i32) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    assert!((({ (*f.borrow_mut())() }) == expected));
}
pub fn test_const_4(f: Ptr<()>, mut expected: i32) {
    assert!(({ ({ (*f.upgrade().deref())() }).clone() } == { expected }));
}
pub fn test_const_5(f: Ptr<()>, mut expected: i32) {
    assert!(({ ({ (*f.upgrade().deref())() }).clone() } == { expected }));
}
pub fn f_6() -> i32 {
    return 1;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_int_0 {}
impl Callable0<i32> for A_int_0 {
    fn call(&self) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| A_int_0Impl::operator_call_1(__it))
    }
}
pub fn test_all_7() -> bool {
    ({ test_0(unimplemented!(), 1) });
    ({ test_0(unimplemented!(), 1) });
    ({ test_1(unimplemented!(), 4) });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_const_4(_f.as_pointer(), 5)
    });
    ({ test_2(unimplemented!(), 1) });
    ({ test_2(unimplemented!(), 1) });
    ({ test_3(unimplemented!(), 4) });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_const_5(_f.as_pointer(), 5)
    });
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
    ({ test_all_7() });
    const _: () = assert!(true, "test_all()");;
    return 0;
}
pub trait A_int_0Impl {
    fn operator_call_1(&self) -> i32;
    fn operator_call_2(&self) -> i32;
}
impl A_int_0Impl for Ptr<A_int_0> {
    fn operator_call_1(&self) -> i32 {
        return 4;
    }
    fn operator_call_2(&self) -> i32 {
        return 5;
    }
}
pub fn __cpp2rust_init_globals() {}
