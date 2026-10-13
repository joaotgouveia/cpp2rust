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
pub struct Static {}
impl Static {
    pub unsafe fn operator_call(mut a: i32, mut b: i32) -> i32 {
        return ((a) * (b));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn operator_call_1(&self) -> i32 {
        return self.v;
    }
    pub unsafe fn operator_call_2(&self, mut a: i32) -> i32 {
        return ((self.v) + (a));
    }
    pub unsafe fn operator_call_3(&self, mut a: i32, mut b: i32) -> i32 {
        return (((self.v) + (a)) + (b));
    }
    pub unsafe fn operator_comma(&self, o: *const S) -> S {
        return S {
            v: (((self.v) * (10)) + ((*o).v)),
        };
    }
    pub unsafe fn to_i32(&self) -> i32 {
        return self.v;
    }
    pub unsafe fn to_bool(&self) -> bool {
        return ((self.v) != (0));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 3 };
    let mut t: S = S { v: 4 };
    (if ((unsafe { S::operator_call_1(&s) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s() == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_call_2(&s, 1) }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s(1) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_call_3(&s, 1, 2) }) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s(1, 2) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_comma(&s, &t) }).v) == (34)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(s, t).v == 34".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: i32 = (unsafe { S::to_i32(&s) });
    (if ((i) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"i == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::to_i32(&s) }) + (1)) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s + 1 == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    if (unsafe { S::to_bool(&s) }) {
        (if (unsafe { S::to_bool(&s) }) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"static_cast<bool>(s)".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    } else {
        (if false {
            &(0);
        } else {
            (unsafe {
                __assert_fail_0(
                    c"false".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    (if (unsafe { S::to_bool(&s) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut z: S = S { v: 0 };
    (if (unsafe { S::to_bool(&s) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe { S::to_bool(&z) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"!z".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::to_bool(&s) }) && (!(unsafe { S::to_bool(&z) })) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s && !z".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut st: Static = <Static>::default();
    (if ((unsafe { Static::operator_call(6, 7) }) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"st(6, 7) == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_call_1(&S { v: 5 }) }) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"S{5}() == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_call_3(&S { v: 5 }, 1, 1) }) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"S{5}(1, 1) == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
