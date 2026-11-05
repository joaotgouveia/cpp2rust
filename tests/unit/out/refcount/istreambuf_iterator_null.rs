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
    let first: Value<std_istreambuf_iterator_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_istreambuf_iterator_char__std_char_traits_char__::new_1({
            Ptr::<std_basic_streambuf_char__std_char_traits_char__>::null()
        }),
    ));
    let last: Value<std_istreambuf_iterator_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_istreambuf_iterator_char__std_char_traits_char__::new_1({
            Ptr::<std_basic_streambuf_char__std_char_traits_char__>::null()
        }),
    ));
    let s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (*first.borrow()).clone() },
                { (*last.borrow()).clone() },
                None,
            ),
        ));
    ({ cpp2rust_assert_fail_0(({ (*s.borrow()).empty() })) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
