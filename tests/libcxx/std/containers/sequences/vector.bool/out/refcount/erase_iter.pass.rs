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
pub struct min_allocator_bool_ {}
impl CppMove for min_allocator_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_unsigned_long_ {
    pub fn new_1(mut _a0: min_allocator_bool_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_bool_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl CppMove for min_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_unsigned_long_ {}
pub fn tests_4() -> bool {
    let a1: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([(1 != 0), (0 != 0), (1 != 0)])));
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_0,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_0 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> =
            Rc::new(RefCell::new(<RandomAccessIterator<bool>>::from(
                l1.decay().begin(),
            )));
        (*i.borrow_mut()).iter_inc();
        let j: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({ (*l1.borrow_mut()).erase_60((*i.borrow()).clone()) }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 2_usize) }));
        assert!(
            (({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> = l1.decay().end();
                distance_5(___first, ___last)
            }) == 2_i64)
        );
        assert!((((*j.borrow()).iter_deref().read() as i32) == (true as i32)));
        assert!(((l1.decay().begin().iter_deref().read() as i32) == 1));
        assert!(
            ((libcc2rs::next_refcount(l1.decay().begin(), 1_i64)
                .iter_deref()
                .read() as i32)
                == (true as i32))
        );
        {
            (*j.borrow_mut()) = ({
                (*l1.borrow_mut()).erase_60(<RandomAccessIterator<bool>>::from(
                    ({ (*j.borrow()).clone() }),
                ))
            });
            (*j.borrow()).clone()
        };
        assert!((*j.borrow()) == l1.decay().end());
        assert!(({ ((&(*l1.borrow())).len() == 1_usize) }));
        assert!(
            (({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> = l1.decay().end();
                distance_5(___first, ___last)
            }) == 1_i64)
        );
        assert!(((l1.decay().begin().iter_deref().read() as i32) == (true as i32)));
        {
            (*j.borrow_mut()) = ({
                let ___position: RandomAccessIterator<bool> =
                    <RandomAccessIterator<bool>>::from(l1.decay().begin());
                (*l1.borrow_mut()).erase_60(___position)
            });
            (*j.borrow()).clone()
        };
        assert!((*j.borrow()) == l1.decay().end());
        assert!(({ ((&(*l1.borrow())).len() == 0_usize) }));
        assert!(
            (({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> = l1.decay().end();
                distance_5(___first, ___last)
            }) == 0_i64)
        );
    }
    {
        let l1: Value<Vec<bool>> = Rc::new(RefCell::new(
            match ({ (a1.as_pointer() as Ptr<bool>).offset((3) as isize) },) {
                (mut __tmp_1,) => {
                    let mut __v = Vec::new();
                    let mut __it = (a1.as_pointer() as Ptr<bool>);
                    while __it != __tmp_1 {
                        __v.push(__it.iter_deref().read() != false);
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        let i: Value<RandomAccessIterator<bool>> =
            Rc::new(RefCell::new(<RandomAccessIterator<bool>>::from(
                l1.decay().begin(),
            )));
        (*i.borrow_mut()).iter_inc();
        let j: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            ({ (*l1.borrow_mut()).erase_60((*i.borrow()).clone()) }),
        ));
        assert!(({ ((&(*l1.borrow())).len() == 2_usize) }));
        assert!(
            (({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> = l1.decay().end();
                distance_5(___first, ___last)
            }) == 2_i64)
        );
        assert!((((*j.borrow()).iter_deref().read() as i32) == (true as i32)));
        assert!(((l1.decay().begin().iter_deref().read() as i32) == 1));
        assert!(
            ((libcc2rs::next_refcount(l1.decay().begin(), 1_i64)
                .iter_deref()
                .read() as i32)
                == (true as i32))
        );
        {
            (*j.borrow_mut()) = ({
                (*l1.borrow_mut()).erase_60(<RandomAccessIterator<bool>>::from(
                    ({ (*j.borrow()).clone() }),
                ))
            });
            (*j.borrow()).clone()
        };
        assert!((*j.borrow()) == l1.decay().end());
        assert!(({ ((&(*l1.borrow())).len() == 1_usize) }));
        assert!(
            (({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> = l1.decay().end();
                distance_5(___first, ___last)
            }) == 1_i64)
        );
        assert!(((l1.decay().begin().iter_deref().read() as i32) == (true as i32)));
        {
            (*j.borrow_mut()) = ({
                let ___position: RandomAccessIterator<bool> =
                    <RandomAccessIterator<bool>>::from(l1.decay().begin());
                (*l1.borrow_mut()).erase_60(___position)
            });
            (*j.borrow()).clone()
        };
        assert!((*j.borrow()) == l1.decay().end());
        assert!(({ ((&(*l1.borrow())).len() == 0_usize) }));
        assert!(
            (({
                let ___first: RandomAccessIterator<bool> = l1.decay().begin();
                let ___last: RandomAccessIterator<bool> = l1.decay().end();
                distance_5(___first, ___last)
            }) == 0_i64)
        );
    }
    return true;
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
    ({ tests_4() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__;
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_unsigned_long_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({
            let ___p: Ptr<u64> = (p.ptr_).clone();
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
