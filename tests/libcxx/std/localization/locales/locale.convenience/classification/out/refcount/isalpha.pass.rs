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
    let l: Value<std_locale> = Rc::new(RefCell::new(<std_locale>::default()));
    ({ cpp2rust_assert_fail_0(!({ isalpha_1((' ' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(!({ isalpha_1(('<' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(!({ isalpha_1(('\x08' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(({ isalpha_1(('A' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(({ isalpha_1(('a' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(({ isalpha_1(('z' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(!({ isalpha_1(('3' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(!({ isalpha_1(('.' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(({ isalpha_1(('f' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(!({ isalpha_1(('9' as i8), l.as_pointer()) })) });
    ({ cpp2rust_assert_fail_0(!({ isalpha_1(('+' as i8), l.as_pointer()) })) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
