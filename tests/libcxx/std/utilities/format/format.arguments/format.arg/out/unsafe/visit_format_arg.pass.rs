extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub struct constexpr_char_traits_wchar_t_ {}
impl constexpr_char_traits_wchar_t_ {
    pub unsafe fn assign_1(c1: *mut (), c2: *const ()) {
        (*c1) = (*c2);
    }
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
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
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_4(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_5(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
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
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_char__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_char__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_char__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
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
impl std::cmp::PartialEq for min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_char__std_integral_constant_unsigned_long__0__ {}
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
pub unsafe fn operator_sub_6(
    mut x: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_7(
    mut x: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
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
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_wchar_t__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_wchar_t__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_wchar_t__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_wchar_t__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
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
impl std::cmp::PartialEq for min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_7(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {}
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
pub unsafe fn test_8(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_9(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_10(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_11(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_12(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_13(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_14(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_15(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_16(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_17(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_18(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_19(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_20(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_21(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_22(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_23(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_24(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_25(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_26(mut value: *const ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: *const () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_27(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_28(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_29(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_30(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_31(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_32(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_33(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_34(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_35(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_36(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_37(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_38(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_39(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_40(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_41(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_42(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_43(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_44(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_45(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_46(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_47(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_48(mut value: *const ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: *const () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_49(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_50(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_51(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_handle_52(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_handle_53(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_handle_54(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_handle_55(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_56(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_57(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_58(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_59(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_60(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_61(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_62(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_63(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_64(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_65(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_66(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_67(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_68() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    (unsafe { test_8(true) });
    (unsafe { test_8(false) });
    (unsafe { test_9(('a' as ())) });
    (unsafe { test_9(('z' as ())) });
    (unsafe { test_9(('0' as ())) });
    (unsafe { test_9(('9' as ())) });
    {
        (unsafe { test_9(('a' as ())) });
        (unsafe { test_9(('z' as ())) });
        (unsafe { test_9(('0' as ())) });
        (unsafe { test_9(('9' as ())) });
    }
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(0_()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_14(0_()) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(0_()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_16(0_()) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(0_()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_handle_52(0_()) });
    (unsafe { test_18(0_()) });
    (unsafe { test_18(unimplemented!()) });
    (unsafe { test_19(0_()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_20(0_()) });
    (unsafe { test_20(unimplemented!()) });
    (unsafe { test_20(unimplemented!()) });
    (unsafe { test_20(unimplemented!()) });
    (unsafe { test_21(0_()) });
    (unsafe { test_21(unimplemented!()) });
    (unsafe { test_21(unimplemented!()) });
    (unsafe { test_21(unimplemented!()) });
    (unsafe { test_21(unimplemented!()) });
    (unsafe { test_22(0_()) });
    (unsafe { test_22(unimplemented!()) });
    (unsafe { test_22(unimplemented!()) });
    (unsafe { test_22(unimplemented!()) });
    (unsafe { test_22(unimplemented!()) });
    (unsafe { test_22(unimplemented!()) });
    (unsafe { test_handle_53(0_()) });
    (unsafe { test_23(-unimplemented!()) });
    (unsafe { test_23(-unimplemented!()) });
    (unsafe { test_23(-0_f64) });
    (unsafe { test_23(0_f64) });
    (unsafe { test_23(unimplemented!()) });
    (unsafe { test_23(unimplemented!()) });
    (unsafe { test_24(-unimplemented!()) });
    (unsafe { test_24(-unimplemented!()) });
    (unsafe { test_24(-0_f64) });
    (unsafe { test_24(0_f64) });
    (unsafe { test_24(unimplemented!()) });
    (unsafe { test_24(unimplemented!()) });
    (unsafe { test_25(-unimplemented!()) });
    (unsafe { test_25(-unimplemented!()) });
    (unsafe { test_25(-0_f64) });
    (unsafe { test_25(0_f64) });
    (unsafe { test_25(unimplemented!()) });
    (unsafe { test_25(unimplemented!()) });
    (unsafe { test_26(unimplemented!()) });
    (unsafe { test_26(unimplemented!()) });
    {
        (unsafe { test_string_view_56(unimplemented!()) });
        (unsafe { test_string_view_56(unimplemented!()) });
        (unsafe { test_string_view_56(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_57(unimplemented!()) });
        (unsafe { test_string_view_57(unimplemented!()) });
        (unsafe { test_string_view_57(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_58(unimplemented!()) });
        (unsafe { test_string_view_58(unimplemented!()) });
        (unsafe { test_string_view_58(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_59(unimplemented!()) });
        (unsafe { test_string_view_59(unimplemented!()) });
        (unsafe { test_string_view_59(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_60(unimplemented!()) });
        (unsafe { test_string_view_60(unimplemented!()) });
        (unsafe { test_string_view_60(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_61(unimplemented!()) });
        (unsafe { test_string_view_61(unimplemented!()) });
        (unsafe { test_string_view_61(unimplemented!()) });
    }
    (unsafe { test_27(Default::default()) });
    let mut i: () = 0_();
    (unsafe { test_28(((&mut i as *mut ()) as ())) });
    let ci: () = 0_();
    (unsafe { test_29(((&ci as *const ()) as ())) });
}
pub unsafe fn test_69() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    (unsafe { test_30(true) });
    (unsafe { test_30(false) });
    (unsafe { test_31(('a' as ())) });
    (unsafe { test_31(('z' as ())) });
    (unsafe { test_31(('0' as ())) });
    (unsafe { test_31(('9' as ())) });
    {
        {
            (unsafe { test_32(('a' as ())) });
            (unsafe { test_32(('z' as ())) });
            (unsafe { test_32(('0' as ())) });
            (unsafe { test_32(('9' as ())) });
        }
    }
    (unsafe { test_35(unimplemented!()) });
    (unsafe { test_35(0_()) });
    (unsafe { test_35(unimplemented!()) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_36(0_()) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_37(0_()) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_38(0_()) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(0_()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_handle_54(0_()) });
    (unsafe { test_40(0_()) });
    (unsafe { test_40(unimplemented!()) });
    (unsafe { test_41(0_()) });
    (unsafe { test_41(unimplemented!()) });
    (unsafe { test_41(unimplemented!()) });
    (unsafe { test_42(0_()) });
    (unsafe { test_42(unimplemented!()) });
    (unsafe { test_42(unimplemented!()) });
    (unsafe { test_42(unimplemented!()) });
    (unsafe { test_43(0_()) });
    (unsafe { test_43(unimplemented!()) });
    (unsafe { test_43(unimplemented!()) });
    (unsafe { test_43(unimplemented!()) });
    (unsafe { test_43(unimplemented!()) });
    (unsafe { test_44(0_()) });
    (unsafe { test_44(unimplemented!()) });
    (unsafe { test_44(unimplemented!()) });
    (unsafe { test_44(unimplemented!()) });
    (unsafe { test_44(unimplemented!()) });
    (unsafe { test_44(unimplemented!()) });
    (unsafe { test_handle_55(0_()) });
    (unsafe { test_45(-unimplemented!()) });
    (unsafe { test_45(-unimplemented!()) });
    (unsafe { test_45(-0_f64) });
    (unsafe { test_45(0_f64) });
    (unsafe { test_45(unimplemented!()) });
    (unsafe { test_45(unimplemented!()) });
    (unsafe { test_46(-unimplemented!()) });
    (unsafe { test_46(-unimplemented!()) });
    (unsafe { test_46(-0_f64) });
    (unsafe { test_46(0_f64) });
    (unsafe { test_46(unimplemented!()) });
    (unsafe { test_46(unimplemented!()) });
    (unsafe { test_47(-unimplemented!()) });
    (unsafe { test_47(-unimplemented!()) });
    (unsafe { test_47(-0_f64) });
    (unsafe { test_47(0_f64) });
    (unsafe { test_47(unimplemented!()) });
    (unsafe { test_47(unimplemented!()) });
    (unsafe { test_48(unimplemented!()) });
    (unsafe { test_48(unimplemented!()) });
    {
        (unsafe { test_string_view_62(unimplemented!()) });
        (unsafe { test_string_view_62(unimplemented!()) });
        (unsafe { test_string_view_62(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_63(unimplemented!()) });
        (unsafe { test_string_view_63(unimplemented!()) });
        (unsafe { test_string_view_63(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_64(unimplemented!()) });
        (unsafe { test_string_view_64(unimplemented!()) });
        (unsafe { test_string_view_64(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_65(unimplemented!()) });
        (unsafe { test_string_view_65(unimplemented!()) });
        (unsafe { test_string_view_65(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_66(unimplemented!()) });
        (unsafe { test_string_view_66(unimplemented!()) });
        (unsafe { test_string_view_66(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_67(unimplemented!()) });
        (unsafe { test_string_view_67(unimplemented!()) });
        (unsafe { test_string_view_67(unimplemented!()) });
    }
    (unsafe { test_49(Default::default()) });
    let mut i: () = 0_();
    (unsafe { test_50(((&mut i as *mut ()) as ())) });
    let ci: () = 0_();
    (unsafe { test_51(((&ci as *const ()) as ())) });
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
    (unsafe { test_68() });
    (unsafe { test_69() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
