extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_max_size: anon_0 = 4;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct dna_string_access_traits {}
impl dna_string_access_traits {
    pub fn begin(r_key: Ptr<()>) -> () {
        return unimplemented!();
    }
    pub fn end(r_key: Ptr<()>) -> () {
        return unimplemented!();
    }
    pub fn e_pos(mut e: ()) -> () {
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
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let t : Value<__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char____dna_string_access_traits____gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char____dna_string_access_traits____gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__  >::default() )) ;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
