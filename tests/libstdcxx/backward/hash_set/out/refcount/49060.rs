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
pub struct modulo2_hash {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct modulo2_eq {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let table : Value<__gnu_cxx_hashtable_std_pair_const_int__int___int__modulo2_hash__std__Select1st_std_pair_const_int__int____modulo2_eq__std_allocator_int__ > = Rc::new(RefCell::new({ let __tmp_0 : Value<modulo2_hash > = Rc::new(RefCell::new(<modulo2_hash  >::default() ));let __tmp_1 : Value<modulo2_eq > = Rc::new(RefCell::new(<modulo2_eq  >::default() ));let __tmp_2 : Value<std__Select1st_std_pair_const_int__int__ > = Rc::new(RefCell::new(<std__Select1st_std_pair_const_int__int__  >::default() ));let __tmp_3 : Value<std_allocator_std_pair_const_int__int__ > = Rc::new(RefCell::new(unimplemented!() )); __gnu_cxx_hashtable_std_pair_const_int__int___int__modulo2_hash__std__Select1st_std_pair_const_int__int____modulo2_eq__std_allocator_int__ :: new_1 ( { 4_()  } , { __tmp_0.as_pointer() } , { __tmp_1.as_pointer() } , { __tmp_2.as_pointer() } , { __tmp_3.as_pointer() } , )  } )) ;
    ({
        let ___obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*table.borrow()).insert_equal(___obj.as_pointer())
    });
    let it: Value<()> = Rc::new(RefCell::new(
        ({
            let ___obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*table.borrow()).insert_equal(___obj.as_pointer())
        }),
    ));
    ({ (*table.borrow()).erase_38(field_ptr!(it.as_pointer(), first)) });
    0
}
pub trait modulo2_eqImpl {
    fn operator_call(&self, left: (), right: ()) -> ();
}
impl modulo2_eqImpl for Ptr<modulo2_eq> {
    fn operator_call(&self, mut left: (), mut right: ()) -> () {
        return ((left % 2_()) == (right % 2_()));
    }
}
pub trait modulo2_hashImpl {
    fn operator_call(&self, key: ()) -> ();
}
impl modulo2_hashImpl for Ptr<modulo2_hash> {
    fn operator_call(&self, mut key: ()) -> () {
        return (key % 2_());
    }
}
pub fn __cpp2rust_init_globals() {}
