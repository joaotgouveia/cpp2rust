extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static g_0: Value<std_vector_unsigned_char__std_allocator_unsigned_char__> =
        Rc::new(RefCell::new(
            std_vector_unsigned_char__std_allocator_unsigned_char__::new_1({ 3_u64 }, None),
        ));
);
pub fn get_1() -> Ptr<std_vector_unsigned_char__std_allocator_unsigned_char__> {
    return g_0.with(|v| v.as_pointer());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    elem!(({ get_1() }), 1_u64).write(4_u8);
    let v : Value<std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ > = Rc::new(RefCell::new(std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ :: new_1 ( { Ptr::alloc( std_vector_unsigned_char__std_allocator_unsigned_char__ :: new_1 ( { 3_u64  } , None , )  )  } , )  )) ;
    elem!((v.as_pointer()), 2_u64).write(5_u8);
    ({ cpp2rust_assert_fail_2((((elem!(({ get_1() }), 1_u64).read()) as i32) == 4)) });
    ({ cpp2rust_assert_fail_2((((elem!((v.as_pointer()), 2_u64).read()) as i32) == 5)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = g_0.with(|_| ());
}
