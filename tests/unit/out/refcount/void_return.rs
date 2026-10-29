extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f1_0(mut first: Ptr<()>, mut last: Ptr<()>) {
    ({ sort_1(first, last) });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None),
    ));
    ({
        let _first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            ({ (*v.borrow()).begin_26() });
        let _last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            ({ (*v.borrow()).end_28() });
        f1_2(_first, _last)
    });
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    ({
        let _first: Ptr<()> = (a.as_pointer() as Ptr<()>);
        let _last: Ptr<()> = (a.as_pointer() as Ptr<()>).offset((2_()) as isize);
        f1_0(_first, _last)
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
