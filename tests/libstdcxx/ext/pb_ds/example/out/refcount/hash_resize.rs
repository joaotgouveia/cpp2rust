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
pub struct int_hash {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let g : Value<__gnu_pbds_gp_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long___false__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_gp_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long___false__std_allocator_char__  >::default() )) ;
    unimplemented!();
    ( (*g.borrow_mut()) as __gnu_pbds_detail_gp_ht_map_int__char__int_hash__std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long__ ) [1_() ]  = ( 'a' as ()  )  ;
    ( (*g.borrow_mut()) as __gnu_pbds_detail_gp_ht_map_int__char__int_hash__std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long__ ) [2_() ]  = ( 'b' as ()  )  ;
    ( (*g.borrow_mut()) as __gnu_pbds_detail_gp_ht_map_int__char__int_hash__std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long__ ) [3_() ]  = ( 'c' as ()  )  ;
    ({
        ( (*g.borrow()) as __gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long_ ) . resize ( 200_()  , )
    });
    unimplemented!();
    ( (*g.borrow_mut()) as __gnu_pbds_detail_gp_ht_map_int__char__int_hash__std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long__ ) [67_() ]  = ( 'g' as ()  )  ;
    ( (*g.borrow_mut()) as __gnu_pbds_detail_gp_ht_map_int__char__int_hash__std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long__ ) [22_() ]  = ( 'f' as ()  )  ;
    unimplemented!();
    ({
        ( (*g.borrow()) as __gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long_ ) . resize ( 106_()  , )
    });
    unimplemented!();
    ( (*g.borrow_mut()) as __gnu_pbds_detail_gp_ht_map_int__char__int_hash__std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_linear_probe_fn_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___true__unsigned_long__ ) [37_() ]  = ( 'f' as ()  )  ;
    unimplemented!();
    return 0_();
}
pub trait int_hashImpl {
    fn operator_call(&self, r_i: Ptr<()>) -> ();
}
impl int_hashImpl for Ptr<int_hash> {
    fn operator_call(&self, r_i: Ptr<()>) -> () {
        return (r_i.read());
    }
}
pub fn __cpp2rust_init_globals() {}
