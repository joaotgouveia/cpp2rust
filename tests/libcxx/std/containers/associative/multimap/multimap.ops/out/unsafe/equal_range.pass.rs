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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct PrivateConstructor {
    val: i32,
}
impl PrivateConstructor {
    pub unsafe fn make(mut v: i32) -> PrivateConstructor {
        return PrivateConstructor::new({ v });
    }
    pub unsafe fn get(&self) -> i32 {
        return self.val;
    }
    unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { val: v };
        this
    }
}
impl std::cmp::Ord for PrivateConstructor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(
                self as *const PrivateConstructor,
                other as *const PrivateConstructor,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(
                other as *const PrivateConstructor,
                self as *const PrivateConstructor,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for PrivateConstructor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PrivateConstructor {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_4(
                self as *const PrivateConstructor,
                other as *const PrivateConstructor,
            )) && !(operator_lt_4(
                other as *const PrivateConstructor,
                self as *const PrivateConstructor,
            ))
        }
    }
}
impl std::cmp::Eq for PrivateConstructor {}
pub unsafe fn operator_lt_4(
    lhs: *const PrivateConstructor,
    rhs: *const PrivateConstructor,
) -> bool {
    return ((unsafe { PrivateConstructor::get(&(*lhs)) })
        < (unsafe { PrivateConstructor::get(&(*rhs)) }));
}
pub unsafe fn operator_lt_5(lhs: *const PrivateConstructor, mut rhs: i32) -> bool {
    return ((unsafe { PrivateConstructor::get(&(*lhs)) }) < (rhs));
}
pub unsafe fn operator_lt_6(mut lhs: i32, rhs: *const PrivateConstructor) -> bool {
    return ((lhs) < (unsafe { PrivateConstructor::get(&(*rhs)) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_not_referenceable {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_nonempty_is_transparent {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_nonempty {
    pub pad_: transparent_less_nonempty_is_transparent,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_no_type {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_private {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_not_a_type {
    pub is_transparent: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct C2Int {
    i_: i32,
}
impl C2Int {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { i_: 0 };
        this
    }
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn get(&self) -> i32 {
        return self.i_;
    }
}
impl std::cmp::Ord for C2Int {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_7(self as *const C2Int, other as *const C2Int) {
                std::cmp::Ordering::Less
            } else if operator_lt_7(other as *const C2Int, self as *const C2Int) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for C2Int {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for C2Int {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_7(self as *const C2Int, other as *const C2Int))
                && !(operator_lt_7(other as *const C2Int, self as *const C2Int))
        }
    }
}
impl std::cmp::Eq for C2Int {}
impl Default for C2Int {
    fn default() -> Self {
        unsafe { C2Int::new_2() }
    }
}
pub unsafe fn operator_lt_8(mut rhs: i32, lhs: *const C2Int) -> bool {
    return ((rhs) < (unsafe { C2Int::get(&(*lhs)) }));
}
pub unsafe fn operator_lt_7(rhs: *const C2Int, lhs: *const C2Int) -> bool {
    return ((unsafe { C2Int::get(&(*rhs)) }) < (unsafe { C2Int::get(&(*lhs)) }));
}
pub unsafe fn operator_lt_9(rhs: *const C2Int, mut lhs: i32) -> bool {
    return ((unsafe { C2Int::get(&(*rhs)) }) < (lhs));
}
pub unsafe fn test_10() -> bool {
    {
        {
            let mut ar: [(i32, f64); 9] = [
                (5, (1 as f64)),
                (5, (2 as f64)),
                (5, (3 as f64)),
                (7, (1 as f64)),
                (7, (2 as f64)),
                (7, (3 as f64)),
                (9, (1 as f64)),
                (9, (2 as f64)),
                (9, (3 as f64)),
            ];
            let mut m: () = unimplemented!();
            let mut r: (
                UnsafeBidirectionalIterator<(i32, f64)>,
                UnsafeBidirectionalIterator<(i32, f64)>,
            ) = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == unimplemented!());
            r = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 9_isize));
            r = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == unimplemented!());
        }
        {
            let mut ar: [(i32, f64); 9] = [
                (5, (1 as f64)),
                (5, (2 as f64)),
                (5, (3 as f64)),
                (7, (1 as f64)),
                (7, (2 as f64)),
                (7, (3 as f64)),
                (9, (1 as f64)),
                (9, (2 as f64)),
                (9, (3 as f64)),
            ];
            let m: () = unimplemented!();
            let mut r: (
                UnsafeBidirectionalIterator<(i32, f64)>,
                UnsafeBidirectionalIterator<(i32, f64)>,
            ) = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == unimplemented!());
            r = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 9_isize));
            r = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == unimplemented!());
        }
    }
    {
        {
            let mut ar: [(i32, f64); 9] = [
                (5, (1 as f64)),
                (5, (2 as f64)),
                (5, (3 as f64)),
                (7, (1 as f64)),
                (7, (2 as f64)),
                (7, (3 as f64)),
                (9, (1 as f64)),
                (9, (2 as f64)),
                (9, (3 as f64)),
            ];
            let mut m: () = unimplemented!();
            let mut r: (
                UnsafeBidirectionalIterator<(i32, f64)>,
                UnsafeBidirectionalIterator<(i32, f64)>,
            ) = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == unimplemented!());
            r = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 9_isize));
            r = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == unimplemented!());
        }
        {
            let mut ar: [(i32, f64); 9] = [
                (5, (1 as f64)),
                (5, (2 as f64)),
                (5, (3 as f64)),
                (7, (1 as f64)),
                (7, (2 as f64)),
                (7, (3 as f64)),
                (9, (1 as f64)),
                (9, (2 as f64)),
                (9, (3 as f64)),
            ];
            let m: () = unimplemented!();
            let mut r: (
                UnsafeBidirectionalIterator<(i32, f64)>,
                UnsafeBidirectionalIterator<(i32, f64)>,
            ) = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == unimplemented!());
            r = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            r = unimplemented!();
            assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
            assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 9_isize));
            r = unimplemented!();
            assert!(r.0 == unimplemented!());
            assert!(r.1 == unimplemented!());
        }
    }
    {
        let mut ar: [(i32, f64); 9] = [
            (5, (1 as f64)),
            (5, (2 as f64)),
            (5, (3 as f64)),
            (7, (1 as f64)),
            (7, (2 as f64)),
            (7, (3 as f64)),
            (9, (1 as f64)),
            (9, (2 as f64)),
            (9, (3 as f64)),
        ];
        let mut m: () = unimplemented!();
        let mut r: (
            UnsafeBidirectionalIterator<(i32, f64)>,
            UnsafeBidirectionalIterator<(i32, f64)>,
        ) = unimplemented!();
        assert!(r.0 == unimplemented!());
        assert!(r.1 == unimplemented!());
        r = unimplemented!();
        assert!(r.0 == unimplemented!());
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 9_isize));
        r = unimplemented!();
        assert!(r.0 == unimplemented!());
        assert!(r.1 == unimplemented!());
        r = unimplemented!();
        assert!(r.0 == unimplemented!());
        assert!(r.1 == unimplemented!());
        r = unimplemented!();
        assert!(r.0 == unimplemented!());
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 9_isize));
        r = unimplemented!();
        assert!(r.0 == unimplemented!());
        assert!(r.1 == unimplemented!());
    }
    {
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut r: (
            UnsafeBidirectionalIterator<(PrivateConstructor, f64)>,
            UnsafeBidirectionalIterator<(PrivateConstructor, f64)>,
        ) = unimplemented!();
        assert!(r.0 == unimplemented!());
        assert!(r.1 == unimplemented!());
        r = unimplemented!();
        assert!(r.0 == unimplemented!());
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        r = unimplemented!();
        assert!(r.0 == libcc2rs::next_unsafe(unimplemented!(), 6_isize));
        assert!(r.1 == libcc2rs::next_unsafe(unimplemented!(), 9_isize));
        r = unimplemented!();
        assert!(r.0 == unimplemented!());
        assert!(r.1 == unimplemented!());
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
    (unsafe { test_10() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
