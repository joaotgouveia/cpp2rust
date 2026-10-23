extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct contains_pq { m_tree : __gnu_pbds_tree_int__unsigned_long__std_greater_int_____gnu_pbds_rb_tree_tag____gnu_pbds_null_node_update__std_allocator_char__ , }
impl contains_pq {
    pub unsafe fn contains(&self, mut i: ()) -> () {
        return unimplemented!();
    }
    pub unsafe fn empty(&self) -> () {
        return (unsafe {
            ( self . m_tree  as __gnu_pbds_detail_bin_search_tree_map_int__unsigned_long__std_greater_int_____gnu_pbds_detail_tree_traits_int__unsigned_long__std_greater_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . empty  ( )
        });
    }
}
impl contains_pq {
    pub unsafe fn push(&mut self, mut i: ()) {
        ( * ( unsafe { let mut _r_value : ()  = unimplemented!()  ; ( self . m_tree  as __gnu_pbds_detail_rb_tree_map_int__unsigned_long__std_greater_int_____gnu_pbds_detail_tree_traits_int__unsigned_long__std_greater_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . insert  ( & mut _r_value , ) } )  . first   ) . second  .prefix_inc() ;
    }
}
impl contains_pq {
    pub unsafe fn pop(&mut self) -> () {
        unimplemented!();
        let mut it: () = (unsafe {
            ( self . m_tree  as __gnu_pbds_detail_bin_search_tree_map_int__unsigned_long__std_greater_int_____gnu_pbds_detail_tree_traits_int__unsigned_long__std_greater_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . begin_18  ( )
        });
        let i: () = (*it).first;
        if (((*it).second.prefix_dec()) == (0_())) {
            (unsafe {
                ( self . m_tree  as __gnu_pbds_detail_rb_tree_map_int__unsigned_long__std_greater_int_____gnu_pbds_detail_tree_traits_int__unsigned_long__std_greater_int_____gnu_pbds_null_node_update____gnu_pbds_rb_tree_tag__std_allocator_char____std_allocator_char__ ) . erase_10  ( unimplemented!() , )
            });
        }
        return i;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut cpq: contains_pq = <contains_pq>::default();
    (unsafe { contains_pq::push(&mut cpq, 4_()) });
    (unsafe { contains_pq::push(&mut cpq, 3_()) });
    (unsafe { contains_pq::push(&mut cpq, 2_()) });
    (unsafe { contains_pq::push(&mut cpq, 1_()) });
    (unsafe { contains_pq::push(&mut cpq, 4_()) });
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
pub unsafe fn __cpp2rust_init_globals() {}
