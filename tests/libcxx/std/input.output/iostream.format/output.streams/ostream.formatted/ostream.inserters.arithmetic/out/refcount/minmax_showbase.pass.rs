extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut fmt: (), mut expected: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_1(mut fmt: (), mut expected: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_2(mut fmt: (), mut expected: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_3(mut fmt: (), mut expected: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_4(mut fmt: (), mut expected: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_5(mut fmt: (), mut expected: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_6(mut fmt: (), mut expected: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_7(mut fmt: (), mut expected: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let mut o: () = unimplemented!();
    let mut d: () = unimplemented!();
    let mut x: () = unimplemented!();
    ({ test_0(o, Ptr::<()>::from_string_literal(b"0100000")) });
    ({ test_0(d, Ptr::<()>::from_string_literal(b"-32768")) });
    ({ test_0(x, Ptr::<()>::from_string_literal(b"0x8000")) });
    ({ test_1(o, Ptr::<()>::from_string_literal(b"0177777")) });
    ({ test_1(d, Ptr::<()>::from_string_literal(b"65535")) });
    ({ test_1(x, Ptr::<()>::from_string_literal(b"0xffff")) });
    ({ test_2(o, Ptr::<()>::from_string_literal(b"020000000000")) });
    ({ test_2(d, Ptr::<()>::from_string_literal(b"-2147483648")) });
    ({ test_2(x, Ptr::<()>::from_string_literal(b"0x80000000")) });
    ({ test_3(o, Ptr::<()>::from_string_literal(b"037777777777")) });
    ({ test_3(d, Ptr::<()>::from_string_literal(b"4294967295")) });
    ({ test_3(x, Ptr::<()>::from_string_literal(b"0xffffffff")) });
    let mut long_is_32: () = unimplemented!();
    let mut long_is_64: () = unimplemented!();
    let mut long_long_is_64: () = unimplemented!();
    {
        {
            ({
                test_4(
                    o,
                    Ptr::<()>::from_string_literal(b"01000000000000000000000"),
                )
            });
            ({ test_4(d, Ptr::<()>::from_string_literal(b"-9223372036854775808")) });
            ({ test_4(x, Ptr::<()>::from_string_literal(b"0x8000000000000000")) });
            ({
                test_5(
                    o,
                    Ptr::<()>::from_string_literal(b"01777777777777777777777"),
                )
            });
            ({ test_5(d, Ptr::<()>::from_string_literal(b"18446744073709551615")) });
            ({ test_5(x, Ptr::<()>::from_string_literal(b"0xffffffffffffffff")) });
        }
    }
    {
        ({
            test_6(
                o,
                Ptr::<()>::from_string_literal(b"01000000000000000000000"),
            )
        });
        ({ test_6(d, Ptr::<()>::from_string_literal(b"-9223372036854775808")) });
        ({ test_6(x, Ptr::<()>::from_string_literal(b"0x8000000000000000")) });
        ({
            test_7(
                o,
                Ptr::<()>::from_string_literal(b"01777777777777777777777"),
            )
        });
        ({ test_7(d, Ptr::<()>::from_string_literal(b"18446744073709551615")) });
        ({ test_7(x, Ptr::<()>::from_string_literal(b"0xffffffffffffffff")) });
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
