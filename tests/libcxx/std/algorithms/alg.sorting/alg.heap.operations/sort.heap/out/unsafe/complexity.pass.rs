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
    pub compared: i32,
    pub copied: i32,
    pub moved: i32,
}
impl Default for Stats {
    fn default() -> Self {
        unsafe {
            Stats {
                compared: 0,
                copied: 0,
                moved: 0,
            }
        }
    }
}
pub static mut stats_0: std::cell::LazyCell<Stats> =
    std::cell::LazyCell::new(|| unsafe { <Stats>::default() });
pub unsafe fn operator_lt_1(a: *const MyInt, b: *const MyInt) -> bool {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut stats_0))
        .compared
        .prefix_inc();
    return (((*a).value) < ((*b).value));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct MyInt {
    pub value: i32,
}
impl MyInt {
    pub unsafe fn new(mut xval: i32) -> Self {
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    let N: i32 = ((1) << (20));
    let mut v: Vec<MyInt> = Vec::new();
    if (N as usize) as usize > v.capacity() as usize {
        let len_0 = v.len();
        v.reserve_exact((N as usize) as usize - len_0 as usize);
    };
    let mut g: () = unimplemented!();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        {
            let __init = MyInt::new({ i });
            v.push(__init)
        };
        i.prefix_inc();
    }
    let mut logn: i32 = 10;
    'loop_: while ((logn) <= (20)) {
        let n: i32 = ((1) << (logn));
        let mut first: UnsafeRandomAccessIterator<MyInt> = v.begin();
        let mut last: UnsafeRandomAccessIterator<MyInt> = v.begin().iter_add((n as isize));
        let debug_elements: i32 = {
            let mut __tmp_0: i32 = 100;
            (*{
                let __a: *const _ = &mut __tmp_0;
                let __b: *const _ = &n;
                if *__b < *__a { __b } else { __a }
            })
        };
        let debug_comparisons: i32 = (((2) * ((debug_elements) + (1))) * (debug_elements));
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const stats_0);
}
