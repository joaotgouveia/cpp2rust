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
pub fn operator_lt_4(lhs: Ptr<value>, rhs: Ptr<value>) -> bool {
    return ({ lhs.with(|__s| __s.a) } < { rhs.with(|__s| __s.a) });
}
pub fn operator_eq_5(lhs: Ptr<value>, rhs: Ptr<value>) -> bool {
    return ({ lhs.with(|__s| __s.a) } == { rhs.with(|__s| __s.a) })
        && ({ lhs.with(|__s| __s.b) } == { rhs.with(|__s| __s.b) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct value {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl CppLess for value {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for value {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
pub fn test_6() -> bool {
    {
        let t1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 5, 6, 7, 12, 13])));
        let t2: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([0, 1, 2, 4, 8, 9, 10, 11, 14, 15])));
        let t3: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ])));
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(unimplemented!());
        let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let to: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut io: [(); 3] = [
            unimplemented!(),
            (*unimplemented!().iter_inc().upgrade().deref()).clone(),
            (*unimplemented!()
                .iter_inc()
                .with_mut(|__v: &mut ()| __v.iter_inc())
                .upgrade()
                .deref())
            .clone(),
        ];
        let mut ro: [Ptr<i32>; 3] = [
            io[(0) as usize].iter_deref(),
            io[(1) as usize].iter_deref(),
            io[(2) as usize].iter_deref(),
        ];
        let mut po: [Ptr<i32>; 3] = [
            (io[(0) as usize].iter_deref()),
            (io[(1) as usize].iter_deref()),
            (io[(2) as usize].iter_deref()),
        ];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(unimplemented!());
        let mut i: usize = 0_usize;
        'loop_: while (i < 3_usize) {
            assert!(((*to.borrow())[(i) as usize] == (io[(i) as usize].iter_deref().read())));
            assert!(((*to.borrow())[(i) as usize] == (ro[(i) as usize].read())));
            assert!(((*to.borrow())[(i) as usize] == (po[(i) as usize].read())));
            i.prefix_inc();
        }
    }
    {
        let t1: Value<Box<[value]>> = Rc::new(RefCell::new(Box::new([
            value { a: 0, b: 0 },
            value { a: 2, b: 0 },
            value { a: 3, b: 0 },
        ])));
        let t2: Value<Box<[value]>> = Rc::new(RefCell::new(Box::new([
            value { a: 0, b: 1 },
            value { a: 1, b: 1 },
            value { a: 2, b: 1 },
            value { a: 4, b: 1 },
        ])));
        let t3: Value<Box<[value]>> = Rc::new(RefCell::new(Box::new([
            value { a: 0, b: 0 },
            value { a: 0, b: 1 },
            value { a: 1, b: 1 },
            value { a: 2, b: 0 },
            value { a: 2, b: 1 },
            value { a: 3, b: 0 },
            value { a: 4, b: 1 },
        ])));
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(unimplemented!());
        let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let t1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 5, 6, 7, 12, 13])));
        let t2: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([0, 1, 2, 4, 8, 9, 10, 11, 14, 15])));
        let t3: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ])));
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(unimplemented!());
        let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(unimplemented!());
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
