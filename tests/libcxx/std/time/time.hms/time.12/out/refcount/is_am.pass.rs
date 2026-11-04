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
pub struct test_macros_detail_is_same_bool__bool_ {}
impl CppMove for test_macros_detail_is_same_bool__bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::chrono::is_am(hours( 0))");;
    const _: () = assert!(true, "std::chrono::is_am(hours(11))");;
    const _: () = assert!(true, "!std::chrono::is_am(hours(12))");;
    const _: () = assert!(true, "!std::chrono::is_am(hours(23))");;
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 12) {
        assert!(
            ({
                let ___h: Value<std_chrono_duration_long__std_ratio_3600__1__> =
                    Rc::new(RefCell::new(
                        std_chrono_duration_long__std_ratio_3600__1__::new_1({ i.as_pointer() }),
                    ));
                is_am_1(___h.as_pointer())
            })
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(12));
    'loop_: while ((*i.borrow()) < 24) {
        assert!(
            !({
                let ___h: Value<std_chrono_duration_long__std_ratio_3600__1__> =
                    Rc::new(RefCell::new(
                        std_chrono_duration_long__std_ratio_3600__1__::new_1({ i.as_pointer() }),
                    ));
                is_am_1(___h.as_pointer())
            })
        );
        (*i.borrow_mut()).prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
