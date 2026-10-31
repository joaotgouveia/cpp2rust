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
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<(Value<i32>, Value<i32>)>,
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(AnyPtr, new_1, value)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_2(mut p: Ptr<i32>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_1(mut _a0: AnyPtr) -> Self {
        Self {
            ptr_: Ptr::<i32>::null(),
        }
    }
    pub fn pointer_to(t: Ptr<i32>) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_2({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__int__ {}
pub fn node_factory_4(c: Ptr<()>, key: Ptr<()>, mapped: Ptr<()>) -> () {
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return unimplemented!();
}
pub fn node_factory_5(c: Ptr<()>, key: Ptr<()>, mapped: Ptr<()>) -> () {
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return unimplemented!();
}
pub fn test_6(c: Ptr<()>) {
    let mut nf: FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>) -> ()> =
        (FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>) -> ()>::new(node_factory_4));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) != 10) {
        let node: Value<()> = Rc::new(RefCell::new(
            ({
                let _arg1: Ptr<()> = (i.as_pointer()).reinterpret_cast::<()>();
                let _arg2: Value<()> = Rc::new(RefCell::new((((*i.borrow()) + 1) as ())));
                nf.call(c2.as_pointer(), _arg1, _arg2.as_pointer())
            }),
        ));
        assert!(!(unimplemented!()));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        assert!(((*it.borrow()) == unimplemented!()) && ((*it.borrow()) != unimplemented!()));
        (*i.borrow_mut()).prefix_inc();
    }
    assert!((unimplemented!() == 10_()));
    {
        let def: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        assert!((*it.borrow()) == unimplemented!());
    }
    {
        let dupl: Value<()> = Rc::new(RefCell::new(
            ({
                let _arg1: Value<()> = Rc::new(RefCell::new((0 as ())));
                let _arg2: Value<()> = Rc::new(RefCell::new((42 as ())));
                nf.call(c2.as_pointer(), _arg1.as_pointer(), _arg2.as_pointer())
            }),
        ));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        assert!((*it.borrow()) != unimplemented!());
        assert!(((*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 42));
    }
    assert!((unimplemented!() == 11_()));
    assert!((unimplemented!() == 2_()));
    let mut i: i32 = 1;
    'loop_: while (i != 10) {
        assert!((unimplemented!() == 1_()));
        assert!(
            ({
                (*(*unimplemented!().iter_deref().upgrade().deref())
                    .1
                    .borrow())
            } == { (i + 1) })
        );
        i.prefix_inc();
    }
}
pub fn test_7(c: Ptr<()>) {
    let mut nf: FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>) -> ()> =
        (FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>) -> ()>::new(node_factory_5));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) != 10) {
        let node: Value<()> = Rc::new(RefCell::new(
            ({
                let _arg1: Ptr<()> = (i.as_pointer()).reinterpret_cast::<()>();
                let _arg2: Value<()> = Rc::new(RefCell::new((((*i.borrow()) + 1) as ())));
                nf.call(c2.as_pointer(), _arg1, _arg2.as_pointer())
            }),
        ));
        assert!(!(unimplemented!()));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        assert!(((*it.borrow()) == unimplemented!()) && ((*it.borrow()) != unimplemented!()));
        (*i.borrow_mut()).prefix_inc();
    }
    assert!((unimplemented!() == 10_()));
    {
        let def: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        assert!((*it.borrow()) == unimplemented!());
    }
    {
        let dupl: Value<()> = Rc::new(RefCell::new(
            ({
                let _arg1: Value<()> = Rc::new(RefCell::new((0 as ())));
                let _arg2: Value<()> = Rc::new(RefCell::new((42 as ())));
                nf.call(c2.as_pointer(), _arg1.as_pointer(), _arg2.as_pointer())
            }),
        ));
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        assert!((*it.borrow()) != unimplemented!());
        assert!(((*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 42));
    }
    assert!((unimplemented!() == 11_()));
    assert!((unimplemented!() == 2_()));
    let mut i: i32 = 1;
    'loop_: while (i != 10) {
        assert!((unimplemented!() == 1_()));
        assert!(
            ({
                (*(*unimplemented!().iter_deref().upgrade().deref())
                    .1
                    .borrow())
            } == { (i + 1) })
        );
        i.prefix_inc();
    }
}
pub fn test_8() -> bool {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_6(m.as_pointer()) });
    let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_7(m2.as_pointer()) });
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
    ({ test_8() });
    return 0;
}
pub trait min_allocator_std_pair_const_int__int__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__int__Impl for Ptr<min_allocator_std_pair_const_int__int__> {}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32>;
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
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
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
