extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn print_prefix_match_0(
    t : Ptr< __gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_trie_prefix_search_node_update__std_allocator_char__  >,
    key: Ptr<()>,
) {
    unimplemented!();
    let match_range: Value<()> = Rc::new(RefCell::new(
        ({
            let _r_key: Ptr<()> = (key).clone();
            ( (*t.upgrade().deref()) as __gnu_pbds_trie_prefix_search_node_update___gnu_pbds_detail_pat_trie_base__Node_citer___gnu_pbds_detail_pat_trie_base__Node_base___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Leaf___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Head___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Inode___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__CIter___gnu_pbds_detail_pat_trie_base__Node_base___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Leaf___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Head___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Inode___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_____true_____gnu_pbds_detail_pat_trie_base__CIter___gnu_pbds_detail_pat_trie_base__Node_base___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Leaf___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Head___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Inode___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_____true___std_allocator_char______gnu_pbds_detail_pat_trie_base__Node_citer___gnu_pbds_detail_pat_trie_base__Node_base___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Leaf___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Head___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Inode___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__CIter___gnu_pbds_detail_pat_trie_base__Node_base___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Leaf___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Head___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Inode___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_____true_____gnu_pbds_detail_pat_trie_base__CIter___gnu_pbds_detail_pat_trie_base__Node_base___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Leaf___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Head___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Inode___gnu_pbds_detail_synth_access_traits___gnu_pbds_detail_types_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type__std_allocator_char___false___true____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char_______gnu_pbds_detail_pat_trie_base__Metadata___gnu_pbds_null_type__std_allocator_char_____true___std_allocator_char______gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char____std_allocator_char__ ) . prefix_range_1 ( _r_key , )
        }),
    ));
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let t : Value<__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_trie_prefix_search_node_update__std_allocator_char__ > = Rc::new(RefCell::new(<__gnu_pbds_trie_std_basic_string_char__std_char_traits_char___std_allocator_char______gnu_pbds_null_type____gnu_pbds_trie_string_access_traits_std_basic_string_char__std_char_traits_char___std_allocator_char______x80_____x7f___false__std_allocator_char______gnu_pbds_pat_trie_tag____gnu_pbds_trie_prefix_search_node_update__std_allocator_char__  >::default() )) ;
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
    ({
        let _key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        print_prefix_match_0(t.as_pointer(), _key.as_pointer())
    });
    ({
        let _key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        ));
        print_prefix_match_0(t.as_pointer(), _key.as_pointer())
    });
    ({
        let _key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"as") },
                None,
            ),
        ));
        print_prefix_match_0(t.as_pointer(), _key.as_pointer())
    });
    ({
        let _key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ad") },
                None,
            ),
        ));
        print_prefix_match_0(t.as_pointer(), _key.as_pointer())
    });
    ({
        let _key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"t") },
                None,
            ),
        ));
        print_prefix_match_0(t.as_pointer(), _key.as_pointer())
    });
    ({
        let _key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tr") },
                None,
            ),
        ));
        print_prefix_match_0(t.as_pointer(), _key.as_pointer())
    });
    ({
        let _key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"trie") },
                None,
            ),
        ));
        print_prefix_match_0(t.as_pointer(), _key.as_pointer())
    });
    ({
        let _key: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"zzz") },
                None,
            ),
        ));
        print_prefix_match_0(t.as_pointer(), _key.as_pointer())
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
