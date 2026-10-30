extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Stats {
    pub compared: (),
    pub copied: (),
    pub moved: (),
}
impl Default for Stats {
    fn default() -> Self {
        unsafe {
            Stats {
                compared: 0_(),
                copied: 0_(),
                moved: 0_(),
            }
        }
    }
}
pub static mut stats_0: std::cell::LazyCell<Stats> =
    std::cell::LazyCell::new(|| unsafe { <Stats>::default() });
pub unsafe fn operator_lt_1(a: *const MyInt, b: *const MyInt) -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut stats_0))
        .compared
        .prefix_inc();
    return (((*a).value) < ((*b).value));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct MyInt {
    pub value: (),
}
impl MyInt {
    pub unsafe fn new(mut xval: ()) -> Self {
        let mut this = Self { value: xval };
        this
    }
    pub unsafe fn copy_from(other: *const MyInt) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut stats_0))
            .copied
            .prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut MyInt) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut stats_0))
            .moved
            .prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, other: *const MyInt) -> *mut MyInt {
        self.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut stats_0))
            .copied
            .prefix_inc();
        return &mut (*(self as *mut MyInt));
    }
    pub unsafe fn move_assign(&mut self, other: *mut MyInt) -> *mut MyInt {
        self.value = (*other).value;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut stats_0))
            .moved
            .prefix_inc();
        return &mut (*(self as *mut MyInt));
    }
}
impl std::cmp::Ord for MyInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const MyInt, other as *const MyInt) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const MyInt, self as *const MyInt) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for MyInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MyInt {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_1(self as *const MyInt, other as *const MyInt))
                && !(operator_lt_1(other as *const MyInt, self as *const MyInt))
        }
    }
}
impl std::cmp::Eq for MyInt {}
impl Clone for MyInt {
    fn clone(&self) -> Self {
        unsafe { MyInt::copy_from(self as *const MyInt) }
    }
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    let N: () = ((1_()) << (20_()));
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut g: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut logn: () = 10_();
    'loop_: while ((logn) <= (20_())) {
        let n: () = ((1_()) << (logn));
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        let debug_elements: () = (*unimplemented!());
        let debug_comparisons: () = (((2_()) * ((debug_elements) + (1_()))) * (debug_elements));
        &(debug_comparisons);
        unimplemented!();
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut stats_0)) = <Stats>::default();
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        logn.prefix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const stats_0);
}
