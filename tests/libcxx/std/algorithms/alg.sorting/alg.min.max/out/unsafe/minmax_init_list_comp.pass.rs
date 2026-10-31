extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Option<unsafe fn(i32 ,i32 ,) -> bool > , new, value)]
pub struct binary_counting_predicate_bool__ptr__int__int___int__int_ {
    p_: Option<unsafe fn(i32, i32) -> bool>,
    count_: usize,
}
impl binary_counting_predicate_bool__ptr__int__int___int__int_ {
    pub unsafe fn new(mut p: Option<unsafe fn(i32, i32) -> bool>) -> Self {
        let mut this = Self {
            p_: p,
            count_: 0_usize,
        };
        this
    }
    pub unsafe fn operator_call(&self, a1: *const i32, a2: *const i32) -> bool {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return (unsafe {
            let _arg0: i32 = (*a1);
            let _arg1: i32 = (*a2);
            (self.p_).unwrap()(_arg0, _arg1)
        });
    }
    pub unsafe fn count(&self) -> usize {
        return self.count_;
    }
}
impl Callable2<*const i32, *const i32, bool>
    for binary_counting_predicate_bool__ptr__int__int___int__int_
{
    fn call(&self, a1: *const i32, a2: *const i32) -> bool {
        unsafe {
            binary_counting_predicate_bool__ptr__int__int___int__int_::operator_call(self, a1, a2)
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(() , new, value)]
pub struct binary_counting_predicate_std_greater_int___int__int_ {
    p_: (),
    count_: usize,
}
impl binary_counting_predicate_std_greater_int___int__int_ {
    pub unsafe fn new(mut p: ()) -> Self {
        let mut this = Self {
            p_: p.clone(),
            count_: 0_usize,
        };
        this
    }
    pub unsafe fn operator_call(&self, a1: *const i32, a2: *const i32) -> bool {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn count(&self) -> usize {
        return self.count_;
    }
}
impl Callable2<*const i32, *const i32, bool>
    for binary_counting_predicate_std_greater_int___int__int_
{
    fn call(&self, a1: *const i32, a2: *const i32) -> bool {
        unsafe {
            binary_counting_predicate_std_greater_int___int__int_::operator_call(self, a1, a2)
        }
    }
}
impl Default for binary_counting_predicate_std_greater_int___int__int_ {
    fn default() -> Self {
        unsafe {
            binary_counting_predicate_std_greater_int___int__int_ {
                p_: unimplemented!(),
                count_: 0_usize,
            }
        }
    }
}
pub unsafe fn all_equal_0(mut _a0: i32, mut _a1: i32) -> bool {
    return false;
}
pub unsafe fn test_all_equal_1(mut il: Vec<i32>) {
    let mut pred: binary_counting_predicate_bool__ptr__int__int___int__int_ =
        binary_counting_predicate_bool__ptr__int__int___int__int_::new({ Some(all_equal_0) });
    let mut p: (i32, i32) = unimplemented!();
    let mut ptr: *const i32 = unimplemented!();
    assert!(((p.0) == (*unimplemented!())));
    assert!(((p.1) == (*ptr.prefix_dec())));
    assert!(
        ((unsafe { binary_counting_predicate_bool__ptr__int__int___int__int_::count(&pred,) })
            <= (((3_usize).wrapping_mul(il.len())).wrapping_div(2_usize)))
    );
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
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    let mut pred: binary_counting_predicate_std_greater_int___int__int_ =
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) });
    assert!((unimplemented!()));
    assert!(
        ((unsafe { binary_counting_predicate_std_greater_int___int__int_::count(&pred,) })
            <= (18_usize))
    );
    (unsafe { test_all_equal_1(Vec::from([0])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2, 3])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2, 3, 4])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2, 3, 4, 5])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2, 3, 4, 5, 6])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2, 3, 4, 5, 6, 7])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])) });
    (unsafe { test_all_equal_1(Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])) });
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
