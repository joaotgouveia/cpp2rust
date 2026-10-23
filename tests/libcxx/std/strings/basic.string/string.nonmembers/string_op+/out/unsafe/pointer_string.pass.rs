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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct tiny_size_allocator_29__char__rebind_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct tiny_size_allocator_29__char_ {}
impl tiny_size_allocator_29__char_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        unimplemented!();
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut ptr: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return 29_();
    }
}
pub unsafe fn test_string_4() {
    let mut test_data: [[*const (); 4]; 2] = [
        [
            c"".as_ptr(),
            c"abcde".as_ptr(),
            c"abcdefghij".as_ptr(),
            c"abcdefghijklmnopqrst".as_ptr(),
        ],
        [
            c"".as_ptr(),
            c"12345".as_ptr(),
            c"1234567890".as_ptr(),
            c"12345678901234567890".as_ptr(),
        ],
    ];
    let mut results: [[*const (); 4]; 4] = [
        [
            c"".as_ptr(),
            c"12345".as_ptr(),
            c"1234567890".as_ptr(),
            c"12345678901234567890".as_ptr(),
        ],
        [
            c"abcde".as_ptr(),
            c"abcde12345".as_ptr(),
            c"abcde1234567890".as_ptr(),
            c"abcde12345678901234567890".as_ptr(),
        ],
        [
            c"abcdefghij".as_ptr(),
            c"abcdefghij12345".as_ptr(),
            c"abcdefghij1234567890".as_ptr(),
            c"abcdefghij12345678901234567890".as_ptr(),
        ],
        [
            c"abcdefghijklmnopqrst".as_ptr(),
            c"abcdefghijklmnopqrst12345".as_ptr(),
            c"abcdefghijklmnopqrst1234567890".as_ptr(),
            c"abcdefghijklmnopqrst12345678901234567890".as_ptr(),
        ],
    ];
    let mut i: () = 0_();
    'loop_: while ((i) != (4_())) {
        let mut k: () = 0_();
        'loop_: while ((k) != (4_())) {
            {
                let mut lhs: *const () = test_data[(0_()) as usize][(i) as usize];
                let rhs: () = unimplemented!();
                unimplemented!();
            }
            {
                let mut lhs: *const () = test_data[(0_()) as usize][(i) as usize];
                let mut rhs: () = unimplemented!();
                unimplemented!();
            }
            k.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub unsafe fn test_string_5() {
    let mut test_data: [[*const (); 4]; 2] = [
        [
            c"".as_ptr(),
            c"abcde".as_ptr(),
            c"abcdefghij".as_ptr(),
            c"abcdefghijklmnopqrst".as_ptr(),
        ],
        [
            c"".as_ptr(),
            c"12345".as_ptr(),
            c"1234567890".as_ptr(),
            c"12345678901234567890".as_ptr(),
        ],
    ];
    let mut results: [[*const (); 4]; 4] = [
        [
            c"".as_ptr(),
            c"12345".as_ptr(),
            c"1234567890".as_ptr(),
            c"12345678901234567890".as_ptr(),
        ],
        [
            c"abcde".as_ptr(),
            c"abcde12345".as_ptr(),
            c"abcde1234567890".as_ptr(),
            c"abcde12345678901234567890".as_ptr(),
        ],
        [
            c"abcdefghij".as_ptr(),
            c"abcdefghij12345".as_ptr(),
            c"abcdefghij1234567890".as_ptr(),
            c"abcdefghij12345678901234567890".as_ptr(),
        ],
        [
            c"abcdefghijklmnopqrst".as_ptr(),
            c"abcdefghijklmnopqrst12345".as_ptr(),
            c"abcdefghijklmnopqrst1234567890".as_ptr(),
            c"abcdefghijklmnopqrst12345678901234567890".as_ptr(),
        ],
    ];
    let mut i: () = 0_();
    'loop_: while ((i) != (4_())) {
        let mut k: () = 0_();
        'loop_: while ((k) != (4_())) {
            {
                let mut lhs: *const () = test_data[(0_()) as usize][(i) as usize];
                let rhs: () = unimplemented!();
                unimplemented!();
            }
            {
                let mut lhs: *const () = test_data[(0_()) as usize][(i) as usize];
                let mut rhs: () = unimplemented!();
                unimplemented!();
            }
            k.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub unsafe fn test_6() -> () {
    (unsafe { test_string_4() });
    (unsafe { test_string_5() });
    {
        let mut str: () = unimplemented!();
        unimplemented!();
        let mut result: () = unimplemented!();
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
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
