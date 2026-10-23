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
#[byte_size(8)]
pub struct min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char8_t__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char16_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char16_t__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char32_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char32_t__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__::new_1({
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char32_t_ {}
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
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub fn for_each_4(
    mut _a0: types_type_list_char__wchar_t__char8_t__char16_t__char32_t_,
    f: TestCaller,
) {
    let f: Value<TestCaller> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ TestCallerImpl::operator_call_1_char(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ TestCallerImpl::operator_call_1_wchar_t(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ TestCallerImpl::operator_call_1_char8_t(&f.as_pointer()) });
            0_()
        };
        let _arg3: () = {
            ({ TestCallerImpl::operator_call_1_char16_t(&f.as_pointer()) });
            0_()
        };
        let _arg4: () = {
            ({ TestCallerImpl::operator_call_1_char32_t(&f.as_pointer()) });
            0_()
        };
        swallow_5(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub fn swallow_5(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: (), mut _a4: ()) {}
pub fn test_6(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_7(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_8(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_9(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_10(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_11(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_12(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_13(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_14(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_15(s: (), mut pos: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let cs: Ptr<()> = s.as_pointer();
    if (pos < unimplemented!()) {
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_string_16() {
    ({ test_6(unimplemented!(), 0_()) });
    ({
        test_6(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_6(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_6(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_6(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub fn test_string_17() {
    ({ test_7(unimplemented!(), 0_()) });
    ({
        test_7(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_7(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_7(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_7(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_2(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub fn test_string_18() {
    ({ test_8(unimplemented!(), 0_()) });
    ({
        test_8(
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_8(
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_8(
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_8(
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub fn test_string_19() {
    ({ test_9(unimplemented!(), 0_()) });
    ({
        test_9(
            std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_9(
            std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_9(
            std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_9(
            std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_3(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub fn test_string_20() {
    ({ test_10(unimplemented!(), 0_()) });
    ({
        test_10(
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_4(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_10(
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_4(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_10(
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_4(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_10(
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_4(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub fn test_string_21() {
    ({ test_11(unimplemented!(), 0_()) });
    ({
        test_11(
            std_basic_string_char8_t__std_char_traits_char8_t___min_allocator_char8_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_4(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_11(
            std_basic_string_char8_t__std_char_traits_char8_t___min_allocator_char8_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_4(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_11(
            std_basic_string_char8_t__std_char_traits_char8_t___min_allocator_char8_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_4(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_11(
            std_basic_string_char8_t__std_char_traits_char8_t___min_allocator_char8_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_4(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub fn test_string_22() {
    ({ test_12(unimplemented!(), 0_()) });
    ({
        test_12(
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_12(
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_12(
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_12(
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub fn test_string_23() {
    ({ test_13(unimplemented!(), 0_()) });
    ({
        test_13(
            std_basic_string_char16_t__std_char_traits_char16_t___min_allocator_char16_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_13(
            std_basic_string_char16_t__std_char_traits_char16_t___min_allocator_char16_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_13(
            std_basic_string_char16_t__std_char_traits_char16_t___min_allocator_char16_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_13(
            std_basic_string_char16_t__std_char_traits_char16_t___min_allocator_char16_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_5(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub fn test_string_24() {
    ({ test_14(unimplemented!(), 0_()) });
    ({
        test_14(
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_14(
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_14(
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_14(
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub fn test_string_25() {
    ({ test_15(unimplemented!(), 0_()) });
    ({
        test_15(
            std_basic_string_char32_t__std_char_traits_char32_t___min_allocator_char32_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    ({
        test_15(
            std_basic_string_char32_t__std_char_traits_char32_t___min_allocator_char32_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    ({
        test_15(
            std_basic_string_char32_t__std_char_traits_char32_t___min_allocator_char32_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    ({
        test_15(
            std_basic_string_char32_t__std_char_traits_char32_t___min_allocator_char32_t__::new_1(
                {
                    ({
                        MultiStringTypeImpl::as_ptr_6(
                            &Rc::new(RefCell::new(MultiStringType::new(
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                {
                                    Ptr::<()>::from_string_literal(&[
                                        49 as (), 50 as (), 51 as (), 0 as (),
                                    ])
                                },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { Ptr::<()>::from_string_literal(b"123") },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            )))
                            .as_pointer(),
                            Ptr::<()>::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestCaller {}
pub fn test_26() -> () {
    ({
        for_each_4(
            <types_type_list_char__wchar_t__char8_t__char16_t__char32_t_>::default(),
            <TestCaller>::default(),
        )
    });
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
    ({ test_26() });
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
pub trait TestCallerImpl {
    fn operator_call_1_char(&self);
    fn operator_call_1_wchar_t(&self);
    fn operator_call_1_char8_t(&self);
    fn operator_call_1_char16_t(&self);
    fn operator_call_1_char32_t(&self);
}
impl TestCallerImpl for Ptr<TestCaller> {
    fn operator_call_1_char(&self) {
        ({ test_string_16() });
        ({ test_string_17() });
    }
    fn operator_call_1_wchar_t(&self) {
        ({ test_string_18() });
        ({ test_string_19() });
    }
    fn operator_call_1_char8_t(&self) {
        ({ test_string_20() });
        ({ test_string_21() });
    }
    fn operator_call_1_char16_t(&self) {
        ({ test_string_22() });
        ({ test_string_23() });
    }
    fn operator_call_1_char32_t(&self) {
        ({ test_string_24() });
        ({ test_string_25() });
    }
}
pub trait min_allocator_char16_t_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char16_t__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char16_t_Impl for Ptr<min_allocator_char16_t_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char16_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char16_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_char32_t_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char32_t__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char32_t_Impl for Ptr<min_allocator_char32_t_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char32_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char32_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_char8_t_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char8_t__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char8_t_Impl for Ptr<min_allocator_char8_t_> {
    fn allocate(&self, mut n: ()) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char8_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
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
pub trait min_pointer_char16_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: ())
    -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char16_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_char16_t__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_char16_t__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
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
pub trait min_pointer_char32_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: ())
    -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char32_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_char32_t__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_char32_t__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
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
pub trait min_pointer_char8_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: ()) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char8_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_char8_t__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_char8_t__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
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
pub trait min_pointer_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
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
pub trait min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
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
        n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__;
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_const_char__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
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
pub trait min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
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
        n: (),
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__;
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
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
    fn operator_inc_8(&self) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
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
