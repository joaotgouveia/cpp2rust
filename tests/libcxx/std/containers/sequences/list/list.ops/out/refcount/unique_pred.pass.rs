extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_2.with(|rc| *rc.borrow()) >= dealloc_count_3.with(|rc| *rc.borrow())));
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_4: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
pub fn g_5(mut x: i32, mut y: i32) -> bool {
    return (x == y);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct PredLWG526 {
    #[offset(0)]
    pub i_: i32,
}
impl PredLWG526 {
    pub fn new(mut i: i32) -> Self {
        Self { i_: i }
    }
}
impl Callable2<Ptr<PredLWG526>, Ptr<PredLWG526>, bool> for PredLWG526 {
    fn call(&self, a1: Ptr<PredLWG526>, a2: Ptr<PredLWG526>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            PredLWG526Impl::operator_call(__it, a1, a2)
        })
    }
}
pub fn test_6() -> bool {
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 1, 1, 4, 4, 4, 4, 3, 3])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 1, 4, 3])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!((unimplemented!() == 5_()));
        assert!(unimplemented!());
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 2, 3, 5, 5, 2, 11])));
        let mut a2: [i32; 6] = [1, 2, 3, 5, 2, 11];
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 3_()));
        assert!((unimplemented!() == 6_()));
        let mut i: usize = 0_usize;
        'loop_: while (i < (unimplemented!() as usize)) {
            assert!(({ PredLWG526Impl::operator_eq(&unimplemented!(), a2[(i) as usize],) }));
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 1, 1, 4, 4, 4, 4, 3, 3])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 1, 4, 3])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 5_()));
        assert!((unimplemented!()));
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
    assert!(({ test_6() }));
    return 0;
}
pub trait PredLWG526Impl {
    fn destructor(&self);
    fn operator_call(&self, lhs: Ptr<PredLWG526>, rhs: Ptr<PredLWG526>) -> bool;
    fn operator_eq(&self, i: i32) -> bool;
}
impl PredLWG526Impl for Ptr<PredLWG526> {
    fn destructor(&self) {
        field!((*self), i_).write(-32767_i32);
    }
    fn operator_call(&self, lhs: Ptr<PredLWG526>, rhs: Ptr<PredLWG526>) -> bool {
        return ({ lhs.with(|__s| __s.i_) } == { rhs.with(|__s| __s.i_) });
    }
    fn operator_eq(&self, mut i: i32) -> bool {
        return (i == (*self).with(|__s| __s.i_));
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
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
