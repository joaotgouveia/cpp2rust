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
impl TemplateConstructor {
    pub unsafe fn new(_a0: *const i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, ptr)]
pub struct TemplateConstructor {}
impl std::cmp::Ord for TemplateConstructor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(
                self as *const TemplateConstructor,
                other as *const TemplateConstructor,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(
                other as *const TemplateConstructor,
                self as *const TemplateConstructor,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for TemplateConstructor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TemplateConstructor {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_4(
                self as *const TemplateConstructor,
                other as *const TemplateConstructor,
            )) && !(operator_lt_4(
                other as *const TemplateConstructor,
                self as *const TemplateConstructor,
            ))
        }
    }
}
impl std::cmp::Eq for TemplateConstructor {}
pub unsafe fn operator_lt_4(
    _a0: *const TemplateConstructor,
    _a1: *const TemplateConstructor,
) -> bool {
    return false;
}
pub unsafe fn test_5() -> bool {
    {
        let mut ar: [(i32, f64); 8] = [
            (1, 1.5_f64),
            (2, 2.5_f64),
            (3, 3.5_f64),
            (4, 4.5_f64),
            (5, 5.5_f64),
            (6, 6.5_f64),
            (7, 7.5_f64),
            (8, 8.5_f64),
        ];
        let mut m: UnsafeMap<i32, (i32, f64)> = unimplemented!();
        assert!(((m.len()) == (8_usize)));
        let mut i: UnsafeBidirectionalIterator<(i32, f64)> =
            m.erase(&libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!(((m.len()) == (7_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 3_isize));
        assert!((((*m.begin().iter_deref()).0) == (1)));
        assert!((((*m.begin().iter_deref()).1) == (1.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (3.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).0) == (6)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).1) == (6.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 5_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 5_isize).iter_deref()).1) == (7.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 6_isize).iter_deref()).0) == (8)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 6_isize).iter_deref()).1) == (8.5_f64)));
        i = m.erase(&libcc2rs::next_unsafe(unimplemented!(), 0_isize));
        assert!(((m.len()) == (6_usize)));
        assert!(i == m.begin());
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (3.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).0) == (6)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).1) == (6.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).1) == (7.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 5_isize).iter_deref()).0) == (8)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 5_isize).iter_deref()).1) == (8.5_f64)));
        i = m.erase(&libcc2rs::next_unsafe(unimplemented!(), 5_isize));
        assert!(((m.len()) == (5_usize)));
        assert!(i == m.end());
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (3.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).0) == (6)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).1) == (6.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).1) == (7.5_f64)));
        i = m.erase(&libcc2rs::next_unsafe(unimplemented!(), 1_isize));
        assert!(((m.len()) == (4_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 1_isize));
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (6)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (6.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).1) == (7.5_f64)));
        i = m.erase(&libcc2rs::next_unsafe(unimplemented!(), 2_isize));
        assert!(((m.len()) == (3_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 2_isize));
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (7.5_f64)));
        i = m.erase(&libcc2rs::next_unsafe(unimplemented!(), 2_isize));
        assert!(((m.len()) == (2_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 2_isize));
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (5.5_f64)));
        i = m.erase(&libcc2rs::next_unsafe(unimplemented!(), 0_isize));
        assert!(((m.len()) == (1_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 0_isize));
        assert!((((*m.begin().iter_deref()).0) == (5)));
        assert!((((*m.begin().iter_deref()).1) == (5.5_f64)));
        i = m.erase(&unimplemented!());
        assert!(((m.len()) == (0_usize)));
        assert!(i == m.begin());
        assert!(i == m.end());
    }
    {
        let mut ar: [(i32, f64); 8] = [
            (1, 1.5_f64),
            (2, 2.5_f64),
            (3, 3.5_f64),
            (4, 4.5_f64),
            (5, 5.5_f64),
            (6, 6.5_f64),
            (7, 7.5_f64),
            (8, 8.5_f64),
        ];
        let mut m: UnsafeMap<i32, (i32, f64)> = unimplemented!();
        assert!(((m.len()) == (8_usize)));
        let mut i: UnsafeBidirectionalIterator<(i32, f64)> =
            m.erase(&libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!(((m.len()) == (7_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 3_isize));
        assert!((((*m.begin().iter_deref()).0) == (1)));
        assert!((((*m.begin().iter_deref()).1) == (1.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (3.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).0) == (6)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).1) == (6.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 5_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 5_isize).iter_deref()).1) == (7.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 6_isize).iter_deref()).0) == (8)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 6_isize).iter_deref()).1) == (8.5_f64)));
        i = m
            .erase(&libcc2rs::next_unsafe(unimplemented!(), 0_isize))
            .clone();
        assert!(((m.len()) == (6_usize)));
        assert!(i == m.begin());
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (3.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).0) == (6)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).1) == (6.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).1) == (7.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 5_isize).iter_deref()).0) == (8)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 5_isize).iter_deref()).1) == (8.5_f64)));
        i = m
            .erase(&libcc2rs::next_unsafe(unimplemented!(), 5_isize))
            .clone();
        assert!(((m.len()) == (5_usize)));
        assert!(i == m.end());
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (3.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).0) == (6)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).1) == (6.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 4_isize).iter_deref()).1) == (7.5_f64)));
        i = m
            .erase(&libcc2rs::next_unsafe(unimplemented!(), 1_isize))
            .clone();
        assert!(((m.len()) == (4_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 1_isize));
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (6)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (6.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 3_isize).iter_deref()).1) == (7.5_f64)));
        i = m
            .erase(&libcc2rs::next_unsafe(unimplemented!(), 2_isize))
            .clone();
        assert!(((m.len()) == (3_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 2_isize));
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (5.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).0) == (7)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 2_isize).iter_deref()).1) == (7.5_f64)));
        i = m
            .erase(&libcc2rs::next_unsafe(unimplemented!(), 2_isize))
            .clone();
        assert!(((m.len()) == (2_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 2_isize));
        assert!((((*m.begin().iter_deref()).0) == (2)));
        assert!((((*m.begin().iter_deref()).1) == (2.5_f64)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).0) == (5)));
        assert!((((*libcc2rs::next_unsafe(m.begin(), 1_isize).iter_deref()).1) == (5.5_f64)));
        i = m
            .erase(&libcc2rs::next_unsafe(unimplemented!(), 0_isize))
            .clone();
        assert!(((m.len()) == (1_usize)));
        assert!(i == libcc2rs::next_unsafe(m.begin(), 0_isize));
        assert!((((*m.begin().iter_deref()).0) == (5)));
        assert!((((*m.begin().iter_deref()).1) == (5.5_f64)));
        i = m.erase(&unimplemented!()).clone();
        assert!(((m.len()) == (0_usize)));
        assert!(i == m.begin());
        assert!(i == m.end());
    }
    {
        let mut c: UnsafeMap<TemplateConstructor, (TemplateConstructor, i32)> = UnsafeMap::new();
        let mut a: TemplateConstructor = {
            let mut __tmp_0: i32 = 0;
            TemplateConstructor::new({ &mut __tmp_0 })
        };
        let mut it: UnsafeBidirectionalIterator<(TemplateConstructor, i32)> = c.find(&a);
        if it != c.end() {
            c.erase(&it.clone());
        }
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
    (unsafe { test_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
