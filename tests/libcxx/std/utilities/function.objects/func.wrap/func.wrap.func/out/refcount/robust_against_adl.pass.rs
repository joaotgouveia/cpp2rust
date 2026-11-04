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
pub struct Callable_Holder_Incomplete__ {}
impl Callable0<()> for Callable_Holder_Incomplete__ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            Callable_Holder_Incomplete__Impl::operator_call(__it)
        })
    }
}
impl CppMove for Callable_Holder_Incomplete__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn no_args_0() -> Ptr<Holder_Incomplete_> {
    return Ptr::<Holder_Incomplete_>::null();
}
pub fn one_arg_1(p: Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn two_args_2(
    p: Ptr<Holder_Incomplete_>,
    mut _a1: Ptr<Holder_Incomplete_>,
) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn three_args_3(
    p: Ptr<Holder_Incomplete_>,
    mut _a1: Ptr<Holder_Incomplete_>,
    mut _a2: Ptr<Holder_Incomplete_>,
) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn four_args_4(
    p: Ptr<Holder_Incomplete_>,
    mut _a1: Ptr<Holder_Incomplete_>,
    mut _a2: Ptr<Holder_Incomplete_>,
    mut _a3: Ptr<Holder_Incomplete_>,
) -> Ptr<Holder_Incomplete_> {
    let p: Value<Ptr<Holder_Incomplete_>> = Rc::new(RefCell::new(p));
    return (*p.borrow()).clone();
}
pub fn one_arg_void_5(mut _a0: Ptr<Holder_Incomplete_>) {}
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
    let mut x: Ptr<Holder_Incomplete_> = Ptr::<Holder_Incomplete_>::null();
    let f: Value<std_function_Holder_Incomplete__ptr___> =
        Rc::new(RefCell::new(std_function_Holder_Incomplete__ptr___::new_1(
            { FnPtr::<fn() -> Ptr<Holder_Incomplete_>>::new(no_args_0) },
        )));
    ({ (*f.borrow_mut())() });
    let g: Value<std_function_Holder_Incomplete__ptr_Holder_Incomplete__ptr__> =
        Rc::new(RefCell::new(
            std_function_Holder_Incomplete__ptr_Holder_Incomplete__ptr__::new_1({
                FnPtr::<fn(Ptr<Holder_Incomplete_>) -> Ptr<Holder_Incomplete_>>::new(one_arg_1)
            }),
        ));
    ({ (*g.borrow_mut())((x).clone()) });
    let h: Value<std_function_void__Holder_Incomplete__ptr__> = Rc::new(RefCell::new(
        std_function_void__Holder_Incomplete__ptr__::new_1({
            FnPtr::<fn(Ptr<Holder_Incomplete_>)>::new(one_arg_void_5)
        }),
    ));
    ({ (*h.borrow_mut())((x).clone()) });
    let i: Value<std_function_void____> =
        Rc::new(RefCell::new(<Callable_Holder_Incomplete__>::default()));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Holder_Incomplete_;
pub trait Callable_Holder_Incomplete__Impl {
    fn operator_call(&self);
}
impl Callable_Holder_Incomplete__Impl for Ptr<Callable_Holder_Incomplete__> {
    fn operator_call(&self) {}
}
pub fn __cpp2rust_init_globals() {}
