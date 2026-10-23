extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0() {
    let mut a : Ptr< ()  >  = Ptr::<() >::from_string_literal(b"13604817 711567 9760686 13278398 3323440 175548 5553651 3028863 10748297 2216688 275779 14778841 14438394 9483441 4229545 14657301 12636508 15978210 1653340 1718567 9272421 14302862 7940348 889045 0 0" )  ;
    let as_: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([3_(), 5_(), 7_()])));
    let sseq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e1 : Value<() > = Rc::new(RefCell::new(std_independent_bits_engine_std_discard_block_engine_std_subtract_with_carry_engine_unsigned_long__24__10__24___223__23___32__unsigned_int_ :: new_1 ( { sseq .as_pointer()  } , )  )) ;
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn test2_1() {
    let mut a : Ptr< ()  >  = Ptr::<() >::from_string_literal(b"241408498702289 172342669275054 191026374555184 61020585639411 231929771458953 142769679250755 198672786411514 183712717244841 227473912549724 62843577252444 68782400568421 159248704678140 0 0" )  ;
    let as_: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([3_(), 5_(), 7_()])));
    let sseq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e1 : Value<() > = Rc::new(RefCell::new(std_independent_bits_engine_std_discard_block_engine_std_subtract_with_carry_engine_unsigned_long__48__5__12___389__11___64__unsigned_long_long_ :: new_1 ( { sseq .as_pointer()  } , )  )) ;
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
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
