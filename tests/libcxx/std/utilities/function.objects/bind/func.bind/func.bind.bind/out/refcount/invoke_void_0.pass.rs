extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn f_1() {
    (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn g_2() -> i32 {
    (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
    return 0;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_void_0 {}
impl Callable0<()> for A_void_0 {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            A_void_0Impl::operator_call_1(__it)
        })
    }
}
impl CppMove for A_void_0 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A_int_0 {}
impl Callable0<i32> for A_int_0 {
    fn call(&self) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| A_int_0Impl::operator_call_1(__it))
    }
}
impl CppMove for A_int_0 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    ({ test_3(({ bind_4(FnPtr::<fn()>::new(f_1)) })) });
    ({
        test_3(
            ({
                let ___f: Value<FnPtr<fn()>> = Rc::new(RefCell::new((FnPtr::<fn()>::new(f_1))));
                bind_5(___f.as_pointer())
            }),
        )
    });
    ({
        test_6(
            ({
                let ___f: Value<A_void_0> = Rc::new(RefCell::new(<A_void_0>::default()));
                bind_7(___f.as_pointer())
            }),
        )
    });
    ({
        let _f: Value<std__Bind_A_void_0____> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_void_0> = Rc::new(RefCell::new(<A_void_0>::default()));
                bind_7(___f.as_pointer())
            }),
        ));
        test_const_8(_f.as_pointer())
    });
    ({ test_9(({ bind_10(FnPtr::<fn()>::new(f_1)) })) });
    ({
        test_9(
            ({
                let ___f: Value<FnPtr<fn()>> = Rc::new(RefCell::new((FnPtr::<fn()>::new(f_1))));
                bind_11(___f.as_pointer())
            }),
        )
    });
    ({
        test_12(
            ({
                let ___f: Value<A_void_0> = Rc::new(RefCell::new(<A_void_0>::default()));
                bind_13(___f.as_pointer())
            }),
        )
    });
    ({
        let _f: Value<std__Bind_result_void__A_void_0____> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_void_0> = Rc::new(RefCell::new(<A_void_0>::default()));
                bind_13(___f.as_pointer())
            }),
        ));
        test_const_14(_f.as_pointer())
    });
    ({ test_15(({ bind_16(FnPtr::<fn() -> i32>::new(g_2)) })) });
    ({
        test_15(
            ({
                let ___f: Value<FnPtr<fn() -> i32>> =
                    Rc::new(RefCell::new((FnPtr::<fn() -> i32>::new(g_2))));
                bind_17(___f.as_pointer())
            }),
        )
    });
    ({
        test_18(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_19(___f.as_pointer())
            }),
        )
    });
    ({
        let _f: Value<std__Bind_result_void__A_int_0____> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_19(___f.as_pointer())
            }),
        ));
        test_const_20(_f.as_pointer())
    });
    return 0;
}
pub trait A_int_0Impl {
    fn operator_call_1(&self) -> i32;
    fn operator_call_2(&self) -> i32;
}
impl A_int_0Impl for Ptr<A_int_0> {
    fn operator_call_1(&self) -> i32 {
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        return 4;
    }
    fn operator_call_2(&self) -> i32 {
        count_0.with(|rc| *rc.borrow_mut() += 2);
        return 5;
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
        count_0.with(|rc| *rc.borrow_mut() += 2);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
