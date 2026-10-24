extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<()>) -> Ptr<()> {
    b"";
    return (value).clone();
}
pub fn is_string_asan_correct_1(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_2(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_3(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_4(_a0: Ptr<()>) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_6.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_7.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_8.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_5.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_7.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_7: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_8: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
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
#[byte_size(1)]
pub struct min_allocator_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_char_ {}
impl safe_allocator_char_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for safe_allocator_char_ {
    fn default() -> Self {
        { safe_allocator_char_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct tiny_size_allocator_32__char__rebind_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct tiny_size_allocator_32__char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct tiny_size_allocator_32__wchar_t__rebind_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct tiny_size_allocator_32__wchar_t_ {}
pub fn test_10(s: (), mut n: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_11(s: (), mut n: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_12(s: (), mut n: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_13(s: (), mut n: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_string_14() {
    {
        ({ test_10(unimplemented!(), 0_(), unimplemented!()) });
        ({ test_10(unimplemented!(), 1_(), unimplemented!()) });
        ({ test_10(unimplemented!(), 10_(), unimplemented!()) });
        ({ test_10(unimplemented!(), 100_(), unimplemented!()) });
        ({ test_10(unimplemented!(), 0_(), unimplemented!()) });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_10(_s, 2_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_10(_s, 5_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_10(_s, 15_(), _expected)
        });
        ({ test_10(unimplemented!(), 0_(), unimplemented!()) });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_10(_s, 10_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_10(_s, 50_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_10(_s, 60_(), _expected)
        });
    }
    {
        let mut str: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_string_15() {
    {
        ({ test_11(unimplemented!(), 0_(), unimplemented!()) });
        ({ test_11(unimplemented!(), 1_(), unimplemented!()) });
        ({ test_11(unimplemented!(), 10_(), unimplemented!()) });
        ({ test_11(unimplemented!(), 100_(), unimplemented!()) });
        ({ test_11(unimplemented!(), 0_(), unimplemented!()) });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_11(_s, 2_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_11(_s, 5_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_11(_s, 15_(), _expected)
        });
        ({ test_11(unimplemented!(), 0_(), unimplemented!()) });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_11(_s, 10_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_11(_s, 50_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_11(_s, 60_(), _expected)
        });
    }
    {
        let mut str: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_string_16() {
    {
        ({ test_12(unimplemented!(), 0_(), unimplemented!()) });
        ({ test_12(unimplemented!(), 1_(), unimplemented!()) });
        ({ test_12(unimplemented!(), 10_(), unimplemented!()) });
        ({ test_12(unimplemented!(), 100_(), unimplemented!()) });
        ({ test_12(unimplemented!(), 0_(), unimplemented!()) });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_12(_s, 2_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_12(_s, 5_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_12(_s, 15_(), _expected)
        });
        ({ test_12(unimplemented!(), 0_(), unimplemented!()) });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_12(_s, 10_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_12(_s, 50_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_12(_s, 60_(), _expected)
        });
    }
    {
        let mut str: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_string_17() {
    {
        ({ test_13(unimplemented!(), 0_(), unimplemented!()) });
        ({ test_13(unimplemented!(), 1_(), unimplemented!()) });
        ({ test_13(unimplemented!(), 10_(), unimplemented!()) });
        ({ test_13(unimplemented!(), 100_(), unimplemented!()) });
        ({ test_13(unimplemented!(), 0_(), unimplemented!()) });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_13(_s, 2_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_13(_s, 5_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_13(_s, 15_(), _expected)
        });
        ({ test_13(unimplemented!(), 0_(), unimplemented!()) });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_13(_s, 10_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_13(_s, 50_(), _expected)
        });
        ({
            let _s: () = unimplemented!();
            let _expected: () = unimplemented!();
            test_13(_s, 60_(), _expected)
        });
    }
    {
        let mut str: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_18() -> () {
    ({ test_string_14() });
    ({ test_string_15() });
    ({ test_string_16() });
    ({ test_string_17() });
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
    ({ test_18() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub trait min_allocator_char_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char_Impl for Ptr<min_allocator_char_> {
    fn allocate(&self, mut n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
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
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait safe_allocator_char_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
}
impl safe_allocator_char_Impl for Ptr<safe_allocator_char_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        let memory: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                memset_9(
                    (*memory.borrow()).to_any(),
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        {
            ({
                let _value: Value<()> = Rc::new(RefCell::new(
                    ({
                        memset_9(
                            (p).to_any(),
                            0_(),
                            (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                        )
                    }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub trait tiny_size_allocator_32__char_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, ptr: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl tiny_size_allocator_32__char_Impl for Ptr<tiny_size_allocator_32__char_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!();
        return unimplemented!();
    }
    fn deallocate(&self, mut ptr: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return 32_();
    }
}
pub trait tiny_size_allocator_32__wchar_t_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, ptr: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl tiny_size_allocator_32__wchar_t_Impl for Ptr<tiny_size_allocator_32__wchar_t_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!();
        return unimplemented!();
    }
    fn deallocate(&self, mut ptr: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return 32_();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_5.with(|_| ());
    let _ = alloc_count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = disable_default_constructor_8.with(|_| ());
}
