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
pub struct A {
    pub v: i32,
}
impl A {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { v: 1 };
        this
    }
    pub unsafe fn new_2(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct B {
    pub v: i32,
}
impl B {
    pub unsafe fn new() -> Self {
        let mut this = Self { v: 2 };
        this
    }
}
impl Default for B {
    fn default() -> Self {
        unsafe { B::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoDefault {
    pub v: i32,
}
impl NoDefault {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
}
pub unsafe fn used_0(mut x: Option<A>) -> i32 {
    let mut x: A = x.unwrap_or_else(|| unsafe { A::new_1() });
    return x.v;
}
pub unsafe fn used_1(mut x: Option<B>) -> i32 {
    let mut x: B = x.unwrap_or_else(|| unsafe { B::new() });
    return x.v;
}
pub unsafe fn scaled_2(mut x: A, mut n: Option<i32>) -> i32 {
    let mut n: i32 = n.unwrap_or_else(|| unsafe { (::std::mem::size_of::<A>() as i32) });
    return ((x.v) * (n));
}
pub unsafe fn always_given_3(mut x: NoDefault) -> i32 {
    return x.v;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_4 {
    pub v: i32,
}
impl S_4 {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn get(&self, mut t: NoDefault) -> i32 {
        return ((self.v) + (t.v));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { used_0(None) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"used<A>() == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { used_0(Some(A::new_2({ 5 }))) }) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"used(A(5)) == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { used_1(None) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"used<B>() == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { scaled_2(A::new_2({ 3 }), None) })
        == ((3) * (::std::mem::size_of::<A>() as i32)))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"scaled(A(3)) == 3 * (int)sizeof(A)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { scaled_2(A::new_2({ 3 }), Some(2)) }) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"scaled(A(3), 2) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { always_given_3(NoDefault::new({ 3 })) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"always_given(NoDefault(3)) == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut s: S_4 = S_4::new({ 1 });
    (if ((unsafe { S_4::get(&s, NoDefault::new({ 4 })) }) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"s.get(NoDefault(4)) == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_5(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
