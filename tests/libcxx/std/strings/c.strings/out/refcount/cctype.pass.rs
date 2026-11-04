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
pub struct test_macros_detail_is_same_int__int_ {}
impl CppMove for test_macros_detail_is_same_int__int_ {
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ cpp2rust_assert_fail_1((({ isalnum_2((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1((({ isalpha_3((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1((({ isblank_4(((' ' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1(!(({ iscntrl_5(((' ' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1(!(({ isdigit_6((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1((({ isgraph_7((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1((({ islower_8((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1((({ isprint_9((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1(!(({ ispunct_10((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1(!(({ isspace_11((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1(!(({ isupper_12((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1((({ isxdigit_13((('a' as i8) as i32)) }) != 0)) });
    ({ cpp2rust_assert_fail_1((({ tolower_14((('A' as i8) as i32)) }) == (('a' as i8) as i32))) });
    ({ cpp2rust_assert_fail_1((({ toupper_15((('a' as i8) as i32)) }) == (('A' as i8) as i32))) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
