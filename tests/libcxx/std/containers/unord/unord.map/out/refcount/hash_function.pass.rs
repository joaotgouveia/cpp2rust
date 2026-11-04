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
    let m : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____  >::default() )) ;
    let p : Value<(Value<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >, Value<bool  >) > = Rc::new(RefCell::new(( { let ___x : Value<(Value<i32  >, Value<Vec<i8  >  >) > = Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"abc" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )); (*m.borrow()) . insert_44 ( ___x.as_pointer() , ) } )  )) ;
    let cm : Ptr< std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____  > = m .as_pointer()  ;
    assert!(
        ({
            ({
                ({ (*cm.upgrade().deref()).hash_function() })(
                    (*(*(*(*p.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .0
                        .borrow()),
                )
            })
        } == { ({ ({ (*cm.upgrade().deref()).hash_function() })(1,) }) })
    );
    assert!(
        ({ ({ ({ (*cm.upgrade().deref()).hash_function() })(1,) }) } == {
            ({
                ({ (*cm.upgrade().deref()).hash_function() })(
                    (*(*(*(*p.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .0
                        .borrow()),
                )
            })
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
