extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(true, "has_size<std::ranges::repeat_view<int, int>>");
const _: () = assert!(!(false), "!has_size<std::ranges::repeat_view<int>>");
const _: () = assert!(
    !(false),
    "!has_size<std::ranges::repeat_view<int, std::unreachable_sentinel_t>>"
);
pub fn test_0() -> bool {
    {
        let rv: Value<std_ranges_repeat_view_int__int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(10));
            std_ranges_repeat_view_int__int_::new_1({ __tmp_0.as_pointer() }, { 20 })
        }));
        assert!(({ (({ (*rv.borrow()).size() }) == 20_u32) }));
    }
    {
        let mut int_max: i32 = 2147483647;
        let rv: Value<std_ranges_repeat_view_int__int_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(10));
            std_ranges_repeat_view_int__int_::new_1({ __tmp_1.as_pointer() }, { int_max })
        }));
        assert!(({ (({ (*rv.borrow()).size() }) == (int_max as u32)) }));
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
