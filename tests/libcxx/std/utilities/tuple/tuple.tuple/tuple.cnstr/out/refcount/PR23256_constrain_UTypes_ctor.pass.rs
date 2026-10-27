extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct UnconstrainedCtor {
    #[offset(0)]
    pub value_: (),
}
impl UnconstrainedCtor {
    pub fn new() -> Self {
        Self { value_: 0_() }
    }
}
impl Default for UnconstrainedCtor {
    fn default() -> Self {
        { UnconstrainedCtor::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct ExplicitUnconstrainedCtor {
    #[offset(0)]
    pub value_: (),
}
impl ExplicitUnconstrainedCtor {
    pub fn new() -> Self {
        Self { value_: 0_() }
    }
}
impl Default for ExplicitUnconstrainedCtor {
    fn default() -> Self {
        { ExplicitUnconstrainedCtor::new() }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        const _: () = assert!(true, "std::is_copy_constructible<std::tuple<A>>::value");;
        const _: () = assert!(true, "std::is_move_constructible<std::tuple<A>>::value");;
        const _: () = assert!(
            true,
            "std::is_copy_constructible<std::tuple<ExplicitA>>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_move_constructible<std::tuple<ExplicitA>>::value"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::is_constructible<\n            std::tuple<A>,\n            std::allocator_arg_t, std::allocator<int>,\n            std::tuple<A> const&\n        >::value"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible<\n            std::tuple<A>,\n            std::allocator_arg_t, std::allocator<int>,\n            std::tuple<A> &&\n        >::value"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible<\n            std::tuple<ExplicitA>,\n            std::allocator_arg_t, std::allocator<int>,\n            std::tuple<ExplicitA> const&\n        >::value"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible<\n            std::tuple<ExplicitA>,\n            std::allocator_arg_t, std::allocator<int>,\n            std::tuple<ExplicitA> &&\n        >::value"
        );;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t.borrow_mut()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t2.borrow_mut()));
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
