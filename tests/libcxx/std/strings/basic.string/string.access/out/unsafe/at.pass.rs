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
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_char__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_const_char__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_char__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
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
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
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
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_char8_t__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_char8_t__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_char8_t__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_char8_t__std_integral_constant_unsigned_long__0__::operator_add_assign(
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
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
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
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_char16_t__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_char16_t__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_char16_t__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_char16_t__std_integral_constant_unsigned_long__0__::operator_add_assign(
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
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ = (*(self
            as *const min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
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
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_char32_t__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_char32_t__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_char32_t__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_char32_t__std_integral_constant_unsigned_long__0__::operator_add_assign(
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
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ = (*(self
            as *const min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
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
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub unsafe fn for_each_4(
    mut _a0: types_type_list_char__wchar_t__char8_t__char16_t__char32_t_,
    mut f: TestCaller,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { TestCaller::operator_call_1_char(&mut f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { TestCaller::operator_call_1_wchar_t(&mut f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { TestCaller::operator_call_1_char8_t(&mut f) });
            0_()
        };
        let _arg3: () = {
            (unsafe { TestCaller::operator_call_1_char16_t(&mut f) });
            0_()
        };
        let _arg4: () = {
            (unsafe { TestCaller::operator_call_1_char32_t(&mut f) });
            0_()
        };
        swallow_5(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn swallow_5(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: (), mut _a4: ()) {}
pub unsafe fn test_6(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_7(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_8(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_9(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_10(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_11(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_12(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_13(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_14(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_15(mut s: (), mut pos: ()) {
    let cs: *const () = &s;
    if ((pos) < (unimplemented!())) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_string_16() {
    (unsafe { test_6(unimplemented!(), 0_()) });
    (unsafe {
        test_6(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_2(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_6(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_2(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_6(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_2(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_6(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_2(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub unsafe fn test_string_17() {
    (unsafe { test_7(unimplemented!(), 0_()) });
    (unsafe {
        test_7(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_2(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_7(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_2(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_7(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_2(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_7(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_2(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub unsafe fn test_string_18() {
    (unsafe { test_8(unimplemented!(), 0_()) });
    (unsafe {
        test_8(
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_3(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_8(
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_3(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_8(
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_3(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_8(
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_3(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub unsafe fn test_string_19() {
    (unsafe { test_9(unimplemented!(), 0_()) });
    (unsafe {
        test_9(
            std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_3(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_3(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_3(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_wchar_t__std_char_traits_wchar_t___min_allocator_wchar_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_3(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub unsafe fn test_string_20() {
    (unsafe { test_10(unimplemented!(), 0_()) });
    (unsafe {
        test_10(
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_4(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_4(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_4(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_4(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub unsafe fn test_string_21() {
    (unsafe { test_11(unimplemented!(), 0_()) });
    (unsafe {
        test_11(
            std_basic_string_char8_t__std_char_traits_char8_t___min_allocator_char8_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_4(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char8_t__std_char_traits_char8_t___min_allocator_char8_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_4(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char8_t__std_char_traits_char8_t___min_allocator_char8_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_4(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char8_t__std_char_traits_char8_t___min_allocator_char8_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_4(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub unsafe fn test_string_22() {
    (unsafe { test_12(unimplemented!(), 0_()) });
    (unsafe {
        test_12(
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_5(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_5(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_5(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_5(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub unsafe fn test_string_23() {
    (unsafe { test_13(unimplemented!(), 0_()) });
    (unsafe {
        test_13(
            std_basic_string_char16_t__std_char_traits_char16_t___min_allocator_char16_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_5(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char16_t__std_char_traits_char16_t___min_allocator_char16_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_5(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char16_t__std_char_traits_char16_t___min_allocator_char16_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_5(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char16_t__std_char_traits_char16_t___min_allocator_char16_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_5(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub unsafe fn test_string_24() {
    (unsafe { test_14(unimplemented!(), 0_()) });
    (unsafe {
        test_14(
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_6(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_6(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_6(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_6(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
pub unsafe fn test_string_25() {
    (unsafe { test_15(unimplemented!(), 0_()) });
    (unsafe {
        test_15(
            std_basic_string_char32_t__std_char_traits_char32_t___min_allocator_char32_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_6(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            0_(),
        )
    });
    (unsafe {
        test_15(
            std_basic_string_char32_t__std_char_traits_char32_t___min_allocator_char32_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_6(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            1_(),
        )
    });
    (unsafe {
        test_15(
            std_basic_string_char32_t__std_char_traits_char32_t___min_allocator_char32_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_6(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            2_(),
        )
    });
    (unsafe {
        test_15(
            std_basic_string_char32_t__std_char_traits_char32_t___min_allocator_char32_t__::new_1(
                {
                    (unsafe {
                        MultiStringType::as_ptr_6(
                            &MultiStringType::new(
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr() },
                                {
                                    ((::std::mem::size_of::<[(); 4]>() as ())
                                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                                        as ())
                                        .wrapping_sub(1_())
                                },
                                { c"123".as_ptr() },
                                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
                            ),
                            std::ptr::null(),
                        )
                    })
                },
                None,
            ),
            3_(),
        )
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestCaller {}
impl TestCaller {
    pub unsafe fn operator_call_1_char(&mut self) {
        (unsafe { test_string_16() });
        (unsafe { test_string_17() });
    }
    pub unsafe fn operator_call_1_wchar_t(&mut self) {
        (unsafe { test_string_18() });
        (unsafe { test_string_19() });
    }
    pub unsafe fn operator_call_1_char8_t(&mut self) {
        (unsafe { test_string_20() });
        (unsafe { test_string_21() });
    }
    pub unsafe fn operator_call_1_char16_t(&mut self) {
        (unsafe { test_string_22() });
        (unsafe { test_string_23() });
    }
    pub unsafe fn operator_call_1_char32_t(&mut self) {
        (unsafe { test_string_24() });
        (unsafe { test_string_25() });
    }
}
pub unsafe fn test_26() -> () {
    (unsafe {
        for_each_4(
            <types_type_list_char__wchar_t__char8_t__char16_t__char32_t_>::default(),
            <TestCaller>::default(),
        )
    });
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
    (unsafe { test_26() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
