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
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn test_4() -> bool {
    {
        let mut ar: [i32; 24] = [
            1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8,
        ];
        let mut m: () = unimplemented!();
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!();
        let mut k: () = i.clone();
        assert!(i == k);
        let mut j: i32 = 1;
        'loop_: while ((j) <= (unimplemented!() as i32)) {
            assert!(((*i.iter_deref()) == (j)));
            {
                j.prefix_inc();
                i.iter_inc()
            };
        }
        assert!(i == unimplemented!());
        let mut j: i32 = (unimplemented!() as i32);
        'loop_: while ((j) >= (1)) {
            i.iter_dec();
            assert!(((*i.iter_deref()) == (j)));
            j.prefix_dec();
        }
        assert!(i == unimplemented!());
    }
    {
        let mut ar: [i32; 24] = [
            1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8,
        ];
        let m: () = unimplemented!();
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!();
        let mut j: i32 = 1;
        'loop_: while ((j) <= (unimplemented!() as i32)) {
            assert!(((*i.iter_deref()) == (j)));
            {
                j.prefix_inc();
                i.iter_inc()
            };
        }
        assert!(i == unimplemented!());
        let mut j: i32 = (unimplemented!() as i32);
        'loop_: while ((j) >= (1)) {
            i.iter_dec();
            assert!(((*i.iter_deref()) == (j)));
            j.prefix_dec();
        }
        assert!(i == unimplemented!());
    }
    {
        let mut ar: [i32; 24] = [
            1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8,
        ];
        let mut m: () = unimplemented!();
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!().clone();
        let mut k: () = i.clone();
        assert!(i == k);
        let mut j: i32 = 1;
        'loop_: while ((j) <= (unimplemented!() as i32)) {
            assert!(((*i.iter_deref()) == (j)));
            {
                j.prefix_inc();
                i.iter_inc()
            };
        }
        assert!(i == unimplemented!());
        let mut j: i32 = (unimplemented!() as i32);
        'loop_: while ((j) >= (1)) {
            i.iter_dec();
            assert!(((*i.iter_deref()) == (j)));
            j.prefix_dec();
        }
        assert!(i == unimplemented!());
    }
    {
        let mut ar: [i32; 24] = [
            1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8,
        ];
        let m: () = unimplemented!();
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!().clone();
        let mut j: i32 = 1;
        'loop_: while ((j) <= (unimplemented!() as i32)) {
            assert!(((*i.iter_deref()) == (j)));
            {
                j.prefix_inc();
                i.iter_inc()
            };
        }
        assert!(i == unimplemented!());
        let mut j: i32 = (unimplemented!() as i32);
        'loop_: while ((j) >= (1)) {
            i.iter_dec();
            assert!(((*i.iter_deref()) == (j)));
            j.prefix_dec();
        }
        assert!(i == unimplemented!());
    }
    {
        let mut ii1: () = unimplemented!();
        let mut ii2: () = unimplemented!();
        let mut ii4: () = ii1.clone();
        let mut cii: () = unimplemented!();
        assert!(ii1 == ii2);
        assert!(ii1 == ii4);
        assert!(!(ii1 != ii2));
        assert!((ii1 == cii));
        assert!((cii == ii1));
        assert!(!(ii1 != cii));
        assert!(!(cii != ii1));
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
    (unsafe { test_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
