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
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(10_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(1_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(9_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(2_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(8_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(3_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(7_()));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
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
        let ___first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            ({ (*v.borrow()).begin_26() });
        let ___last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            ({ (*v.borrow()).end_28() });
        sort_0(___first, ___last)
    });
    let mut i: () = 0_();
    'loop_: while (i < ({ (*v.borrow()).size() }).wrapping_sub(1_())) {
        unimplemented!();
        i.prefix_inc();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
