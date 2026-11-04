extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let m : Value<std_map_std_tuple_int__int___unsigned_long__std_less_std_tuple_int__int____std_allocator_std_pair_const_std_tuple_int__int___unsigned_long___ > = Rc::new(RefCell::new(<std_map_std_tuple_int__int___unsigned_long__std_less_std_tuple_int__int____std_allocator_std_pair_const_std_tuple_int__int___unsigned_long___  >::default() )) ;
    elem!(
        m.as_pointer(),
        ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(2));
            let ___args_1: Value<i32> = Rc::new(RefCell::new(3));
            make_tuple_1(___args_0.as_pointer(), ___args_1.as_pointer())
        })
    )
    .write(7_u64);
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
