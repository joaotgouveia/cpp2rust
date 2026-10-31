extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_1(x: *const DefaultOnly, y: *const DefaultOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const DefaultOnly, y: *const DefaultOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct DefaultOnly {
    data_: i32,
}
impl DefaultOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self { data_: -1_i32 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_dec();
    }
}
impl std::cmp::Ord for DefaultOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_2(self as *const DefaultOnly, other as *const DefaultOnly) {
                std::cmp::Ordering::Less
            } else if operator_lt_2(other as *const DefaultOnly, self as *const DefaultOnly) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for DefaultOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const DefaultOnly, other as *const DefaultOnly) }
    }
}
impl std::cmp::Eq for DefaultOnly {}
impl Default for DefaultOnly {
    fn default() -> Self {
        unsafe { DefaultOnly::new() }
    }
}
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_6)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_3)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) = 0_usize;
    }
}
pub static mut outstanding_bytes_3: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_4: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_5: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_6: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut DefaultOnly,
}
impl min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_7(
    mut _a0: min_allocator_DefaultOnly_,
    mut _a1: min_allocator_DefaultOnly_,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_DefaultOnly_ {}
impl std::cmp::PartialEq for min_allocator_DefaultOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_7(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_DefaultOnly_ {}
pub unsafe fn check_allocator_8(mut n: u32, alloc: Option<*const ()>) {
    let mut __alloc_default: Option<()> = None;
    let mut alloc: *const () =
        alloc.unwrap_or_else(|| unsafe { __alloc_default.insert(unimplemented!()) as *const () });
    let mut d: () = unimplemented!();
    assert!(unimplemented!());
    assert!(((unimplemented!() as usize) == (n as usize)));
}
pub unsafe fn check_allocator_9(mut n: u32, alloc: Option<*const min_allocator_DefaultOnly_>) {
    let mut __alloc_default: Option<min_allocator_DefaultOnly_> = None;
    let mut alloc: *const min_allocator_DefaultOnly_ = alloc.unwrap_or_else(|| unsafe {
        __alloc_default.insert(<min_allocator_DefaultOnly_>::default())
            as *const min_allocator_DefaultOnly_
    });
    let mut d: () = unimplemented!();
    assert!(
        (unsafe {
            let _arg0: min_allocator_DefaultOnly_ = unimplemented!();
            let _arg1: min_allocator_DefaultOnly_ = (*alloc);
            operator_eq_7(_arg0, _arg1)
        })
    );
    assert!(((unimplemented!() as usize) == (n as usize)));
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
    {
        const _: () = assert!(true, "(std::is_constructible<C, std::size_t>::value)");;
        const _: () = assert!(true, "(!std::is_convertible<std::size_t, C>::value)");;
    }
    {
        let mut N: u32 = 10_u32;
        let mut c: () = unimplemented!();
        let mut n: u32 = 0_u32;
        let mut i: () = <()>::from(unimplemented!());
        let mut e: () = <()>::from(unimplemented!());
        'loop_: while i != e {
            assert!(
                (unsafe {
                    let _x: *const DefaultOnly = &(*i.iter_deref());
                    let mut _y: DefaultOnly = DefaultOnly::new();
                    operator_eq_1(_x, &mut _y)
                })
            );
            {
                i.iter_inc();
                n.prefix_inc()
            };
        }
        assert!(((n) == (N)));
    }
    {
        let mut N: u32 = 10_u32;
        let mut c: () = unimplemented!();
        let mut n: u32 = 0_u32;
        let mut i: () = <()>::from(unimplemented!());
        let mut e: () = <()>::from(unimplemented!());
        'loop_: while i != e {
            assert!(
                (unsafe {
                    let _x: *const DefaultOnly = &(*i.iter_deref());
                    let mut _y: DefaultOnly = DefaultOnly::new();
                    operator_eq_1(_x, &mut _y)
                })
            );
            {
                i.iter_inc();
                n.prefix_inc()
            };
        }
        assert!(((n) == (N)));
        (unsafe { check_allocator_8(0_u32, None) });
        (unsafe { check_allocator_8(3_u32, None) });
    }
    {
        let mut N: u32 = 10_u32;
        let mut c: () = unimplemented!();
        let mut n: u32 = 0_u32;
        let mut i: () = <()>::from(unimplemented!());
        let mut e: () = <()>::from(unimplemented!());
        'loop_: while i != e {
            assert!(
                (unsafe {
                    let _x: *const DefaultOnly = &(*i.iter_deref());
                    let mut _y: DefaultOnly = DefaultOnly::new();
                    operator_eq_1(_x, &mut _y)
                })
            );
            {
                i.iter_inc();
                n.prefix_inc()
            };
        }
        assert!(((n) == (N)));
        (unsafe { check_allocator_9(0_u32, None) });
        (unsafe { check_allocator_9(3_u32, None) });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_3);
    std::cell::LazyCell::force(&*&raw const alloc_count_4);
    std::cell::LazyCell::force(&*&raw const dealloc_count_5);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_6);
}
