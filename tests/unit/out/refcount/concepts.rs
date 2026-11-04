extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(true, "Small<int>");
const _: () = assert!(true, "sizeof(int) == 4");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Sized {}
impl CppMove for Sized {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn is_small_0() -> bool {
    return true;
}
pub fn is_small_1() -> bool {
    return false;
}
pub fn has_size_2() -> bool {
    return true;
}
pub fn has_size_3() -> bool {
    return false;
}
pub fn pick_4(mut x: i32) -> i32 {
    {
        return 1;
    }
    return 2;
}
pub fn pick_5(mut x: i64) -> i32 {
    {}
    return 2;
}
pub fn pick_6(mut x: f32) -> i32 {
    {}
    return 2;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    const _: () = assert!(true, "!Small<long>");;
    ({ cpp2rust_assert_fail_7(({ is_small_0() })) });
    ({ cpp2rust_assert_fail_7(!({ is_small_1() })) });
    ({ cpp2rust_assert_fail_7(true) });
    ({ cpp2rust_assert_fail_7(!(false)) });
    ({ cpp2rust_assert_fail_7(({ has_size_2() })) });
    ({ cpp2rust_assert_fail_7(!({ has_size_3() })) });
    ({ cpp2rust_assert_fail_7((({ pick_4(1) }) == 1)) });
    ({ cpp2rust_assert_fail_7((({ pick_5(1_i64) }) == 2)) });
    ({ cpp2rust_assert_fail_7((({ pick_6(1_f32) }) == 2)) });
    return 0;
}
pub trait SizedImpl {
    fn size(&self) -> i32;
}
impl SizedImpl for Ptr<Sized> {
    fn size(&self) -> i32 {
        return 4;
    }
}
pub fn __cpp2rust_init_globals() {}
