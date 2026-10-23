extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn f_1() {
    (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn g_2() -> () {
    (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
    return 0_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_void_0 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_int_0 {}
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
    ({ test_3(({ bind_4(FnPtr::<fn()>::new(f_1)) })) });
    ({
        test_5(
            ({
                let ___f: Value<FnPtr<fn()>> = Rc::new(RefCell::new((FnPtr::<fn()>::new(f_1))));
                bind_6(___f.as_pointer())
            }),
        )
    });
    ({
        test_7(
            ({
                let ___f: Value<A_void_0> = Rc::new(RefCell::new(<A_void_0>::default()));
                bind_8(___f.as_pointer())
            }),
        )
    });
    ({
        let _f: Value<std___bind_A_void_0_> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_void_0> = Rc::new(RefCell::new(<A_void_0>::default()));
                bind_8(___f.as_pointer())
            }),
        ));
        test_const_9(_f.as_pointer())
    });
    ({ test_10(({ bind_11(FnPtr::<fn()>::new(f_1)) })) });
    ({
        test_12(
            ({
                let ___f: Value<FnPtr<fn()>> = Rc::new(RefCell::new((FnPtr::<fn()>::new(f_1))));
                bind_13(___f.as_pointer())
            }),
        )
    });
    ({
        test_14(
            ({
                let ___f: Value<A_void_0> = Rc::new(RefCell::new(<A_void_0>::default()));
                bind_15(___f.as_pointer())
            }),
        )
    });
    ({
        let _f: Value<std___bind_r_void__A_void_0_> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_void_0> = Rc::new(RefCell::new(<A_void_0>::default()));
                bind_15(___f.as_pointer())
            }),
        ));
        test_const_16(_f.as_pointer())
    });
    ({ test_17(({ bind_18(FnPtr::<fn() -> ()>::new(g_2)) })) });
    ({
        test_19(
            ({
                let ___f: Value<FnPtr<fn() -> ()>> =
                    Rc::new(RefCell::new((FnPtr::<fn() -> ()>::new(g_2))));
                bind_20(___f.as_pointer())
            }),
        )
    });
    ({
        test_21(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_22(___f.as_pointer())
            }),
        )
    });
    ({
        let _f: Value<std___bind_r_void__A_int_0_> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_22(___f.as_pointer())
            }),
        ));
        test_const_23(_f.as_pointer())
    });
    return 0_();
}
pub trait A_int_0Impl {
    fn operator_call_1(&self) -> ();
    fn operator_call_2(&self) -> ();
}
impl A_int_0Impl for Ptr<A_int_0> {
    fn operator_call_1(&self) -> () {
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        return 4_();
    }
    fn operator_call_2(&self) -> () {
        count_0.with(|rc| *rc.borrow_mut() += 2_());
        return 5_();
    }
}
pub trait A_void_0Impl {
    fn operator_call_1(&self);
    fn operator_call_2(&self);
}
impl A_void_0Impl for Ptr<A_void_0> {
    fn operator_call_1(&self) {
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn operator_call_2(&self) {
        count_0.with(|rc| *rc.borrow_mut() += 2_());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
