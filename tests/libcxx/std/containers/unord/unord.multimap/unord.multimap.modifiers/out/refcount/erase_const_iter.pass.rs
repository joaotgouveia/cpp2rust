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
impl TemplateConstructor {
    pub fn new(_a0: Ptr<i32>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, ptr)]
#[byte_size(1)]
pub struct TemplateConstructor {}
impl CppEq for TemplateConstructor {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for TemplateConstructor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_4(_a0: Ptr<TemplateConstructor>, _a1: Ptr<TemplateConstructor>) -> bool {
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Hash {}
impl Callable1<Ptr<TemplateConstructor>, usize> for Hash {
    fn call(&self, a1: Ptr<TemplateConstructor>) -> usize {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| HashImpl::operator_call(__it, a1))
    }
}
impl CppMove for Hash {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
        let i : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  ) )) ;
        let i_next : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(((*i.borrow()) ).clone() )) ;
        {
            i_next.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i_next.as_pointer()
        };
        let mut es: Vec<i8> = (*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()).clone();
        let j : Value<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . erase_48 ( ((*i.borrow()) ).clone()  , ) } )  )) ;
        assert!((*j.borrow()) == (*i_next.borrow()));
        assert!(({ (({ (*c.borrow()).size() }) == 5_u64) }));
        let eq : Value<(Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >, Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >) > = Rc::new(RefCell::new(match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_0,) => { (
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_0.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_0.upgrade().deref()) .1.as_pointer()))),
    ) } } )) ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_5(___first, ___last)
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
            CheckConsecutiveKeys_6(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_1,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_1.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_1.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_5(___first, ___last)
                }) == 1_isize)
            })
        );
        let k : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(((*(*eq.borrow()) .0.borrow()) ).clone() )) ;
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_2,) => (if Ptr::<i8>::from_string_literal(b"two")
                        .with_c_str(|__s| es[..es.len().saturating_sub(1)] == *__s)
                    {
                        Ptr::<i8>::from_string_literal(b"four")
                    } else {
                        Ptr::<i8>::from_string_literal(b"two")
                    })
                    .with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_3,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_3.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_3.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_5(___first, ___last)
                }) == 1_isize)
            })
        );
        {
            let __a1 = ({ (*(*eq.borrow()).0.borrow()).clone() });
            k.as_pointer().with_mut(|__it| *__it = __a1.clone());
            k.as_pointer()
        };
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_4,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_4.upgrade().deref())
                            [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_5,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_5.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_5.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_5(___first, ___last)
                }) == 1_isize)
            })
        );
        {
            let __a1 = ({ (*(*eq.borrow()).0.borrow()).clone() });
            k.as_pointer().with_mut(|__it| *__it = __a1.clone());
            k.as_pointer()
        };
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_6,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_6.upgrade().deref())
                            [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
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
                    distance_7(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_5(___first, ___last)
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
        let i : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  ) )) ;
        let i_next : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(((*i.borrow()) ).clone() )) ;
        {
            i_next.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i_next.as_pointer()
        };
        let mut es: Vec<i8> = (*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()).clone();
        let j : Value<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . erase_48 ( ((*i.borrow()) ).clone()  , ) } )  )) ;
        assert!((*j.borrow()) == (*i_next.borrow()));
        assert!(({ (({ (*c.borrow()).size() }) == 5_u64) }));
        let eq : Value<(Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >, Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >) > = Rc::new(RefCell::new(match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_7,) => { (
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_7.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_7.upgrade().deref()) .1.as_pointer()))),
    ) } } )) ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_5(___first, ___last)
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
            CheckConsecutiveKeys_6(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_8,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_8.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_8.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_5(___first, ___last)
                }) == 1_isize)
            })
        );
        let k : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_ > = Rc::new(RefCell::new(((*(*eq.borrow()) .0.borrow()) ).clone() )) ;
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_9,) => (if Ptr::<i8>::from_string_literal(b"two")
                        .with_c_str(|__s| es[..es.len().saturating_sub(1)] == *__s)
                    {
                        Ptr::<i8>::from_string_literal(b"four")
                    } else {
                        Ptr::<i8>::from_string_literal(b"two")
                    })
                    .with_c_str(|__s| {
                        (*__tmp_9.upgrade().deref())
                            [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_10,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_10.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_10.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_5(___first, ___last)
                }) == 1_isize)
            })
        );
        {
            let __a1 = ({ (*(*eq.borrow()).0.borrow()).clone() });
            k.as_pointer().with_mut(|__it| *__it = __a1.clone());
            k.as_pointer()
        };
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_11,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_11.upgrade().deref())
                            [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_12,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_12.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_  >>::cpp_from_move((*__tmp_12.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_5(___first, ___last)
                }) == 1_isize)
            })
        );
        {
            let __a1 = ({ (*(*eq.borrow()).0.borrow()).clone() });
            k.as_pointer().with_mut(|__it| *__it = __a1.clone());
            k.as_pointer()
        };
        assert!(({ ((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*k.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_13,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_13.upgrade().deref())
                            [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
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
                    distance_7(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__false_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_5(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
    }
    {
        let m : Value<std_unordered_multimap_TemplateConstructor__int__Hash__std_equal_to_TemplateConstructor___std_allocator_std_pair_const_TemplateConstructor__int___ > = Rc::new(RefCell::new(<std_unordered_multimap_TemplateConstructor__int__Hash__std_equal_to_TemplateConstructor___std_allocator_std_pair_const_TemplateConstructor__int___  >::default() )) ;
        let a: Value<TemplateConstructor> = Rc::new(RefCell::new({
            let __tmp_14: Value<i32> = Rc::new(RefCell::new(0));
            TemplateConstructor::new({ __tmp_14.as_pointer() })
        }));
        let it: Value<
            std___detail__Node_iterator_std_pair_const_TemplateConstructor__int___false__true_,
        > = Rc::new(RefCell::new(({ (*m.borrow()).find_60(a.as_pointer()) })));
        if (*it.borrow()) != ({ (*m.borrow()).end_30() }) {
            ({ (*m.borrow()).erase_49((*it.borrow()).clone()) });
        }
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__;
pub trait HashImpl {
    fn operator_call(&self, _a0: Ptr<TemplateConstructor>) -> usize;
}
impl HashImpl for Ptr<Hash> {
    fn operator_call(&self, _a0: Ptr<TemplateConstructor>) -> usize {
        return 0_usize;
    }
}
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
