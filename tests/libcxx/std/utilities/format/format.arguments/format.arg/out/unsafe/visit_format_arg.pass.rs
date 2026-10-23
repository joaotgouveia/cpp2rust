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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___bool_ =
        (unsafe { make_format_args_9(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_1({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_10(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___char_ =
        (unsafe { make_format_args_11(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_12(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___char_ =
        (unsafe { make_format_args_11(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_13(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___char_ =
        (unsafe { make_format_args_11(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_14(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___char_ =
        (unsafe { make_format_args_11(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_15(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___signed_char_ =
        (unsafe { make_format_args_16(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_3({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_17(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___short_ =
        (unsafe { make_format_args_18(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_4({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_19(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___int_ =
        (unsafe { make_format_args_20(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_5({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_21(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___long_ =
        (unsafe { make_format_args_22(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_6({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_23(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___long_long_ =
        (unsafe { make_format_args_24(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_7({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_25(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_char_ =
        (unsafe { make_format_args_26(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_8({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_27(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_short_ = ( unsafe { make_format_args_28 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_9({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_29(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_int_ =
        (unsafe { make_format_args_30(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_10({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_31(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_long_ =
        (unsafe { make_format_args_32(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_11({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_33(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_long_long_ = ( unsafe { make_format_args_34 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_12({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_35(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___float_ =
        (unsafe { make_format_args_36(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_13({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_37(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___double_ =
        (unsafe { make_format_args_38(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_14({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_39(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___long_double_ =
        (unsafe { make_format_args_40(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_15({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_41(mut value: *const ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___const_char_ptr_ = ( unsafe { make_format_args_42 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_16({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: *const () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_43(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___std_nullptr_t_ =
        (unsafe { make_format_args_44(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_17({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_45(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___void_ptr_ =
        (unsafe { make_format_args_46(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_18({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_47(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___const_void_ptr_ = ( unsafe { make_format_args_48 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_19({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_49(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___bool_ =
        (unsafe { make_format_args_50(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_1({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_51(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___wchar_t_ =
        (unsafe { make_format_args_52(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_2({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_53(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_ =
        (unsafe { make_format_args_54(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_55(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_ =
        (unsafe { make_format_args_54(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_56(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_ =
        (unsafe { make_format_args_54(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_57(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___signed_char_ = ( unsafe { make_format_args_58 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_4({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_59(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___short_ =
        (unsafe { make_format_args_60(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_5({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_61(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int_ =
        (unsafe { make_format_args_62(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_6({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_63(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_ =
        (unsafe { make_format_args_64(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_7({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_65(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_long_ = ( unsafe { make_format_args_66 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_8({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_67(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_char_ = ( unsafe { make_format_args_68 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_9({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_69(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_short_ = ( unsafe { make_format_args_70 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_10({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_71(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_int_ = ( unsafe { make_format_args_72 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_11({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_73(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_long_ = ( unsafe { make_format_args_74 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_12({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_75(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_long_long_ = ( unsafe { make_format_args_76 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_13({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_77(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___float_ =
        (unsafe { make_format_args_78(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_14({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_79(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___double_ =
        (unsafe { make_format_args_80(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_15({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_81(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_double_ = ( unsafe { make_format_args_82 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_16({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_83(mut value: *const ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___const_wchar_t_ptr_ = ( unsafe { make_format_args_84 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_17({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: *const () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_85(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_nullptr_t_ = ( unsafe { make_format_args_86 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_18({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_87(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___void_ptr_ = ( unsafe { make_format_args_88 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_19({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_89(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___const_void_ptr_ = ( unsafe { make_format_args_90 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_20({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_handle_91(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char_____int128_ =
        (unsafe { make_format_args_92(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_20({ &store });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_handle_93(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned___int128_ = ( unsafe { make_format_args_94 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_21({ &store });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_handle_95(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t_____int128_ = ( unsafe { make_format_args_96 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_21({ &store });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_handle_97(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned___int128_ = ( unsafe { make_format_args_98 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_22({ &store });
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_99(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___std_string_view_ = ( unsafe { make_format_args_100 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_22({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_101(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___std_basic_string_view_char__constexpr_char_traits_char___ = ( unsafe { make_format_args_102 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_23({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_103(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___std_string_ =
        (unsafe { make_format_args_104(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_24({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_105(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___std_basic_string_char__constexpr_char_traits_char___std_allocator_char___ = ( unsafe { make_format_args_106 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_25({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_107(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___std_basic_string_char__std_char_traits_char___min_allocator_char___ = ( unsafe { make_format_args_108 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_26({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_109(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___std_basic_string_char__constexpr_char_traits_char___min_allocator_char___ = ( unsafe { make_format_args_110 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_27({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_111(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_wstring_view_ = ( unsafe { make_format_args_112 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_23({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_113(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_basic_string_view_wchar_t__constexpr_char_traits_wchar_t___ = ( unsafe { make_format_args_114 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_24({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_115(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_wstring_ = ( unsafe { make_format_args_116 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_25({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_117(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_basic_string_wchar_t__constexpr_char_traits_wchar_t___std_allocator_wchar_t___ = ( unsafe { make_format_args_118 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_26({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_119(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t___ = ( unsafe { make_format_args_120 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_27({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_view_121(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_basic_string_wchar_t__constexpr_char_traits_wchar_t___min_allocator_wchar_t___ = ( unsafe { make_format_args_122 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_28({ &store });
    unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_123() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    (unsafe { test_8(true) });
    (unsafe { test_8(false) });
    (unsafe { test_10(('a' as ())) });
    (unsafe { test_10(('z' as ())) });
    (unsafe { test_10(('0' as ())) });
    (unsafe { test_10(('9' as ())) });
    {
        (unsafe { test_10(('a' as ())) });
        (unsafe { test_10(('z' as ())) });
        (unsafe { test_10(('0' as ())) });
        (unsafe { test_10(('9' as ())) });
    }
    (unsafe { test_15((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_15(0_()) });
    (unsafe { test_15((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_17((unsafe { std_numeric_limits_short_::min() })) });
    (unsafe { test_17((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_17(0_()) });
    (unsafe { test_17((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_17((unsafe { std_numeric_limits_short_::max() })) });
    (unsafe { test_19((unsafe { std_numeric_limits_int_::min() })) });
    (unsafe { test_19((unsafe { std_numeric_limits_short_::min() })) });
    (unsafe { test_19((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_19(0_()) });
    (unsafe { test_19((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_19((unsafe { std_numeric_limits_short_::max() })) });
    (unsafe { test_19((unsafe { std_numeric_limits_int_::max() })) });
    (unsafe { test_21((unsafe { std_numeric_limits_long_::min() })) });
    (unsafe { test_21((unsafe { std_numeric_limits_int_::min() })) });
    (unsafe { test_21((unsafe { std_numeric_limits_short_::min() })) });
    (unsafe { test_21((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_21(0_()) });
    (unsafe { test_21((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_21((unsafe { std_numeric_limits_short_::max() })) });
    (unsafe { test_21((unsafe { std_numeric_limits_int_::max() })) });
    (unsafe { test_21((unsafe { std_numeric_limits_long_::max() })) });
    (unsafe { test_23((unsafe { std_numeric_limits_long_long_::min() })) });
    (unsafe { test_23((unsafe { std_numeric_limits_long_::min() })) });
    (unsafe { test_23((unsafe { std_numeric_limits_int_::min() })) });
    (unsafe { test_23((unsafe { std_numeric_limits_short_::min() })) });
    (unsafe { test_23((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_23(0_()) });
    (unsafe { test_23((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_23((unsafe { std_numeric_limits_short_::max() })) });
    (unsafe { test_23((unsafe { std_numeric_limits_int_::max() })) });
    (unsafe { test_23((unsafe { std_numeric_limits_long_::max() })) });
    (unsafe { test_23((unsafe { std_numeric_limits_long_long_::max() })) });
    (unsafe { test_handle_91(0_()) });
    (unsafe { test_25(0_()) });
    (unsafe { test_25((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_27(0_()) });
    (unsafe { test_27((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_27((unsafe { std_numeric_limits_unsigned_short_::max() })) });
    (unsafe { test_29(0_()) });
    (unsafe { test_29((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_29((unsafe { std_numeric_limits_unsigned_short_::max() })) });
    (unsafe { test_29((unsafe { std_numeric_limits_unsigned_int_::max() })) });
    (unsafe { test_31(0_()) });
    (unsafe { test_31((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_31((unsafe { std_numeric_limits_unsigned_short_::max() })) });
    (unsafe { test_31((unsafe { std_numeric_limits_unsigned_int_::max() })) });
    (unsafe { test_31((unsafe { std_numeric_limits_unsigned_long_::max() })) });
    (unsafe { test_33(0_()) });
    (unsafe { test_33((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_33((unsafe { std_numeric_limits_unsigned_short_::max() })) });
    (unsafe { test_33((unsafe { std_numeric_limits_unsigned_int_::max() })) });
    (unsafe { test_33((unsafe { std_numeric_limits_unsigned_long_::max() })) });
    (unsafe { test_33((unsafe { std_numeric_limits_unsigned_long_long_::max() })) });
    (unsafe { test_handle_93(0_()) });
    (unsafe { test_35(-(unsafe { std_numeric_limits_float_::max() })) });
    (unsafe { test_35(-(unsafe { std_numeric_limits_float_::min() })) });
    (unsafe { test_35(-0_f64) });
    (unsafe { test_35(0_f64) });
    (unsafe { test_35((unsafe { std_numeric_limits_float_::min() })) });
    (unsafe { test_35((unsafe { std_numeric_limits_float_::max() })) });
    (unsafe { test_37(-(unsafe { std_numeric_limits_double_::max() })) });
    (unsafe { test_37(-(unsafe { std_numeric_limits_double_::min() })) });
    (unsafe { test_37(-0_f64) });
    (unsafe { test_37(0_f64) });
    (unsafe { test_37((unsafe { std_numeric_limits_double_::min() })) });
    (unsafe { test_37((unsafe { std_numeric_limits_double_::max() })) });
    (unsafe { test_39(-(unsafe { std_numeric_limits_long_double_::max() })) });
    (unsafe { test_39(-(unsafe { std_numeric_limits_long_double_::min() })) });
    (unsafe { test_39(-0_f64) });
    (unsafe { test_39(0_f64) });
    (unsafe { test_39((unsafe { std_numeric_limits_long_double_::min() })) });
    (unsafe { test_39((unsafe { std_numeric_limits_long_double_::max() })) });
    (unsafe { test_41(unimplemented!()) });
    (unsafe { test_41(unimplemented!()) });
    {
        (unsafe { test_string_view_99(unimplemented!()) });
        (unsafe { test_string_view_99(unimplemented!()) });
        (unsafe { test_string_view_99(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_101(unimplemented!()) });
        (unsafe { test_string_view_101(unimplemented!()) });
        (unsafe { test_string_view_101(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_103(unimplemented!()) });
        (unsafe { test_string_view_103(unimplemented!()) });
        (unsafe { test_string_view_103(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_105(unimplemented!()) });
        (unsafe { test_string_view_105(unimplemented!()) });
        (unsafe { test_string_view_105(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_107(unimplemented!()) });
        (unsafe { test_string_view_107(unimplemented!()) });
        (unsafe { test_string_view_107(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_109(unimplemented!()) });
        (unsafe { test_string_view_109(unimplemented!()) });
        (unsafe { test_string_view_109(unimplemented!()) });
    }
    (unsafe { test_43(Default::default()) });
    let mut i: () = 0_();
    (unsafe { test_45(((&mut i as *mut ()) as ())) });
    let ci: () = 0_();
    (unsafe { test_47(((&ci as *const ()) as ())) });
}
pub unsafe fn test_124() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    (unsafe { test_49(true) });
    (unsafe { test_49(false) });
    (unsafe { test_51(('a' as ())) });
    (unsafe { test_51(('z' as ())) });
    (unsafe { test_51(('0' as ())) });
    (unsafe { test_51(('9' as ())) });
    {
        {
            (unsafe { test_53(('a' as ())) });
            (unsafe { test_53(('z' as ())) });
            (unsafe { test_53(('0' as ())) });
            (unsafe { test_53(('9' as ())) });
        }
    }
    (unsafe { test_57((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_57(0_()) });
    (unsafe { test_57((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_59((unsafe { std_numeric_limits_short_::min() })) });
    (unsafe { test_59((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_59(0_()) });
    (unsafe { test_59((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_59((unsafe { std_numeric_limits_short_::max() })) });
    (unsafe { test_61((unsafe { std_numeric_limits_int_::min() })) });
    (unsafe { test_61((unsafe { std_numeric_limits_short_::min() })) });
    (unsafe { test_61((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_61(0_()) });
    (unsafe { test_61((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_61((unsafe { std_numeric_limits_short_::max() })) });
    (unsafe { test_61((unsafe { std_numeric_limits_int_::max() })) });
    (unsafe { test_63((unsafe { std_numeric_limits_long_::min() })) });
    (unsafe { test_63((unsafe { std_numeric_limits_int_::min() })) });
    (unsafe { test_63((unsafe { std_numeric_limits_short_::min() })) });
    (unsafe { test_63((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_63(0_()) });
    (unsafe { test_63((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_63((unsafe { std_numeric_limits_short_::max() })) });
    (unsafe { test_63((unsafe { std_numeric_limits_int_::max() })) });
    (unsafe { test_63((unsafe { std_numeric_limits_long_::max() })) });
    (unsafe { test_65((unsafe { std_numeric_limits_long_long_::min() })) });
    (unsafe { test_65((unsafe { std_numeric_limits_long_::min() })) });
    (unsafe { test_65((unsafe { std_numeric_limits_int_::min() })) });
    (unsafe { test_65((unsafe { std_numeric_limits_short_::min() })) });
    (unsafe { test_65((unsafe { std_numeric_limits_signed_char_::min() })) });
    (unsafe { test_65(0_()) });
    (unsafe { test_65((unsafe { std_numeric_limits_signed_char_::max() })) });
    (unsafe { test_65((unsafe { std_numeric_limits_short_::max() })) });
    (unsafe { test_65((unsafe { std_numeric_limits_int_::max() })) });
    (unsafe { test_65((unsafe { std_numeric_limits_long_::max() })) });
    (unsafe { test_65((unsafe { std_numeric_limits_long_long_::max() })) });
    (unsafe { test_handle_95(0_()) });
    (unsafe { test_67(0_()) });
    (unsafe { test_67((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_69(0_()) });
    (unsafe { test_69((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_69((unsafe { std_numeric_limits_unsigned_short_::max() })) });
    (unsafe { test_71(0_()) });
    (unsafe { test_71((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_71((unsafe { std_numeric_limits_unsigned_short_::max() })) });
    (unsafe { test_71((unsafe { std_numeric_limits_unsigned_int_::max() })) });
    (unsafe { test_73(0_()) });
    (unsafe { test_73((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_73((unsafe { std_numeric_limits_unsigned_short_::max() })) });
    (unsafe { test_73((unsafe { std_numeric_limits_unsigned_int_::max() })) });
    (unsafe { test_73((unsafe { std_numeric_limits_unsigned_long_::max() })) });
    (unsafe { test_75(0_()) });
    (unsafe { test_75((unsafe { std_numeric_limits_unsigned_char_::max() })) });
    (unsafe { test_75((unsafe { std_numeric_limits_unsigned_short_::max() })) });
    (unsafe { test_75((unsafe { std_numeric_limits_unsigned_int_::max() })) });
    (unsafe { test_75((unsafe { std_numeric_limits_unsigned_long_::max() })) });
    (unsafe { test_75((unsafe { std_numeric_limits_unsigned_long_long_::max() })) });
    (unsafe { test_handle_97(0_()) });
    (unsafe { test_77(-(unsafe { std_numeric_limits_float_::max() })) });
    (unsafe { test_77(-(unsafe { std_numeric_limits_float_::min() })) });
    (unsafe { test_77(-0_f64) });
    (unsafe { test_77(0_f64) });
    (unsafe { test_77((unsafe { std_numeric_limits_float_::min() })) });
    (unsafe { test_77((unsafe { std_numeric_limits_float_::max() })) });
    (unsafe { test_79(-(unsafe { std_numeric_limits_double_::max() })) });
    (unsafe { test_79(-(unsafe { std_numeric_limits_double_::min() })) });
    (unsafe { test_79(-0_f64) });
    (unsafe { test_79(0_f64) });
    (unsafe { test_79((unsafe { std_numeric_limits_double_::min() })) });
    (unsafe { test_79((unsafe { std_numeric_limits_double_::max() })) });
    (unsafe { test_81(-(unsafe { std_numeric_limits_long_double_::max() })) });
    (unsafe { test_81(-(unsafe { std_numeric_limits_long_double_::min() })) });
    (unsafe { test_81(-0_f64) });
    (unsafe { test_81(0_f64) });
    (unsafe { test_81((unsafe { std_numeric_limits_long_double_::min() })) });
    (unsafe { test_81((unsafe { std_numeric_limits_long_double_::max() })) });
    (unsafe { test_83(unimplemented!()) });
    (unsafe { test_83(unimplemented!()) });
    {
        (unsafe { test_string_view_111(unimplemented!()) });
        (unsafe { test_string_view_111(unimplemented!()) });
        (unsafe { test_string_view_111(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_113(unimplemented!()) });
        (unsafe { test_string_view_113(unimplemented!()) });
        (unsafe { test_string_view_113(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_115(unimplemented!()) });
        (unsafe { test_string_view_115(unimplemented!()) });
        (unsafe { test_string_view_115(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_117(unimplemented!()) });
        (unsafe { test_string_view_117(unimplemented!()) });
        (unsafe { test_string_view_117(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_119(unimplemented!()) });
        (unsafe { test_string_view_119(unimplemented!()) });
        (unsafe { test_string_view_119(unimplemented!()) });
    }
    {
        (unsafe { test_string_view_121(unimplemented!()) });
        (unsafe { test_string_view_121(unimplemented!()) });
        (unsafe { test_string_view_121(unimplemented!()) });
    }
    (unsafe { test_85(Default::default()) });
    let mut i: () = 0_();
    (unsafe { test_87(((&mut i as *mut ()) as ())) });
    let ci: () = 0_();
    (unsafe { test_89(((&ci as *const ()) as ())) });
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
    (unsafe { test_123() });
    (unsafe { test_124() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
