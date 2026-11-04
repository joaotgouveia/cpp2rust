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
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static randomness_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct Payload {
    #[offset(0)]
    pub val: i32,
    #[offset(4)]
    pub side: i32,
}
impl Payload {
    pub fn new_1(mut v: i32) -> Self {
        Self { val: v, side: 0 }
    }
    pub fn new_2(mut v: i32, mut s: i32) -> Self {
        Self { val: v, side: s }
    }
}
impl CppLess for Payload {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PayloadImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Payload {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(PayloadImpl::operator_lt(__lhs, __rhs.clone()))
            && !(PayloadImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
impl CppMove for Payload {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn greater_5(lhs: Ptr<Payload>, rhs: Ptr<Payload>) -> bool {
    return ({ lhs.with(|__s| __s.val) } > { rhs.with(|__s| __s.val) });
}
pub fn test_stable_6(mut N: i32) {
    let v: Value<Vec<Payload>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        {
            let __a1 = Payload::new_1({ (i / 2) });
            (*v.borrow_mut()).push(__a1)
        };
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let __rhs = i;
        field!(v.decay().offset((i as usize) as isize), side).write(__rhs);
        i.prefix_inc();
    }
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!((unimplemented!() == (N as isize)));
    let j: Value<()> = Rc::new(RefCell::new(<BidirectionalIterator<Payload>>::from(
        unimplemented!(),
    )));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(({ ((*j.borrow()).iter_deref().with(|__s| __s.val) == (((N - 1) - i) / 2)) }));
        {
            i.prefix_inc();
            (*j.borrow_mut()).iter_inc()
        };
    }
    let it: Value<()> = Rc::new(RefCell::new(<BidirectionalIterator<Payload>>::from(
        unimplemented!(),
    )));
    'loop_: while (*it.borrow()) != <BidirectionalIterator<Payload>>::from(unimplemented!()) {
        let next: Value<()> = Rc::new(RefCell::new(libcc2rs::next_refcount(
            ({ (*it.borrow()).clone() }),
            1_isize,
        )));
        if ((*next.borrow()) != <BidirectionalIterator<Payload>>::from(unimplemented!()))
            && ((*it.borrow()).iter_deref().with(|__s| __s.val)
                == (*next.borrow()).iter_deref().with(|__s| __s.val))
        {
            assert!(
                ({
                    ((*it.borrow()).iter_deref().with(|__s| __s.side)
                        < (*next.borrow()).iter_deref().with(|__s| __s.side))
                })
            );
        }
        (*it.borrow_mut()).iter_inc();
    }
}
pub fn test_7() -> bool {
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            4, 8, 1, 0, 5, 7, 2, 3, 6, 11, 10, 9,
        ])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
        ])));
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        {
            let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                4, 8, 1, 0, 5, 7, 2, 3, 6, 11, 10, 9,
            ])));
            let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
            ])));
            let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            assert!((unimplemented!()));
        }
        let mut i: i32 = 0;
        'loop_: while (i < 40) {
            ({ test_stable_6(i) });
            i.prefix_inc();
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
    assert!(({ test_7() }));
    return 0;
}
pub trait PayloadImpl {
    fn operator_lt(&self, rhs: Ptr<Payload>) -> bool;
}
impl PayloadImpl for Ptr<Payload> {
    fn operator_lt(&self, rhs: Ptr<Payload>) -> bool {
        return ({ (*self).with(|__s| __s.val) } < { rhs.with(|__s| __s.val) });
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
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = randomness_4.with(|_| ());
}
