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
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn operator_add_1(&mut self, mut a: i32) -> i32 {
        return ((self.v) + (a));
    }
    pub unsafe fn operator_add_2(&self, mut a: i32) -> i32 {
        return (((self.v) + (a)) + (1));
    }
    pub unsafe fn operator_add_3(&mut self, mut a: i32) -> i32 {
        return (((self.v) + (a)) + (2));
    }
    pub unsafe fn operator_sub_4(&mut self, mut a: i32) -> i32 {
        return ((self.v) - (a));
    }
    pub unsafe fn operator_sub_5(&mut self, mut a: i32) -> i32 {
        return (((self.v) - (a)) - (1));
    }
    pub unsafe fn operator_mul_6(&self, mut a: i32) -> i32 {
        return ((self.v) * (a));
    }
    pub unsafe fn operator_mul_7(&self, mut a: i32) -> i32 {
        return (((self.v) * (a)) * (2));
    }
    pub unsafe fn operator_index_8(&mut self, mut i: i32) -> i32 {
        return ((self.v) + (i));
    }
    pub unsafe fn operator_index_9(&self, mut i: i32) -> i32 {
        return (((self.v) + (i)) + (100));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 10 };
    let cs: S = S { v: 10 };
    let mut vs: S = S { v: 10 };
    (if ((unsafe { S::operator_add_1(&mut s, 1) }) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s + 1 == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_add_2(&cs, 1) }) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"cs + 1 == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_add_3(&mut vs, 1) }) == (13)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"vs + 1 == 13".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_sub_4(&mut s, 1) }) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s - 1 == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_sub_5(&mut S { v: 10 }, 1) }) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"S{10} - 1 == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_mul_6(&s, 3) }) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s * 3 == 30".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_mul_6(&cs, 3) }) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"cs * 3 == 30".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_mul_7(&S { v: 10 }, 3) }) == (60)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"S{10} * 3 == 60".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_index_8(&mut s, 2) }) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"s[2] == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::operator_index_9(&cs, 2) }) == (112)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"cs[2] == 112".as_ptr(),
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
