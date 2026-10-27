extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_string_ref__const_std_string_ref_ {}
pub fn test_eprintf_1(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<Ptr<()>>,
    args_3: Ptr<Ptr<()>>,
    args_4: Ptr<()>,
    args_5: Ptr<Ptr<()>>,
) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
            (args_3.read()),
            (args_4.read()),
            (args_5.read()),
        )
    });
}
pub fn test_log_2(condition: Ptr<()>, file: Ptr<()>, line: (), message: Ptr<()>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    let message: Value<Ptr<()>> = Rc::new(RefCell::new(message));
    let msg: Value<Ptr<()>> = Rc::new(RefCell::new(if !(*condition.borrow()).is_null() {
        Ptr::<()>::from_string_literal(b"Assertion failure: ")
    } else {
        Ptr::<()>::from_string_literal(b"Unconditional failure:")
    }));
    ({
        test_eprintf_1(
            Ptr::<()>::from_string_literal(b"%s%s %s %d\n%s"),
            msg.as_pointer(),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
            message.as_pointer(),
        )
    });
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
    let mut value: Ptr<()> = Ptr::<()>::from_string_literal(b"hello world");
    let mut value_str: () = unimplemented!();
    let mut pathstr_value: () = unimplemented!();
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
