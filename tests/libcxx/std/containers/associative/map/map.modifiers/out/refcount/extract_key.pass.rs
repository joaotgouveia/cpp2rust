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
pub fn operator_eq_4(
    mut _a0: min_allocator_std_pair_const_int__int__,
    mut _a1: min_allocator_std_pair_const_int__int__,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__int__ {}
impl CppEq for min_allocator_std_pair_const_int__int__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.read(), __rhs.read())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Counter_base {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Destructor, Default)]
#[cpp_from(i32, new_1, ptr)]
#[byte_size(4)]
pub struct Counter_int_ {
    #[offset(0)]
    data_: i32,
}
impl Counter_int_ {
    pub fn new_1(data: Ptr<i32>) -> Self {
        let __this: Counter_int_ = Self {
            data_: (data.read()),
        };
        (*gConstructed_5.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(rhs: Ptr<Counter_int_>) -> Self {
        let __this: Counter_int_ = Self {
            data_: rhs.with(|__s| __s.data_),
        };
        (*gConstructed_5.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for Counter_int_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        Counter_int_Impl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Counter_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        Counter_int_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        let __src: Value<Counter_int_> = Rc::new(RefCell::new(Counter_int_ {
            data_: self.data_.clone(),
        }));
        Counter_int_::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static gConstructed_5: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn test_6(c: Ptr<Map<i32, (Value<i32>, Value<i32>)>>, mut first: Ptr<i32>, mut last: Ptr<i32>) {
    let mut sz: usize = (*c.upgrade().deref()).len();
    assert!(({ (unimplemented!() as usize) } == { sz }));
    let mut copy: Ptr<i32> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(!(unimplemented!()));
        sz.prefix_dec();
        {
            assert!(({ (unimplemented!().read()) } == { (copy.read()) }));
            let __rhs = (first.read());
            unimplemented!().write(__rhs);
            assert!(({ (unimplemented!().read()) } == { (first.read()) }));
        }
        assert!(unimplemented!());
        assert!(({ sz } == { (*c.upgrade().deref()).len() }));
        copy.prefix_inc();
    }
    assert!(((*c.upgrade().deref()).len() == 0_usize));
    let mut copy: Ptr<i32> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        copy.prefix_inc();
    }
}
pub fn test_7(
    c: Ptr<Map<Counter_int_, (Value<Counter_int_>, Value<Counter_int_>)>>,
    mut first: Ptr<Counter_int_>,
    mut last: Ptr<Counter_int_>,
) {
    let mut sz: usize = (*c.upgrade().deref()).len();
    assert!(({ (unimplemented!() as usize) } == { sz }));
    let mut copy: Ptr<Counter_int_> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(!(unimplemented!()));
        sz.prefix_dec();
        {
            assert!(
                ({
                    let _x: Ptr<Counter_int_> = (copy).clone();
                    Counter_int_Impl::operator_eq(&unimplemented!(), _x)
                })
            );
            ({
                let _rhs: Ptr<Counter_int_> = (first).clone();
                Counter_int_Impl::copy_assign(&unimplemented!(), _rhs)
            });
            assert!(
                ({
                    let _x: Ptr<Counter_int_> = (first).clone();
                    Counter_int_Impl::operator_eq(&unimplemented!(), _x)
                })
            );
        }
        assert!(unimplemented!());
        assert!(({ sz } == { (*c.upgrade().deref()).len() }));
        copy.prefix_inc();
    }
    assert!(((*c.upgrade().deref()).len() == 0_usize));
    let mut copy: Ptr<Counter_int_> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        copy.prefix_inc();
    }
}
pub fn test_8(c: Ptr<Map<i32, (Value<i32>, Value<i32>)>>, mut first: Ptr<i32>, mut last: Ptr<i32>) {
    let mut sz: usize = (*c.upgrade().deref()).len();
    assert!(({ (unimplemented!() as usize) } == { sz }));
    let mut copy: Ptr<i32> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(!(unimplemented!()));
        sz.prefix_dec();
        {
            assert!(({ (unimplemented!().read()) } == { (copy.read()) }));
            let __rhs = (first.read());
            unimplemented!().write(__rhs);
            assert!(({ (unimplemented!().read()) } == { (first.read()) }));
        }
        assert!(
            ({
                let _arg0: min_allocator_std_pair_const_int__int__ = unimplemented!();
                operator_eq_4(_arg0, unimplemented!())
            })
        );
        assert!(({ sz } == { (*c.upgrade().deref()).len() }));
        copy.prefix_inc();
    }
    assert!(((*c.upgrade().deref()).len() == 0_usize));
    let mut copy: Ptr<i32> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        copy.prefix_inc();
    }
}
pub fn test_9() -> bool {
    {
        let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(unimplemented!()));
        let keys: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
        ({
            let _first: Ptr<i32> = unimplemented!();
            let _last: Ptr<i32> = (keys.as_pointer() as Ptr<i32>).to_end();
            test_6(m.as_pointer(), _first, _last)
        });
    }
    {
        let m: Value<Map<Counter_int_, (Value<Counter_int_>, Value<Counter_int_>)>> =
            Rc::new(RefCell::new(unimplemented!()));
        {
            let keys: Value<Box<[Counter_int_]>> = Rc::new(RefCell::new(Box::new([
                {
                    let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
                    Counter_int_::new_1({ __tmp_0.as_pointer() })
                },
                {
                    let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
                    Counter_int_::new_1({ __tmp_1.as_pointer() })
                },
                {
                    let __tmp_2: Value<i32> = Rc::new(RefCell::new(3));
                    Counter_int_::new_1({ __tmp_2.as_pointer() })
                },
                {
                    let __tmp_3: Value<i32> = Rc::new(RefCell::new(4));
                    Counter_int_::new_1({ __tmp_3.as_pointer() })
                },
                {
                    let __tmp_4: Value<i32> = Rc::new(RefCell::new(5));
                    Counter_int_::new_1({ __tmp_4.as_pointer() })
                },
                {
                    let __tmp_5: Value<i32> = Rc::new(RefCell::new(6));
                    Counter_int_::new_1({ __tmp_5.as_pointer() })
                },
            ])));
            assert!((gConstructed_5.with(|rc| *rc.borrow()) == (12 + 6)));
            ({
                let _first: Ptr<Counter_int_> = unimplemented!();
                let _last: Ptr<Counter_int_> = (keys.as_pointer() as Ptr<Counter_int_>).to_end();
                test_7(m.as_pointer(), _first, _last)
            });
        }
        assert!((gConstructed_5.with(|rc| *rc.borrow()) == 0));
    }
    {
        let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(unimplemented!()));
        let keys: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6])));
        ({
            let _first: Ptr<i32> = unimplemented!();
            let _last: Ptr<i32> = (keys.as_pointer() as Ptr<i32>).to_end();
            test_8(m.as_pointer(), _first, _last)
        });
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
    ({ test_9() });
    return 0;
}
pub trait Counter_int_Impl {
    fn copy_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_>;
    fn move_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn destructor(&self);
    fn get(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> bool;
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> bool;
}
impl Counter_int_Impl for Ptr<Counter_int_> {
    fn copy_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        field!((*self), data_).write({ rhs.with(|__s| __s.data_) });
        return (*self).clone();
    }
    fn destructor(&self) {
        (*gConstructed_5.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } == { x.with(|__s| __s.data_) });
    }
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } < { x.with(|__s| __s.data_) });
    }
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
    let _ = gConstructed_5.with(|_| ());
}
