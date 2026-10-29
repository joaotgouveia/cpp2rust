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
            std_vector_unsigned_char__std_allocator_unsigned_char__::new_1({ 3_() }, None),
        ));
);
pub fn get_1() -> Ptr<std_vector_unsigned_char__std_allocator_unsigned_char__> {
    return g_0.with(|v| v.as_pointer());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    elem!(({ get_1() }), 1_()).write(4_());
    let v : Value<std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ > = Rc::new(RefCell::new(std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ :: new_1 ( { Ptr::alloc( std_vector_unsigned_char__std_allocator_unsigned_char__ :: new_1 ( { 3_()  } , None , )  )  } , )  )) ;
    elem!((v.as_pointer()), 2_()).write(5_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = g_0.with(|_| ());
}
