extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct TakesTwoInts {
    #[offset(0)]
    pub a_: i32,
    #[offset(4)]
    pub b_: i32,
}
impl TakesTwoInts {
    pub fn new(mut a: i32, mut b: i32) -> Self {
        Self { a_: a, b_: b }
    }
}
pub fn test_0() -> bool {
    {
        let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!().with(|__s| __s.a_) == 1));
        assert!((unimplemented!().with(|__s| __s.b_) == 2));
        assert!((unimplemented!() == 1_usize));
    }
    {
        let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!().with(|__s| __s.a_) == 1));
        assert!((unimplemented!().with(|__s| __s.b_) == 2));
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
