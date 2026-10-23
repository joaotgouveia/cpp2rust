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
pub struct between {
    m_b: (),
    m_e: (),
}
impl between {
    pub unsafe fn new(mut b: (), mut e: ()) -> Self {
        let mut this = Self { m_b: b, m_e: e };
        unimplemented!();
        this
    }
    pub unsafe fn operator_call(&mut self, r_val: *const ()) -> () {
        return (((*r_val).first) >= (self.m_b)) && (((*r_val).first) < (self.m_e));
    }
}
pub unsafe fn some_op_sequence_0(
    mut r_c : __gnu_pbds_list_update_int__char__std_equal_to_int_____gnu_pbds_lu_move_to_front_policy_std_allocator_char____std_allocator_char__,
) {
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        (unsafe {
            let mut _r_val: () = unimplemented!();
            ( r_c as __gnu_pbds_detail_lu_map_int__char__std_equal_to_int___std_allocator_char_____gnu_pbds_lu_move_to_front_policy_std_allocator_char___ ) . insert  ( & mut _r_val , )
        });
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        ( r_c as __gnu_pbds_detail_lu_map_int__char__std_equal_to_int___std_allocator_char_____gnu_pbds_lu_move_to_front_policy_std_allocator_char___ ) . erase_if  ( between :: new ( { 10_() } , { 90_() } , )  , )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn some_op_sequence_1(
    mut r_c : __gnu_pbds_cc_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__,
) {
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        (unsafe {
            let mut _r_val: () = unimplemented!();
            ( r_c as __gnu_pbds_detail_cc_ht_map_int__char__std_tr1_hash_int___std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) . insert  ( & mut _r_val , )
        });
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        ( r_c as __gnu_pbds_detail_cc_ht_map_int__char__std_tr1_hash_int___std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) . erase_if  ( between :: new ( { 10_() } , { 90_() } , )  , )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn some_op_sequence_2(
    mut r_c : __gnu_pbds_gp_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__,
) {
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        (unsafe {
            let mut _r_val: () = unimplemented!();
            ( r_c as __gnu_pbds_detail_gp_ht_map_int__char__std_tr1_hash_int___std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) . insert  ( & mut _r_val , )
        });
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        ( r_c as __gnu_pbds_detail_gp_ht_map_int__char__std_tr1_hash_int___std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) . erase_if  ( between :: new ( { 10_() } , { 90_() } , )  , )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn some_op_sequence_3(
    mut r_c : __gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_rb_tree_tag____gnu_pbds_null_node_update__std_allocator_char__,
) {
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        (unsafe {
            let mut _r_value: () = unimplemented!();
            ( r_c as __gnu_pbds_detail_rb_tree_map_int__char__std_less_int_____gnu_pbds_detail_tree_traits_int__char__std_less_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . insert  ( & mut _r_value , )
        });
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        ( r_c as __gnu_pbds_detail_rb_tree_map_int__char__std_less_int_____gnu_pbds_detail_tree_traits_int__char__std_less_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . erase_if  ( between :: new ( { 10_() } , { 90_() } , )  , )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn some_op_sequence_4(
    mut r_c : __gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_splay_tree_tag____gnu_pbds_null_node_update__std_allocator_char__,
) {
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        (unsafe {
            let mut _r_value: () = unimplemented!();
            ( r_c as __gnu_pbds_detail_splay_tree_map_int__char__std_less_int_____gnu_pbds_detail_tree_traits_int__char__std_less_int_____gnu_pbds_null_node_update____gnu_pbds_splay_tree_tag__std_allocator_char____std_allocator_char__ ) . insert  ( & mut _r_value , )
        });
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        ( r_c as __gnu_pbds_detail_splay_tree_map_int__char__std_less_int_____gnu_pbds_detail_tree_traits_int__char__std_less_int_____gnu_pbds_null_node_update____gnu_pbds_splay_tree_tag__std_allocator_char____std_allocator_char__ ) . erase_if  ( between :: new ( { 10_() } , { 90_() } , )  , )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn some_op_sequence_5(
    mut r_c : __gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_ov_tree_tag____gnu_pbds_null_node_update__std_allocator_char__,
) {
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        (unsafe {
            let mut _r_value: () = unimplemented!();
            ( r_c as __gnu_pbds_detail_ov_tree_map_int__char__std_less_int_____gnu_pbds_detail_tree_traits_int__char__std_less_int_____gnu_pbds_null_node_update____gnu_pbds_ov_tree_tag__std_allocator_char____std_allocator_char__ ) . insert  ( & mut _r_value , )
        });
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        ( r_c as __gnu_pbds_detail_ov_tree_map_int__char__std_less_int_____gnu_pbds_detail_tree_traits_int__char__std_less_int_____gnu_pbds_null_node_update____gnu_pbds_ov_tree_tag__std_allocator_char____std_allocator_char__ ) . erase_if  ( between :: new ( { 10_() } , { 90_() } , )  , )
    });
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe {
        some_op_sequence_0 ( <__gnu_pbds_list_update_int__char__std_equal_to_int_____gnu_pbds_lu_move_to_front_policy_std_allocator_char____std_allocator_char__ >::default() , )
    });
    (unsafe {
        some_op_sequence_1 ( <__gnu_pbds_cc_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ >::default() , )
    });
    (unsafe {
        some_op_sequence_2 ( <__gnu_pbds_gp_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ >::default() , )
    });
    (unsafe {
        some_op_sequence_3 ( <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_rb_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() , )
    });
    (unsafe {
        some_op_sequence_4 ( <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_splay_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() , )
    });
    (unsafe {
        some_op_sequence_5 ( <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_ov_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() , )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
