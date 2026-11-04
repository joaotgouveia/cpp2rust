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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct explicit_allocator_bool_ {}
impl explicit_allocator_bool_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppMove for explicit_allocator_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for explicit_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn tests_4() -> bool {
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(vec![false; 100_usize]));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            match ({ v.decay().begin().iter_add((10_i64 as isize)) },) {
                (mut __tmp_0,) => {
                    let __off = __tmp_0.index();
                    (*v.borrow_mut()).insert(__off, (1 != 0));
                    (__tmp_0).clone()
                }
            },
        ));
        assert!(({ ((&(*v.borrow())).len() == 101_usize) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add((10_i64 as isize)));
        let mut j: usize = 0_usize;
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((v.decay().offset(j as isize).read() as i32) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset(j as isize).read() as i32) == 1));
        j.prefix_inc();
        'loop_: while (j < (&(*v.borrow())).len()) {
            assert!(((v.decay().offset(j as isize).read() as i32) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(vec![false; 100_usize]));
        'loop_: while ((&(*v.borrow())).len() < (*v.borrow()).capacity()) {
            {
                let a0_clone = false.clone();
                (*v.borrow_mut()).push(a0_clone)
            };
        }
        let mut sz: usize = (&(*v.borrow())).len();
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            match ({ v.decay().begin().iter_add((10_i64 as isize)) },) {
                (mut __tmp_1,) => {
                    let __off = __tmp_1.index();
                    (*v.borrow_mut()).insert(__off, (1 != 0));
                    (__tmp_1).clone()
                }
            },
        ));
        assert!(({ ((&(*v.borrow())).len() == (sz).wrapping_add(1_usize)) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add((10_i64 as isize)));
        let mut j: usize = 0_usize;
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((v.decay().offset(j as isize).read() as i32) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset(j as isize).read() as i32) == 1));
        j.prefix_inc();
        'loop_: while (j < (&(*v.borrow())).len()) {
            assert!(((v.decay().offset(j as isize).read() as i32) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(vec![false; 100_usize]));
        'loop_: while ((&(*v.borrow())).len() < (*v.borrow()).capacity()) {
            {
                let a0_clone = false.clone();
                (*v.borrow_mut()).push(a0_clone)
            };
        }
        (*v.borrow_mut()).pop();
        (*v.borrow_mut()).pop();
        let mut sz: usize = (&(*v.borrow())).len();
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            match ({ v.decay().begin().iter_add((10_i64 as isize)) },) {
                (mut __tmp_2,) => {
                    let __off = __tmp_2.index();
                    (*v.borrow_mut()).insert(__off, (1 != 0));
                    (__tmp_2).clone()
                }
            },
        ));
        assert!(({ ((&(*v.borrow())).len() == (sz).wrapping_add(1_usize)) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add((10_i64 as isize)));
        let mut j: usize = 0_usize;
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((v.decay().offset(j as isize).read() as i32) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset(j as isize).read() as i32) == 1));
        j.prefix_inc();
        'loop_: while (j < (&(*v.borrow())).len()) {
            assert!(((v.decay().offset(j as isize).read() as i32) == 0));
            j.prefix_inc();
        }
    }
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(vec![false; 10_usize]));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            match ({ v.decay().begin().iter_add((10_i64 as isize)) },) {
                (mut __tmp_3,) => {
                    let __off = __tmp_3.index();
                    (*v.borrow_mut()).insert(__off, (1 != 0));
                    (__tmp_3).clone()
                }
            },
        ));
        assert!(({ ((&(*v.borrow())).len() == 11_usize) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add((10_i64 as isize)));
        assert!((((*i.borrow()).iter_deref().read() as i32) == 1));
    }
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(vec![false; 100_usize]));
        let i: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(
            match ({ v.decay().begin().iter_add((10_i64 as isize)) },) {
                (mut __tmp_4,) => {
                    let __off = __tmp_4.index();
                    (*v.borrow_mut()).insert(__off, (1 != 0));
                    (__tmp_4).clone()
                }
            },
        ));
        assert!(({ ((&(*v.borrow())).len() == 101_usize) }));
        assert!((*i.borrow()) == v.decay().begin().iter_add((10_i64 as isize)));
        let mut j: usize = 0_usize;
        j = 0_usize;
        'loop_: while (j < 10_usize) {
            assert!(((v.decay().offset(j as isize).read() as i32) == 0));
            j.prefix_inc();
        }
        assert!(((v.decay().offset(j as isize).read() as i32) == 1));
        j.prefix_inc();
        'loop_: while (j < (&(*v.borrow())).len()) {
            assert!(((v.decay().offset(j as isize).read() as i32) == 0));
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
