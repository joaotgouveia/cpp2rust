extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_convertible_imp_0(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_1() -> () {
    return ({ test_convertible_imp_0(0_()) });
}
pub fn check_random_device_valid_2(token: Ptr<()>) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn check_random_device_invalid_3(token: Ptr<()>) {
    (&(*token.upgrade().deref()));
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
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        &(*r.borrow_mut());
    }
    {
        ({
            let _token: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_random_device_valid_2(_token.as_pointer())
        });
        ({
            let _token: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_random_device_invalid_3(_token.as_pointer())
        });
        ({
            let _token: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_random_device_invalid_3(_token.as_pointer())
        });
        ({
            let _token: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_random_device_invalid_3(_token.as_pointer())
        });
        ({
            let _token: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            check_random_device_invalid_3(_token.as_pointer())
        });
    }
    {
        let mut ec: () = unimplemented!();
        ec = unimplemented!();
        unimplemented!();
        ec = unimplemented!();
        unimplemented!();
        ec = unimplemented!();
        unimplemented!();
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    const _: () = assert!(true, "test_convertible<std::random_device>()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
