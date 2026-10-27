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
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub fn assign_1(c1: Ptr<()>, c2: Ptr<()>) {
        c1.write({ (c2.read()) });
    }
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
    pub fn copy(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        {
            unimplemented!();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((s1).clone()));
        'loop_: while (n != 0) {
            ({
                let _c1: Ptr<()> = (s1).clone();
                let _c2: Ptr<()> = (s2).clone();
                constexpr_char_traits_char_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return (*r.borrow()).clone();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_wchar_t_ {}
impl constexpr_char_traits_wchar_t_ {
    pub fn assign_1(c1: Ptr<()>, c2: Ptr<()>) {
        c1.write({ (c2.read()) });
    }
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_wchar_t_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
    pub fn copy(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        {
            unimplemented!();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((s1).clone()));
        'loop_: while (n != 0) {
            ({
                let _c1: Ptr<()> = (s1).clone();
                let _c2: Ptr<()> = (s2).clone();
                constexpr_char_traits_wchar_t_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return (*r.borrow()).clone();
    }
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
pub fn operator_sub_4(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_5(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
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
impl std::cmp::PartialEq for min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_5(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_char__std_integral_constant_unsigned_long__0__ {}
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
pub fn operator_sub_6(
    mut x: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_7(
    mut x: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_wchar_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
impl std::cmp::PartialEq for min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_7(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_wchar_t_ {}
pub fn test_8(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_9(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_10(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_11(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_12(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_13(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_14(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_15(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_16(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_17(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_18(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_19(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_20(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_21(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_22(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_23(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_24(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_25(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_26(mut value: Ptr<()>) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: Ptr<()> = unimplemented!();
    unimplemented!();
}
pub fn test_27(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_28(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_29(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_30(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_31(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_32(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_33(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_34(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_35(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_36(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_37(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_38(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_39(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_40(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_41(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_42(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_43(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_44(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_45(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_46(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_47(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_48(mut value: Ptr<()>) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: Ptr<()> = unimplemented!();
    unimplemented!();
}
pub fn test_49(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_50(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_51(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_handle_52(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_handle_53(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_handle_54(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_handle_55(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_string_view_56(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_57(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_58(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_59(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_60(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_61(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_62(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_63(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_64(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_65(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_66(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_67(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_68() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    ({ test_8(true) });
    ({ test_8(false) });
    ({ test_9(('a' as ())) });
    ({ test_9(('z' as ())) });
    ({ test_9(('0' as ())) });
    ({ test_9(('9' as ())) });
    {
        ({ test_9(('a' as ())) });
        ({ test_9(('z' as ())) });
        ({ test_9(('0' as ())) });
        ({ test_9(('9' as ())) });
    }
    ({ test_13(unimplemented!()) });
    ({ test_13(0_()) });
    ({ test_13(unimplemented!()) });
    ({ test_14(unimplemented!()) });
    ({ test_14(unimplemented!()) });
    ({ test_14(0_()) });
    ({ test_14(unimplemented!()) });
    ({ test_14(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(0_()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_16(unimplemented!()) });
    ({ test_16(unimplemented!()) });
    ({ test_16(unimplemented!()) });
    ({ test_16(unimplemented!()) });
    ({ test_16(0_()) });
    ({ test_16(unimplemented!()) });
    ({ test_16(unimplemented!()) });
    ({ test_16(unimplemented!()) });
    ({ test_16(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(0_()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_handle_52(0_()) });
    ({ test_18(0_()) });
    ({ test_18(unimplemented!()) });
    ({ test_19(0_()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_20(0_()) });
    ({ test_20(unimplemented!()) });
    ({ test_20(unimplemented!()) });
    ({ test_20(unimplemented!()) });
    ({ test_21(0_()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_22(0_()) });
    ({ test_22(unimplemented!()) });
    ({ test_22(unimplemented!()) });
    ({ test_22(unimplemented!()) });
    ({ test_22(unimplemented!()) });
    ({ test_22(unimplemented!()) });
    ({ test_handle_53(0_()) });
    ({ test_23(-unimplemented!()) });
    ({ test_23(-unimplemented!()) });
    ({ test_23(-0_f64) });
    ({ test_23(0_f64) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_24(-unimplemented!()) });
    ({ test_24(-unimplemented!()) });
    ({ test_24(-0_f64) });
    ({ test_24(0_f64) });
    ({ test_24(unimplemented!()) });
    ({ test_24(unimplemented!()) });
    ({ test_25(-unimplemented!()) });
    ({ test_25(-unimplemented!()) });
    ({ test_25(-0_f64) });
    ({ test_25(0_f64) });
    ({ test_25(unimplemented!()) });
    ({ test_25(unimplemented!()) });
    ({ test_26(unimplemented!()) });
    ({ test_26(unimplemented!()) });
    {
        ({ test_string_view_56(unimplemented!()) });
        ({ test_string_view_56(unimplemented!()) });
        ({ test_string_view_56(unimplemented!()) });
    }
    {
        ({ test_string_view_57(unimplemented!()) });
        ({ test_string_view_57(unimplemented!()) });
        ({ test_string_view_57(unimplemented!()) });
    }
    {
        ({ test_string_view_58(unimplemented!()) });
        ({ test_string_view_58(unimplemented!()) });
        ({ test_string_view_58(unimplemented!()) });
    }
    {
        ({ test_string_view_59(unimplemented!()) });
        ({ test_string_view_59(unimplemented!()) });
        ({ test_string_view_59(unimplemented!()) });
    }
    {
        ({ test_string_view_60(unimplemented!()) });
        ({ test_string_view_60(unimplemented!()) });
        ({ test_string_view_60(unimplemented!()) });
    }
    {
        ({ test_string_view_61(unimplemented!()) });
        ({ test_string_view_61(unimplemented!()) });
        ({ test_string_view_61(unimplemented!()) });
    }
    ({ test_27(Default::default()) });
    let i: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_28(((i.as_pointer()) as Ptr<()>).to_any()) });
    let ci: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_29(((ci.as_pointer()) as Ptr<()>).to_any()) });
}
pub fn test_69() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    ({ test_30(true) });
    ({ test_30(false) });
    ({ test_31(('a' as ())) });
    ({ test_31(('z' as ())) });
    ({ test_31(('0' as ())) });
    ({ test_31(('9' as ())) });
    {
        {
            ({ test_32(('a' as ())) });
            ({ test_32(('z' as ())) });
            ({ test_32(('0' as ())) });
            ({ test_32(('9' as ())) });
        }
    }
    ({ test_35(unimplemented!()) });
    ({ test_35(0_()) });
    ({ test_35(unimplemented!()) });
    ({ test_36(unimplemented!()) });
    ({ test_36(unimplemented!()) });
    ({ test_36(0_()) });
    ({ test_36(unimplemented!()) });
    ({ test_36(unimplemented!()) });
    ({ test_37(unimplemented!()) });
    ({ test_37(unimplemented!()) });
    ({ test_37(unimplemented!()) });
    ({ test_37(0_()) });
    ({ test_37(unimplemented!()) });
    ({ test_37(unimplemented!()) });
    ({ test_37(unimplemented!()) });
    ({ test_38(unimplemented!()) });
    ({ test_38(unimplemented!()) });
    ({ test_38(unimplemented!()) });
    ({ test_38(unimplemented!()) });
    ({ test_38(0_()) });
    ({ test_38(unimplemented!()) });
    ({ test_38(unimplemented!()) });
    ({ test_38(unimplemented!()) });
    ({ test_38(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_39(0_()) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_handle_54(0_()) });
    ({ test_40(0_()) });
    ({ test_40(unimplemented!()) });
    ({ test_41(0_()) });
    ({ test_41(unimplemented!()) });
    ({ test_41(unimplemented!()) });
    ({ test_42(0_()) });
    ({ test_42(unimplemented!()) });
    ({ test_42(unimplemented!()) });
    ({ test_42(unimplemented!()) });
    ({ test_43(0_()) });
    ({ test_43(unimplemented!()) });
    ({ test_43(unimplemented!()) });
    ({ test_43(unimplemented!()) });
    ({ test_43(unimplemented!()) });
    ({ test_44(0_()) });
    ({ test_44(unimplemented!()) });
    ({ test_44(unimplemented!()) });
    ({ test_44(unimplemented!()) });
    ({ test_44(unimplemented!()) });
    ({ test_44(unimplemented!()) });
    ({ test_handle_55(0_()) });
    ({ test_45(-unimplemented!()) });
    ({ test_45(-unimplemented!()) });
    ({ test_45(-0_f64) });
    ({ test_45(0_f64) });
    ({ test_45(unimplemented!()) });
    ({ test_45(unimplemented!()) });
    ({ test_46(-unimplemented!()) });
    ({ test_46(-unimplemented!()) });
    ({ test_46(-0_f64) });
    ({ test_46(0_f64) });
    ({ test_46(unimplemented!()) });
    ({ test_46(unimplemented!()) });
    ({ test_47(-unimplemented!()) });
    ({ test_47(-unimplemented!()) });
    ({ test_47(-0_f64) });
    ({ test_47(0_f64) });
    ({ test_47(unimplemented!()) });
    ({ test_47(unimplemented!()) });
    ({ test_48(unimplemented!()) });
    ({ test_48(unimplemented!()) });
    {
        ({ test_string_view_62(unimplemented!()) });
        ({ test_string_view_62(unimplemented!()) });
        ({ test_string_view_62(unimplemented!()) });
    }
    {
        ({ test_string_view_63(unimplemented!()) });
        ({ test_string_view_63(unimplemented!()) });
        ({ test_string_view_63(unimplemented!()) });
    }
    {
        ({ test_string_view_64(unimplemented!()) });
        ({ test_string_view_64(unimplemented!()) });
        ({ test_string_view_64(unimplemented!()) });
    }
    {
        ({ test_string_view_65(unimplemented!()) });
        ({ test_string_view_65(unimplemented!()) });
        ({ test_string_view_65(unimplemented!()) });
    }
    {
        ({ test_string_view_66(unimplemented!()) });
        ({ test_string_view_66(unimplemented!()) });
        ({ test_string_view_66(unimplemented!()) });
    }
    {
        ({ test_string_view_67(unimplemented!()) });
        ({ test_string_view_67(unimplemented!()) });
        ({ test_string_view_67(unimplemented!()) });
    }
    ({ test_49(Default::default()) });
    let i: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_50(((i.as_pointer()) as Ptr<()>).to_any()) });
    let ci: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_51(((ci.as_pointer()) as Ptr<()>).to_any()) });
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
    ({ test_68() });
    ({ test_69() });
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
pub trait min_allocator_wchar_t_Impl {
    fn allocate(&self, n: ()) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_wchar_t_Impl for Ptr<min_allocator_wchar_t_> {
    fn allocate(&self, mut n: ()) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_wchar_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>;
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
        n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__;
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_char__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_char__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
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
pub trait min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
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
pub trait min_pointer_wchar_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: ()) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_wchar_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_wchar_t__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
