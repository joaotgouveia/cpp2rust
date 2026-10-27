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
pub struct string_hash {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let h : Value<__gnu_pbds_cc_hash_table_std_basic_string_char__std_char_traits_char___std_allocator_char____int__string_hash__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_______gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___true__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_cc_hash_table_std_basic_string_char__std_char_traits_char___std_allocator_char____int__string_hash__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_______gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___true__std_allocator_char__  >::default() )) ;
    elem!(( h .as_pointer() as __gnu_pbds_detail_cc_ht_map_std_basic_string_char__std_char_traits_char___std_allocator_char____int__string_hash__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_char___true____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) , unimplemented!() ) .write( 0_()  ) ;
    elem!(( h .as_pointer() as __gnu_pbds_detail_cc_ht_map_std_basic_string_char__std_char_traits_char___std_allocator_char____int__string_hash__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_char___true____gnu_pbds_direct_mask_range_hashing_unsigned_long_____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_exponential_size_policy_unsigned_long_____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) , unimplemented!() ) .write( 1_()  ) ;
    return 0_();
}
pub trait string_hashImpl {
    fn operator_call(&self, str: ()) -> ();
}
impl string_hashImpl for Ptr<string_hash> {
    fn operator_call(&self, mut str: ()) -> () {
        let b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut hash: () = 0_();
        'loop_: while unimplemented!() {
            hash = { (hash).wrapping_mul(5_()) };
            {
                let rhs_0 = (hash).wrapping_add((unimplemented!().read()));
                hash = rhs_0
            };
            unimplemented!();
        }
        return hash;
    }
}
pub fn __cpp2rust_init_globals() {}
