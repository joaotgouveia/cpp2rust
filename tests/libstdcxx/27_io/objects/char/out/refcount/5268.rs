extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test04_0() {
    let b1: Value<std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_stringbuf_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    ({
        ((*libcc2rs::cout().upgrade().deref()) as std_basic_ios_char__std_char_traits_char__)
            .rdbuf_18((b1.as_pointer()))
    });
    {
        let __s = libcc2rs::cout();
        let _ = Ptr::<i8>::from_string_literal(b"hello\n").with_c_str(|__cs| {
            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
        });
        __s
    };
    ({
        ((*libcc2rs::cout().upgrade().deref()) as std_basic_ios_char__std_char_traits_char__)
            .rdbuf_18(Ptr::<::std::fs::File>::null())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test04_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
