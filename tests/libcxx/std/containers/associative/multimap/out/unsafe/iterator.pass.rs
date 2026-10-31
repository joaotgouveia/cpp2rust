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
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (i32, f64),
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__double__ {}
pub unsafe fn test_4() -> bool {
    {
        let mut ar: [(i32, f64); 24] = [
            (1, (1 as f64)),
            (1, 1.5_f64),
            (1, (2 as f64)),
            (2, (1 as f64)),
            (2, 1.5_f64),
            (2, (2 as f64)),
            (3, (1 as f64)),
            (3, 1.5_f64),
            (3, (2 as f64)),
            (4, (1 as f64)),
            (4, 1.5_f64),
            (4, (2 as f64)),
            (5, (1 as f64)),
            (5, 1.5_f64),
            (5, (2 as f64)),
            (6, (1 as f64)),
            (6, 1.5_f64),
            (6, (2 as f64)),
            (7, (1 as f64)),
            (7, 1.5_f64),
            (7, (2 as f64)),
            (8, (1 as f64)),
            (8, 1.5_f64),
            (8, (2 as f64)),
        ];
        let mut m: () = unimplemented!();
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!();
        let mut k: () = <UnsafeBidirectionalIterator<(i32, f64)>>::from((i).clone());
        assert!(<UnsafeBidirectionalIterator<(i32, f64)>>::from((i).clone()) == k);
        let mut j: i32 = 1;
        'loop_: while ((j) <= (8)) {
            let mut d: f64 = 1_f64;
            'loop_: while ((d) <= (2_f64)) {
                assert!((((*i.iter_deref()).0) == (j)));
                assert!((((*i.iter_deref()).1) == (d)));
                (*i.iter_deref()).1 = 2.5_f64;
                assert!((((*i.iter_deref()).1) == (2.5_f64)));
                {
                    d += 0.5_f64;
                    i.iter_inc()
                };
            }
            j.prefix_inc();
        }
        assert!(i == unimplemented!());
        let mut j: i32 = 8;
        'loop_: while ((j) >= (1)) {
            let mut d: f64 = 1_f64;
            'loop_: while ((d) <= (2_f64)) {
                i.iter_dec();
                assert!((((*i.iter_deref()).0) == (j)));
                assert!((((*i.iter_deref()).1) == (2.5_f64)));
                (*i.iter_deref()).1 = d;
                assert!((((*i.iter_deref()).1) == (d)));
                d += 0.5_f64;
            }
            j.prefix_dec();
        }
        assert!(i == unimplemented!());
    }
    {
        let mut ar: [(i32, f64); 24] = [
            (1, (1 as f64)),
            (1, 1.5_f64),
            (1, (2 as f64)),
            (2, (1 as f64)),
            (2, 1.5_f64),
            (2, (2 as f64)),
            (3, (1 as f64)),
            (3, 1.5_f64),
            (3, (2 as f64)),
            (4, (1 as f64)),
            (4, 1.5_f64),
            (4, (2 as f64)),
            (5, (1 as f64)),
            (5, 1.5_f64),
            (5, (2 as f64)),
            (6, (1 as f64)),
            (6, 1.5_f64),
            (6, (2 as f64)),
            (7, (1 as f64)),
            (7, 1.5_f64),
            (7, (2 as f64)),
            (8, (1 as f64)),
            (8, 1.5_f64),
            (8, (2 as f64)),
        ];
        let m: () = unimplemented!();
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!();
        let mut j: i32 = 1;
        'loop_: while ((j) <= (8)) {
            let mut d: f64 = 1_f64;
            'loop_: while ((d) <= (2_f64)) {
                assert!((((*i.iter_deref()).0) == (j)));
                assert!((((*i.iter_deref()).1) == (d)));
                {
                    d += 0.5_f64;
                    i.iter_inc()
                };
            }
            j.prefix_inc();
        }
        assert!(i == unimplemented!());
        let mut j: i32 = 8;
        'loop_: while ((j) >= (1)) {
            let mut d: f64 = 2_f64;
            'loop_: while ((d) >= (1_f64)) {
                i.iter_dec();
                assert!((((*i.iter_deref()).0) == (j)));
                assert!((((*i.iter_deref()).1) == (d)));
                d -= 0.5_f64;
            }
            j.prefix_dec();
        }
        assert!(i == unimplemented!());
    }
    {
        let mut ar: [(i32, f64); 24] = [
            (1, (1 as f64)),
            (1, 1.5_f64),
            (1, (2 as f64)),
            (2, (1 as f64)),
            (2, 1.5_f64),
            (2, (2 as f64)),
            (3, (1 as f64)),
            (3, 1.5_f64),
            (3, (2 as f64)),
            (4, (1 as f64)),
            (4, 1.5_f64),
            (4, (2 as f64)),
            (5, (1 as f64)),
            (5, 1.5_f64),
            (5, (2 as f64)),
            (6, (1 as f64)),
            (6, 1.5_f64),
            (6, (2 as f64)),
            (7, (1 as f64)),
            (7, 1.5_f64),
            (7, (2 as f64)),
            (8, (1 as f64)),
            (8, 1.5_f64),
            (8, (2 as f64)),
        ];
        let mut m: () = unimplemented!();
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!().clone();
        let mut k: () = <UnsafeBidirectionalIterator<(i32, f64)>>::from((i).clone());
        assert!(<UnsafeBidirectionalIterator<(i32, f64)>>::from((i).clone()) == k);
        let mut j: i32 = 1;
        'loop_: while ((j) <= (8)) {
            let mut d: f64 = 1_f64;
            'loop_: while ((d) <= (2_f64)) {
                assert!((((*i.iter_deref()).0) == (j)));
                assert!((((*i.iter_deref()).1) == (d)));
                (*i.iter_deref()).1 = 2.5_f64;
                assert!((((*i.iter_deref()).1) == (2.5_f64)));
                {
                    d += 0.5_f64;
                    i.iter_inc()
                };
            }
            j.prefix_inc();
        }
        assert!(i == unimplemented!());
        let mut j: i32 = 8;
        'loop_: while ((j) >= (1)) {
            let mut d: f64 = 1_f64;
            'loop_: while ((d) <= (2_f64)) {
                i.iter_dec();
                assert!((((*i.iter_deref()).0) == (j)));
                assert!((((*i.iter_deref()).1) == (2.5_f64)));
                (*i.iter_deref()).1 = d;
                assert!((((*i.iter_deref()).1) == (d)));
                d += 0.5_f64;
            }
            j.prefix_dec();
        }
        assert!(i == unimplemented!());
    }
    {
        let mut ar: [(i32, f64); 24] = [
            (1, (1 as f64)),
            (1, 1.5_f64),
            (1, (2 as f64)),
            (2, (1 as f64)),
            (2, 1.5_f64),
            (2, (2 as f64)),
            (3, (1 as f64)),
            (3, 1.5_f64),
            (3, (2 as f64)),
            (4, (1 as f64)),
            (4, 1.5_f64),
            (4, (2 as f64)),
            (5, (1 as f64)),
            (5, 1.5_f64),
            (5, (2 as f64)),
            (6, (1 as f64)),
            (6, 1.5_f64),
            (6, (2 as f64)),
            (7, (1 as f64)),
            (7, 1.5_f64),
            (7, (2 as f64)),
            (8, (1 as f64)),
            (8, 1.5_f64),
            (8, (2 as f64)),
        ];
        let m: () = unimplemented!();
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        let mut i: () = unimplemented!();
        i = unimplemented!().clone();
        let mut j: i32 = 1;
        'loop_: while ((j) <= (8)) {
            let mut d: f64 = 1_f64;
            'loop_: while ((d) <= (2_f64)) {
                assert!((((*i.iter_deref()).0) == (j)));
                assert!((((*i.iter_deref()).1) == (d)));
                {
                    d += 0.5_f64;
                    i.iter_inc()
                };
            }
            j.prefix_inc();
        }
        assert!(i == unimplemented!());
        let mut j: i32 = 8;
        'loop_: while ((j) >= (1)) {
            let mut d: f64 = 2_f64;
            'loop_: while ((d) >= (1_f64)) {
                i.iter_dec();
                assert!((((*i.iter_deref()).0) == (j)));
                assert!((((*i.iter_deref()).1) == (d)));
                d -= 0.5_f64;
            }
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
        assert!((<UnsafeBidirectionalIterator<(i32, f64)>>::from((ii1).clone()) == cii));
        assert!((cii == <UnsafeBidirectionalIterator<(i32, f64)>>::from((ii1).clone())));
        assert!(!(<UnsafeBidirectionalIterator<(i32, f64)>>::from((ii1).clone()) != cii));
        assert!(!(cii != <UnsafeBidirectionalIterator<(i32, f64)>>::from((ii1).clone())));
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
