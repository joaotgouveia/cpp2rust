extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct between {
    #[offset(0)]
    m_b: (),
    #[offset(4)]
    m_e: (),
}
impl between {
    pub fn new(mut b: (), mut e: ()) -> Self {
        let __this: between = Self { m_b: b, m_e: e };
        unimplemented!();
        __this
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({
        some_op_sequence_0 ( <__gnu_pbds_list_update_int__char__std_equal_to_int_____gnu_pbds_lu_move_to_front_policy_std_allocator_char____std_allocator_char__  >::default()  , )
    });
    ({
        some_op_sequence_1 ( <__gnu_pbds_cc_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__  >::default()  , )
    });
    ({
        some_op_sequence_2 ( <__gnu_pbds_gp_hash_table_int__char__std_tr1_hash_int___std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__  >::default()  , )
    });
    ({
        some_op_sequence_3 ( <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_rb_tree_tag____gnu_pbds_null_node_update__std_allocator_char__  >::default()  , )
    });
    ({
        some_op_sequence_4 ( <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_splay_tree_tag____gnu_pbds_null_node_update__std_allocator_char__  >::default()  , )
    });
    ({
        some_op_sequence_5 ( <__gnu_pbds_tree_int__char__std_less_int_____gnu_pbds_ov_tree_tag____gnu_pbds_null_node_update__std_allocator_char__  >::default()  , )
    });
    return 0_();
}
pub trait betweenImpl {
    fn operator_call(&self, r_val: Ptr<()>) -> ();
}
impl betweenImpl for Ptr<between> {
    fn operator_call(&self, r_val: Ptr<()>) -> () {
        return ({ r_val.with(|__s| __s.first) } >= { (*self).with(|__s| __s.m_b) })
            && ({ r_val.with(|__s| __s.first) } < { (*self).with(|__s| __s.m_e) });
    }
}
pub fn __cpp2rust_init_globals() {}
