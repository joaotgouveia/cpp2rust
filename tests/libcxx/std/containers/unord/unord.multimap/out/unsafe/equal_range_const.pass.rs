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
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (i32, Vec<libc::c_char>),
}
impl min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
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
    {
        let mut a: [(i32, Vec<libc::c_char>); 10] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fiftyA".as_ptr(),
                    libcc2rs::c_str_len(c"fiftyA".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fiftyB".as_ptr(),
                    libcc2rs::c_str_len(c"fiftyB".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let c: () = unimplemented!();
        let mut r: ((), ()) = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        assert!((((*r.0.iter_deref()).0) == (30)));
        assert!(
            (*r.0.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1
                )
                .to_vec()
        );
        r = unimplemented!();
        assert!(((unimplemented!()) == (0_isize)));
        r = unimplemented!();
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            assert!((((*r.0.iter_deref()).0) == (50)));
            assert!(unimplemented!() != unimplemented!());
            unimplemented!();
            r.0.iter_inc();
            i.prefix_inc();
        }
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 10] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fiftyA".as_ptr(),
                    libcc2rs::c_str_len(c"fiftyA".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fiftyB".as_ptr(),
                    libcc2rs::c_str_len(c"fiftyB".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let c: () = unimplemented!();
        let mut r: ((), ()) = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        assert!((((*r.0.iter_deref()).0) == (30)));
        assert!(
            (*r.0.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1
                )
                .to_vec()
        );
        r = unimplemented!();
        assert!(((unimplemented!()) == (0_isize)));
        r = unimplemented!();
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            assert!((((*r.0.iter_deref()).0) == (50)));
            assert!(unimplemented!() != unimplemented!());
            unimplemented!();
            r.0.iter_inc();
            i.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
