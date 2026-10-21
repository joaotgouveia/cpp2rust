extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(20)]
pub struct S {
    #[offset(0)]
    pub head: (),
    #[offset(4)]
    #[byte_size(12)]
    pub tail: Box<[()]>,
    #[offset(16)]
    #[byte_size(4)]
    pub buf: Box<[()]>,
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                head: unimplemented!(),
                tail: (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                buf: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
thread_local!(
    pub static s_0: Value<S> = Rc::new(RefCell::new(S {
        head: 5_(),
        tail: Box::new([0; 3]),
        buf: Box::new([0; 4]),
    }));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (((i < 3_()) as ()) != 0) {
        unimplemented!();
        i.postfix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (((i < 4_()) as ()) != 0) {
        unimplemented!();
        i.postfix_inc();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = s_0.with(|_| ());
}
