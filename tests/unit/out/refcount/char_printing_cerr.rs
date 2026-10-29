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
    let mut vec_: std_vector_unsigned_char__std_allocator_unsigned_char__ =
        std_vector_unsigned_char__std_allocator_unsigned_char__::new_1({ unimplemented!() }, None);
    let mut i: () = 27_();
    let str: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bar.") },
                None,
            ),
        ));
    write!(cerr_0.with(|v| v.as_pointer()), "{:} a", i,);
    cerr_0.with(|v| v.as_pointer()).write_all(
        &([
            (&[vec_[0_()] as u8] as &[u8]),
            (&[vec_[1_()] as u8] as &[u8]),
            (&[('o' as ()) as u8] as &[u8]),
        ]
        .concat()),
    );
    write!(
        cerr_0.with(|v| v.as_pointer()),
        "{:}\n0x{:x}",
        (*str.borrow_mut()),
        27_(),
    );
    cerr_0.with(|v| v.as_pointer()).write_all(
        &([
            (b" a\xc3\xa7ordas?" as &[u8]),
            (&[('\n' as ()) as u8] as &[u8]),
            (b"Sim, 0x" as &[u8]),
        ]
        .concat()),
    );
    write!(cerr_0.with(|v| v.as_pointer()), "{:x}.\n", i,);
    write!(cerr_0.with(|v| v.as_pointer()), "Hello, World!\n",);
    cerr_0.with(|v| v.as_pointer()).write_all(
        &([
            (&[vec_[0_()] as u8] as &[u8]),
            (&[('\n' as ()) as u8] as &[u8]),
            (&[vec_[1_()] as u8] as &[u8]),
            (&[('\n' as ()) as u8] as &[u8]),
        ]
        .concat()),
    );
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
