extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(
    u: std_unique_ptr_int__std_default_delete_int__,
) -> std_unique_ptr_int__std_default_delete_int__ {
    let u: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(u));
    (*u.borrow_mut()).write(10_());
    return (*u.borrow_mut()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let f: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<()> = Rc::new(RefCell::new(8_()));
            make_unique_1(___args.as_pointer())
        }),
    ));
    (*f.borrow_mut()).write(9_());
    let mut f_ptr1: Ptr<()> = ({ (*f.borrow()).get() });
    f_ptr1.write(10_());
    let mut f_ptr2: Ptr<()> = (f.as_pointer());
    f_ptr2.write(11_());
    (*f.borrow_mut()) = ({
        let ___args: Value<()> = Rc::new(RefCell::new(9_()));
        make_unique_1(___args.as_pointer())
    });
    let __rhs = ({ fn_0((*f.borrow_mut()).clone()) });
    (*f.borrow_mut()) = __rhs;
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
