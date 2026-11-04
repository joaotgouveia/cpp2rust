extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let x : Value<std_vector___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_____std_allocator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_____ > = Rc::new(RefCell::new(std_vector___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_____std_allocator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_____ :: new_1 ( { 2_u64  } , None , )  )) ;
    let i2: Value<__gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___> =
        Rc::new(RefCell::new(
            <__gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___>::default(),
        ));
    let i3: Value<__gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___> =
        Rc::new(RefCell::new(
            <__gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___>::default(),
        ));
    let ci1: Value<
        __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int___,
    > = Rc::new(RefCell::new(
        __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int___::new_1({
            i2.as_pointer()
        }),
    ));
    (*i2.borrow_mut()) = (*i3.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
