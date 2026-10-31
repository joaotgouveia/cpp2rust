extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub fn test_eprintf_1(
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
pub fn test_log_2(condition: Ptr<i8>, file: Ptr<i8>, line: i32, message: Ptr<i8>) {
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
        test_eprintf_1(
            Ptr::<i8>::from_string_literal(b"%s%s %s %d\n%s"),
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    let mut value: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut str_value: Vec<i8> = {
        let mut __bytes = value.to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    };
    let pathstr_value: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = value
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = value
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(unimplemented!() == (*pathstr_value.borrow()));
        assert!((unimplemented!().decay() == unimplemented!()));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
