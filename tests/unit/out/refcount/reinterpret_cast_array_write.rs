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
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8, 4_u8])));
    let mut words: Ptr<u16> = (arr.as_pointer() as Ptr<u8>).reinterpret_cast::<u16>();
    ({ cpp2rust_assert_fail_0((((elem!(words, 0).read()) as i32) == 513)) });
    ({ cpp2rust_assert_fail_0((((elem!(words, 1).read()) as i32) == 1027)) });
    elem!(words, 0).write(48042_u16);
    ({ cpp2rust_assert_fail_0((((*arr.borrow())[(0) as usize] as i32) == 170)) });
    ({ cpp2rust_assert_fail_0((((*arr.borrow())[(1) as usize] as i32) == 187)) });
    ({ cpp2rust_assert_fail_0((((*arr.borrow())[(2) as usize] as i32) == 3)) });
    ({ cpp2rust_assert_fail_0((((*arr.borrow())[(3) as usize] as i32) == 4)) });
    words = (arr.as_pointer() as Ptr<u8>)
        .offset((1) as isize)
        .reinterpret_cast::<u16>();
    ({ cpp2rust_assert_fail_0((((elem!(words, 0).read()) as i32) == 955)) });
    elem!(words, 0).write(0_u16);
    ({ cpp2rust_assert_fail_0((((*arr.borrow())[(0) as usize] as i32) == 170)) });
    ({ cpp2rust_assert_fail_0((((*arr.borrow())[(1) as usize] as i32) == 0)) });
    ({ cpp2rust_assert_fail_0((((*arr.borrow())[(2) as usize] as i32) == 0)) });
    ({ cpp2rust_assert_fail_0((((*arr.borrow())[(3) as usize] as i32) == 4)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
