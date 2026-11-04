extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0() -> i32 {
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
impl CppMove for A_int_0 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_all_1() -> bool {
    ({ test_2(({ bind_3(FnPtr::<fn() -> i32>::new(f_0)) }), 1) });
    ({
        test_2(
            ({
                let ___f: Value<FnPtr<fn() -> i32>> =
                    Rc::new(RefCell::new((FnPtr::<fn() -> i32>::new(f_0))));
                bind_4(___f.as_pointer())
            }),
            1,
        )
    });
    ({
        test_5(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_6(___f.as_pointer())
            }),
            4,
        )
    });
    ({
        let _f: Value<std__Bind_A_int_0____> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_6(___f.as_pointer())
            }),
        ));
        test_const_7(_f.as_pointer(), 5)
    });
    ({ test_8(({ bind_9(FnPtr::<fn() -> i32>::new(f_0)) }), 1) });
    ({
        test_8(
            ({
                let ___f: Value<FnPtr<fn() -> i32>> =
                    Rc::new(RefCell::new((FnPtr::<fn() -> i32>::new(f_0))));
                bind_10(___f.as_pointer())
            }),
            1,
        )
    });
    ({
        test_11(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_12(___f.as_pointer())
            }),
            4,
        )
    });
    ({
        let _f: Value<std__Bind_result_int__A_int_0____> = Rc::new(RefCell::new(
            ({
                let ___f: Value<A_int_0> = Rc::new(RefCell::new(<A_int_0>::default()));
                bind_12(___f.as_pointer())
            }),
        ));
        test_const_13(_f.as_pointer(), 5)
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
    ({ test_all_1() });
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
