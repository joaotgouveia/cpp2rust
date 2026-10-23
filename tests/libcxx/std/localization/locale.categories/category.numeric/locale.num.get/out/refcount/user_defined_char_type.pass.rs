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
pub struct Char {
    #[offset(0)]
    pub underlying_: (),
}
impl Char {
    pub fn new_1(mut c: ()) -> Self {
        Self { underlying_: c }
    }
    pub fn new_2(mut i: ()) -> Self {
        Self { underlying_: i }
    }
    pub fn new_3(mut i: i32) -> Self {
        Self {
            underlying_: (i as ()),
        }
    }
}
thread_local!(
    pub static id_0: Value<std_locale_id> = Rc::new(RefCell::new(<std_locale_id>::default()));
);
thread_local!(
    pub static id_1: Value<std_locale_id> = Rc::new(RefCell::new(<std_locale_id>::default()));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let l: Value<std_locale> = Rc::new(RefCell::new(std_locale::new_1(
        { ({ std_locale::classic() }) },
        { Ptr::alloc(unimplemented!()) },
    )));
    return 0_();
}
pub trait CharImpl {
    fn to_i32(&self) -> i32;
}
impl CharImpl for Ptr<Char> {
    fn to_i32(&self) -> i32 {
        return ((*self).with(|__s| __s.underlying_) as i32);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = id_0.with(|_| ());
    let _ = id_1.with(|_| ());
}
