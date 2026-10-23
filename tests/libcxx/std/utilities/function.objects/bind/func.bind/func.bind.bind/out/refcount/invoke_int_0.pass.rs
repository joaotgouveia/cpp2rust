extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0() -> () {
    return 1_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_int_0 {}
pub fn test_all_1() -> () {
    ({ test_2(({ bind_3(FnPtr::<fn() -> ()>::new(f_0)) }), 1_()) });
    ({
        test_4(
            ({
                let ___f: Value<FnPtr<fn() -> ()>> =
                    Rc::new(RefCell::new((FnPtr::<fn() -> ()>::new(f_0))));
                bind_5(___f.as_pointer())
            }),
            1_(),
        )
    });
    ({
        test_6(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_7(___f.as_pointer())
            }),
            4_(),
        )
    });
    ({
        let _f: Value<std___bind_A_int_0_> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_7(___f.as_pointer())
            }),
        ));
        test_const_8(_f.as_pointer(), 5_())
    });
    ({ test_9(({ bind_10(FnPtr::<fn() -> ()>::new(f_0)) }), 1_()) });
    ({
        test_11(
            ({
                let ___f: Value<FnPtr<fn() -> ()>> =
                    Rc::new(RefCell::new((FnPtr::<fn() -> ()>::new(f_0))));
                bind_12(___f.as_pointer())
            }),
            1_(),
        )
    });
    ({
        test_13(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_14(___f.as_pointer())
            }),
            4_(),
        )
    });
    ({
        let _f: Value<std___bind_r_int__A_int_0_> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_14(___f.as_pointer())
            }),
        ));
        test_const_15(_f.as_pointer(), 5_())
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
    ({ test_all_1() });
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
