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
fn main_0() -> () {
    let arr: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"foo") },
                None,
            ),
        ));
    (*arr.borrow_mut())[1_()] = ('b' as ());
    let mut p: Ptr<()> = ({ (*arr.borrow()).c_str() }).offset((1_()) as isize);
    unimplemented!();
    unimplemented!();
    let mut names: [Ptr<()>; 1] = [({ (*arr.borrow()).data_159() })];
    let copy: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (names[(0_()) as usize]).clone() },
                None,
            ),
        ));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
