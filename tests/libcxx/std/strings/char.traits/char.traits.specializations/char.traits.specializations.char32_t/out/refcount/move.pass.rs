extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let s1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    assert!((unimplemented!() == (s1.as_pointer() as Ptr::<u32>)));
    assert!(((*s1.borrow())[(0) as usize] == 2_u32));
    assert!(((*s1.borrow())[(1) as usize] == 3_u32));
    assert!(((*s1.borrow())[(2) as usize] == 3_u32));
    (*s1.borrow_mut())[(2) as usize] = 0_u32;
    assert!((unimplemented!() == (s1.as_pointer() as Ptr::<u32>).offset((1) as isize)));
    assert!(((*s1.borrow())[(0) as usize] == 2_u32));
    assert!(((*s1.borrow())[(1) as usize] == 2_u32));
    assert!(((*s1.borrow())[(2) as usize] == 3_u32));
    assert!((unimplemented!()).is_null());
    assert!((unimplemented!() == (s1.as_pointer() as Ptr::<u32>)));
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
