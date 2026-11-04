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
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
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
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_74 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_77 ( b  , ) } )  )) ;
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(5));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(6));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_75 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_78 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(5));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(6));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cbegin_76 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cend_79 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(5));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(6));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cbegin_76 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cend_79 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(5));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(6));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_74 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_77 ( b  , ) } )  )) ;
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(5));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(6));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_74(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_77(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_4((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . begin_75 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . end_78 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(5));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(6));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).begin_75(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).end_78(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cbegin_76 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cend_79 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(5));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(6));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        let mut b: u64 = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(0));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        let i : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cbegin_76 ( b  , ) } )  )) ;
        let j : Value<std___detail__Local_const_iterator_int__int__std___detail__Identity__std_hash_int___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__false_ > = Rc::new(RefCell::new(( { (*c.borrow()) . cend_79 ( b  , ) } )  )) ;
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(1));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(2));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 2_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(3));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(4));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 1_isize) }));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(5));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
        b = ({
            let ___key: Value<i32> = Rc::new(RefCell::new(6));
            (*c.borrow()).bucket(___key.as_pointer())
        });
        {
            let __a1 = ({ (*c.borrow()).cbegin_76(b) });
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        {
            let __a1 = ({ (*c.borrow()).cend_79(b) });
            j.as_pointer().with_mut(|__it| *__it = __a1);
            j.as_pointer()
        };
        assert!(({ (({ distance_5((*i.borrow()).clone(), (*j.borrow()).clone(),) }) == 0_isize) }));
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
