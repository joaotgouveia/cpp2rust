extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_eprintf_0(
    mut fmt: Ptr<i8>,
    args_1: Ptr<Ptr<i8>>,
    args_2: Ptr<Ptr<i8>>,
    args_3: Ptr<Ptr<i8>>,
    args_4: Ptr<i32>,
    args_5: Ptr<Ptr<i8>>,
) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &Box::<[VaArg]>::from([
                (args_1.read()).into(),
                (args_2.read()).into(),
                (args_3.read()).into(),
                (args_4.read()).into(),
                (args_5.read()).into(),
            ]),
        );
        libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn test_log_1(condition: Ptr<i8>, file: Ptr<i8>, line: i32, message: Ptr<i8>) {
    let condition: Value<Ptr<i8>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<i8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    let message: Value<Ptr<i8>> = Rc::new(RefCell::new(message));
    let msg: Value<Ptr<i8>> = Rc::new(RefCell::new(if !(*condition.borrow()).is_null() {
        Ptr::<i8>::from_string_literal(b"Assertion failure: ")
    } else {
        Ptr::<i8>::from_string_literal(b"Unconditional failure:")
    }));
    ({
        test_eprintf_0(
            Ptr::<i8>::from_string_literal(b"%s%s %s %d\n%s"),
            msg.as_pointer(),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
            message.as_pointer(),
        )
    });
}
const _: () = assert!(true, "std::is_signed<std::messages_base::catalog>::value");
const _: () = assert!(true, "std::is_integral<std::messages_base::catalog>::value");
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
    let mb: Value<std_messages_base> = Rc::new(RefCell::new(<std_messages_base>::default()));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
