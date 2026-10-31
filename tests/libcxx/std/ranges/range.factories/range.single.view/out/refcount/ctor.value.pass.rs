extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(64)]
pub struct BigType {
    #[offset(0)]
    #[byte_size(64)]
    pub buffer: Box<[i8]>,
}
impl Default for BigType {
    fn default() -> Self {
        {
            BigType {
                buffer: Box::new([
                    10_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                    0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                    0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                    0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                    0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
                ]),
            }
        }
    }
}
pub fn test_0() -> bool {
    {
        let mut bt: BigType = <BigType>::default();
        let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((array_elem!(unimplemented!(), buffer, 0).read()) as i32) == 10));
        assert!((unimplemented!() == 1_usize));
    }
    {
        let mut bt: BigType = <BigType>::default();
        let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((array_elem!(unimplemented!(), buffer, 0).read()) as i32) == 10));
        assert!((unimplemented!() == 1_usize));
    }
    {
        let mut bt: BigType = <BigType>::default();
        let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((array_elem!(unimplemented!(), buffer, 0).read()) as i32) == 10));
        assert!((unimplemented!() == 1_usize));
    }
    {
        let mut bt: BigType = <BigType>::default();
        let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((((array_elem!(unimplemented!(), buffer, 0).read()) as i32) == 10));
        assert!((unimplemented!() == 1_usize));
    }
    return true;
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
