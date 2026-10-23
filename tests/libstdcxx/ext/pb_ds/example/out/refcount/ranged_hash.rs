extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct simple_string_ranged_hash_fn {
    #[offset(0)]
    m_container_size: (),
}
impl simple_string_ranged_hash_fn {
    pub fn new() -> Self {
        Self {
            m_container_size: 0_(),
        }
    }
}
impl Default for simple_string_ranged_hash_fn {
    fn default() -> Self {
        { simple_string_ranged_hash_fn::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let h : Value<__gnu_pbds_cc_hash_table_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_null_type__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____simple_string_ranged_hash_fn____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_cc_hash_table_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_null_type__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____simple_string_ranged_hash_fn____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long___false__std_allocator_char__  >::default() )) ;
    ({
        let _r_val: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"Hello, ") },
                None,
            ),
        ));
        ( (*h.borrow()) as __gnu_pbds_detail_cc_ht_set_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_null_type__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_char___false__simple_string_ranged_hash_fn____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) . insert ( _r_val.as_pointer() , )
    });
    ({
        let _r_val: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"world") },
                None,
            ),
        ));
        ( (*h.borrow()) as __gnu_pbds_detail_cc_ht_set_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_null_type__std_equal_to_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_char___false__simple_string_ranged_hash_fn____gnu_pbds_hash_standard_resize_policy___gnu_pbds_hash_prime_size_policy____gnu_pbds_hash_load_check_resize_trigger_false__unsigned_long___false__unsigned_long__ ) . insert ( _r_val.as_pointer() , )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait simple_string_ranged_hash_fnImpl {
    fn notify_resized(&self, size: ());
    fn operator_call(&self, r_string: Ptr<()>) -> ();
    fn swap(&self, other: Ptr<simple_string_ranged_hash_fn>);
}
impl simple_string_ranged_hash_fnImpl for Ptr<simple_string_ranged_hash_fn> {
    fn notify_resized(&self, mut size: ()) {
        field!((*self), m_container_size).write(size);
    }
    fn operator_call(&self, r_string: Ptr<()>) -> () {
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut hash: () = 0_();
        if ((*self).with(|__s| __s.m_container_size) < 100_()) {
            'loop_: while unimplemented!() {
                {
                    let rhs_0 = (hash).wrapping_add((unimplemented!().read()));
                    hash = rhs_0
                };
            }
        } else {
            'loop_: while unimplemented!() {
                {
                    let rhs_0 = (hash).wrapping_add((unimplemented!().read()));
                    hash = rhs_0
                };
                hash = { (hash).wrapping_mul(5_()) };
            }
        }
        return (hash).wrapping_rem((*self).with(|__s| __s.m_container_size));
    }
    fn swap(&self, other: Ptr<simple_string_ranged_hash_fn>) {
        ({
            let ___a: Ptr<()> = field_ptr!((*self), m_container_size);
            let ___b: Ptr<()> = field_ptr!(other, m_container_size);
            swap_0(___a, ___b)
        });
    }
}
pub fn __cpp2rust_init_globals() {}
