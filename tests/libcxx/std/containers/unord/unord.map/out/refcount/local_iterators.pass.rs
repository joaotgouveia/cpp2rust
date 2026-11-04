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
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_88 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_91 ( b  , ) } )  )) ;
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_88(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_91(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 1) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_0,) => Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_88(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_91(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_1,) => Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_88(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_91(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_2,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_88(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_91(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_3,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_3.upgrade().deref())
                            [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
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
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_89 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_92 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_89(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_92(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 1) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_4,) => Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                        (*__tmp_4.upgrade().deref())
                            [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_89(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_92(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_5,) => Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_89(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_92(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_6,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_6.upgrade().deref())
                            [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_89(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_92(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_7,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_7.upgrade().deref())
                            [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
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
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cbegin_90 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cend_93 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 1) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_8,) => Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                        (*__tmp_8.upgrade().deref())
                            [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_9,) => Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                        (*__tmp_9.upgrade().deref())
                            [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_10,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_10.upgrade().deref())
                            [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_11,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_11.upgrade().deref())
                            [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
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
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cbegin_90 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cend_93 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 1) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_12,) => Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                        (*__tmp_12.upgrade().deref())
                            [..(*__tmp_12.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_13,) => Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                        (*__tmp_13.upgrade().deref())
                            [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_14,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_14.upgrade().deref())
                            [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
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
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_88 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_91 ( b  , ) } )  )) ;
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_88(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_91(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 1) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_16,) => Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                        (*__tmp_16.upgrade().deref())
                            [..(*__tmp_16.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_88(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_91(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_17,) => Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                        (*__tmp_17.upgrade().deref())
                            [..(*__tmp_17.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_88(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_91(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
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
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_88(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_91(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_19,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_19.upgrade().deref())
                            [..(*__tmp_19.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
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
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_89 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_92 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_89(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_92(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 1) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_20,) => Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                        (*__tmp_20.upgrade().deref())
                            [..(*__tmp_20.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_89(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_92(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_21,) => Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                        (*__tmp_21.upgrade().deref())
                            [..(*__tmp_21.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_89(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_92(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_22,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_22.upgrade().deref())
                            [..(*__tmp_22.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_89(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_92(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_23,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_23.upgrade().deref())
                            [..(*__tmp_23.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
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
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cbegin_90 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cend_93 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 1) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_24,) => Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                        (*__tmp_24.upgrade().deref())
                            [..(*__tmp_24.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_25,) => Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                        (*__tmp_25.upgrade().deref())
                            [..(*__tmp_25.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_26,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_26.upgrade().deref())
                            [..(*__tmp_26.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_27,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_27.upgrade().deref())
                            [..(*__tmp_27.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
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
        let c : Value<std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_map_int__std_basic_string_char__std_char_traits_char___std_allocator_char____std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   )  } , { ( a .as_pointer()  as Ptr< (Value<i32  >, Value<Vec<i8  >  >)  >   ) . offset ( ( ( ( 240usize as usize ) ) . wrapping_div ( ( 40usize as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 5_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cbegin_90 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std___detail__Select1st__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__false__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cend_93 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 1) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_28,) => Ptr::<i8>::from_string_literal(b"one").with_c_str(|__s| {
                        (*__tmp_28.upgrade().deref())
                            [..(*__tmp_28.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_29,) => Ptr::<i8>::from_string_literal(b"two").with_c_str(|__s| {
                        (*__tmp_29.upgrade().deref())
                            [..(*__tmp_29.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_30,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_30.upgrade().deref())
                            [..(*__tmp_30.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_90(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_93(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_31,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_31.upgrade().deref())
                            [..(*__tmp_31.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
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
