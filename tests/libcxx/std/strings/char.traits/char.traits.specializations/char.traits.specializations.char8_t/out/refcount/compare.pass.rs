extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constexpr_0() -> bool {
    return ((({
        std_char_traits_char8_t_::compare(
            Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
            Ptr::<u8>::from_string_literal(&[50 as u8, 50 as u8, 51 as u8, 0 as u8]),
            3_usize,
        )
    }) < 0)
        && (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[50 as u8, 50 as u8, 51 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                3_usize,
            )
        }) > 0))
        && (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                3_usize,
            )
        }) == 0);
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
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[0 as u8]),
                Ptr::<u8>::from_string_literal(&[0 as u8]),
                0_usize,
            )
        }) == 0)
    );
    assert!(
        (({ std_char_traits_char8_t_::compare(Ptr::<u8>::null(), Ptr::<u8>::null(), 0_usize,) })
            == 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 0 as u8]),
                1_usize,
            )
        }) == 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[50 as u8, 0 as u8]),
                1_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[50 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 0 as u8]),
                1_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 0 as u8]),
                2_usize,
            )
        }) == 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 51 as u8, 0 as u8]),
                2_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[50 as u8, 50 as u8, 0 as u8]),
                2_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 51 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 0 as u8]),
                2_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[50 as u8, 50 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 0 as u8]),
                2_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                3_usize,
            )
        }) == 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[50 as u8, 50 as u8, 51 as u8, 0 as u8]),
                3_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 51 as u8, 51 as u8, 0 as u8]),
                3_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 52 as u8, 0 as u8]),
                3_usize,
            )
        }) < 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[50 as u8, 50 as u8, 51 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                3_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 51 as u8, 51 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                3_usize,
            )
        }) > 0)
    );
    assert!(
        (({
            std_char_traits_char8_t_::compare(
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 52 as u8, 0 as u8]),
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]),
                3_usize,
            )
        }) > 0)
    );
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
