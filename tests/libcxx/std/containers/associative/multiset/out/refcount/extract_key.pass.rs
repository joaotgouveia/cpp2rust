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
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_4(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppEq for min_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.read(), __rhs.read())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Counter_base {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Counter_int_ {
    #[offset(0)]
    data_: (),
}
impl Counter_int_ {
    pub fn new_1(data: Ptr<()>) -> Self {
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
    pub static gConstructed_5: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn test_6(c: Ptr<()>, mut first: Ptr<()>, mut last: Ptr<()>) {
    let mut sz: () = unimplemented!();
    unimplemented!();
    let mut copy: Ptr<()> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        sz.prefix_dec();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        copy.prefix_inc();
    }
    unimplemented!();
    let mut copy: Ptr<()> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        copy.prefix_inc();
    }
}
pub fn test_7(c: Ptr<()>, mut first: Ptr<Counter_int_>, mut last: Ptr<Counter_int_>) {
    let mut sz: () = unimplemented!();
    unimplemented!();
    let mut copy: Ptr<Counter_int_> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        sz.prefix_dec();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        copy.prefix_inc();
    }
    unimplemented!();
    let mut copy: Ptr<Counter_int_> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        copy.prefix_inc();
    }
}
pub fn test_8(c: Ptr<()>, mut first: Ptr<()>, mut last: Ptr<()>) {
    let mut sz: () = unimplemented!();
    unimplemented!();
    let mut copy: Ptr<()> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        sz.prefix_dec();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        copy.prefix_inc();
    }
    unimplemented!();
    let mut copy: Ptr<()> = (first).clone();
    'loop_: while ({ (copy).clone() } != { (last).clone() }) {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        copy.prefix_inc();
    }
}
pub fn test_9() -> () {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut keys: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        ({
            let _first: Ptr<()> = unimplemented!();
            let _last: Ptr<()> = unimplemented!();
            test_6(m.as_pointer(), _first, _last)
        });
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr: Ptr<()> = unimplemented!();
        let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let mut keys: [Counter_int_; 6] = [
                {
                    let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
                    Counter_int_::new_1({ __tmp_0.as_pointer() })
                },
                {
                    let __tmp_1: Value<()> = Rc::new(RefCell::new(2_()));
                    Counter_int_::new_1({ __tmp_1.as_pointer() })
                },
                {
                    let __tmp_2: Value<()> = Rc::new(RefCell::new(3_()));
                    Counter_int_::new_1({ __tmp_2.as_pointer() })
                },
                {
                    let __tmp_3: Value<()> = Rc::new(RefCell::new(4_()));
                    Counter_int_::new_1({ __tmp_3.as_pointer() })
                },
                {
                    let __tmp_4: Value<()> = Rc::new(RefCell::new(5_()));
                    Counter_int_::new_1({ __tmp_4.as_pointer() })
                },
                {
                    let __tmp_5: Value<()> = Rc::new(RefCell::new(6_()));
                    Counter_int_::new_1({ __tmp_5.as_pointer() })
                },
            ];
            unimplemented!();
            ({
                let _first: Ptr<Counter_int_> = unimplemented!();
                let _last: Ptr<Counter_int_> = unimplemented!();
                test_7(m.as_pointer(), _first, _last)
            });
        }
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut keys: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        ({
            let _first: Ptr<()> = unimplemented!();
            let _last: Ptr<()> = unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_9() });
    return 0_();
}
pub trait Counter_int_Impl {
    fn copy_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn move_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn destructor(&self);
    fn get(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> ();
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> ();
}
impl Counter_int_Impl for Ptr<Counter_int_> {
    fn destructor(&self) {
        (*gConstructed_5.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> () {
        return ({ (*self).with(|__s| __s.data_) } == { x.with(|__s| __s.data_) });
    }
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> () {
        return ({ (*self).with(|__s| __s.data_) } < { x.with(|__s| __s.data_) });
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
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
