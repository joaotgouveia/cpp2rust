extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct simple_string_ranged_hash_fn {
    m_container_size: (),
}
impl simple_string_ranged_hash_fn {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            m_container_size: 0_(),
        };
        this
    }
    pub unsafe fn notify_resized(&mut self, mut size: ()) {
        self.m_container_size = size;
    }
    pub unsafe fn operator_call(&mut self, r_string: *const ()) -> () {
        let mut it: () = unimplemented!();
        let mut hash: () = 0_();
        if ((self.m_container_size) < (100_())) {
            'loop_: while unimplemented!() {
                hash = (hash).wrapping_add((*unimplemented!()));
            }
        } else {
            'loop_: while unimplemented!() {
                hash = (hash).wrapping_add((*unimplemented!()));
                hash = (hash).wrapping_mul(5_());
            }
        }
        return (hash).wrapping_rem(self.m_container_size);
    }
    pub unsafe fn swap(&mut self, other: *mut simple_string_ranged_hash_fn) {
        (unsafe {
            let ___a: *mut () = &mut self.m_container_size;
            let ___b: *mut () = &mut (*other).m_container_size;
            swap_0(___a, ___b)
        });
    }
}
impl Default for simple_string_ranged_hash_fn {
    fn default() -> Self {
        unsafe { simple_string_ranged_hash_fn::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut h : __gnu_pbds_cc_hash_table_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_null_type__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____simple_string_ranged_hash_fn____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ = <__gnu_pbds_cc_hash_table_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_null_type__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____simple_string_ranged_hash_fn____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ >::default() ;
    (unsafe {
        let mut _r_val: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"Hello, ".as_ptr() },
                None,
            );
        ( h as __gnu_pbds_detail_cc_ht_set_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_null_type__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_char___false__simple_string_ranged_hash_fn____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) . insert  ( & mut _r_val , )
    });
    (unsafe {
        let mut _r_val: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"world".as_ptr() },
                None,
            );
        ( h as __gnu_pbds_detail_cc_ht_set_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_null_type__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_char___false__simple_string_ranged_hash_fn____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) . insert  ( & mut _r_val , )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
