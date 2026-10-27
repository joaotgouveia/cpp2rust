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
    pub unsafe fn operator_call(&self, r_i: *const ()) -> () {
        return (*r_i);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut g : __gnu_pbds_gp_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long___false__std_allocator_char__ = <__gnu_pbds_gp_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long___false__std_allocator_char__ >::default() ;
    unimplemented!();
    g[(1_()) as usize] = ('a' as ());
    g[(2_()) as usize] = ('b' as ());
    g[(3_()) as usize] = ('c' as ());
    (unsafe {
        ( g as __gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long_ ) . resize  ( 200_() , )
    });
    unimplemented!();
    g[(67_()) as usize] = ('g' as ());
    g[(22_()) as usize] = ('f' as ());
    unimplemented!();
    (unsafe {
        ( g as __gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long_ ) . resize  ( 106_() , )
    });
    unimplemented!();
    g[(37_()) as usize] = ('f' as ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
