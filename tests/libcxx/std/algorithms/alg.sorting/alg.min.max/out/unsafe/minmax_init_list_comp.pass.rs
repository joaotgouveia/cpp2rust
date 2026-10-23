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
pub struct binary_counting_predicate_bool__ptr__int__int___int__int_ {
    p_: Option<unsafe fn((), ()) -> ()>,
    count_: (),
}
impl binary_counting_predicate_bool__ptr__int__int___int__int_ {
    pub unsafe fn new(mut p: Option<unsafe fn((), ()) -> ()>) -> Self {
        let mut this = Self {
            p_: p,
            count_: 0_(),
        };
        this
    }
    pub unsafe fn operator_call(&self, a1: *const (), a2: *const ()) -> () {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return (unsafe {
            let _arg0: () = (*a1);
            let _arg1: () = (*a2);
            (self.p_).unwrap()(_arg0, _arg1)
        });
    }
    pub unsafe fn count(&self) -> () {
        return self.count_;
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct binary_counting_predicate_std_greater_int___int__int_ {
    p_: (),
    count_: (),
}
impl binary_counting_predicate_std_greater_int___int__int_ {
    pub unsafe fn new(mut p: ()) -> Self {
        let mut this = Self {
            p_: p.clone(),
            count_: 0_(),
        };
        this
    }
    pub unsafe fn operator_call(&self, a1: *const (), a2: *const ()) -> () {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn count(&self) -> () {
        return self.count_;
    }
}
impl Default for binary_counting_predicate_std_greater_int___int__int_ {
    fn default() -> Self {
        unsafe {
            binary_counting_predicate_std_greater_int___int__int_ {
                p_: unimplemented!(),
                count_: unimplemented!(),
            }
        }
    }
}
pub unsafe fn all_equal_0(mut _a0: (), mut _a1: ()) -> () {
    return false;
}
pub unsafe fn test_all_equal_1(mut il: ()) {
    let mut pred: binary_counting_predicate_bool__ptr__int__int___int__int_ =
        binary_counting_predicate_bool__ptr__int__int___int__int_::new({ Some(all_equal_0) });
    let mut p: () = unimplemented!();
    let mut ptr: *const () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_int___int__int_ =
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) });
    unimplemented!();
    unimplemented!();
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    (unsafe { test_all_equal_1(unimplemented!()) });
    {
        const _: () = assert!(
            true,
            "(std::minmax({{1, 2, 3}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{1, 3, 2}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{2, 1, 3}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{2, 3, 1}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{3, 1, 2}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{3, 2, 1}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
