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
                { 512_u64 },
                { ('1' as i8) },
                None,
            ),
        ));
    (*s.borrow_mut())[1_u64] = ('.' as i8);
    let d: Value<f64> = Rc::new(RefCell::new(0_f64));
    ({
        let ___first: Ptr<i8> = ({ (*s.borrow()).data_168() });
        let ___last: Ptr<i8> =
            ({ (*s.borrow()).data_168() }).offset(({ (*s.borrow()).size() }) as isize);
        from_chars_0(___first, ___last, d.as_pointer(), None)
    });
    0
}
pub fn __cpp2rust_init_globals() {}
