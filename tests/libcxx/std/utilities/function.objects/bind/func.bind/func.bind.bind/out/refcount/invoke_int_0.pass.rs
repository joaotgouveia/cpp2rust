extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(f: (), mut expected: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    unimplemented!();
}
pub fn test_1(f: (), mut expected: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    unimplemented!();
}
pub fn test_2(f: (), mut expected: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    unimplemented!();
}
pub fn test_3(f: (), mut expected: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    unimplemented!();
}
pub fn test_4(f: (), mut expected: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    unimplemented!();
}
pub fn test_5(f: (), mut expected: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    unimplemented!();
}
pub fn test_const_6(f: Ptr<()>, mut expected: ()) {
    unimplemented!();
}
pub fn test_const_7(f: Ptr<()>, mut expected: ()) {
    unimplemented!();
}
pub fn f_8() -> () {
    return 1_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_int_0 {}
pub fn test_all_9() -> () {
    ({ test_0(({ bind_10(FnPtr::<fn() -> ()>::new(f_8)) }), 1_()) });
    ({
        test_1(
            ({
                let ___f: Value<FnPtr<fn() -> ()>> =
                    Rc::new(RefCell::new((FnPtr::<fn() -> ()>::new(f_8))));
                bind_11(___f.as_pointer())
            }),
            1_(),
        )
    });
    ({
        test_2(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_12(___f.as_pointer())
            }),
            4_(),
        )
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_12(___f.as_pointer())
            }),
        ));
        test_const_6(_f.as_pointer(), 5_())
    });
    ({ test_3(unimplemented!(), 1_()) });
    ({ test_4(unimplemented!(), 1_()) });
    ({ test_5(unimplemented!(), 4_()) });
    ({
        let _f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_const_7(_f.as_pointer(), 5_())
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_all_9() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub trait A_int_0Impl {
    fn operator_call_1(&self) -> ();
    fn operator_call_2(&self) -> ();
}
impl A_int_0Impl for Ptr<A_int_0> {
    fn operator_call_1(&self) -> () {
        return 4_();
    }
    fn operator_call_2(&self) -> () {
        return 5_();
    }
}
pub fn __cpp2rust_init_globals() {}
