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
    (*u.borrow_mut()).write(10);
    return (*u.borrow_mut()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let f: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(8));
            make_unique_1(___args.as_pointer())
        }),
    ));
    (*f.borrow_mut()).write(9);
    let mut f_ptr1: Ptr<i32> = ({ (*f.borrow()).get() });
    f_ptr1.write(10);
    let mut f_ptr2: Ptr<i32> = (f.as_pointer());
    f_ptr2.write(11);
    (*f.borrow_mut()) = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(9));
        make_unique_1(___args.as_pointer())
    });
    let __rhs = ({ fn_0((*f.borrow_mut()).clone()) });
    (*f.borrow_mut()) = __rhs;
    ({ cpp2rust_assert_fail_2((((*f.borrow()).read()) == 10)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
