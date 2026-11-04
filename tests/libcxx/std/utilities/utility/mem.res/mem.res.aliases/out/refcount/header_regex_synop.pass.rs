extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        ({ test_match_result_typedef_0() });
        ({ test_match_result_typedef_1() });
        ({ test_match_result_typedef_2() });
        ({ test_match_result_typedef_3() });
    }
    {
        let s : Value<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_pmr_polymorphic_allocator_char_____std_pmr_polymorphic_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_pmr_polymorphic_allocator_char______ > = Rc::new(RefCell::new(<std_match_results___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_pmr_polymorphic_allocator_char_____std_pmr_polymorphic_allocator_std_sub_match___gnu_cxx___normal_iterator_const_char_ptr__std_basic_string_char__std_char_traits_char___std_pmr_polymorphic_allocator_char______  >::default() )) ;
        assert!(
            (({ ({ (*s.borrow()).get_allocator() }).resource() })
                == ({ get_default_resource_4() }))
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
