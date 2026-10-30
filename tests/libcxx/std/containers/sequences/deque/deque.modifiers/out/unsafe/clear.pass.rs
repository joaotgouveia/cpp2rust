extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotConstructible {}
impl NotConstructible {}
impl std::cmp::PartialEq for NotConstructible {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const NotConstructible,
                other as *const NotConstructible,
            )
        }
    }
}
impl std::cmp::Eq for NotConstructible {}
pub unsafe fn operator_eq_0(_a0: *const NotConstructible, _a1: *const NotConstructible) -> bool {
    return true;
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
pub struct min_allocator_NotConstructible_ {}
impl min_allocator_NotConstructible_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        return min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_NotConstructible_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe { <std_allocator_NotConstructible_>::default().deallocate(p.ptr_, n) });
    }
}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_NotConstructible_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( unsafe { <std_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ >::default() . allocate  ( n , ) } )  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            <std_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ >::default() . deallocate  ( p . ptr_  , n , )
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_int_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe { <std_allocator_int_>::default().deallocate(p.ptr_, n) });
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( unsafe { <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ >::default() . allocate  ( n , ) } )  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>::default()
                .deallocate(p.ptr_, n)
        });
    }
}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
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
        let mut c: Vec<NotConstructible> = Vec::new();
        const _: () = assert!(true, "");;
        (unsafe { c.clear() });
        assert!(
            ((unsafe {
                let ___first: UnsafeRandomAccessIterator<NotConstructible> =
                    (unsafe { c.begin_24() });
                let ___last: UnsafeRandomAccessIterator<NotConstructible> = (unsafe { c.end_26() });
                distance_5(___first, ___last)
            }) == (0_isize))
        );
        const _: () = assert!(true, "");;
    }
    {
        let t: [i32; 5] = [0, 1, 2, 3, 4];
        let mut c: Vec<i32> = std_deque_int__std_allocator_int__::new_1(
            { (unsafe { begin_6(&t) }) },
            { t.as_ptr().add(t.len()) },
            None,
        );
        const _: () = assert!(true, "");;
        (unsafe { c.clear() });
        assert!(
            ((unsafe {
                let ___first: UnsafeRandomAccessIterator<i32> = (unsafe { c.begin_24() });
                let ___last: UnsafeRandomAccessIterator<i32> = (unsafe { c.end_26() });
                distance_7(___first, ___last)
            }) == (0_isize))
        );
        const _: () = assert!(true, "");;
        (unsafe { c.clear() });
        assert!(
            ((unsafe {
                let ___first: UnsafeRandomAccessIterator<i32> = (unsafe { c.begin_24() });
                let ___last: UnsafeRandomAccessIterator<i32> = (unsafe { c.end_26() });
                distance_7(___first, ___last)
            }) == (0_isize))
        );
        const _: () = assert!(true, "");;
    }
    {
        let mut c: std_deque_NotConstructible__min_allocator_NotConstructible__ =
            <std_deque_NotConstructible__min_allocator_NotConstructible__>::default();
        const _: () = assert!(true, "");;
        (unsafe { c.clear() });
        assert!(
            ((unsafe {
                let ___first: UnsafeRandomAccessIterator<NotConstructible> =
                    (unsafe { c.begin_24() });
                let ___last: UnsafeRandomAccessIterator<NotConstructible> = (unsafe { c.end_26() });
                distance_8(___first, ___last)
            }) == (0_isize))
        );
        const _: () = assert!(true, "");;
    }
    {
        let t: [i32; 5] = [0, 1, 2, 3, 4];
        let mut c: std_deque_int__min_allocator_int__ = std_deque_int__min_allocator_int__::new_1(
            { (unsafe { begin_6(&t) }) },
            { t.as_ptr().add(t.len()) },
            None,
        );
        const _: () = assert!(true, "");;
        (unsafe { c.clear() });
        assert!(
            ((unsafe {
                let ___first: UnsafeRandomAccessIterator<i32> = (unsafe { c.begin_24() });
                let ___last: UnsafeRandomAccessIterator<i32> = (unsafe { c.end_26() });
                distance_9(___first, ___last)
            }) == (0_isize))
        );
        const _: () = assert!(true, "");;
        (unsafe { c.clear() });
        assert!(
            ((unsafe {
                let ___first: UnsafeRandomAccessIterator<i32> = (unsafe { c.begin_24() });
                let ___last: UnsafeRandomAccessIterator<i32> = (unsafe { c.end_26() });
                distance_9(___first, ___last)
            }) == (0_isize))
        );
        const _: () = assert!(true, "");;
    }
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
