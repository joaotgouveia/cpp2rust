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
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_InTemplate_true___InTemplate_true__ {}
impl CppMove for test_macros_detail_is_same_InTemplate_true___InTemplate_true__ {
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
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut p: bool = true;
        ({ cpp2rust_assert_fail_2(p) });
    }
    {
        const _: () = assert!(true, "std::is_constant_evaluated()");;
        let mut p: bool = ({ is_constant_evaluated_3() });
        ({ cpp2rust_assert_fail_2(!(p)) });
        const _: () = assert!(true, "");;
        ({ cpp2rust_assert_fail_2((local_static_4.with(|rc| *rc.borrow()) == 42)) });
    }
    return 0;
}
thread_local!(
    static local_static_4: Value<i32> = Rc::new(RefCell::new(if ({ is_constant_evaluated_3() }) {
        42
    } else {
        -1_i32
    }));
);
pub fn __cpp2rust_init_globals() {}
