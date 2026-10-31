extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn CheckConsecutiveKeys_0(mut pos: (), mut end: (), key: (), values: *mut ()) {
    'loop_: while !(unimplemented!()) {
        assert!(pos != end);
        assert!((((*pos.iter_deref()).0) == (key)));
        assert!(unimplemented!() != unimplemented!());
        unimplemented!();
        pos.iter_inc();
    }
    assert!((pos == end) || (((*pos.iter_deref()).0) != (key)));
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
        let mut a: [(i32, Vec<libc::c_char>); 6] = [
            (
                1,
                std::slice::from_raw_parts(
                    c"one".as_ptr(),
                    libcc2rs::c_str_len(c"one".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"two".as_ptr(),
                    libcc2rs::c_str_len(c"two".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                3,
                std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                4,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                1,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c: () = unimplemented!();
        let mut i: () = <()>::from(unimplemented!());
        let mut j: () = libcc2rs::next_unsafe(i.clone(), 2_isize);
        let mut k: () = unimplemented!();
        assert!(k == i);
        assert!(((unimplemented!()) == (6_())));
        let mut eq: ((), ()) = unimplemented!();
        assert!(((unimplemented!()) == (2_isize)));
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), &mut s)
        });
        eq = unimplemented!();
        assert!(((unimplemented!()) == (2_isize)));
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (2 as ()), &mut s)
        });
        eq = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        k = eq.0.clone();
        assert!((((*k.iter_deref()).0) == (3)));
        assert!(
            (*k.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1
                )
                .to_vec()
        );
        eq = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        k = eq.0.clone();
        assert!((((*k.iter_deref()).0) == (4)));
        assert!(
            (*k.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        k = unimplemented!();
        assert!(((unimplemented!()) == (4_())));
        eq = unimplemented!();
        assert!(((unimplemented!()) == (2_isize)));
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), &mut s)
        });
        eq = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        k = eq.0.clone();
        assert!((((*k.iter_deref()).0) == (3)));
        assert!(
            (*k.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1
                )
                .to_vec()
        );
        eq = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        k = eq.0.clone();
        assert!((((*k.iter_deref()).0) == (4)));
        assert!(
            (*k.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        k = unimplemented!();
        assert!(((unimplemented!()) == (0_())));
        assert!(k == unimplemented!());
    }
    {
        {
            let mut map: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut map: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut map: () = unimplemented!();
        let mut collision_val: usize = ((2_()).wrapping_add(unimplemented!()) as usize);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        let mut erase: ((), ()) = unimplemented!();
        unimplemented!();
        'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
            let v: *const (u64, u64) = &(*v.iter_deref());
            assert!((((*v).0) == (1_u64)) || (((*v).0 as usize) == (collision_val)));
        }
    }
    {
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut pair: ((), ()) = unimplemented!();
            assert!(pair.0 != pair.1);
            unimplemented!();
        }
        {
            let mut pair: ((), ()) = unimplemented!();
            assert!(pair.0 != pair.1);
            unimplemented!();
        }
        unimplemented!();
        assert!(((unimplemented!()) == (1_())));
        assert!(unimplemented!());
        assert!((*unimplemented!().iter_inc()) == unimplemented!());
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 6] = [
            (
                1,
                std::slice::from_raw_parts(
                    c"one".as_ptr(),
                    libcc2rs::c_str_len(c"one".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"two".as_ptr(),
                    libcc2rs::c_str_len(c"two".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                3,
                std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                4,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                1,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c: () = unimplemented!();
        let mut i: () = <()>::from(unimplemented!());
        let mut j: () = libcc2rs::next_unsafe(i.clone(), 2_isize);
        let mut k: () = unimplemented!();
        assert!(k == i);
        assert!(((unimplemented!()) == (6_())));
        let mut eq: ((), ()) = unimplemented!();
        assert!(((unimplemented!()) == (2_isize)));
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), &mut s)
        });
        eq = unimplemented!();
        assert!(((unimplemented!()) == (2_isize)));
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (2 as ()), &mut s)
        });
        eq = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        k = eq.0.clone();
        assert!((((*k.iter_deref()).0) == (3)));
        assert!(
            (*k.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1
                )
                .to_vec()
        );
        eq = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        k = eq.0.clone();
        assert!((((*k.iter_deref()).0) == (4)));
        assert!(
            (*k.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        k = unimplemented!();
        assert!(((unimplemented!()) == (4_())));
        eq = unimplemented!();
        assert!(((unimplemented!()) == (2_isize)));
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), &mut s)
        });
        eq = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        k = eq.0.clone();
        assert!((((*k.iter_deref()).0) == (3)));
        assert!(
            (*k.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1
                )
                .to_vec()
        );
        eq = unimplemented!();
        assert!(((unimplemented!()) == (1_isize)));
        k = eq.0.clone();
        assert!((((*k.iter_deref()).0) == (4)));
        assert!(
            (*k.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        k = unimplemented!();
        assert!(((unimplemented!()) == (0_())));
        assert!(k == unimplemented!());
    }
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyHash_5 {}
impl MyHash_5 {
    pub unsafe fn operator_call(&self, mut v: usize) -> usize {
        return v;
    }
}
impl Callable1<usize, usize> for MyHash_5 {
    fn call(&self, a1: usize) -> usize {
        unsafe { MyHash_5::operator_call(self, a1) }
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
