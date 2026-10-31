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
pub struct test_macros_detail_is_same_std_chrono_year_month_day_last__std_chrono_year_month_day_last_
{}
pub fn test_1() -> bool {
    let December: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let ymdl: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 0;
        'loop_: while (i <= 10) {
            let ymdl1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == (1234 - i)));
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    {
        let ymdl: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: u32 = 0_u32;
        'loop_: while (i <= 10_u32) {
            let ymdl1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == 1234));
            assert!((unimplemented!() == (12_u32 as u32).wrapping_sub(i)));
            i.prefix_inc();
        }
        let mut i: u32 = 12_u32;
        'loop_: while (i <= 15_u32) {
            let ymdl1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == 1233));
            assert!((unimplemented!() == ((12_u32 as u32).wrapping_sub(i)).wrapping_add(12_u32)));
            i.prefix_inc();
        }
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
