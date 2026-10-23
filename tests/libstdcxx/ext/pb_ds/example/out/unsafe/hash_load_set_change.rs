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
    let mut trigger: __gnu_pbds_hash_load_check_resize_trigger_true__unsigned_long_ =
        <__gnu_pbds_hash_load_check_resize_trigger_true__unsigned_long_>::default();
    let mut resize : __gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_true__unsigned_long___true__unsigned_long_ = { let mut __tmp_0 : __gnu_pbds_hash_exponential_size_policy_unsigned_long_  = <__gnu_pbds_hash_exponential_size_policy_unsigned_long_ >::default() ; __gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_true__unsigned_long___true__unsigned_long_ :: new_1 ( { & mut __tmp_0 } , { & trigger } , )  } ;
    let mut r_c : __gnu_pbds_cc_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_true__unsigned_long___true__unsigned_long___false__std_allocator_char__ = { let mut __tmp_1 : int_hash  = <int_hash >::default() ;let mut __tmp_2 : ()  = unimplemented!() ;let mut __tmp_3 : __gnu_pbds_direct_mask_range_hashing_unsigned_long_  = unsafe { std::mem::zeroed::<__gnu_pbds_direct_mask_range_hashing_unsigned_long_ >() } ; __gnu_pbds_cc_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_true__unsigned_long___true__unsigned_long___false__std_allocator_char__ :: new_1 ( { & mut __tmp_1 } , { & mut __tmp_2 } , { & mut __tmp_3 } , { & resize } , )  } ;
    r_c[(1_()) as usize] = ('a' as ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    r_c[(2_()) as usize] = ('b' as ());
    r_c[(3_()) as usize] = ('c' as ());
    r_c[(4_()) as usize] = ('d' as ());
    unimplemented!();
    (unsafe {
        (r_c as __gnu_pbds_hash_load_check_resize_trigger_true__unsigned_long_)
            .set_loads(unimplemented!())
    });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
