extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0() {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([3_(), 5_(), 7_()])));
    let sseq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e2 : Value<() > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ :: new_1 ( { sseq .as_pointer()  } , )  )) ;
    unimplemented!();
    ({ (*e1.borrow()).seed(sseq.as_pointer()) });
    unimplemented!();
}
pub fn test2_1() {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([3_(), 5_(), 7_()])));
    let sseq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e2 : Value<() > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__64__312__156__31__13043109905998158313__29__6148914691236517205__17__8202884508482404352__37__18444473444759240704__43__6364136223846793005_ :: new_1 ( { sseq .as_pointer()  } , )  )) ;
    unimplemented!();
    ({ (*e1.borrow()).seed(sseq.as_pointer()) });
    unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test1_0() });
    ({ test2_1() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
