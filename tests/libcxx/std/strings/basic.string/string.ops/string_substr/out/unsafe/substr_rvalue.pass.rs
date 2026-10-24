extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_pointer_in_range_0(
    mut first: *const (),
    mut last: *const (),
    mut p: *const (),
) -> () {
    if ((first) == (p)) {
        return true;
    }
    'loop_: while ((first) != (last)) {
        if ((first) == (p)) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
pub unsafe fn is_pointer_in_range_1(
    mut first: *const (),
    mut last: *const (),
    mut p: *const (),
) -> () {
    if ((first) == (p)) {
        return true;
    }
    'loop_: while ((first) != (last)) {
        if ((first) == (p)) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
pub unsafe fn is_pointer_in_range_2(
    mut first: *const (),
    mut last: *const (),
    mut p: *const (),
) -> () {
    if ((first) == (p)) {
        return true;
    }
    'loop_: while ((first) != (last)) {
        if ((first) == (p)) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
pub unsafe fn is_pointer_in_range_3(
    mut first: *const (),
    mut last: *const (),
    mut p: *const (),
) -> () {
    if ((first) == (p)) {
        return true;
    }
    'loop_: while ((first) != (last)) {
        if ((first) == (p)) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
pub unsafe fn is_pointer_in_range_4(
    mut first: *const (),
    mut last: *const (),
    mut p: *const (),
) -> () {
    if ((first) == (p)) {
        return true;
    }
    'loop_: while ((first) != (last)) {
        if ((first) == (p)) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub unsafe fn assign_1(c1: *mut (), c2: *const ()) {
        (*c1) = (*c2);
    }
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
    }
    pub unsafe fn lt(mut c1: (), mut c2: ()) -> () {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const (), mut s2: *const (), mut n: ()) -> () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s1);
                let _c2: () = (*s2);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if (unsafe {
                let _c1: () = (*s2);
                let _c2: () = (*s1);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub unsafe fn length(mut s: *const ()) -> () {
        let mut len: () = 0_();
        'loop_: while !(unsafe {
            let _c1: () = (*s);
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
    pub unsafe fn move_(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        if (((s1).cast_const()) == (s2)) {
            return s1;
        }
        let mut r: *mut () = s1;
        if (unsafe {
            let _first: *const () = (s1).cast_const();
            let _last: *const () = (s1.offset((n) as isize)).cast_const();
            is_pointer_in_range_0(_first, _last, s2)
        }) {
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.postfix_inc());
                    let _c2: *const () = &(*s2.postfix_inc());
                    constexpr_char_traits_char_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            s1 = (s1).wrapping_add(n as usize);
            s2 = (s2).wrapping_add(n as usize);
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.prefix_dec());
                    let _c2: *const () = &(*s2.prefix_dec());
                    constexpr_char_traits_char_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return r;
    }
    pub unsafe fn copy(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        {
            unimplemented!();
        }
        let mut r: *mut () = s1;
        'loop_: while (n != 0) {
            (unsafe {
                let _c1: *mut () = &mut (*s1);
                let _c2: *const () = &(*s2);
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
        return r;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct constexpr_char_traits_char16_t_ {}
impl constexpr_char_traits_char16_t_ {
    pub unsafe fn assign_1(c1: *mut (), c2: *const ()) {
        (*c1) = (*c2);
    }
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
    }
    pub unsafe fn lt(mut c1: (), mut c2: ()) -> () {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const (), mut s2: *const (), mut n: ()) -> () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s1);
                let _c2: () = (*s2);
                constexpr_char_traits_char16_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if (unsafe {
                let _c1: () = (*s2);
                let _c2: () = (*s1);
                constexpr_char_traits_char16_t_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub unsafe fn length(mut s: *const ()) -> () {
        let mut len: () = 0_();
        'loop_: while !(unsafe {
            let _c1: () = (*s);
            let _c2: () = 0_();
            constexpr_char_traits_char16_t_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
    pub unsafe fn move_(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        if (((s1).cast_const()) == (s2)) {
            return s1;
        }
        let mut r: *mut () = s1;
        if (unsafe {
            let _first: *const () = (s1).cast_const();
            let _last: *const () = (s1.offset((n) as isize)).cast_const();
            is_pointer_in_range_1(_first, _last, s2)
        }) {
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.postfix_inc());
                    let _c2: *const () = &(*s2.postfix_inc());
                    constexpr_char_traits_char16_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            s1 = (s1).wrapping_add(n as usize);
            s2 = (s2).wrapping_add(n as usize);
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.prefix_dec());
                    let _c2: *const () = &(*s2.prefix_dec());
                    constexpr_char_traits_char16_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return r;
    }
    pub unsafe fn copy(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        {
            unimplemented!();
        }
        let mut r: *mut () = s1;
        'loop_: while (n != 0) {
            (unsafe {
                let _c1: *mut () = &mut (*s1);
                let _c2: *const () = &(*s2);
                constexpr_char_traits_char16_t_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return r;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct constexpr_char_traits_char32_t_ {}
impl constexpr_char_traits_char32_t_ {
    pub unsafe fn assign_1(c1: *mut (), c2: *const ()) {
        (*c1) = (*c2);
    }
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
    }
    pub unsafe fn lt(mut c1: (), mut c2: ()) -> () {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const (), mut s2: *const (), mut n: ()) -> () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s1);
                let _c2: () = (*s2);
                constexpr_char_traits_char32_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if (unsafe {
                let _c1: () = (*s2);
                let _c2: () = (*s1);
                constexpr_char_traits_char32_t_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub unsafe fn length(mut s: *const ()) -> () {
        let mut len: () = 0_();
        'loop_: while !(unsafe {
            let _c1: () = (*s);
            let _c2: () = 0_();
            constexpr_char_traits_char32_t_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
    pub unsafe fn move_(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        if (((s1).cast_const()) == (s2)) {
            return s1;
        }
        let mut r: *mut () = s1;
        if (unsafe {
            let _first: *const () = (s1).cast_const();
            let _last: *const () = (s1.offset((n) as isize)).cast_const();
            is_pointer_in_range_2(_first, _last, s2)
        }) {
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.postfix_inc());
                    let _c2: *const () = &(*s2.postfix_inc());
                    constexpr_char_traits_char32_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            s1 = (s1).wrapping_add(n as usize);
            s2 = (s2).wrapping_add(n as usize);
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.prefix_dec());
                    let _c2: *const () = &(*s2.prefix_dec());
                    constexpr_char_traits_char32_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return r;
    }
    pub unsafe fn copy(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        {
            unimplemented!();
        }
        let mut r: *mut () = s1;
        'loop_: while (n != 0) {
            (unsafe {
                let _c1: *mut () = &mut (*s1);
                let _c2: *const () = &(*s2);
                constexpr_char_traits_char32_t_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return r;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct constexpr_char_traits_wchar_t_ {}
impl constexpr_char_traits_wchar_t_ {
    pub unsafe fn assign_1(c1: *mut (), c2: *const ()) {
        (*c1) = (*c2);
    }
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
    }
    pub unsafe fn lt(mut c1: (), mut c2: ()) -> () {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const (), mut s2: *const (), mut n: ()) -> () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s1);
                let _c2: () = (*s2);
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if (unsafe {
                let _c1: () = (*s2);
                let _c2: () = (*s1);
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub unsafe fn length(mut s: *const ()) -> () {
        let mut len: () = 0_();
        'loop_: while !(unsafe {
            let _c1: () = (*s);
            let _c2: () = 0_();
            constexpr_char_traits_wchar_t_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
    pub unsafe fn move_(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        if (((s1).cast_const()) == (s2)) {
            return s1;
        }
        let mut r: *mut () = s1;
        if (unsafe {
            let _first: *const () = (s1).cast_const();
            let _last: *const () = (s1.offset((n) as isize)).cast_const();
            is_pointer_in_range_3(_first, _last, s2)
        }) {
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.postfix_inc());
                    let _c2: *const () = &(*s2.postfix_inc());
                    constexpr_char_traits_wchar_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            s1 = (s1).wrapping_add(n as usize);
            s2 = (s2).wrapping_add(n as usize);
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.prefix_dec());
                    let _c2: *const () = &(*s2.prefix_dec());
                    constexpr_char_traits_wchar_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return r;
    }
    pub unsafe fn copy(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        {
            unimplemented!();
        }
        let mut r: *mut () = s1;
        'loop_: while (n != 0) {
            (unsafe {
                let _c1: *mut () = &mut (*s1);
                let _c2: *const () = &(*s2);
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
        return r;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct constexpr_char_traits_char8_t_ {}
impl constexpr_char_traits_char8_t_ {
    pub unsafe fn assign_1(c1: *mut (), c2: *const ()) {
        (*c1) = (*c2);
    }
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
    }
    pub unsafe fn lt(mut c1: (), mut c2: ()) -> () {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const (), mut s2: *const (), mut n: ()) -> () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s1);
                let _c2: () = (*s2);
                constexpr_char_traits_char8_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if (unsafe {
                let _c1: () = (*s2);
                let _c2: () = (*s1);
                constexpr_char_traits_char8_t_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub unsafe fn length(mut s: *const ()) -> () {
        let mut len: () = 0_();
        'loop_: while !(unsafe {
            let _c1: () = (*s);
            let _c2: () = 0_();
            constexpr_char_traits_char8_t_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
    pub unsafe fn move_(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        if (((s1).cast_const()) == (s2)) {
            return s1;
        }
        let mut r: *mut () = s1;
        if (unsafe {
            let _first: *const () = (s1).cast_const();
            let _last: *const () = (s1.offset((n) as isize)).cast_const();
            is_pointer_in_range_4(_first, _last, s2)
        }) {
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.postfix_inc());
                    let _c2: *const () = &(*s2.postfix_inc());
                    constexpr_char_traits_char8_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            s1 = (s1).wrapping_add(n as usize);
            s2 = (s2).wrapping_add(n as usize);
            'loop_: while (n != 0) {
                (unsafe {
                    let _c1: *mut () = &mut (*s1.prefix_dec());
                    let _c2: *const () = &(*s2.prefix_dec());
                    constexpr_char_traits_char8_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return r;
    }
    pub unsafe fn copy(mut s1: *mut (), mut s2: *const (), mut n: ()) -> *mut () {
        {
            unimplemented!();
        }
        let mut r: *mut () = s1;
        'loop_: while (n != 0) {
            (unsafe {
                let _c1: *mut () = &mut (*s1);
                let _c2: *const () = &(*s2);
                constexpr_char_traits_char8_t_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return r;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_8)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_5)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7)) = 0_();
    }
}
pub static mut outstanding_bytes_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char16_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(
        mut p: min_pointer_char16_t__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char32_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(
        mut p: min_pointer_char32_t__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_wchar_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(
        mut p: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(
        mut p: min_pointer_char8_t__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
pub unsafe fn operator_eq_9(mut _a0: min_allocator_char_, mut _a1: min_allocator_char_) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char_ {}
pub unsafe fn operator_eq_10(
    mut _a0: min_allocator_char16_t_,
    mut _a1: min_allocator_char16_t_,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char16_t_ {}
impl min_allocator_char16_t_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char16_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char16_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_char16_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char16_t_ {}
pub unsafe fn operator_eq_11(
    mut _a0: min_allocator_char32_t_,
    mut _a1: min_allocator_char32_t_,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char32_t_ {}
impl min_allocator_char32_t_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char32_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char32_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_char32_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char32_t_ {}
pub unsafe fn operator_eq_12(
    mut _a0: min_allocator_wchar_t_,
    mut _a1: min_allocator_wchar_t_,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_wchar_t_ {}
impl min_allocator_wchar_t_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_wchar_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_wchar_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_wchar_t_ {}
pub unsafe fn operator_eq_13(
    mut _a0: min_allocator_char8_t_,
    mut _a1: min_allocator_char8_t_,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char8_t_ {}
impl min_allocator_char8_t_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char8_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_char8_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_13(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char8_t_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: (),
    pub throw_after: (),
    pub count: (),
    pub alloc_count: (),
    pub allocated_size: (),
    pub construct_count: (),
    pub destroy_count: (),
    pub copied: (),
    pub moved: (),
    pub converted: (),
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        unimplemented!();
        self.count = 0_();
        self.time_to_throw = 0_();
        self.alloc_count = 0_();
        self.allocated_size = 0_();
        self.construct_count = 0_();
        self.destroy_count = 0_();
        self.throw_after = 2147483647_();
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0_();
        self.moved = 0_();
        self.converted = 0_();
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
pub static mut destructed_value_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_16(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_17(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_16(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0_(),
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: (),
        mut id: (),
        mut stats: *mut test_allocator_statistics,
    ) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: stats,
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_void_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const test_allocator_void_,
    ) -> *mut test_allocator_void_ {
        self.data_ = (*_a0).data_;
        self.id_ = (*_a0).id_;
        self.stats_ = (*_a0).stats_;
        return &mut (*(self as *mut test_allocator_void_));
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_16(
                self as *const test_allocator_void_,
                other as *const test_allocator_void_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_void_::copy_from(self as *const test_allocator_void_) }
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        unsafe {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_18(x: *const test_allocator_char_, y: *const test_allocator_char_) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_char__rebind_char_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_char_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_char_ {
    pub unsafe fn copy_from(a: *const test_allocator_char_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl std::cmp::PartialEq for test_allocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_18(
                self as *const test_allocator_char_,
                other as *const test_allocator_char_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char_ {}
impl Clone for test_allocator_char_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_char_::copy_from(self as *const test_allocator_char_) }
    }
}
impl Default for test_allocator_char_ {
    fn default() -> Self {
        unsafe {
            test_allocator_char_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_19(
    x: *const test_allocator_char16_t_,
    y: *const test_allocator_char16_t_,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_char16_t__rebind_char16_t_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_char16_t_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_char16_t_ {
    pub unsafe fn copy_from(a: *const test_allocator_char16_t_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl std::cmp::PartialEq for test_allocator_char16_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_19(
                self as *const test_allocator_char16_t_,
                other as *const test_allocator_char16_t_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char16_t_ {}
impl Clone for test_allocator_char16_t_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_char16_t_::copy_from(self as *const test_allocator_char16_t_) }
    }
}
impl Default for test_allocator_char16_t_ {
    fn default() -> Self {
        unsafe {
            test_allocator_char16_t_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_20(
    x: *const test_allocator_char32_t_,
    y: *const test_allocator_char32_t_,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_char32_t__rebind_char32_t_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_char32_t_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_char32_t_ {
    pub unsafe fn copy_from(a: *const test_allocator_char32_t_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl std::cmp::PartialEq for test_allocator_char32_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_20(
                self as *const test_allocator_char32_t_,
                other as *const test_allocator_char32_t_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char32_t_ {}
impl Clone for test_allocator_char32_t_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_char32_t_::copy_from(self as *const test_allocator_char32_t_) }
    }
}
impl Default for test_allocator_char32_t_ {
    fn default() -> Self {
        unsafe {
            test_allocator_char32_t_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_21(
    x: *const test_allocator_wchar_t_,
    y: *const test_allocator_wchar_t_,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_wchar_t__rebind_wchar_t_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_wchar_t_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_wchar_t_ {
    pub unsafe fn copy_from(a: *const test_allocator_wchar_t_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl std::cmp::PartialEq for test_allocator_wchar_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_21(
                self as *const test_allocator_wchar_t_,
                other as *const test_allocator_wchar_t_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_wchar_t_ {}
impl Clone for test_allocator_wchar_t_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_wchar_t_::copy_from(self as *const test_allocator_wchar_t_) }
    }
}
impl Default for test_allocator_wchar_t_ {
    fn default() -> Self {
        unsafe {
            test_allocator_wchar_t_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_22(
    x: *const test_allocator_char8_t_,
    y: *const test_allocator_char8_t_,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_char8_t__rebind_char8_t_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_char8_t_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_char8_t_ {
    pub unsafe fn copy_from(a: *const test_allocator_char8_t_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl std::cmp::PartialEq for test_allocator_char8_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_22(
                self as *const test_allocator_char8_t_,
                other as *const test_allocator_char8_t_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char8_t_ {}
impl Clone for test_allocator_char8_t_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_char8_t_::copy_from(self as *const test_allocator_char8_t_) }
    }
}
impl Default for test_allocator_char8_t_ {
    fn default() -> Self {
        unsafe {
            test_allocator_char8_t_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Tag_X {}
impl Tag_X {
    pub unsafe fn new_1(mut _a0: Ctor_Tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: Ctor_Tag, _a1: *const Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Ctor_Tag, _a1: *mut Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
}
pub unsafe fn is_string_asan_correct_23(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_24(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_25(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_26(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_27(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_28(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_29(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_30(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_31(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_32(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_33(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_34(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_35(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_36(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_37(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_38(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_39(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_40(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_41(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_42(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_43(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_44(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_45(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_46(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_47(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_48(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_49(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_50(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_51(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_52(_a0: *const ()) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct should_throw_exception_t {}
pub static mut should_throw_exception_53: std::cell::LazyCell<should_throw_exception_t> =
    std::cell::LazyCell::new(|| unsafe { <should_throw_exception_t>::default() });
pub unsafe fn test_54(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_55(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_56(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_57(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_58(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_59(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_60(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_61(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_62(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_63(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_64(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_65(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_66(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_67(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_68(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_69(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_70(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_71(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_72(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_73(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_74(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_75(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_76(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_77(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_78(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_79(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_80(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_81(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_82(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_83(mut orig: (), mut pos: (), mut n: (), expected: ()) {
    let mut str: () = (unsafe { (*unimplemented!()).substr_131(pos, n) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_84(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_85(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_86(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_87(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_88(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_89(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_90(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_91(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_92(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_93(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_94(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_95(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_96(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_97(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_98(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_99(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_100(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_101(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_102(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_103(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_104(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_105(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_106(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_107(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_108(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_109(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_110(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_111(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_112(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_113(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub unsafe fn test_string_114() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_84(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_84(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_84(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_84(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_115() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_85(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_85(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_85(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_85(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_116() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_86(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_86(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_86(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_86(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_117() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_87(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_87(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_87(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_87(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_118() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_88(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_88(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_88(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_88(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_119() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_89(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_89(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_89(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_89(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_120() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_90(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_90(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_90(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_90(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_121() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_91(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_91(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_91(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_91(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_122() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_92(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_92(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_92(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_92(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_123() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_93(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_93(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_93(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_93(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_124() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_94(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_94(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_94(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_94(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_125() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_95(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_95(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_95(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_95(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_126() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_96(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_96(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_96(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_96(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_127() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_97(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_97(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_97(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_97(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_128() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_98(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_98(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_98(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_98(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_129() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_99(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_99(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_99(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_99(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_130() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_100(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_100(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_100(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_100(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_131() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_101(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_101(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_101(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_101(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_132() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_102(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_102(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_102(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_102(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_133() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_103(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_103(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_103(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_103(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_134() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_104(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_104(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_104(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_104(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_135() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_105(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_105(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_105(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_105(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_136() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_106(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_106(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_106(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_106(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_137() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_107(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_107(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_107(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_107(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_138() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_108(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_108(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_108(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_108(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_139() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_109(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_109(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_109(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_109(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_140() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_110(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_110(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_110(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_110(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_141() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_111(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_111(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_111(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_111(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_142() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_112(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_112(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_112(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_112(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_string_143() {
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        test_113(
            unimplemented!(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        test_113(
            unimplemented!(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 1_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 5_(), 5_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 12_(), 5_(), _expected)
    });
    (unsafe {
        test_113(
            unimplemented!(),
            13_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 0_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 10_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 20_(), 10_(), _expected)
    });
    (unsafe {
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 26_(), 10_(), _expected)
    });
    (unsafe {
        test_113(
            unimplemented!(),
            27_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut should_throw_exception_53)),
        )
    });
}
pub unsafe fn test_allocators_144() {
    (unsafe { test_string_114() });
    (unsafe { test_string_115() });
    (unsafe { test_string_116() });
}
pub unsafe fn test_allocators_145() {
    (unsafe { test_string_117() });
    (unsafe { test_string_118() });
    (unsafe { test_string_119() });
}
pub unsafe fn test_allocators_146() {
    (unsafe { test_string_120() });
    (unsafe { test_string_121() });
    (unsafe { test_string_122() });
}
pub unsafe fn test_allocators_147() {
    (unsafe { test_string_123() });
    (unsafe { test_string_124() });
    (unsafe { test_string_125() });
}
pub unsafe fn test_allocators_148() {
    (unsafe { test_string_126() });
    (unsafe { test_string_127() });
    (unsafe { test_string_128() });
}
pub unsafe fn test_allocators_149() {
    (unsafe { test_string_129() });
    (unsafe { test_string_130() });
    (unsafe { test_string_131() });
}
pub unsafe fn test_allocators_150() {
    (unsafe { test_string_132() });
    (unsafe { test_string_133() });
    (unsafe { test_string_134() });
}
pub unsafe fn test_allocators_151() {
    (unsafe { test_string_135() });
    (unsafe { test_string_136() });
    (unsafe { test_string_137() });
}
pub unsafe fn test_allocators_152() {
    (unsafe { test_string_138() });
    (unsafe { test_string_139() });
    (unsafe { test_string_140() });
}
pub unsafe fn test_allocators_153() {
    (unsafe { test_string_141() });
    (unsafe { test_string_142() });
    (unsafe { test_string_143() });
}
pub unsafe fn test_char_traits_154() {
    (unsafe { test_allocators_144() });
    (unsafe { test_allocators_145() });
}
pub unsafe fn test_char_traits_155() {
    (unsafe { test_allocators_146() });
    (unsafe { test_allocators_147() });
}
pub unsafe fn test_char_traits_156() {
    (unsafe { test_allocators_148() });
    (unsafe { test_allocators_149() });
}
pub unsafe fn test_char_traits_157() {
    (unsafe { test_allocators_150() });
    (unsafe { test_allocators_151() });
}
pub unsafe fn test_char_traits_158() {
    (unsafe { test_allocators_152() });
    (unsafe { test_allocators_153() });
}
pub unsafe fn test_159() -> () {
    (unsafe { test_char_traits_154() });
    (unsafe { test_char_traits_155() });
    (unsafe { test_char_traits_156() });
    (unsafe { test_char_traits_157() });
    (unsafe { test_char_traits_158() });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_159() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_5);
    std::cell::LazyCell::force(&*&raw const alloc_count_6);
    std::cell::LazyCell::force(&*&raw const dealloc_count_7);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_8);
    std::cell::LazyCell::force(&*&raw const destructed_value_14);
    std::cell::LazyCell::force(&*&raw const moved_value_15);
    std::cell::LazyCell::force(&*&raw const should_throw_exception_53);
}
