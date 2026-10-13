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
    pub unsafe fn operator_eq(&self, o: *const S) -> bool {
        return ((self.v) == ((*o).v));
    }
    pub unsafe fn operator_ne(&self, o: *const S) -> bool {
        return ((self.v) != ((*o).v));
    }
    pub unsafe fn operator_lt_3(&self, o: *const S) -> bool {
        return ((self.v) < ((*o).v));
    }
    pub unsafe fn operator_gt(&self, o: *const S) -> bool {
        return ((self.v) > ((*o).v));
    }
    pub unsafe fn operator_le(&self, o: *const S) -> bool {
        return ((self.v) <= ((*o).v));
    }
    pub unsafe fn operator_ge(&self, o: *const S) -> bool {
        return ((self.v) >= ((*o).v));
    }
    pub unsafe fn operator_lt_7(&self, mut o: i32) -> bool {
        return ((self.v) < (o));
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if S::operator_lt_3(self, other as *const S) {
                std::cmp::Ordering::Less
            } else if S::operator_lt_3(other, self as *const S) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { S::operator_eq(self, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 1 };
    let mut b: S = S { v: 2 };
    let mut c: S = S { v: 1 };
    (if (unsafe { S::operator_eq(&a, &c) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a == c".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_ne(&a, &b) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a != b".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_lt_3(&a, &b) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a < b".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_gt(&b, &a) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"b > a".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_le(&a, &c) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a <= c".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_ge(&a, &c) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a >= c".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe { S::operator_lt_3(&b, &a) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"!(b < a)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_lt_7(&a, 5) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a < 5".as_ptr(),
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
