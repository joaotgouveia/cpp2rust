extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_equal_to_int_ {
    #[offset(0)]
    pub data_: i32,
}
impl test_equal_to_int_ {
    pub fn new_1() -> Self {
        Self { data_: 0 }
    }
}
impl Callable2<Ptr<i32>, Ptr<i32>, bool> for test_equal_to_int_ {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            test_equal_to_int_Impl::operator_call(__it, a1, a2)
        })
    }
}
impl CppMove for test_equal_to_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_equal_to_int_ {
    fn default() -> Self {
        { test_equal_to_int_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_hash_int_ {
    #[offset(0)]
    data_: i32,
}
impl test_hash_int_ {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 0);
        Self { data_: data }
    }
}
impl Callable1<Ptr<i32>, usize> for test_hash_int_ {
    fn call(&self, a1: Ptr<i32>) -> usize {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            test_hash_int_Impl::operator_call(__it, a1)
        })
    }
}
impl CppMove for test_hash_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_hash_int_ {
    fn default() -> Self {
        { test_hash_int_::new(None) }
    }
}
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
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { Vec::from([ match (Rc::new(RefCell::new((Rc::new(RefCell::new(4 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_0,) => { (
        Rc::new(RefCell::new(((*__tmp_0.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_0.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_1,) => { (
        Rc::new(RefCell::new(((*__tmp_1.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_1.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_2,) => { (
        Rc::new(RefCell::new(((*__tmp_2.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_2.upgrade().deref()) .1.as_pointer()))),
    ) } }  , ] )  } , None , None , None , None , )  )) ;
        (*c.borrow_mut()) = Vec::from([
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_3,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_3.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_3.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_4,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_4.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_4.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(3)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"three").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_5,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_5.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_5.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(4)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_6,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_6.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_6.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_7,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_7.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_7.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_8,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_8.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_8.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
        ]);
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        assert!(match (({
            let ___k: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).at_82(___k.as_pointer())
        }),)
        {
            (__tmp_9,) => {
                Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                    (*__tmp_9.upgrade().deref())
                        [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(match (({
            let ___k: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).at_82(___k.as_pointer())
        }),)
        {
            (__tmp_10,) => {
                Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                    (*__tmp_10.upgrade().deref())
                        [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(match (({
            let ___k: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).at_82(___k.as_pointer())
        }),)
        {
            (__tmp_11,) => {
                Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                    (*__tmp_11.upgrade().deref())
                        [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(match (({
            let ___k: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).at_82(___k.as_pointer())
        }),)
        {
            (__tmp_12,) => {
                Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                    (*__tmp_12.upgrade().deref())
                        [..(*__tmp_12.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_5(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_6(
                        ((({ (*c.borrow()).load_factor() })
                            - ((({ (*c.borrow()).size() }) as f32)
                                / (({ (*c.borrow()).bucket_count() }) as f32)))
                            as f64),
                    )
                }) < (1.1920929e-07_f32 as f64))
            })
        );
        assert!(({ (({ (*c.borrow()).max_load_factor_95() }) == 1_f32) }));
    }
    {
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { Vec::from([ match (Rc::new(RefCell::new((Rc::new(RefCell::new(4 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_13,) => { (
        Rc::new(RefCell::new(((*__tmp_13.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_13.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_14,) => { (
        Rc::new(RefCell::new(((*__tmp_14.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_14.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_15,) => { (
        Rc::new(RefCell::new(((*__tmp_15.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_15.upgrade().deref()) .1.as_pointer()))),
    ) } }  , ] )  } , None , None , None , None , )  )) ;
        (*c.borrow_mut()) = Vec::from([
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_16,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_16.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_16.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_17,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_17.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_17.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(3)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"three").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_18,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_18.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_18.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(4)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_19,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_19.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_19.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_20,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_20.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_20.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            )))
            .as_pointer(),)
            {
                (__tmp_21,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_21.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(<Vec<i8> as CppFrom<Vec<i8>>>::cpp_from_move(
                        (*__tmp_21.upgrade().deref()).1.as_pointer(),
                    ))),
                ),
            },
        ]);
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        assert!(match (({
            let ___k: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).at_82(___k.as_pointer())
        }),)
        {
            (__tmp_22,) => {
                Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                    (*__tmp_22.upgrade().deref())
                        [..(*__tmp_22.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(match (({
            let ___k: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).at_82(___k.as_pointer())
        }),)
        {
            (__tmp_23,) => {
                Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                    (*__tmp_23.upgrade().deref())
                        [..(*__tmp_23.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(match (({
            let ___k: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).at_82(___k.as_pointer())
        }),)
        {
            (__tmp_24,) => {
                Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                    (*__tmp_24.upgrade().deref())
                        [..(*__tmp_24.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(match (({
            let ___k: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).at_82(___k.as_pointer())
        }),)
        {
            (__tmp_25,) => {
                Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                    (*__tmp_25.upgrade().deref())
                        [..(*__tmp_25.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_4(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_5(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_6(
                        ((({ (*c.borrow()).load_factor() })
                            - ((({ (*c.borrow()).size() }) as f32)
                                / (({ (*c.borrow()).bucket_count() }) as f32)))
                            as f64),
                    )
                }) < (1.1920929e-07_f32 as f64))
            })
        );
        assert!(({ (({ (*c.borrow()).max_load_factor_95() }) == 1_f32) }));
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
pub trait test_equal_to_int_Impl {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool;
}
impl test_equal_to_int_Impl for Ptr<test_equal_to_int_> {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool {
        return ({ (a.read()) } == { (b.read()) });
    }
}
pub trait test_hash_int_Impl {
    fn operator_call(&self, x: Ptr<i32>) -> usize;
    fn operator_eq(&self, c: Ptr<test_hash_int_>) -> bool {
        unimplemented!()
    }
}
impl test_hash_int_Impl for Ptr<test_hash_int_> {
    fn operator_call(&self, x: Ptr<i32>) -> usize {
        return ({ <std_hash_int_>::default()((x.read())) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
