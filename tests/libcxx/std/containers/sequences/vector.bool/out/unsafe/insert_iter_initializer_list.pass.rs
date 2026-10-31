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
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut bool,
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut u64,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut u64) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut u64 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut u64 {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut u64 {
        return &mut (*self.ptr_.offset((n) as isize));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn new_1(mut _a0: min_allocator_bool_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(min_allocator_bool_, new_1, value)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl min_allocator_unsigned_long_ {}
pub unsafe fn tests_4() -> bool {
    {
        let mut d: Vec<bool> = unimplemented!();
        let mut i: UnsafeRandomAccessIterator<bool> = unimplemented!();
        assert!((((&d).len()) == (14_usize)));
        assert!(i == d.begin().iter_add(2_()));
        assert!(((*(&mut d).as_mut_ptr().add(0_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(1_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(2_usize as usize) as i32) == (false as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(3_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(4_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(5_usize as usize) as i32) == (false as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(6_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(7_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(8_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(9_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(10_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(11_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(12_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(13_usize as usize) as i32) == (true as i32)));
    }
    {
        let mut d: Vec<bool> = unimplemented!();
        let mut i: UnsafeRandomAccessIterator<bool> = unimplemented!();
        assert!((((&d).len()) == (14_usize)));
        assert!(i == d.begin().iter_add(2_()));
        assert!(((*(&mut d).as_mut_ptr().add(0_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(1_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(2_usize as usize) as i32) == (false as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(3_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(4_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(5_usize as usize) as i32) == (false as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(6_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(7_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(8_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(9_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(10_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(11_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(12_usize as usize) as i32) == (true as i32)));
        assert!(((*(&mut d).as_mut_ptr().add(13_usize as usize) as i32) == (true as i32)));
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
    (unsafe { tests_4() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
