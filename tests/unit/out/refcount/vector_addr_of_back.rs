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
    let outer : Value<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ > = Rc::new(RefCell::new(<std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____  >::default() )) ;
    let inner: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({ (*outer.borrow()).push_back_57(inner.as_pointer()) });
    let mut sink: Ptr<std_vector_int__std_allocator_int__> = ({ (*outer.borrow()).back_53() });
    unimplemented!();
    let mut p : Ptr< std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____  >  = ( outer .as_pointer()  )  ;
    sink = ({ (*p.upgrade().deref()).back_53() });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
