extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct Emplaceable {
    int_: i32,
    double_: f64,
}
impl Emplaceable {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            int_: 0,
            double_: 0_f64,
        };
        this
    }
    pub unsafe fn new_2(mut i: i32, mut d: f64) -> Self {
        let mut this = Self {
            int_: i,
            double_: d,
        };
        this
    }
    pub unsafe fn move_from(x: *mut Emplaceable) -> Self {
        let mut this = Self {
            int_: (*x).int_,
            double_: (*x).double_,
        };
        (*x).int_ = 0;
        (*x).double_ = 0_f64;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut Emplaceable) -> *mut Emplaceable {
        self.int_ = (*x).int_;
        (*x).int_ = 0;
        self.double_ = (*x).double_;
        (*x).double_ = 0_f64;
        return &mut (*(self as *mut Emplaceable));
    }
    pub unsafe fn operator_eq(&self, x: *const Emplaceable) -> bool {
        return ((self.int_) == ((*x).int_)) && ((self.double_) == ((*x).double_));
    }
    pub unsafe fn operator_lt(&self, x: *const Emplaceable) -> bool {
        return ((self.int_) < ((*x).int_))
            || (((self.int_) == ((*x).int_)) && ((self.double_) < ((*x).double_)));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.int_;
    }
}
impl std::cmp::Ord for Emplaceable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Emplaceable::operator_lt(self, other as *const Emplaceable) {
                std::cmp::Ordering::Less
            } else if Emplaceable::operator_lt(other, self as *const Emplaceable) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Emplaceable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Emplaceable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Emplaceable::operator_eq(self, other as *const Emplaceable) }
    }
}
impl std::cmp::Eq for Emplaceable {}
impl Default for Emplaceable {
    fn default() -> Self {
        unsafe { Emplaceable::new_1() }
    }
}
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
pub struct min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (i32, Emplaceable),
}
impl min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__Emplaceable__ {}
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
        let mut c: () = unimplemented!();
        let mut e: () = <()>::from(unimplemented!());
        let mut r: () = unimplemented!();
        assert!(((unimplemented!()) == (1_())));
        assert!((((*r.iter_deref()).0) == (3)));
        assert!(
            (unsafe {
                let mut _x: Emplaceable = Emplaceable::new_1();
                Emplaceable::operator_eq(&(*r.iter_deref()).1, &mut _x)
            })
        );
        r = unimplemented!();
        assert!(((unimplemented!()) == (2_())));
        assert!((((*r.iter_deref()).0) == (3)));
        assert!(
            (unsafe {
                let mut _x: Emplaceable = Emplaceable::new_2({ 5 }, { 6_f64 });
                Emplaceable::operator_eq(&(*r.iter_deref()).1, &mut _x)
            })
        );
        const _: () = assert!(true, "");;
        r = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!((((*r.iter_deref()).0) == (3)));
        assert!(
            (unsafe {
                let mut _x: Emplaceable = Emplaceable::new_2({ 6 }, { 7_f64 });
                Emplaceable::operator_eq(&(*r.iter_deref()).1, &mut _x)
            })
        );
        const _: () = assert!(true, "");;
        r = unimplemented!();
        assert!((((*r.iter_deref()).0) == (3)));
        const _: () = assert!(true, "");;
        r = libcc2rs::next_unsafe(r.clone(), 2_isize);
        assert!((((*r.iter_deref()).0) == (3)));
        const _: () = assert!(true, "");;
    }
    {
        let mut c: () = unimplemented!();
        let mut e: () = <()>::from(unimplemented!());
        let mut r: () = unimplemented!();
        assert!(((unimplemented!()) == (1_())));
        assert!((((*r.iter_deref()).0) == (3)));
        assert!(
            (unsafe {
                let mut _x: Emplaceable = Emplaceable::new_1();
                Emplaceable::operator_eq(&(*r.iter_deref()).1, &mut _x)
            })
        );
        r = unimplemented!();
        assert!(((unimplemented!()) == (2_())));
        assert!((((*r.iter_deref()).0) == (3)));
        assert!(
            (unsafe {
                let mut _x: Emplaceable = Emplaceable::new_2({ 5 }, { 6_f64 });
                Emplaceable::operator_eq(&(*r.iter_deref()).1, &mut _x)
            })
        );
        const _: () = assert!(true, "");;
        r = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!((((*r.iter_deref()).0) == (3)));
        assert!(
            (unsafe {
                let mut _x: Emplaceable = Emplaceable::new_2({ 6 }, { 7_f64 });
                Emplaceable::operator_eq(&(*r.iter_deref()).1, &mut _x)
            })
        );
        const _: () = assert!(true, "");;
        r = unimplemented!();
        assert!((((*r.iter_deref()).0) == (3)));
        const _: () = assert!(true, "");;
        r = libcc2rs::next_unsafe(r.clone(), 2_isize);
        assert!((((*r.iter_deref()).0) == (3)));
        const _: () = assert!(true, "");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
