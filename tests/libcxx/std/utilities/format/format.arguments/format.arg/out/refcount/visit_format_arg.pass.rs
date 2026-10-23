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
pub fn test_8(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___bool_> =
        Rc::new(RefCell::new(({ make_format_args_9(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_1({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_10(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___char_> =
        Rc::new(RefCell::new(({ make_format_args_11(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_12(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___char_> =
        Rc::new(RefCell::new(({ make_format_args_11(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_13(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___char_> =
        Rc::new(RefCell::new(({ make_format_args_11(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_14(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___char_> =
        Rc::new(RefCell::new(({ make_format_args_11(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_15(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___signed_char_,
    > = Rc::new(RefCell::new(({ make_format_args_16(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_3({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_17(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___short_> =
        Rc::new(RefCell::new(({ make_format_args_18(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_4({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_19(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___int_> =
        Rc::new(RefCell::new(({ make_format_args_20(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_5({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_21(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___long_> =
        Rc::new(RefCell::new(({ make_format_args_22(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_6({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_23(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___long_long_> =
        Rc::new(RefCell::new(({ make_format_args_24(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_7({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_25(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_char_,
    > = Rc::new(RefCell::new(({ make_format_args_26(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_8({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_27(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_short_,
    > = Rc::new(RefCell::new(({ make_format_args_28(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_9({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_29(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_int_,
    > = Rc::new(RefCell::new(({ make_format_args_30(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_10({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_31(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_long_,
    > = Rc::new(RefCell::new(({ make_format_args_32(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_11({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_33(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_long_long_,
    > = Rc::new(RefCell::new(({ make_format_args_34(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_12({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_35(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___float_> =
        Rc::new(RefCell::new(({ make_format_args_36(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_13({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_37(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___double_> =
        Rc::new(RefCell::new(({ make_format_args_38(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_14({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_39(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___long_double_,
    > = Rc::new(RefCell::new(({ make_format_args_40(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_15({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_41(value: Ptr<()>) {
    let value: Value<Ptr<()>> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___const_char_ptr_,
    > = Rc::new(RefCell::new(({ make_format_args_42(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_16({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: Ptr<()> = unimplemented!();
    unimplemented!();
}
pub fn test_43(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___std_nullptr_t_,
    > = Rc::new(RefCell::new(({ make_format_args_44(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_17({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_45(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___void_ptr_> =
        Rc::new(RefCell::new(({ make_format_args_46(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_18({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_47(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___const_void_ptr_,
    > = Rc::new(RefCell::new(({ make_format_args_48(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_19({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_49(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___bool_> =
        Rc::new(RefCell::new(({ make_format_args_50(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_1({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_51(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___wchar_t_,
    > = Rc::new(RefCell::new(({ make_format_args_52(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_2({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_53(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_> =
        Rc::new(RefCell::new(({ make_format_args_54(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_55(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_> =
        Rc::new(RefCell::new(({ make_format_args_54(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_56(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_> =
        Rc::new(RefCell::new(({ make_format_args_54(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_57(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___signed_char_,
    > = Rc::new(RefCell::new(({ make_format_args_58(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_4({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_59(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___short_,
    > = Rc::new(RefCell::new(({ make_format_args_60(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_5({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_61(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int_> =
        Rc::new(RefCell::new(({ make_format_args_62(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_6({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_63(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_> =
        Rc::new(RefCell::new(({ make_format_args_64(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_7({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_65(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_long_,
    > = Rc::new(RefCell::new(({ make_format_args_66(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_8({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_67(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_char_,
    > = Rc::new(RefCell::new(({ make_format_args_68(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_9({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_69(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_short_,
    > = Rc::new(RefCell::new(({ make_format_args_70(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_10({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_71(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_int_,
    > = Rc::new(RefCell::new(({ make_format_args_72(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_11({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_73(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_long_,
    > = Rc::new(RefCell::new(({ make_format_args_74(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_12({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_75(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_long_long_,
    > = Rc::new(RefCell::new(({ make_format_args_76(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_13({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_77(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___float_,
    > = Rc::new(RefCell::new(({ make_format_args_78(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_14({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_79(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___double_,
    > = Rc::new(RefCell::new(({ make_format_args_80(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_15({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_81(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_double_,
    > = Rc::new(RefCell::new(({ make_format_args_82(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_16({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_83(value: Ptr<()>) {
    let value: Value<Ptr<()>> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___const_wchar_t_ptr_,
    > = Rc::new(RefCell::new(({ make_format_args_84(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_17({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: Ptr<()> = unimplemented!();
    unimplemented!();
}
pub fn test_85(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_nullptr_t_,
    > = Rc::new(RefCell::new(({ make_format_args_86(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_18({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_87(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___void_ptr_,
    > = Rc::new(RefCell::new(({ make_format_args_88(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_19({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_89(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___const_void_ptr_,
    > = Rc::new(RefCell::new(({ make_format_args_90(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_20({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub fn test_handle_91(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char_____int128_> =
        Rc::new(RefCell::new(({ make_format_args_92(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_20({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_handle_93(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned___int128_,
    > = Rc::new(RefCell::new(({ make_format_args_94(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_21({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_handle_95(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t_____int128_,
    > = Rc::new(RefCell::new(({ make_format_args_96(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_21({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_handle_97(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned___int128_,
    > = Rc::new(RefCell::new(({ make_format_args_98(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_22({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_string_view_99(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___std_string_view_,
    > = Rc::new(RefCell::new(({ make_format_args_100(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_22({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_101(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store : Value<std___format_arg_store_std_basic_format_context_char_ptr__char___std_basic_string_view_char__constexpr_char_traits_char___ > = Rc::new(RefCell::new(( { make_format_args_102 ( value .as_pointer()  , ) } )  )) ;
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_23({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_103(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___std_string_> =
        Rc::new(RefCell::new(({ make_format_args_104(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_24({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_105(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store : Value<std___format_arg_store_std_basic_format_context_char_ptr__char___std_basic_string_char__constexpr_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(( { make_format_args_106 ( value .as_pointer()  , ) } )  )) ;
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_25({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_107(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store : Value<std___format_arg_store_std_basic_format_context_char_ptr__char___std_basic_string_char__std_char_traits_char___min_allocator_char___ > = Rc::new(RefCell::new(( { make_format_args_108 ( value .as_pointer()  , ) } )  )) ;
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_26({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_109(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store : Value<std___format_arg_store_std_basic_format_context_char_ptr__char___std_basic_string_char__constexpr_char_traits_char___min_allocator_char___ > = Rc::new(RefCell::new(( { make_format_args_110 ( value .as_pointer()  , ) } )  )) ;
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_27({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_111(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_wstring_view_,
    > = Rc::new(RefCell::new(({ make_format_args_112(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_23({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_113(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store : Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_basic_string_view_wchar_t__constexpr_char_traits_wchar_t___ > = Rc::new(RefCell::new(( { make_format_args_114 ( value .as_pointer()  , ) } )  )) ;
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_24({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_115(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_wstring_,
    > = Rc::new(RefCell::new(({ make_format_args_116(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_25({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_117(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store : Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_basic_string_wchar_t__constexpr_char_traits_wchar_t___std_allocator_wchar_t___ > = Rc::new(RefCell::new(( { make_format_args_118 ( value .as_pointer()  , ) } )  )) ;
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_26({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_119(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store : Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t___ > = Rc::new(RefCell::new(( { make_format_args_120 ( value .as_pointer()  , ) } )  )) ;
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_27({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_string_view_121(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store : Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_basic_string_wchar_t__constexpr_char_traits_wchar_t___min_allocator_wchar_t___ > = Rc::new(RefCell::new(( { make_format_args_122 ( value .as_pointer()  , ) } )  )) ;
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_28({
            store.as_pointer()
        }),
    ));
    unimplemented!();
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn test_123() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    ({ test_8(true) });
    ({ test_8(false) });
    ({ test_10(('a' as ())) });
    ({ test_10(('z' as ())) });
    ({ test_10(('0' as ())) });
    ({ test_10(('9' as ())) });
    {
        ({ test_10(('a' as ())) });
        ({ test_10(('z' as ())) });
        ({ test_10(('0' as ())) });
        ({ test_10(('9' as ())) });
    }
    ({ test_15(unimplemented!()) });
    ({ test_15(0_()) });
    ({ test_15(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(0_()) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(0_()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(0_()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(0_()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_handle_91(0_()) });
    ({ test_25(0_()) });
    ({ test_25(unimplemented!()) });
    ({ test_27(0_()) });
    ({ test_27(unimplemented!()) });
    ({ test_27(unimplemented!()) });
    ({ test_29(0_()) });
    ({ test_29(unimplemented!()) });
    ({ test_29(unimplemented!()) });
    ({ test_29(unimplemented!()) });
    ({ test_31(0_()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_33(0_()) });
    ({ test_33(unimplemented!()) });
    ({ test_33(unimplemented!()) });
    ({ test_33(unimplemented!()) });
    ({ test_33(unimplemented!()) });
    ({ test_33(unimplemented!()) });
    ({ test_handle_93(0_()) });
    ({ test_35(-unimplemented!()) });
    ({ test_35(-unimplemented!()) });
    ({ test_35(-0_f64) });
    ({ test_35(0_f64) });
    ({ test_35(unimplemented!()) });
    ({ test_35(unimplemented!()) });
    ({ test_37(-unimplemented!()) });
    ({ test_37(-unimplemented!()) });
    ({ test_37(-0_f64) });
    ({ test_37(0_f64) });
    ({ test_37(unimplemented!()) });
    ({ test_37(unimplemented!()) });
    ({ test_39(-unimplemented!()) });
    ({ test_39(-unimplemented!()) });
    ({ test_39(-0_f64) });
    ({ test_39(0_f64) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_41(unimplemented!()) });
    ({ test_41(unimplemented!()) });
    {
        ({ test_string_view_99(unimplemented!()) });
        ({ test_string_view_99(unimplemented!()) });
        ({ test_string_view_99(unimplemented!()) });
    }
    {
        ({ test_string_view_101(unimplemented!()) });
        ({ test_string_view_101(unimplemented!()) });
        ({ test_string_view_101(unimplemented!()) });
    }
    {
        ({ test_string_view_103(unimplemented!()) });
        ({ test_string_view_103(unimplemented!()) });
        ({ test_string_view_103(unimplemented!()) });
    }
    {
        ({ test_string_view_105(unimplemented!()) });
        ({ test_string_view_105(unimplemented!()) });
        ({ test_string_view_105(unimplemented!()) });
    }
    {
        ({ test_string_view_107(unimplemented!()) });
        ({ test_string_view_107(unimplemented!()) });
        ({ test_string_view_107(unimplemented!()) });
    }
    {
        ({ test_string_view_109(unimplemented!()) });
        ({ test_string_view_109(unimplemented!()) });
        ({ test_string_view_109(unimplemented!()) });
    }
    ({ test_43(Default::default()) });
    let i: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_45(((i.as_pointer()) as Ptr<()>).to_any()) });
    let ci: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_47(((ci.as_pointer()) as Ptr<()>).to_any()) });
}
pub fn test_124() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    ({ test_49(true) });
    ({ test_49(false) });
    ({ test_51(('a' as ())) });
    ({ test_51(('z' as ())) });
    ({ test_51(('0' as ())) });
    ({ test_51(('9' as ())) });
    {
        {
            ({ test_53(('a' as ())) });
            ({ test_53(('z' as ())) });
            ({ test_53(('0' as ())) });
            ({ test_53(('9' as ())) });
        }
    }
    ({ test_57(unimplemented!()) });
    ({ test_57(0_()) });
    ({ test_57(unimplemented!()) });
    ({ test_59(unimplemented!()) });
    ({ test_59(unimplemented!()) });
    ({ test_59(0_()) });
    ({ test_59(unimplemented!()) });
    ({ test_59(unimplemented!()) });
    ({ test_61(unimplemented!()) });
    ({ test_61(unimplemented!()) });
    ({ test_61(unimplemented!()) });
    ({ test_61(0_()) });
    ({ test_61(unimplemented!()) });
    ({ test_61(unimplemented!()) });
    ({ test_61(unimplemented!()) });
    ({ test_63(unimplemented!()) });
    ({ test_63(unimplemented!()) });
    ({ test_63(unimplemented!()) });
    ({ test_63(unimplemented!()) });
    ({ test_63(0_()) });
    ({ test_63(unimplemented!()) });
    ({ test_63(unimplemented!()) });
    ({ test_63(unimplemented!()) });
    ({ test_63(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(0_()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_handle_95(0_()) });
    ({ test_67(0_()) });
    ({ test_67(unimplemented!()) });
    ({ test_69(0_()) });
    ({ test_69(unimplemented!()) });
    ({ test_69(unimplemented!()) });
    ({ test_71(0_()) });
    ({ test_71(unimplemented!()) });
    ({ test_71(unimplemented!()) });
    ({ test_71(unimplemented!()) });
    ({ test_73(0_()) });
    ({ test_73(unimplemented!()) });
    ({ test_73(unimplemented!()) });
    ({ test_73(unimplemented!()) });
    ({ test_73(unimplemented!()) });
    ({ test_75(0_()) });
    ({ test_75(unimplemented!()) });
    ({ test_75(unimplemented!()) });
    ({ test_75(unimplemented!()) });
    ({ test_75(unimplemented!()) });
    ({ test_75(unimplemented!()) });
    ({ test_handle_97(0_()) });
    ({ test_77(-unimplemented!()) });
    ({ test_77(-unimplemented!()) });
    ({ test_77(-0_f64) });
    ({ test_77(0_f64) });
    ({ test_77(unimplemented!()) });
    ({ test_77(unimplemented!()) });
    ({ test_79(-unimplemented!()) });
    ({ test_79(-unimplemented!()) });
    ({ test_79(-0_f64) });
    ({ test_79(0_f64) });
    ({ test_79(unimplemented!()) });
    ({ test_79(unimplemented!()) });
    ({ test_81(-unimplemented!()) });
    ({ test_81(-unimplemented!()) });
    ({ test_81(-0_f64) });
    ({ test_81(0_f64) });
    ({ test_81(unimplemented!()) });
    ({ test_81(unimplemented!()) });
    ({ test_83(unimplemented!()) });
    ({ test_83(unimplemented!()) });
    {
        ({ test_string_view_111(unimplemented!()) });
        ({ test_string_view_111(unimplemented!()) });
        ({ test_string_view_111(unimplemented!()) });
    }
    {
        ({ test_string_view_113(unimplemented!()) });
        ({ test_string_view_113(unimplemented!()) });
        ({ test_string_view_113(unimplemented!()) });
    }
    {
        ({ test_string_view_115(unimplemented!()) });
        ({ test_string_view_115(unimplemented!()) });
        ({ test_string_view_115(unimplemented!()) });
    }
    {
        ({ test_string_view_117(unimplemented!()) });
        ({ test_string_view_117(unimplemented!()) });
        ({ test_string_view_117(unimplemented!()) });
    }
    {
        ({ test_string_view_119(unimplemented!()) });
        ({ test_string_view_119(unimplemented!()) });
        ({ test_string_view_119(unimplemented!()) });
    }
    {
        ({ test_string_view_121(unimplemented!()) });
        ({ test_string_view_121(unimplemented!()) });
        ({ test_string_view_121(unimplemented!()) });
    }
    ({ test_85(Default::default()) });
    let i: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_87(((i.as_pointer()) as Ptr<()>).to_any()) });
    let ci: Value<()> = Rc::new(RefCell::new(0_()));
    ({ test_89(((ci.as_pointer()) as Ptr<()>).to_any()) });
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
    ({ test_123() });
    ({ test_124() });
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
