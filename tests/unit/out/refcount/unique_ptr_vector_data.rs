extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_0(
    u : Ptr< std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____  >,
) -> Ptr<u8> {
    return ({ ((*u.upgrade().deref()).read()).data_57() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v : Value<std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ > = Rc::new(RefCell::new(std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ :: new_1 ( { Ptr::alloc( std_vector_unsigned_char__std_allocator_unsigned_char__ :: new_1 ( { 3_u64  } , None , )  )  } , )  )) ;
    let mut p: Ptr<u8> = ({ ((*v.borrow()).read()).data_57() });
    elem!(p, 2).write(1_u8);
    elem!(({ first_0(v.as_pointer(),) }), 0).write(5_u8);
    ({ cpp2rust_assert_fail_1((((elem!(p, 0).read()) as i32) == 5)) });
    ({ cpp2rust_assert_fail_1((((elem!(({ first_0(v.as_pointer(),) }), 2).read()) as i32) == 1)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
