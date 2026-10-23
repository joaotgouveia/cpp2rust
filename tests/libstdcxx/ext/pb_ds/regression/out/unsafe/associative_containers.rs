extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn assoc_link_regression_test_0_0() {
    {
        let mut c : __gnu_pbds_cc_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ = <__gnu_pbds_cc_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_1(&mut c) });
    }
    {
        let mut c : __gnu_pbds_gp_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ = <__gnu_pbds_gp_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_2(&mut c) });
    }
    {
        let mut c : __gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_rb_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ = <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_rb_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_3(&mut c) });
    }
    {
        let mut c : __gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_splay_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ = <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_splay_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_4(&mut c) });
    }
    {
        let mut c : __gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_ov_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ = <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_ov_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_5(&mut c) });
    }
    {
        let mut c : __gnu_pbds_list_update_int__char__std_equal_to_int_____gnu_pbds_lu_move_to_front_policy_std_allocator_char____std_allocator_char__ = <__gnu_pbds_list_update_int__char__std_equal_to_int_____gnu_pbds_lu_move_to_front_policy_std_allocator_char____std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_6(&mut c) });
    }
}
pub unsafe fn assoc_link_regression_test_1_7() {
    {
        let mut c : __gnu_pbds_cc_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ = <__gnu_pbds_cc_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_1(&mut c) });
    }
    {
        let mut c : __gnu_pbds_gp_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ = <__gnu_pbds_gp_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_2(&mut c) });
    }
    {
        let mut c : __gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_rb_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ = <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_rb_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_3(&mut c) });
    }
    {
        let mut c : __gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_splay_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ = <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_splay_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_4(&mut c) });
    }
    {
        let mut c : __gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_ov_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ = <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_ov_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_5(&mut c) });
    }
    {
        let mut c : __gnu_pbds_list_update_int__char__std_equal_to_int_____gnu_pbds_lu_move_to_front_policy_std_allocator_char____std_allocator_char__ = <__gnu_pbds_list_update_int__char__std_equal_to_int_____gnu_pbds_lu_move_to_front_policy_std_allocator_char____std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_6(&mut c) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { assoc_link_regression_test_0_0() });
    (unsafe { assoc_link_regression_test_1_7() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
