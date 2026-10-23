extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_max_size: anon_0 = 4;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct dna_string_access_traits {}
impl dna_string_access_traits {
    pub unsafe fn begin(r_key: *const ()) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(r_key: *const ()) -> () {
        return unimplemented!();
    }
    pub unsafe fn e_pos(mut e: ()) -> () {
        switch!(match e {
            __v if __v == ('A' as ()) => {
                return 0_();
            }
            __v if __v == ('C' as ()) => {
                return 1_();
            }
            __v if __v == ('G' as ()) => {
                return 2_();
            }
            __v if __v == ('T' as ()) => {
                return 3_();
            }
            _ => {
                unimplemented!();
            }
        });
        panic!("ub: non-void function does not return a value")
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut t : __gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char____dna_string_access_traits____gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__ = <__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char____dna_string_access_traits____gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() ;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
