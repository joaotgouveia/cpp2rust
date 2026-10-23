extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let r : Value<__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__  >::default() )) ;
    let mut i: () = 0_();
    'loop_: while (i < 100_()) {
        ({
            let _r_val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ( (*r.borrow()) as __gnu_pbds_detail_pat_trie_map_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_detail_trie_traits_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_null_node_update____gnu_pbds_pat_trie_tag__std_allocator_char____std_allocator_char__ ) . insert ( _r_val.as_pointer() , )
        });
        i.prefix_inc();
    }
    let larger_r : Value<__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_null_node_update__std_allocator_char__  >::default() )) ;
    ({
        let _r_key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { 1000_() },
                { ('a' as ()) },
                None,
            ),
        ));
        ( (*r.borrow()) as __gnu_pbds_detail_pat_trie_map_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_detail_trie_traits_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_null_node_update____gnu_pbds_pat_trie_tag__std_allocator_char____std_allocator_char__ ) . split ( _r_key.as_pointer() , larger_r .as_pointer()  , )
    });
    unimplemented!();
    unimplemented!();
    ({
        let _r_key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
                { Ptr::<()>::from_string_literal(b"aaa") },
                None,
            ),
        ));
        ( (*r.borrow()) as __gnu_pbds_detail_pat_trie_map_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_detail_trie_traits_std_basic_string_char__std_char_traits_char___std_allocator_char____char____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_null_node_update____gnu_pbds_pat_trie_tag__std_allocator_char____std_allocator_char__ ) . split ( _r_key.as_pointer() , larger_r .as_pointer()  , )
    });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
