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
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (i32, i32),
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*mut ::libc::c_void , new_1, value)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_2(mut p: *mut i32) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_1(mut _a0: *mut ::libc::c_void) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn pointer_to(
        t: *mut i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_2({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__int__ {}
pub unsafe fn node_factory_4(
    c: *mut UnsafeMap<i32, (i32, i32)>,
    key: *const i32,
    mapped: *const i32,
) -> () {
    let mut p: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = unimplemented!();
    assert!((p.1 as bool));
    return unimplemented!();
}
pub unsafe fn node_factory_5(
    c: *mut UnsafeMap<i32, (i32, i32)>,
    key: *const i32,
    mapped: *const i32,
) -> () {
    let mut p: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = unimplemented!();
    assert!((p.1 as bool));
    return unimplemented!();
}
pub unsafe fn test_6(c: *mut UnsafeMap<i32, (i32, i32)>) {
    let mut nf: Option<unsafe fn(*mut UnsafeMap<i32, (i32, i32)>, *const i32, *const i32) -> ()> =
        (Some(node_factory_4));
    let mut c2: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        let mut node: () = (unsafe {
            let _arg1: *const i32 = &i;
            let mut _arg2: i32 = ((i) + (1));
            (nf).unwrap()(&mut c2, _arg1, &mut _arg2)
        });
        assert!(!(unimplemented!()));
        let mut prev: usize = (*c).len();
        let mut it: UnsafeBidirectionalIterator<(i32, i32)> = unimplemented!();
        assert!(unimplemented!());
        assert!((((prev).wrapping_add(1_usize)) == ((*c).len())));
        assert!((((*it.iter_deref()).0) == (i)));
        assert!((((*it.iter_deref()).1) == ((i) + (1))));
        i.prefix_inc();
    }
    assert!((((*c).len()) == (10_usize)));
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        assert!(((unimplemented!()) == (1_usize)));
        assert!(((*(*c).subscript(&i)) == ((i) + (1))));
        i.prefix_inc();
    }
}
pub unsafe fn test_7(c: *mut UnsafeMap<i32, (i32, i32)>) {
    let mut nf: Option<unsafe fn(*mut UnsafeMap<i32, (i32, i32)>, *const i32, *const i32) -> ()> =
        (Some(node_factory_5));
    let mut c2: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        let mut node: () = (unsafe {
            let _arg1: *const i32 = &i;
            let mut _arg2: i32 = ((i) + (1));
            (nf).unwrap()(&mut c2, _arg1, &mut _arg2)
        });
        assert!(!(unimplemented!()));
        let mut prev: usize = (*c).len();
        let mut it: UnsafeBidirectionalIterator<(i32, i32)> = unimplemented!();
        assert!(unimplemented!());
        assert!((((prev).wrapping_add(1_usize)) == ((*c).len())));
        assert!((((*it.iter_deref()).0) == (i)));
        assert!((((*it.iter_deref()).1) == ((i) + (1))));
        i.prefix_inc();
    }
    assert!((((*c).len()) == (10_usize)));
    let mut i: i32 = 0;
    'loop_: while ((i) != (10)) {
        assert!(((unimplemented!()) == (1_usize)));
        assert!(((*(*c).subscript(&i)) == ((i) + (1))));
        i.prefix_inc();
    }
}
pub unsafe fn test_8() -> bool {
    let mut m: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (unsafe { test_6(&mut m) });
    let mut m2: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (unsafe { test_7(&mut m2) });
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
    (unsafe { test_8() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
