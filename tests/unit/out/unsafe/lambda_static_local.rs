extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut a_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut next: lambda_1 = (lambda_1 {});
    (if ((unsafe { lambda_1::operator_call() }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"next() == 1".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                10_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { lambda_1::operator_call() }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"next() == 2".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                11_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut copy: lambda_1 = next;
    (if ((unsafe { lambda_1::operator_call() }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"copy() == 3".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                14_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { lambda_1::operator_call() }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"next() == 4".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                15_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut per_type: lambda_4 = (lambda_4 {});
    (if ((unsafe { lambda_4::operator_call_5(1) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"per_type(1) == 1".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                22_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { lambda_4::operator_call_5(2) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"per_type(2) == 2".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                23_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { lambda_4::operator_call_7(1.0E+0) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"per_type(1.0) == 1".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                24_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { lambda_4::operator_call_5(3) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"per_type(3) == 3".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                25_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut read: lambda_9 = (lambda_9 {});
    (if ((unsafe { lambda_9::operator_call(0) }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"read(0) == 0".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                28_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)) = 1;
    (if ((unsafe { lambda_9::operator_call(0) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"read(0) == 1".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                30_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut local: lambda_10 = (lambda_10 {});
    (if ((unsafe { lambda_10::operator_call() }) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"local() == 6".as_ptr(),
                c"lambda_static_local.cpp".as_ptr(),
                40_u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct lambda_1 {}
impl lambda_1 {
    pub unsafe fn operator_call() -> i32 {
        static mut n_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });;
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut n_2)).prefix_inc();
    }
}
impl Callable0<i32> for lambda_1 {
    fn call(&self) -> i32 {
        unsafe { lambda_1::operator_call() }
    }
}
impl lambda_1 {
    pub fn to_free_function(&self) -> Option<unsafe fn() -> i32> {
        Some(lambda_1::operator_call)
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct lambda_4 {}
impl lambda_4 {
    pub unsafe fn operator_call_5(mut x: i32) -> i32 {
        static mut calls_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_6)).postfix_inc();
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_6));
    }
    pub unsafe fn operator_call_7(mut x: f64) -> i32 {
        static mut calls_8: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_8)).postfix_inc();
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_8));
    }
}
impl Callable1<i32, i32> for lambda_4 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_4::operator_call_5(a1) }
    }
}
impl Callable1<f64, i32> for lambda_4 {
    fn call(&self, a1: f64) -> i32 {
        unsafe { lambda_4::operator_call_7(a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct lambda_9 {}
impl lambda_9 {
    pub unsafe fn operator_call(mut _a0: i32) -> i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0));
    }
}
impl Callable1<i32, i32> for lambda_9 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_9::operator_call(a1) }
    }
}
impl lambda_9 {
    pub fn to_free_function(&self) -> Option<unsafe fn(i32) -> i32> {
        Some(lambda_9::operator_call)
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct P_11 {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct lambda_10 {}
impl lambda_10 {
    pub unsafe fn operator_call() -> i32 {
        let mut p: P_11 = P_11 { x: 2, y: 3 };
        return ((p.x) * (p.y));
    }
}
impl Callable0<i32> for lambda_10 {
    fn call(&self) -> i32 {
        unsafe { lambda_10::operator_call() }
    }
}
impl lambda_10 {
    pub fn to_free_function(&self) -> Option<unsafe fn() -> i32> {
        Some(lambda_10::operator_call)
    }
}
pub unsafe fn __assert_fail_3(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const a_0);
}
