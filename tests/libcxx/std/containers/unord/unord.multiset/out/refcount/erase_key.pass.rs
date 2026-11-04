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
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 6_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 6_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 4_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).erase_48(___x.as_pointer())
            }) == 0_u64)
        );
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
    }
    {
        let m : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ > = Rc::new(RefCell::new(<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__  >::default() )) ;
        let m2 : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ > = Rc::new(RefCell::new(<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__  >::default() )) ;
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 10) {
            ({ (*m.borrow()).insert_35(i.as_pointer()) });
            ({ (*m.borrow()).insert_35(i.as_pointer()) });
            ({ (*m2.borrow()).insert_35(i.as_pointer()) });
            ({ (*m2.borrow()).insert_35(i.as_pointer()) });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<std___detail__Node_iterator_int__true__false_> =
            Rc::new(RefCell::new(({ (*m2.borrow()).begin_27() })));
        let mut ctr: i32 = 0;
        'loop_: while (*i.borrow()) != ({ (*m2.borrow()).end_29() }) {
            if ((ctr.postfix_inc() % 2) == 0) {
                ({ (*m2.borrow()).erase_47((*i.borrow_mut()).iter_post_inc()) });
            } else {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
            }
        }
        assert!(({ only_deletions_4(m.as_pointer(), m2.as_pointer(),) }));
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
