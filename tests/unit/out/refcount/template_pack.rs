extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0(mut args_0: i32, mut args_1: i32, mut args_2: i32) -> i32 {
    return (args_0 + (args_1 + (args_2 + 0)));
}
pub fn sum_1(mut args_0: i32, mut args_1: i32) -> i32 {
    return (args_0 + (args_1 + 0));
}
pub fn sum_2(mut args: i32) -> i32 {
    return (args + 0);
}
pub fn first_3(mut x: i32, mut args_1: i32, mut args_2: i32) -> i32 {
    return (x + ({ sum_1(args_1, args_2) }));
}
pub fn first_4(mut x: i32, mut args: i32) -> i32 {
    return (x + ({ sum_2(args) }));
}
pub fn sizeof_pack_5() -> i32 {
    return (0 as i32);
}
pub fn sizeof_pack_6(mut args: i32) -> i32 {
    return (1 as i32);
}
pub fn sizeof_pack_7(mut args_0: i32, mut args_1: i32) -> i32 {
    return (2 as i32);
}
pub fn sizeof_pack_8(mut args_0: i32, mut args_1: i32, mut args_2: i32) -> i32 {
    return (3 as i32);
}
pub fn sizeof_pack_9(mut args_0: i32, mut args_1: i32, mut args_2: i32, mut args_3: i32) -> i32 {
    return (4 as i32);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ cpp2rust_assert_fail_10((({ sum_0(1, 2, 3) }) == 6)) });
    ({ cpp2rust_assert_fail_10((({ sum_1(4, 5) }) == 9)) });
    ({ cpp2rust_assert_fail_10((({ first_3(10, 1, 2) }) == 13)) });
    ({ cpp2rust_assert_fail_10((({ first_4(10, 0) }) == 10)) });
    ({ cpp2rust_assert_fail_10((({ sizeof_pack_5() }) == 0)) });
    ({ cpp2rust_assert_fail_10((({ sizeof_pack_6(1) }) == 1)) });
    ({ cpp2rust_assert_fail_10((({ sizeof_pack_7(1, 2) }) == 2)) });
    ({ cpp2rust_assert_fail_10((({ sizeof_pack_8(1, 2, 3) }) == 3)) });
    ({ cpp2rust_assert_fail_10((({ sizeof_pack_9(1, 2, 3, 4) }) == 4)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
