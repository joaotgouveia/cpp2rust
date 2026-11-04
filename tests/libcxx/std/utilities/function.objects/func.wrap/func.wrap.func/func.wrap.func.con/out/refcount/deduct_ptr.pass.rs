extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct R {}
impl CppMove for R {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A1 {}
impl CppMove for A1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A2 {}
impl CppMove for A2 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A3 {}
impl CppMove for A3 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn f0_0() -> R {
    return <R>::default();
}
pub fn f1_1(mut _a0: A1) -> R {
    return <R>::default();
}
pub fn f2_2(mut _a0: A1, mut _a1: A2) -> R {
    return <R>::default();
}
pub fn f3_3(mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
    return <R>::default();
}
pub fn f4_4(_a0: Option<A1>) -> R {
    let mut _a0: A1 = _a0.unwrap_or_else(|| <A1>::default());
    return <R>::default();
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
    {
        let a: Value<std_function_R____> = Rc::new(RefCell::new(std_function_R____::new_1({
            FnPtr::<fn() -> R>::new(f0_0)
        })));
        const _: () = assert!(true, "");;
        let b: Value<std_function_R____> = Rc::new(RefCell::new((FnPtr::<fn() -> R>::new(f0_0))));
        const _: () = assert!(true, "");;
        let c: Value<std_function_R____> = Rc::new(RefCell::new(std_function_R____::new_1({
            FnPtr::<fn() -> R>::new(f0_0)
        })));
        const _: () = assert!(true, "");;
        let d: Value<std_function_R____> = Rc::new(RefCell::new((FnPtr::<fn() -> R>::new(f0_0))));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<std_function_R__A1__> = Rc::new(RefCell::new(std_function_R__A1__::new_1({
            FnPtr::<fn(A1) -> R>::new(f1_1)
        })));
        const _: () = assert!(true, "");;
        let b: Value<std_function_R__A1__> =
            Rc::new(RefCell::new((FnPtr::<fn(A1) -> R>::new(f1_1))));
        const _: () = assert!(true, "");;
        let c: Value<std_function_R__A1__> = Rc::new(RefCell::new(std_function_R__A1__::new_1({
            FnPtr::<fn(A1) -> R>::new(f1_1)
        })));
        const _: () = assert!(true, "");;
        let d: Value<std_function_R__A1__> =
            Rc::new(RefCell::new((FnPtr::<fn(A1) -> R>::new(f1_1))));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<std_function_R__A1__A2__> =
            Rc::new(RefCell::new(std_function_R__A1__A2__::new_1({
                FnPtr::<fn(A1, A2) -> R>::new(f2_2)
            })));
        const _: () = assert!(true, "");;
        let b: Value<std_function_R__A1__A2__> =
            Rc::new(RefCell::new((FnPtr::<fn(A1, A2) -> R>::new(f2_2))));
        const _: () = assert!(true, "");;
        let c: Value<std_function_R__A1__A2__> =
            Rc::new(RefCell::new(std_function_R__A1__A2__::new_1({
                FnPtr::<fn(A1, A2) -> R>::new(f2_2)
            })));
        const _: () = assert!(true, "");;
        let d: Value<std_function_R__A1__A2__> =
            Rc::new(RefCell::new((FnPtr::<fn(A1, A2) -> R>::new(f2_2))));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<std_function_R__A1__A2__A3__> =
            Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_1({
                FnPtr::<fn(A1, A2, A3) -> R>::new(f3_3)
            })));
        const _: () = assert!(true, "");;
        let b: Value<std_function_R__A1__A2__A3__> =
            Rc::new(RefCell::new((FnPtr::<fn(A1, A2, A3) -> R>::new(f3_3))));
        const _: () = assert!(true, "");;
        let c: Value<std_function_R__A1__A2__A3__> =
            Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_1({
                FnPtr::<fn(A1, A2, A3) -> R>::new(f3_3)
            })));
        const _: () = assert!(true, "");;
        let d: Value<std_function_R__A1__A2__A3__> =
            Rc::new(RefCell::new((FnPtr::<fn(A1, A2, A3) -> R>::new(f3_3))));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<std_function_R__A1__> = Rc::new(RefCell::new(std_function_R__A1__::new_1({
            FnPtr::<fn(A1) -> R>::new(f4_4)
        })));
        const _: () = assert!(true, "");;
        let b: Value<std_function_R__A1__> =
            Rc::new(RefCell::new((FnPtr::<fn(A1) -> R>::new(f4_4))));
        const _: () = assert!(true, "");;
        let c: Value<std_function_R__A1__> = Rc::new(RefCell::new(std_function_R__A1__::new_1({
            FnPtr::<fn(A1) -> R>::new(f4_4)
        })));
        const _: () = assert!(true, "");;
        let d: Value<std_function_R__A1__> =
            Rc::new(RefCell::new((FnPtr::<fn(A1) -> R>::new(f4_4))));
        const _: () = assert!(true, "");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
