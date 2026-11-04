extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
impl CppMove for min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
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
        let a: Value<Box<[(Value<i32>, Value<Vec<i8>>)]>> = Rc::new(RefCell::new(Box::new([
            (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(3)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"three").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(4)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
        ])));
        let c : Value<std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 6_u64) }));
        let eq : Value<(Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >, Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >) > = Rc::new(RefCell::new(match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_0,) => { (
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_0.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_0.upgrade().deref()) .1.as_pointer()))),
    ) } } )) ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        let s : Value<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() )) ;
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_1,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_1.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_1.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 2, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_2,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_2.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_2.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        let k : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(((*(*eq.borrow()) .0.borrow()) ).clone() )) ;
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_3,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_3.upgrade().deref())
                            [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(4 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_4,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_4.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_4.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_5,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_6,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_6.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_6.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_7,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_7.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_7.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_8,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_8.upgrade().deref())
                            [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(4 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_9,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_9.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_9.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_10,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_10.upgrade().deref())
                            [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_11,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_11.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_11.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_12,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_12.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_12.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_13,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_13.upgrade().deref())
                            [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(4 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_14,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_14.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_14.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_15,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_15.upgrade().deref())
                            [..(*__tmp_15.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_16,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_16.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_16.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_17,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_17.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_17.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_18,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_18.upgrade().deref())
                            [..(*__tmp_18.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_19,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_19.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_19.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_20,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_20.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_20.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_21,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_21.upgrade().deref())
                            [..(*__tmp_21.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_22,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_22.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_22.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_23,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_23.upgrade().deref())
                            [..(*__tmp_23.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_24,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_24.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_24.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_25,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_25.upgrade().deref())
                            [..(*__tmp_25.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_26,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_26.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_26.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 0_isize)
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_27,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_27.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_27.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 0_isize)
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
    }
    {
        let a: Value<Box<[(Value<i32>, Value<Vec<i8>>)]>> = Rc::new(RefCell::new(Box::new([
            (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(3)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"three").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(4)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
        ])));
        let c : Value<std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 6_u64) }));
        let eq : Value<(Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >, Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >) > = Rc::new(RefCell::new(match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_28,) => { (
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_28.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_28.upgrade().deref()) .1.as_pointer()))),
    ) } } )) ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        let s : Value<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() )) ;
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_29,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_29.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_29.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 2, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_30,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_30.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_30.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        let k : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(((*(*eq.borrow()) .0.borrow()) ).clone() )) ;
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_31,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_31.upgrade().deref())
                            [..(*__tmp_31.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(4 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_32,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_32.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_32.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_33,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_33.upgrade().deref())
                            [..(*__tmp_33.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_34,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_34.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_34.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_35,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_35.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_35.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_36,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_36.upgrade().deref())
                            [..(*__tmp_36.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(4 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_37,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_37.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_37.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_38,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_38.upgrade().deref())
                            [..(*__tmp_38.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_39,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_39.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_39.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_40,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_40.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_40.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_41,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_41.upgrade().deref())
                            [..(*__tmp_41.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(4 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_42,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_42.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_42.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_43,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_43.upgrade().deref())
                            [..(*__tmp_43.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_44,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_44.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_44.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_45,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_45.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_45.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_46,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_46.upgrade().deref())
                            [..(*__tmp_46.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_47,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_47.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_47.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_5(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_48,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_48.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_48.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_49,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_49.upgrade().deref())
                            [..(*__tmp_49.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_50,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_50.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_50.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_51,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_51.upgrade().deref())
                            [..(*__tmp_51.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_52,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_52.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_52.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 1_isize)
            })
        );
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_53,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_53.upgrade().deref())
                            [..(*__tmp_53.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_54,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_54.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_54.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 0_isize)
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).erase_50(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
        match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(3 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_55,) => { eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_55.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_55.upgrade().deref()) .1.as_pointer()))),
    )) } }  ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_4(___first, ___last)
                }) == 0_isize)
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_6(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
    }
    {
        let m : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int___  >::default() )) ;
        let m2 : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__int___  >::default() )) ;
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            let mut j: i32 = 0;
            'loop_: while (j < 2) {
                ({
                    let ___x: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
                        Rc::new(RefCell::new(i)),
                        Rc::new(RefCell::new(j)),
                    )));
                    (*m.borrow()).insert_37_struct_std_pair_int__int_(___x.as_pointer())
                });
                ({
                    let ___x: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
                        Rc::new(RefCell::new(i)),
                        Rc::new(RefCell::new(j)),
                    )));
                    (*m2.borrow()).insert_37_struct_std_pair_int__int_(___x.as_pointer())
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
        let i: Value<std___detail__Node_iterator_std_pair_const_int__int___false__false_> =
            Rc::new(RefCell::new(({ (*m2.borrow()).begin_27() })));
        let mut ctr: i32 = 0;
        'loop_: while (*i.borrow()) != ({ (*m2.borrow()).end_30() }) {
            if ((ctr.postfix_inc() % 2) == 0) {
                ({ (*m2.borrow()).erase_49((*i.borrow_mut()).iter_post_inc()) });
            } else {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
            }
        }
        assert!(({ only_deletions_7(m.as_pointer(), m2.as_pointer(),) }));
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__;
pub trait min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl for Ptr<min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____> {
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
