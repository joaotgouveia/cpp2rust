extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[X]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[XY]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^X]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^XY]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[\\S]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^\\S]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[\\s]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^\\s]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[\\s\\S]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^Y\\s]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^X\\s]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[\\w]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[\\w]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"_"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^\\w]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^\\w]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"_"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[\\W]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[\\W]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"_"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^\\W]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"X"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"[^\\W]") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"_"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"^ab[\\d\\D]cd") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"abZcd"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"^ab[\\d\\D]cd") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"ab5cd"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"^ab[\\D]cd") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"abZcd"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"^ab\\Dcd") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"abZcd"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"^ab[\\d]cd") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"ab5cd"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"^ab\\dcd") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"ab5cd"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"^ab\\dcd") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"abZcd"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        !({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"^ab\\Dcd") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"ab5cd"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"_(\\s|\\S)+_") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"_xyz_"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    assert!(
        ({
            let ___re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"_[\\s\\S]+_") },
                    None,
                )),
            );
            regex_match_0(
                Ptr::<i8>::from_string_literal(b"_xyz_"),
                ___re.as_pointer(),
                None,
            )
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
