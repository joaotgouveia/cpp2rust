extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_0(u: Ptr<()>) -> Ptr<u8> {
    return (unimplemented!()).reinterpret_cast::<u8>();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v : Value<() > = Rc::new(RefCell::new(std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ :: new ( { Ptr::alloc( unimplemented!() )  } , )  )) ;
    let mut p: Ptr<u8> = (unimplemented!()).reinterpret_cast::<u8>();
    elem!(p, 2_()).write(1_u8);
    elem!(({ first_0(v.as_pointer(),) }), 0_()).write(5_u8);
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
