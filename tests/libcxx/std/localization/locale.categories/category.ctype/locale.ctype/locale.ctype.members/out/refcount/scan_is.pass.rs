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
    {
        let f: Ptr<()> = unimplemented!();
        let in_: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                32 as i32, 65 as i32, 7 as i32, 46 as i32, 97 as i32, 49 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        let mut m: Vec<u16> = (0..((*in_.borrow()).len() - 1) as usize)
            .map(|_| 0_u16)
            .collect::<Vec<_>>();
        assert!(((unimplemented!() - in_.decay()) as i64 == 0_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 0_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 2_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 1_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 4_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 1_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 5_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 3_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 1_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 0_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 1_i64));
        assert!(((unimplemented!() - in_.decay()) as i64 == 1_i64));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
