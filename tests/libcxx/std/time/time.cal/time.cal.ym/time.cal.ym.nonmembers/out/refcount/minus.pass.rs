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
pub struct test_macros_detail_is_same_std_chrono_year_month__std_chrono_year_month_ {}
impl CppMove for test_macros_detail_is_same_std_chrono_year_month__std_chrono_year_month_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_2629746__1____std_chrono_duration_long__std_ratio_2629746__1___
{}
impl CppMove for test_macros_detail_is_same_std_chrono_duration_long__std_ratio_2629746__1____std_chrono_duration_long__std_ratio_2629746__1___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub fn test_2() -> bool {
    {
        let ym: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 0;
        'loop_: while (i <= 10) {
            let ym1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == (1234 - i)));
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    {
        let ym: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 0;
        'loop_: while (i <= 10) {
            let ym1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == 1234));
            assert!(unimplemented!());
            i.prefix_inc();
        }
        let mut i: i32 = 12;
        'loop_: while (i <= 15) {
            let ym1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == 1233));
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    {
        let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 1;
        'loop_: while (i <= 12) {
            let mut j: i32 = 1;
            'loop_: while (j <= 12) {
                let diff: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                assert!((unimplemented!() == ((i - j) as ())));
                j.prefix_inc();
            }
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
