extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct contains_pq { #[offset(0)] #[byte_size(32)] m_tree : __gnu_pbds_tree_int__unsigned_long__std_greater_int_____gnu_pbds_rb_tree_tag____gnu_pbds_null_node_update__std_allocator_char__  , }
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let cpq: Value<contains_pq> = Rc::new(RefCell::new(<contains_pq>::default()));
    ({ contains_pqImpl::push(&cpq.as_pointer(), 4_()) });
    ({ contains_pqImpl::push(&cpq.as_pointer(), 3_()) });
    ({ contains_pqImpl::push(&cpq.as_pointer(), 2_()) });
    ({ contains_pqImpl::push(&cpq.as_pointer(), 1_()) });
    ({ contains_pqImpl::push(&cpq.as_pointer(), 4_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait contains_pqImpl {
    fn push(&self, i: ());
    fn pop(&self) -> ();
    fn contains(&self, i: ()) -> ();
    fn empty(&self) -> ();
}
impl contains_pqImpl for Ptr<contains_pq> {
    fn contains(&self, i: ()) -> () {
        let i: Value<()> = Rc::new(RefCell::new(i));
        return unimplemented!();
    }
    fn empty(&self) -> () {
        return ({
            ( (*(* self ) .upgrade().deref()) . m_tree  as __gnu_pbds_detail_bin_search_tree_map_int__unsigned_long__std_greater_int_____gnu_pbds_detail_tree_traits_int__unsigned_long__std_greater_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . empty ( )
        });
    }
    fn push(&self, mut i: ()) {
        field!(( { let _r_value : Value<() > = Rc::new(RefCell::new(unimplemented!()   )); ( (*(* self ) .upgrade().deref()) . m_tree  as __gnu_pbds_detail_rb_tree_map_int__unsigned_long__std_greater_int_____gnu_pbds_detail_tree_traits_int__unsigned_long__std_greater_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . insert ( _r_value.as_pointer() , ) } )  . first  , second) .with_mut(|__v| __v. prefix_inc ()) ;
    }
    fn pop(&self) -> () {
        unimplemented!();
        let it: Value<()> = Rc::new(RefCell::new(
            ({
                ( (*(* self ) .upgrade().deref()) . m_tree  as __gnu_pbds_detail_bin_search_tree_map_int__unsigned_long__std_greater_int_____gnu_pbds_detail_tree_traits_int__unsigned_long__std_greater_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . begin_18 ( )
            }),
        ));
        let mut i: () = { ((*it.borrow()).read()).first };
        if (field!((*it.borrow_mut()), second).with_mut(|__v| __v.prefix_dec()) == 0_()) {
            ({
                ( (*(* self ) .upgrade().deref()) . m_tree  as __gnu_pbds_detail_rb_tree_map_int__unsigned_long__std_greater_int_____gnu_pbds_detail_tree_traits_int__unsigned_long__std_greater_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . erase_10 ( unimplemented!()  , )
            });
        }
        return i;
    }
}
pub fn __cpp2rust_init_globals() {}
