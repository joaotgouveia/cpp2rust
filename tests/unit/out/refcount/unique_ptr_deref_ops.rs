extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let p: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<()> = Rc::new(RefCell::new(10_()));
            make_unique_0(___args.as_pointer())
        }),
    ));
    {
        (*p.borrow_mut()).with_mut(|__v| *__v = *__v + 5_())
    };
    {
        (*p.borrow_mut()).with_mut(|__v| *__v = *__v - 3_())
    };
    {
        (*p.borrow_mut()).with_mut(|__v| *__v = *__v * 2_())
    };
    let q: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<()> = Rc::new(RefCell::new(1_()));
            make_unique_0(___args.as_pointer())
        }),
    ));
    let mut sum: () = (((*p.borrow()).read()) + ((*q.borrow()).read()));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
