extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
pub fn DoNotOptimize_2(value: Ptr<()>) -> Ptr<()> {
    b"";
    return (value).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_4.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_5.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_6.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_3.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_4.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_6: Value<()> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_7(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_8(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_8(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
impl CppEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.read(), __rhs.read())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_10(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
impl CppEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.read(), __rhs.read())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        { safe_allocator_int_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_unsigned_long_ {}
impl safe_allocator_unsigned_long_ {}
pub fn make_11(mut size: (), mut start: ()) -> () {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < size) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn make_12(mut size: (), mut start: ()) -> () {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < size) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn make_13(mut size: (), mut start: ()) -> () {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < size) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub fn test_get_basic_14(c: Ptr<()>, mut start_value: ()) {
    let mut n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_get_basic_15(c: Ptr<()>, mut start_value: ()) {
    let mut n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_get_basic_16(c: Ptr<()>, mut start_value: ()) {
    let mut n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_get_basic_17(c: Ptr<()>, mut start_value: ()) {
    let mut n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_get_basic_18(c: Ptr<()>, mut start_value: ()) {
    let mut n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_get_basic_19(c: Ptr<()>, mut start_value: ()) {
    let mut n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_get_20() {
    let mut start_value: () = 35_();
    let c: Value<()> = Rc::new(RefCell::new(({ make_11(10_(), start_value) })));
    let cc: Ptr<()> = c.as_pointer();
    ({ test_get_basic_14(c.as_pointer(), start_value) });
    ({
        let _c: Ptr<()> = (cc).clone();
        let _start_value: () = start_value;
        test_get_basic_15(_c, _start_value)
    });
}
pub fn test_get_21() {
    let mut start_value: () = 35_();
    let c: Value<()> = Rc::new(RefCell::new(({ make_12(10_(), start_value) })));
    let cc: Ptr<()> = c.as_pointer();
    ({ test_get_basic_16(c.as_pointer(), start_value) });
    ({
        let _c: Ptr<()> = (cc).clone();
        let _start_value: () = start_value;
        test_get_basic_17(_c, _start_value)
    });
}
pub fn test_get_22() {
    let mut start_value: () = 35_();
    let c: Value<()> = Rc::new(RefCell::new(({ make_13(10_(), start_value) })));
    let cc: Ptr<()> = c.as_pointer();
    ({ test_get_basic_18(c.as_pointer(), start_value) });
    ({
        let _c: Ptr<()> = (cc).clone();
        let _start_value: () = start_value;
        test_get_basic_19(_c, _start_value)
    });
}
pub fn test_set_23() {
    let mut start_value: () = 35_();
    let mut n: () = 10_();
    let mut c: () = ({ make_11(n, start_value) });
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        let __rhs = ((start_value + i) + 1_());
        unimplemented!().write(__rhs);
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        let __rhs = ((start_value + i) + 2_());
        unimplemented!().write(__rhs);
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!().write((start_value + 3_()));
    unimplemented!();
    unimplemented!();
    unimplemented!().write(((start_value + n) + 2_()));
    unimplemented!();
}
pub fn test_set_24() {
    let mut start_value: () = 35_();
    let mut n: () = 10_();
    let mut c: () = ({ make_12(n, start_value) });
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        let __rhs = ((start_value + i) + 1_());
        unimplemented!().write(__rhs);
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        let __rhs = ((start_value + i) + 2_());
        unimplemented!().write(__rhs);
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!().write((start_value + 3_()));
    unimplemented!();
    unimplemented!();
    unimplemented!().write(((start_value + n) + 2_()));
    unimplemented!();
}
pub fn test_set_25() {
    let mut start_value: () = 35_();
    let mut n: () = 10_();
    let mut c: () = ({ make_13(n, start_value) });
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        let __rhs = ((start_value + i) + 1_());
        unimplemented!().write(__rhs);
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < n) {
        unimplemented!();
        let __rhs = ((start_value + i) + 2_());
        unimplemented!().write(__rhs);
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!().write((start_value + 3_()));
    unimplemented!();
    unimplemented!();
    unimplemented!().write(((start_value + n) + 2_()));
    unimplemented!();
}
pub fn test_26() {
    ({ test_get_20() });
    ({ test_set_23() });
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cc: Ptr<()> = c.as_pointer();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_27() {
    ({ test_get_21() });
    ({ test_set_24() });
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cc: Ptr<()> = c.as_pointer();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_28() {
    ({ test_get_22() });
    ({ test_set_25() });
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cc: Ptr<()> = c.as_pointer();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn tests_29() -> () {
    ({ test_26() });
    ({ test_27() });
    ({ test_28() });
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
    ({ tests_29() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>;
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_const_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl::operator_sub_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
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
        n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_add(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_sub(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
    fn to_(&self) -> () {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_sub(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_sub_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>
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
pub trait safe_allocator_int_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
}
impl safe_allocator_int_Impl for Ptr<safe_allocator_int_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        let memory: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
        {
            unimplemented!();
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        {
            ({
                let _value: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                DoNotOptimize_2(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub trait safe_allocator_unsigned_long_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl safe_allocator_unsigned_long_Impl for Ptr<safe_allocator_unsigned_long_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_3.with(|_| ());
    let _ = alloc_count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = disable_default_constructor_6.with(|_| ());
}
