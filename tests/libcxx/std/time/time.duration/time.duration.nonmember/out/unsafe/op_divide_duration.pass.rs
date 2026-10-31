extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__1____std_chrono_duration_long__std_ratio_1__1___
{}
pub unsafe fn truncate_fp_1(mut val: f64) -> f64 {
    let mut sink: f64 = val;
    return sink;
}
pub unsafe fn truncate_fp_2(mut val: f64) -> f64 {
    let mut sink: f64 = val;
    return sink;
}
pub unsafe fn truncate_fp_3(mut val: f32) -> f32 {
    let mut sink: f32 = val;
    return sink;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Rep {
    data_: i32,
}
impl Rep {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: -1_i32 };
        this
    }
    pub unsafe fn new_2(mut i: i32) -> Self {
        let mut this = Self { data_: i };
        this
    }
    pub unsafe fn operator_eq_3(&self, mut i: i32) -> bool {
        return ((self.data_) == (i));
    }
    pub unsafe fn operator_eq_4(&self, r: *const Rep) -> bool {
        return ((self.data_) == ((*r).data_));
    }
    pub unsafe fn operator_mul_assign(&mut self, mut x: Rep) -> *mut Rep {
        self.data_ *= x.data_;
        return &mut (*(self as *mut Rep));
    }
    pub unsafe fn operator_div_assign(&mut self, mut x: Rep) -> *mut Rep {
        self.data_ /= x.data_;
        return &mut (*(self as *mut Rep));
    }
}
impl std::cmp::PartialEq for Rep {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Rep::operator_eq_4(self, other as *const Rep) }
    }
}
impl std::cmp::Eq for Rep {}
impl Default for Rep {
    fn default() -> Self {
        unsafe { Rep::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotARep {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i64, to_i64_2)]
pub struct RepConstConvertibleLWG3050 {}
impl RepConstConvertibleLWG3050 {
    pub unsafe fn to_i64_2(&self) -> i64 {
        return 2_i64;
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
    {
        let mut ns1: () = unimplemented!();
        let mut ns2: () = unimplemented!();
        assert!(((unimplemented!()) == (3_i64)));
    }
    {
        let mut us1: () = unimplemented!();
        let mut ns2: () = unimplemented!();
        assert!(((unimplemented!()) == (3000_i64)));
    }
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        assert!(((unimplemented!()) == (6)));
    }
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        assert!(((unimplemented!()) == (unsafe { truncate_fp_2(((20_f64) / (3_f64)),) })));
    }
    {
        let ns1: () = unimplemented!();
        let ns2: () = unimplemented!();
        const _: () = assert!(true, "ns1 / ns2 == 3");;
    }
    {
        let us1: () = unimplemented!();
        let ns2: () = unimplemented!();
        const _: () = assert!(true, "us1 / ns2 == 3000");;
    }
    {
        let s1: () = unimplemented!();
        let s2: () = unimplemented!();
        const _: () = assert!(true, "s1 / s2 == 6");;
    }
    {
        let s1: () = unimplemented!();
        let s2: () = unimplemented!();
        const _: () = assert!(true, "s1 / s2 == 20./3");;
    }
    {
        let mut d: () = unimplemented!();
        let mut x: RepConstConvertibleLWG3050 = <RepConstConvertibleLWG3050>::default();
        {
            let mut r: () = unimplemented!();
            assert!(((unimplemented!()) == (2_())));
            const _: () = assert!(true, "");;
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
