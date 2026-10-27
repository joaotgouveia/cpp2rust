extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_print_0() {
    let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut file: *mut ::libc::FILE = ({
        let ___s: () = (unimplemented!() as Ptr<()>).to_any();
        let ___len: () = unimplemented!();
        fmemopen_1(___s, ___len, Ptr::<()>::from_string_literal(b"wb"))
    });
    unimplemented!();
    unimplemented!();
    let mut pos: () = libc::ftell(file) as i64;
    libc::fclose(file);
    unimplemented!();
    unimplemented!();
}
pub fn test_println_2() {
    let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut file: *mut ::libc::FILE = ({
        let ___s: () = (unimplemented!() as Ptr<()>).to_any();
        let ___len: () = unimplemented!();
        fmemopen_1(___s, ___len, Ptr::<()>::from_string_literal(b"wb"))
    });
    unimplemented!();
    unimplemented!();
    let mut pos: () = libc::ftell(file) as i64;
    libc::fclose(file);
    unimplemented!();
    unimplemented!();
}
pub fn test_println_blank_line_3() {
    let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut file: *mut ::libc::FILE = ({
        let ___s: () = (unimplemented!() as Ptr<()>).to_any();
        let ___len: () = unimplemented!();
        fmemopen_1(___s, ___len, Ptr::<()>::from_string_literal(b"wb"))
    });
    unimplemented!();
    unimplemented!();
    let mut pos: () = libc::ftell(file) as i64;
    libc::fclose(file);
    unimplemented!();
    unimplemented!();
}
pub fn test_vprint_unicode_4() {}
pub fn test_vprint_nonunicode_5() {
    let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut file: *mut ::libc::FILE = ({
        let ___s: () = (unimplemented!() as Ptr<()>).to_any();
        let ___len: () = unimplemented!();
        fmemopen_1(___s, ___len, Ptr::<()>::from_string_literal(b"wb"))
    });
    unimplemented!();
    let mut c: () = ('!' as ());
    unimplemented!();
    let mut pos: () = libc::ftell(file) as i64;
    libc::fclose(file);
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
    ({ test_print_0() });
    ({ test_println_2() });
    ({ test_println_blank_line_3() });
    ({ test_vprint_unicode_4() });
    ({ test_vprint_nonunicode_5() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
