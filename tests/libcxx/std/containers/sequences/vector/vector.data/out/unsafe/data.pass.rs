extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const *mut ::libc::c_void) -> *const *mut ::libc::c_void {
    c"";
    return value;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)) = 0_usize;
    }
}
pub static mut outstanding_bytes_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_3: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_4: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_5(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_6(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_7(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_6(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut i32) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut i32 {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_8(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_int__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const i32,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *const ::libc::c_void,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_9(
    mut x: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<Nasty>()) as i64
        as isize);
}
pub unsafe fn operator_eq_10(
    mut x: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_11(
    mut x: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_Nasty__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_10(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut Nasty,
}
impl min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut Nasty) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut Nasty {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut Nasty {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_Nasty__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_Nasty__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_Nasty__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_Nasty__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_Nasty__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
pub unsafe fn operator_eq_12(
    mut x: min_pointer_const_Nasty__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_Nasty__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_Nasty__std_integral_constant_unsigned_long__0__ {
    ptr_: *const Nasty,
}
impl min_pointer_const_Nasty__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
}
impl std::cmp::PartialEq for min_pointer_const_Nasty__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_Nasty__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_Nasty_ {}
impl min_allocator_Nasty_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_Nasty__std_integral_constant_unsigned_long__0__ {
        return min_pointer_Nasty__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_Nasty__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        let mut memory: *mut i32 = unimplemented!();
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in
                    0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in
                        0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        unsafe { safe_allocator_int_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_Nasty_ {}
impl safe_allocator_Nasty_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut Nasty {
        let mut memory: *mut Nasty = unimplemented!();
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in
                    0..((::std::mem::size_of::<Nasty>() as u64).wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut Nasty, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..((::std::mem::size_of::<Nasty>() as u64)
                        .wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_Nasty_ {
    fn default() -> Self {
        unsafe { safe_allocator_Nasty_::new() }
    }
}
pub unsafe fn is_contiguous_container_asan_correct_13(_a0: *const Vec<i32>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_14(_a0: *const Vec<Nasty>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_15(_a0: *const Vec<i32>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_16(_a0: *const Vec<Nasty>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_17(_a0: *const Vec<i32>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_18(_a0: *const Vec<Nasty>) -> bool {
    return true;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct Nasty {
    pub i_: i32,
}
impl Nasty {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { i_: 0 };
        this
    }
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn operator_addr(&self) -> *mut Nasty {
        assert!(false);
        return std::ptr::null_mut();
    }
}
impl Default for Nasty {
    fn default() -> Self {
        unsafe { Nasty::new_2() }
    }
}
pub unsafe fn tests_19() -> bool {
    {
        let mut v: Vec<i32> = Vec::new();
        assert!((v.as_mut_ptr()).is_null());
        assert!((unsafe { is_contiguous_container_asan_correct_13(&v,) }));
    }
    {
        let mut v: Vec<i32> = (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>();
        assert!(((v.as_mut_ptr()) == (unimplemented!())));
        assert!((unsafe { is_contiguous_container_asan_correct_13(&v,) }));
    }
    {
        let mut v: Vec<Nasty> = (0..(100_usize) as usize)
            .map(|_| <Nasty>::default())
            .collect::<Vec<_>>();
        assert!(((v.as_mut_ptr()) == (unimplemented!())));
        assert!((unsafe { is_contiguous_container_asan_correct_14(&v,) }));
    }
    {
        let mut v: Vec<i32> = Vec::new();
        assert!((v.as_mut_ptr()).is_null());
        assert!((unsafe { is_contiguous_container_asan_correct_15(&v,) }));
    }
    {
        let mut v: Vec<i32> = (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>();
        assert!(((v.as_mut_ptr()) == (unimplemented!())));
        assert!((unsafe { is_contiguous_container_asan_correct_15(&v,) }));
    }
    {
        let mut v: Vec<Nasty> = (0..(100_usize) as usize)
            .map(|_| <Nasty>::default())
            .collect::<Vec<_>>();
        assert!(((v.as_mut_ptr()) == (unimplemented!())));
        assert!((unsafe { is_contiguous_container_asan_correct_16(&v,) }));
    }
    {
        let mut v: Vec<i32> = Vec::new();
        assert!((v.as_mut_ptr()).is_null());
        assert!((unsafe { is_contiguous_container_asan_correct_17(&v,) }));
    }
    {
        let mut v: Vec<i32> = (0..(100_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>();
        assert!(((v.as_mut_ptr()) == (unimplemented!())));
        assert!((unsafe { is_contiguous_container_asan_correct_17(&v,) }));
    }
    {
        let mut v: Vec<Nasty> = (0..(100_usize) as usize)
            .map(|_| <Nasty>::default())
            .collect::<Vec<_>>();
        assert!(((v.as_mut_ptr()) == (unimplemented!())));
        assert!((unsafe { is_contiguous_container_asan_correct_18(&v,) }));
    }
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { tests_19() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
