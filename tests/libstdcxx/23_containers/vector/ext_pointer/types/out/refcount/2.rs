extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N_X {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut w: () = unimplemented!();
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({
        let ___position: () = unimplemented!();
        let ___first: () = unimplemented!();
        let ___last: () = unimplemented!();
        (*v.borrow()) . insert_66_class___gnu_cxx___normal_iterator_class___gnu_cxx__Pointer_adapter_class___gnu_cxx__Relative_pointer_impl_const_struct_N_X_____class_std_vector_struct_N_X__class___gnu_cxx__ExtPtr_allocator_struct_N_X______void ( ___position , ___first , ___last , )
    });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
