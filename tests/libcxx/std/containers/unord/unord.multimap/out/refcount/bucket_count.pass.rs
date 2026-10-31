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
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<Box<[(Value<i32>, Value<Vec<i8>>)]>> = Rc::new(RefCell::new(Box::new([
            (
                Rc::new(RefCell::new(10)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"ten").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(20)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"twenty").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(30)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"thirty").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(40)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"forty").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(50)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"fifty").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(60)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"sixty").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(70)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"seventy").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(80)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"eighty").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
        ])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() >= 8_()));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
