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
pub struct string_hash {}
impl string_hash {
    pub unsafe fn operator_call(&self, mut str: ()) -> () {
        let mut b: () = unimplemented!();
        let mut e: () = unimplemented!();
        let mut hash: () = 0_();
        'loop_: while unimplemented!() {
            hash = (hash).wrapping_mul(5_());
            hash = (hash).wrapping_add((*unimplemented!()));
            unimplemented!();
        }
        return hash;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut h : __gnu_pbds_cc_hash_table_std_basic_string_char__std_char_traits_char___std_allocator_char____int__string_hash__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_______gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___true__std_allocator_char__ = <__gnu_pbds_cc_hash_table_std_basic_string_char__std_char_traits_char___std_allocator_char____int__string_hash__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_______gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___true__std_allocator_char__ >::default() ;
    h[(unimplemented!()) as usize] = 0_();
    h[(unimplemented!()) as usize] = 1_();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
