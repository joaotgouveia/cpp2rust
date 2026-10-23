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
    let r_c : Value<__gnu_pbds_cc_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mod_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_cc_hash_table_int__char__int_hash__std_equal_to_int_____gnu_pbds_direct_mod_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__  >::default() )) ;
    ( (*r_c.borrow_mut()) as __gnu_pbds_detail_cc_ht_map_int__char__int_hash__std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mod_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) [32_() ]  = ( 'b' as ()  )  ;
    ( (*r_c.borrow_mut()) as __gnu_pbds_detail_cc_ht_map_int__char__int_hash__std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mod_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) [1024_() ]  = ( 'c' as ()  )  ;
    ( (*r_c.borrow_mut()) as __gnu_pbds_detail_cc_ht_map_int__char__int_hash__std_equal_to_int___std_allocator_char___false____gnu_pbds_direct_mod_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) [4096_() ]  = ( 'd' as ()  )  ;
    return 0_();
}
pub trait int_hashImpl {
    fn operator_call(&self, i: ()) -> ();
}
impl int_hashImpl for Ptr<int_hash> {
    fn operator_call(&self, mut i: ()) -> () {
        return i;
    }
}
pub fn __cpp2rust_init_globals() {}
