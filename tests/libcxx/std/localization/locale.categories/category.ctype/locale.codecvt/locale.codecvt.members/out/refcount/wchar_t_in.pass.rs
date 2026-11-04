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
    let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let from: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"some text").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let mut expected: Vec<i32> = match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
        (__tmp_0,) => {
            let mut __bytes: Vec<u8> = Vec::new();
            if !(__tmp_0).clone().is_null() {
                (__tmp_0)
                    .clone()
                    .with_mut(|__f: &mut std::fs::File| {
                        std::io::Read::read_to_end(__f, &mut __bytes)
                    })
                    .expect("couldn't read the file");
            }
            __bytes.push(0);
            CChar::from_byte_vec(__bytes)
        }
    };
    let to: Value<Vec<i32>> = Rc::new(RefCell::new(
        vec![(unimplemented!() as i32); ({ ((*from.borrow()).len() - 1) }) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i32))
            .collect(),
    ));
    let f: Ptr<()> = unimplemented!();
    let mbs: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mut from_next: Ptr<()> = Ptr::<()>::null();
    let mut to_next: Ptr<()> = Ptr::<()>::null();
    let mut r: () = unimplemented!();
    assert!(((r as i32) == (unimplemented!() as i32)));
    assert!(
        ({
            ({ ((((from_next).clone() - from.decay()) as i64) as usize) } == {
                ((*from.borrow()).len() - 1)
            })
        })
    );
    assert!(({ ((((to_next).clone() - to.decay()) as i64) as usize) } == { (expected.len() - 1) }));
    assert!(({ ((((to_next).clone() - to.decay()) as i64) as usize) } == { (expected.len() - 1) }));
    assert!(unimplemented!());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
