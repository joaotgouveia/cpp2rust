extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub fn test_1() -> bool {
    {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([3, 2, 1])));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut b: Ptr<i32> = unimplemented!();
        let mut e: Ptr<i32> = unimplemented!();
        assert!(((*il.borrow()).len() == 3_usize));
        assert!(({ ((((e).clone() - (b).clone()) as i64) as usize) } == { (*il.borrow()).len() }));
        assert!(((b.postfix_inc().read()) == 3));
        assert!(((b.postfix_inc().read()) == 2));
        assert!(((b.postfix_inc().read()) == 1));
    }
    {
        let il: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!((unimplemented!() == unimplemented!()));
        assert!((unimplemented!() == unimplemented!()));
        let cil: Ptr<Vec<i32>> = il.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(({ unimplemented!() } == { unimplemented!() }));
        assert!(({ unimplemented!() } == { unimplemented!() }));
    }
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
