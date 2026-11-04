extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b""),
                Ptr::<i8>::from_string_literal(b""),
                0_usize,
            )
        }) == 0)
    );
    assert!(
        (({ std_char_traits_char_::compare(Ptr::<i8>::null(), Ptr::<i8>::null(), 0_usize,) }) == 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"1"),
                Ptr::<i8>::from_string_literal(b"1"),
                1_usize,
            )
        }) == 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"1"),
                Ptr::<i8>::from_string_literal(b"2"),
                1_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"2"),
                Ptr::<i8>::from_string_literal(b"1"),
                1_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"12"),
                Ptr::<i8>::from_string_literal(b"12"),
                2_usize,
            )
        }) == 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"12"),
                Ptr::<i8>::from_string_literal(b"13"),
                2_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"12"),
                Ptr::<i8>::from_string_literal(b"22"),
                2_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"13"),
                Ptr::<i8>::from_string_literal(b"12"),
                2_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"22"),
                Ptr::<i8>::from_string_literal(b"12"),
                2_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"123"),
                Ptr::<i8>::from_string_literal(b"123"),
                3_usize,
            )
        }) == 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"123"),
                Ptr::<i8>::from_string_literal(b"223"),
                3_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"123"),
                Ptr::<i8>::from_string_literal(b"133"),
                3_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"123"),
                Ptr::<i8>::from_string_literal(b"124"),
                3_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"223"),
                Ptr::<i8>::from_string_literal(b"123"),
                3_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"133"),
                Ptr::<i8>::from_string_literal(b"123"),
                3_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char_::compare(
                Ptr::<i8>::from_string_literal(b"124"),
                Ptr::<i8>::from_string_literal(b"123"),
                3_usize,
            )
        }) > 0)
    );
    {
        let a: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([(-1_i32 as i8), 0_i8])));
        let b: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 0_i8])));
        assert!(
            (({
                std_char_traits_char_::compare(
                    (a.as_pointer() as Ptr<i8>),
                    (b.as_pointer() as Ptr<i8>),
                    1_usize,
                )
            }) > 0)
        );
    }
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
