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
pub struct int_hash {}
impl int_hash {
    pub unsafe fn operator_call(&self, mut i: ()) -> () {
        return i;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut r_c : __gnu_pbds_cc_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mod_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ = <__gnu_pbds_cc_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mod_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ >::default() ;
    r_c[(32_()) as usize] = ('b' as ());
    r_c[(1024_()) as usize] = ('c' as ());
    r_c[(4096_()) as usize] = ('d' as ());
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
