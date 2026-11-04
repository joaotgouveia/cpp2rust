extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0() {
    let e: Value<std_subtract_with_carry_engine_unsigned_long__24__10__24_> = Rc::new(
        RefCell::new(<std_subtract_with_carry_engine_unsigned_long__24__10__24_>::default()),
    );
    assert!(({ (({ (*e.borrow_mut())() }) == 15039276_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 16323925_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 14283486_u64) }));
}
pub fn test2_1() {
    let e: Value<std_subtract_with_carry_engine_unsigned_long__48__5__12_> = Rc::new(RefCell::new(
        <std_subtract_with_carry_engine_unsigned_long__48__5__12_>::default(),
    ));
    assert!(({ (({ (*e.borrow_mut())() }) == 23459059301164_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 28639057539807_u64) }));
    assert!(({ (({ (*e.borrow_mut())() }) == 276846226770426_u64) }));
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
    ({ test1_0() });
    ({ test2_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
