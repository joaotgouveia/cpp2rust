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
        let i: Value<std___detail__Node_const_iterator_int__true__false_> = Rc::new(RefCell::new(
            <std___detail__Node_const_iterator_int__true__false_>::from(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(2));
                    (*c.borrow()).find_58(___x.as_pointer())
                }),
            ),
        ));
        let j: Value<std___detail__Node_const_iterator_int__true__false_> = Rc::new(RefCell::new(
            libcc2rs::next_refcount(({ (*i.borrow()).clone() }), 2_isize),
        ));
        let k: Value<std___detail__Node_iterator_int__true__false_> = Rc::new(RefCell::new(
            ({
                let ___first: std___detail__Node_const_iterator_int__true__false_ =
                    (*i.borrow()).clone();
                let ___last: std___detail__Node_const_iterator_int__true__false_ =
                    (*i.borrow()).clone();
                (*c.borrow()).erase_49(___first, ___last)
            }),
        ));
        assert!((*k.borrow()) == (*i.borrow()));
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
        (*k.borrow_mut()) =
            ({ (*c.borrow()).erase_49((*i.borrow()).clone(), (*j.borrow()).clone()) });
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
        (*k.borrow_mut()) = ({
            let ___first: std___detail__Node_const_iterator_int__true__false_ =
                ({ (*c.borrow()).cbegin_31() });
            let ___last: std___detail__Node_const_iterator_int__true__false_ =
                ({ (*c.borrow()).cend_32() });
            (*c.borrow()).erase_49(___first, ___last)
        });
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
        assert!((*k.borrow()) == ({ (*c.borrow()).end_29() }));
    }
    {
        let map : Value<std_unordered_multiset_unsigned_long__MyHash__std_equal_to_unsigned_long___std_allocator_unsigned_long__ > = Rc::new(RefCell::new(<std_unordered_multiset_unsigned_long__MyHash__std_equal_to_unsigned_long___std_allocator_unsigned_long__  >::default() )) ;
        let collision_val: Value<usize> = Rc::new(RefCell::new(
            ((2_u64).wrapping_add(({ (*map.borrow()).bucket_count() })) as usize),
        ));
        ({ (*map.borrow()).rehash(3_u64) });
        ({
            let ___x: Value<u64> = Rc::new(RefCell::new(1_u64));
            (*map.borrow()).insert_36(___x.as_pointer())
        });
        ({ (*map.borrow()).insert_35((collision_val.as_pointer()).reinterpret_cast::<u64>()) });
        ({
            let ___x: Value<u64> = Rc::new(RefCell::new(2_u64));
            (*map.borrow()).insert_36(___x.as_pointer())
        });
        const _: () = assert!(true, "");;
        let erase: Value<(
            Value<std___detail__Node_iterator_unsigned_long__true__true_>,
            Value<std___detail__Node_iterator_unsigned_long__true__true_>,
        )> = Rc::new(RefCell::new(
            ({
                let ___x: Value<u64> = Rc::new(RefCell::new(2_u64));
                (*map.borrow()).equal_range_66(___x.as_pointer())
            }),
        ));
        ({
            let ___first: std___detail__Node_const_iterator_unsigned_long__true__true_ =
                <std___detail__Node_const_iterator_unsigned_long__true__true_>::from(
                    ({ (*(*erase.borrow()).0.borrow()).clone() }),
                );
            let ___last: std___detail__Node_const_iterator_unsigned_long__true__true_ =
                <std___detail__Node_const_iterator_unsigned_long__true__true_>::from(
                    ({ (*(*erase.borrow()).1.borrow()).clone() }),
                );
            (*map.borrow()).erase_49(___first, ___last)
        });
        'loop_: for v in IterRange::new(
            ({ (*map.borrow()).begin_27() }),
            ({ (*map.borrow()).end_29() }),
        ) {
            let v: Ptr<u64> = v.iter_deref();
            assert!(
                ({
                    ((v.read()) == 1_u64)
                        || ({ ((v.read()) as usize) } == { (*collision_val.borrow()) })
                })
            );
        }
    }
    {
        let m : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__ > = Rc::new(RefCell::new(<std_unordered_multiset_int__std_hash_int___std_equal_to_int___std_allocator_int__  >::default() )) ;
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(1));
            (*m.borrow()).insert_36(___x.as_pointer())
        });
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(2));
            (*m.borrow()).insert_36(___x.as_pointer())
        });
        {
            let pair: Value<(
                Value<std___detail__Node_iterator_int__true__false_>,
                Value<std___detail__Node_iterator_int__true__false_>,
            )> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(1));
                    (*m.borrow()).equal_range_66(___x.as_pointer())
                }),
            ));
            assert!((*(*pair.borrow()).0.borrow()) != (*(*pair.borrow()).1.borrow()));
            ({
                let ___first: std___detail__Node_const_iterator_int__true__false_ =
                    <std___detail__Node_const_iterator_int__true__false_>::from(
                        ({ (*(*pair.borrow()).0.borrow()).clone() }),
                    );
                let ___last: std___detail__Node_const_iterator_int__true__false_ =
                    <std___detail__Node_const_iterator_int__true__false_>::from(
                        ({ (*(*pair.borrow()).1.borrow()).clone() }),
                    );
                (*m.borrow()).erase_49(___first, ___last)
            });
        }
        {
            let pair: Value<(
                Value<std___detail__Node_iterator_int__true__false_>,
                Value<std___detail__Node_iterator_int__true__false_>,
            )> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(2));
                    (*m.borrow()).equal_range_66(___x.as_pointer())
                }),
            ));
            assert!((*(*pair.borrow()).0.borrow()) != (*(*pair.borrow()).1.borrow()));
            ({
                let ___first: std___detail__Node_const_iterator_int__true__false_ =
                    <std___detail__Node_const_iterator_int__true__false_>::from(
                        ({ (*(*pair.borrow()).0.borrow()).clone() }),
                    );
                let ___last: std___detail__Node_const_iterator_int__true__false_ =
                    <std___detail__Node_const_iterator_int__true__false_>::from(
                        ({ (*(*pair.borrow()).1.borrow()).clone() }),
                    );
                (*m.borrow()).erase_49(___first, ___last)
            });
        }
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(3));
            (*m.borrow()).insert_36(___x.as_pointer())
        });
        assert!(({ (({ (*m.borrow()).size() }) == 1_u64) }));
        assert!(((({ (*m.borrow()).begin_27() }).iter_deref().read()) == 3));
        assert!(
            (*match (Rc::new(RefCell::new(({ (*m.borrow()).begin_27() }))).as_pointer(),) {
                (__tmp_0,) => {
                    {
                        __tmp_0.with_mut(|__it| {
                            __it.iter_inc();
                        });
                        (__tmp_0).clone()
                    }
                }
            }
            .upgrade()
            .deref())
                == ({ (*m.borrow()).end_29() })
        );
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c : Value<std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ > = Rc::new(RefCell::new(std_unordered_multiset_int__std_hash_int___std_equal_to_int___min_allocator_int__ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ( a .as_pointer()  as Ptr::<i32> ) . offset ( ( ( ( ::std::mem::size_of::<[ i32 ; 6]>() as usize ) ) . wrapping_div ( ( ::std::mem::size_of::<i32>() as usize ) ) ) as isize )  } , None , None , None , None , )  )) ;
        let i: Value<std___detail__Node_const_iterator_int__true__false_> = Rc::new(RefCell::new(
            <std___detail__Node_const_iterator_int__true__false_>::from(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(2));
                    (*c.borrow()).find_58(___x.as_pointer())
                }),
            ),
        ));
        let j: Value<std___detail__Node_const_iterator_int__true__false_> = Rc::new(RefCell::new(
            libcc2rs::next_refcount(({ (*i.borrow()).clone() }), 2_isize),
        ));
        let k: Value<std___detail__Node_iterator_int__true__false_> = Rc::new(RefCell::new(
            ({
                let ___first: std___detail__Node_const_iterator_int__true__false_ =
                    (*i.borrow()).clone();
                let ___last: std___detail__Node_const_iterator_int__true__false_ =
                    (*i.borrow()).clone();
                (*c.borrow()).erase_49(___first, ___last)
            }),
        ));
        assert!((*k.borrow()) == (*i.borrow()));
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
        (*k.borrow_mut()) =
            ({ (*c.borrow()).erase_49((*i.borrow()).clone(), (*j.borrow()).clone()) });
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
        (*k.borrow_mut()) = ({
            let ___first: std___detail__Node_const_iterator_int__true__false_ =
                ({ (*c.borrow()).cbegin_31() });
            let ___last: std___detail__Node_const_iterator_int__true__false_ =
                ({ (*c.borrow()).cend_32() });
            (*c.borrow()).erase_49(___first, ___last)
        });
        assert!(({ (({ (*c.borrow()).size() }) == 0_u64) }));
        assert!((*k.borrow()) == ({ (*c.borrow()).end_29() }));
    }
    return 0;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyHash_4 {}
impl Callable1<usize, usize> for MyHash_4 {
    fn call(&self, a1: usize) -> usize {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            MyHash_4Impl::operator_call(__it, a1)
        })
    }
}
impl CppMove for MyHash_4 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait MyHash_4Impl {
    fn operator_call(&self, v: usize) -> usize;
}
impl MyHash_4Impl for Ptr<MyHash_4> {
    fn operator_call(&self, v: usize) -> usize {
        let v: Value<usize> = Rc::new(RefCell::new(v));
        return (*v.borrow());
    }
}
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
