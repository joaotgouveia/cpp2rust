extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const ()) -> *const () {
    c"";
    return value;
}
pub unsafe fn is_string_asan_correct_1(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_2(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_3(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_4(_a0: *const ()) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct increasing_allocator_char_ {
    pub min_elements: (),
}
impl increasing_allocator_char_ {
    pub unsafe fn allocate_at_least(&mut self, mut n: ()) -> () {
        if ((n) < (self.min_elements)) {
            n = self.min_elements;
        }
        self.min_elements = (self.min_elements).wrapping_add(1000_());
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
}
impl Default for increasing_allocator_char_ {
    fn default() -> Self {
        unsafe {
            increasing_allocator_char_ {
                min_elements: 1000_(),
            }
        }
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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_char_ {}
impl safe_allocator_char_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        let mut memory: *mut () = unimplemented!();
        {
            (unsafe {
                memset_9(
                    (memory as ()),
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_9(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_char_ {
    fn default() -> Self {
        unsafe { safe_allocator_char_::new() }
    }
}
pub unsafe fn test_10(s: *mut ()) {
    let mut old_cap: () = unimplemented!();
    let mut s0: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_11(s: *mut ()) {
    let mut old_cap: () = unimplemented!();
    let mut s0: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_12(s: *mut ()) {
    let mut old_cap: () = unimplemented!();
    let mut s0: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_13() {
    let mut s: () = unimplemented!();
    (unsafe { test_10(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_10(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_10(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_10(&mut s) });
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) <= (9_())) {
        unimplemented!();
        (unsafe { test_10(&mut s) });
        i.prefix_inc();
    }
}
pub unsafe fn test_string_14() {
    let mut s: () = unimplemented!();
    (unsafe { test_11(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_11(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_11(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_11(&mut s) });
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) <= (9_())) {
        unimplemented!();
        (unsafe { test_11(&mut s) });
        i.prefix_inc();
    }
}
pub unsafe fn test_string_15() {
    let mut s: () = unimplemented!();
    (unsafe { test_12(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_12(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_12(&mut s) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_12(&mut s) });
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) <= (9_())) {
        unimplemented!();
        (unsafe { test_12(&mut s) });
        i.prefix_inc();
    }
}
pub unsafe fn test_16() -> () {
    (unsafe { test_string_13() });
    (unsafe { test_string_14() });
    (unsafe { test_string_15() });
    {
        let mut s: () =
            std_basic_string_char__std_char_traits_char___increasing_allocator_char__::new_1(
                { c"String does not fit in the internal buffer".as_ptr() },
                None,
            );
        let mut capacity: () = unimplemented!();
        let mut size: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_16() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_5);
    std::cell::LazyCell::force(&*&raw const alloc_count_6);
    std::cell::LazyCell::force(&*&raw const dealloc_count_7);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_8);
}
