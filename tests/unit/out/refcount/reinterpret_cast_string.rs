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
    let s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<i8>::from_string_literal(b"ABCD") },
                None,
            ),
        ));
    let mut bytes: Ptr<u8> = ({ (*s.borrow()).c_str() }).reinterpret_cast::<u8>();
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 0).read()) as i32) == (('A' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 1).read()) as i32) == (('B' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 2).read()) as i32) == (('C' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 3).read()) as i32) == (('D' as i8) as i32))) });
    ({ cpp2rust_assert_fail_0((((elem!(bytes, 4).read()) as i32) == 0)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
