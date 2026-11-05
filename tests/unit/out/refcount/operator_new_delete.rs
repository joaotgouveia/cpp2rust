extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: Ptr<i32> =
        ({ operator_new_0((::std::mem::size_of::<i32>() as u64)) }).reinterpret_cast::<i32>();
    a.write(42);
    ({ cpp2rust_assert_fail_1(((a.read()) == 42)) });
    ({ operator_delete_2((a).to_any()) });
    let mut arr: Ptr<i32> = ({
        operator_new_arr_3(((::std::mem::size_of::<i32>() as u64).wrapping_mul(2_u64) as u64))
    })
    .reinterpret_cast::<i32>();
    elem!(arr, 0).write(0);
    elem!(arr, 1).write(1);
    ({ cpp2rust_assert_fail_1((((elem!(arr, 0).read()) + (elem!(arr, 1).read())) == 1)) });
    ({ operator_delete_arr_4((arr).to_any()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
