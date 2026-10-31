extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub x: i32,
}
impl S {
    pub fn new() -> Self {
        Self { x: 1 }
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
thread_local!(
    pub static cnt_dtor_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);

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
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 100_usize));
        let mut i: i32 = 0;
        'loop_: while (i < 100) {
            assert!(((unimplemented!().read()) == 0));
            i.prefix_inc();
        }
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 100_usize));
        let mut i: i32 = 0;
        'loop_: while (i < 100) {
            assert!(((unimplemented!().read()) == 0_f64));
            i.prefix_inc();
        }
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 100_usize));
        let mut i: i32 = 0;
        'loop_: while (i < 100) {
            assert!((unimplemented!() == 0_usize));
            i.prefix_inc();
        }
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 100_usize));
        let mut i: i32 = 0;
        'loop_: while (i < 100) {
            assert!(({ (unimplemented!()).clone().x } == 1));
            i.prefix_inc();
        }
    }
    assert!((cnt_dtor_0.with(|rc| *rc.borrow()) == 100_usize));
    return 0;
}
pub trait SImpl {
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*cnt_dtor_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = cnt_dtor_0.with(|_| ());
}
