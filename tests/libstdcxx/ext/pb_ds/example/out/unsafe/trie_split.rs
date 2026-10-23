extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut r : __gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__ = <__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() ;
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        (unsafe {
            let mut _r_val: () = unimplemented!();
            ( r as __gnu_pbds_detail_pat_trie_map_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_detail_trie_traits_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_null_node_update____gnu_pbds_pat_trie_tag__std_allocator_char____std_allocator_char__ ) . insert  ( & mut _r_val , )
        });
        i.prefix_inc();
    }
    let mut larger_r : __gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__ = <__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__ >::default() ;
    (unsafe {
        let mut _r_key: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { 1000_() },
                { ('a' as ()) },
                None,
            );
        ( r as __gnu_pbds_detail_pat_trie_map_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_detail_trie_traits_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_null_node_update____gnu_pbds_pat_trie_tag__std_allocator_char____std_allocator_char__ ) . split  ( & mut _r_key , & mut larger_r , )
    });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let mut _r_key: () = unimplemented!();
        ( r as __gnu_pbds_detail_pat_trie_map_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_detail_trie_traits_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_null_node_update____gnu_pbds_pat_trie_tag__std_allocator_char____std_allocator_char__ ) . split  ( & mut _r_key , & mut larger_r , )
    });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
