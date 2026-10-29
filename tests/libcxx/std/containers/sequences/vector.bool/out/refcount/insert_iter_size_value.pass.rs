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
impl min_allocator_unsigned_long_ {
    pub fn new_1(mut _a0: min_allocator_bool_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct explicit_allocator_bool_ {}
impl explicit_allocator_bool_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for explicit_allocator_bool_ {
    fn default() -> Self {
        { explicit_allocator_bool_::new() }
    }
}
impl explicit_allocator_unsigned_long_ {
    pub fn new_1(mut _a0: explicit_allocator_bool_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct explicit_allocator_unsigned_long_ {}
impl explicit_allocator_unsigned_long_ {}
pub fn tests_4() -> bool {
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <bool>::default())
                .collect::<Vec<_>>(),
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new({
            let __it = ({ (*v.borrow()).cbegin() }).iter_add(10_i64);
            let __off = __it.index();
            (*v.borrow_mut()).splice(__off..__off, std::iter::repeat_n((1 != 0), 5_usize));
            __it
        }));
        assert!(((*v.borrow()).len() == 105_usize));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_i64));
        let mut j: usize = <usize>::default();
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((({ (*v.borrow())[j] } as bool) as i32) == 0));
            j.prefix_inc();
        }
        'loop_: while (j < 15_usize) {
            assert!(((({ (*v.borrow())[j] } as bool) as i32) == 1));
            j.prefix_inc();
        }
        j.prefix_inc();
        'loop_: while (j < (*v.borrow()).len()) {
            assert!(((({ (*v.borrow())[j] } as bool) as i32) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <bool>::default())
                .collect::<Vec<_>>(),
        ));
        'loop_: while ((*v.borrow()).len() < (*v.borrow()).capacity()) {
            {
                let a0_clone = false.clone();
                (*v.borrow_mut()).push(a0_clone)
            };
        }
        let mut sz: usize = (*v.borrow()).len();
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new({
            let __it = ({ (*v.borrow()).cbegin() }).iter_add(10_i64);
            let __off = __it.index();
            (*v.borrow_mut()).splice(__off..__off, std::iter::repeat_n((1 != 0), 5_usize));
            __it
        }));
        assert!(((*v.borrow()).len() == (sz).wrapping_add(5_usize)));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_i64));
        let mut j: usize = <usize>::default();
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((({ (*v.borrow())[j] } as bool) as i32) == 0));
            j.prefix_inc();
        }
        'loop_: while (j < 15_usize) {
            assert!(((({ (*v.borrow())[j] } as bool) as i32) == 1));
            j.prefix_inc();
        }
        j.prefix_inc();
        'loop_: while (j < (*v.borrow()).len()) {
            assert!(((({ (*v.borrow())[j] } as bool) as i32) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <bool>::default())
                .collect::<Vec<_>>(),
        ));
        'loop_: while ((*v.borrow()).len() < (*v.borrow()).capacity()) {
            {
                let a0_clone = false.clone();
                (*v.borrow_mut()).push(a0_clone)
            };
        }
        (*v.borrow_mut()).pop();
        (*v.borrow_mut()).pop();
        let mut sz: usize = (*v.borrow()).len();
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new({
            let __it = ({ (*v.borrow()).cbegin() }).iter_add(10_i64);
            let __off = __it.index();
            (*v.borrow_mut()).splice(__off..__off, std::iter::repeat_n((1 != 0), 5_usize));
            __it
        }));
        assert!(((*v.borrow()).len() == (sz).wrapping_add(5_usize)));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_i64));
        let mut j: usize = <usize>::default();
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((({ (*v.borrow())[j] } as bool) as i32) == 0));
            j.prefix_inc();
        }
        'loop_: while (j < 15_usize) {
            assert!(((({ (*v.borrow())[j] } as bool) as i32) == 1));
            j.prefix_inc();
        }
        j.prefix_inc();
        'loop_: while (j < (*v.borrow()).len()) {
            assert!(((({ (*v.borrow())[j] } as bool) as i32) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(
            (0..(10_usize) as usize)
                .map(|_| <bool>::default())
                .collect::<Vec<_>>(),
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new({
            let __it = ({ (*v.borrow()).cbegin() }).iter_add(10_i64);
            let __off = __it.index();
            (*v.borrow_mut()).splice(__off..__off, std::iter::repeat_n((1 != 0), 5_usize));
            __it
        }));
        assert!(((*v.borrow()).len() == 15_usize));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_i64));
    }
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(
            (0..(100_usize) as usize)
                .map(|_| <bool>::default())
                .collect::<Vec<_>>(),
        ));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new({
            let __it = ({ (*v.borrow()).cbegin() }).iter_add(10_i64);
            let __off = __it.index();
            (*v.borrow_mut()).splice(__off..__off, std::iter::repeat_n((1 != 0), 5_usize));
            __it
        }));
        assert!(((*v.borrow()).len() == 105_usize));
        assert!((*i.borrow()) == v.decay().begin().iter_add(10_i64));
        let mut j: usize = <usize>::default();
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((((*v.borrow())[j] as bool) as i32) == 0));
            j.prefix_inc();
        }
        'loop_: while (j < 15_usize) {
            assert!(((((*v.borrow())[j] as bool) as i32) == 1));
            j.prefix_inc();
        }
        j.prefix_inc();
        'loop_: while (j < (*v.borrow()).len()) {
            assert!(((((*v.borrow())[j] as bool) as i32) == 0));
            j.prefix_inc();
        }
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
pub trait explicit_allocator_bool_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: usize) {
        unimplemented!()
    }
}
impl explicit_allocator_bool_Impl for Ptr<explicit_allocator_bool_> {}
pub trait explicit_allocator_unsigned_long_Impl {
    fn allocate(&self, n: usize) -> Ptr<u64>;
    fn deallocate(&self, p: Ptr<u64>, n: usize);
}
impl explicit_allocator_unsigned_long_Impl for Ptr<explicit_allocator_unsigned_long_> {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        return ({ <std_allocator_unsigned_long_>::default().allocate(n) })
            .reinterpret_cast::<u64>();
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut n: usize) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
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
