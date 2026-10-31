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
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let j: Value<()> = Rc::new(RefCell::new(libcc2rs::next_refcount(
            (*i.borrow()).clone(),
            2_isize,
        )));
        let k: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((*k.borrow()) == (*i.borrow()));
        assert!((unimplemented!() == 6_()));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 1_()));
        (*k.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 4_()));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 1_()));
        (*k.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 0_()));
        assert!((*k.borrow()) == unimplemented!());
    }
    {
        let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut collision_val: usize = ((2_()).wrapping_add(unimplemented!()) as usize);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        let erase: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
            let v: Ptr<u64> = v.iter_deref();
            assert!(((v.read()) == 1_u64) || ({ ((v.read()) as usize) } == { collision_val }));
        }
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        {
            let pair: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
            assert!((*(*pair.borrow()).0.borrow()) != (*(*pair.borrow()).1.borrow()));
            unimplemented!();
        }
        {
            let pair: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
            assert!((*(*pair.borrow()).0.borrow()) != (*(*pair.borrow()).1.borrow()));
            unimplemented!();
        }
        unimplemented!();
        assert!((unimplemented!() == 1_()));
        assert!(((unimplemented!().iter_deref().read()) == 3));
        assert!((unimplemented!().iter_inc()).clone() == unimplemented!());
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let j: Value<()> = Rc::new(RefCell::new(libcc2rs::next_refcount(
            (*i.borrow()).clone(),
            2_isize,
        )));
        let k: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((*k.borrow()) == (*i.borrow()));
        assert!((unimplemented!() == 6_()));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 1_()));
        (*k.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 4_()));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 1_()));
        (*k.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 0_()));
        assert!((*k.borrow()) == unimplemented!());
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
}
