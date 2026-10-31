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
pub struct test_macros_detail_is_same_std_chrono_weekday__std_chrono_weekday_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_chrono_weekday_ref__std_chrono_weekday_ref_ {}
pub fn euclidian_subtraction_2(mut lhs: u32, mut rhs: u32) -> u32 {
    let mut modulus: u32 = ((6_u32 as u32).wrapping_sub((0_u32 as u32)) as u32).wrapping_add(1_u32);
    let ret: Value<u32> = Rc::new(RefCell::new((lhs).wrapping_sub(rhs)));
    if ((*ret.borrow()) < 0_u32) {
        (*ret.borrow_mut()) = { (*ret.borrow()).wrapping_add(modulus) };
    }
    if ((*ret.borrow()) > 6_u32) {
        (*ret.borrow_mut()) = { (*ret.borrow()).wrapping_add(modulus) };
    }
    return (*ret.borrow());
}
pub fn test_3() -> bool {
    let mut i: u32 = 0_u32;
    'loop_: while (i <= 6_u32) {
        let wd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == ({ euclidian_subtraction_2(i, 1_u32,) })));
        assert!((unimplemented!() == ({ euclidian_subtraction_2(i, 1_u32,) })));
        assert!((unimplemented!() == ({ euclidian_subtraction_2(i, 2_u32,) })));
        i.prefix_inc();
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ test_3() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
