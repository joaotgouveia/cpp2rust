extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct modulo2_hash {}
impl modulo2_hash {
    pub unsafe fn operator_call(&self, key: ()) -> () {
        return ((key) % (2_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct modulo2_eq {}
impl modulo2_eq {
    pub unsafe fn operator_call(&self, left: (), right: ()) -> () {
        return (((left) % (2_())) == ((right) % (2_())));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut table : __gnu_cxx_hashtable_std_pair_const_int__int___int__modulo2_hash__std__Select1st_std_pair_const_int__int____modulo2_eq__std_allocator_int__ = { let mut __tmp_0 : modulo2_hash  = <modulo2_hash >::default() ;let mut __tmp_1 : modulo2_eq  = <modulo2_eq >::default() ;let mut __tmp_2 : std__Select1st_std_pair_const_int__int__  = unsafe { std::mem::zeroed::<std__Select1st_std_pair_const_int__int__ >() } ;let mut __tmp_3 : ()  = unimplemented!() ; __gnu_cxx_hashtable_std_pair_const_int__int___int__modulo2_hash__std__Select1st_std_pair_const_int__int____modulo2_eq__std_allocator_int__ :: new_1 ( { 4_() } , { & mut __tmp_0 } , { & mut __tmp_1 } , { & mut __tmp_2 } , { & mut __tmp_3 } , )  } ;
    (unsafe {
        let mut ___obj: () = unimplemented!();
        table.insert_equal(&mut ___obj)
    });
    let mut it: () = (unsafe {
        let mut ___obj: () = unimplemented!();
        table.insert_equal(&mut ___obj)
    });
    (unsafe { table.erase_38(&(*unimplemented!()).first) });
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
