extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(v: Ptr<std_vector_int__std_allocator_int__>, v3: std_vector_int__std_allocator_int__) {
    let v3: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(v3));
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(20_()));
        (*v.upgrade().deref()).push_back_58(___x.as_pointer())
    });
    let mut x: () = unimplemented!();
    let v2: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    let mut v4: Ptr<std_vector_int__std_allocator_int__> = (v3.as_pointer());
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(0_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(1_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(3_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    x = (elem!(v, 2_()).read());
    (*v2.borrow_mut())[0_()] = 1_();
    elem!((if true { (v3.as_pointer()) } else { (v).clone() }), 0_()).write(7_());
    (*v2.borrow_mut()) = { (*v.upgrade().deref()).clone() };
    elem!((v4), 1_()).write(13_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(20_()));
        (*v.upgrade().deref()).push_back_58(___x.as_pointer())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    let v2: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(4_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(5_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(6_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(8_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(9_()));
        (*v2.borrow()).push_back_58(___x.as_pointer())
    });
    ({ fn_0(v.as_pointer(), (*v2.borrow()).clone()) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
