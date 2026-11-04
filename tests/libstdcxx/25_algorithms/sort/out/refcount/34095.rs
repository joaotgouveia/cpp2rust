extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let v : Value<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ > = Rc::new(RefCell::new(std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ :: new_1 ( { 20000_u64  } , None , )  )) ;
    ({
        let ___first: __gnu_cxx___normal_iterator_std_vector_int__std_allocator_int___ptr__std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int_____   = ( { (*v.borrow()) . begin_28 ( ) } )   ;
        let ___last: __gnu_cxx___normal_iterator_std_vector_int__std_allocator_int___ptr__std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int_____   = ( { (*v.borrow()) . end_30 ( ) } )   ;
        sort_1(___first, ___last)
    });
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
