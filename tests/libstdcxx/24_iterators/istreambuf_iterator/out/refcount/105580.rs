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
    let in_: Value<std_basic_istringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new({
            let __tmp_0: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"Hello, world").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            std_basic_istringstream_char__std_char_traits_char___std_allocator_char__::new_1(
                { __tmp_0.as_pointer() },
                None,
            )
        }));
    let mut it: Ptr<::std::fs::File> =
        std_istreambuf_iterator_char__std_char_traits_char__::new_1({ in_.as_pointer() });
    let mut end: Ptr<::std::fs::File> = Ptr::null();
    let mut ss: Vec<i8> = match ({ (it).clone() },) {
        (mut __tmp_1,) => {
            let mut __bytes: Vec<u8> = Vec::new();
            if !__tmp_1.is_null() {
                __tmp_1
                    .with_mut(|__f: &mut ::std::fs::File| {
                        ::std::io::Read::read_to_end(__f, &mut __bytes)
                    })
                    .expect("couldn't read the file");
            }
            __bytes.push(0);
            <i8>::from_byte_vec(__bytes)
        }
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
