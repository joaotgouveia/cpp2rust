extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
        let a: Value<std_any> = Rc::new(RefCell::new(<std_any>::default()));
        const _: () = assert!(true, "noexcept(swap(a, a))");;
    }
    {
        let a1: Value<std_any> = Rc::new(RefCell::new(1));
        let a2: Value<std_any> = Rc::new(RefCell::new(2));
        ({ swap_0(a1.as_pointer(), a2.as_pointer()) });
        ({ cpp2rust_assert_fail_1((({ any_cast_2(a1.as_pointer()) }) == 2)) });
        ({ cpp2rust_assert_fail_1((({ any_cast_2(a2.as_pointer()) }) == 1)) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
