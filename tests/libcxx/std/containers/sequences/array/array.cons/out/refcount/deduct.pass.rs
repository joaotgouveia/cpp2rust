extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> bool {
    {
        let arr: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
        const _: () = assert!(true, "std::is_same_v<decltype(arr), std::array<int, 3>>");;
        assert!(((arr.decay().offset(0_usize as isize).read()) == 1));
        assert!(((arr.decay().offset(1_usize as isize).read()) == 2));
        assert!(((arr.decay().offset(2_usize as isize).read()) == 3));
    }
    {
        let mut l1: i64 = 42_i64;
        let arr: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::from([1_i64, 4_i64, 9_i64, l1])));
        const _: () = assert!(true, "std::is_same_v<decltype(arr)::value_type, long>");;
        const _: () = assert!(true, "arr.size() == 4");;
        assert!(((arr.decay().offset(0_usize as isize).read()) == 1_i64));
        assert!(((arr.decay().offset(1_usize as isize).read()) == 4_i64));
        assert!(((arr.decay().offset(2_usize as isize).read()) == 9_i64));
        assert!(((arr.decay().offset(3_usize as isize).read()) == l1));
    }
    {
        let source: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([4_f64, 5_f64])));
        let arr: Value<Vec<f64>> = Rc::new(RefCell::new((*source.borrow()).clone()));
        const _: () = assert!(true, "std::is_same_v<decltype(arr), decltype(source)>");;
        const _: () = assert!(true, "std::is_same_v<decltype(arr), std::array<double, 2>>");;
        assert!(((arr.decay().offset(0_usize as isize).read()) == 4_f64));
        assert!(((arr.decay().offset(1_usize as isize).read()) == 5_f64));
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
    ({ tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
