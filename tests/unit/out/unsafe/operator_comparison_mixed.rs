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
    pub unsafe fn operator_eq(&self, mut o: i32) -> bool {
        return ((self.v) == (o));
    }
    pub unsafe fn operator_ne(&self, mut o: i32) -> bool {
        return ((self.v) != (o));
    }
    pub unsafe fn operator_lt(&self, mut o: i32) -> bool {
        return ((self.v) < (o));
    }
    pub unsafe fn operator_gt(&self, mut o: f64) -> bool {
        return ((self.v as f64) > (o));
    }
    pub unsafe fn operator_le(&self, mut o: i64) -> bool {
        return ((self.v as i64) <= (o));
    }
    pub unsafe fn operator_ge(&self, mut o: *const libc::c_char) -> bool {
        return ((self.v) >= (((*o) as i32) - (('0' as libc::c_char) as i32)));
    }
}
pub unsafe fn operator_eq_0(mut a: i32, b: *const S) -> bool {
    return ((a) == ((*b).v));
}
pub unsafe fn operator_ne_1(mut a: i32, b: *const S) -> bool {
    return ((a) != ((*b).v));
}
pub unsafe fn operator_lt_2(mut a: i32, b: *const S) -> bool {
    return ((a) < ((*b).v));
}
pub unsafe fn operator_gt_3(mut a: f64, b: *const S) -> bool {
    return ((a) > ((*b).v as f64));
}
pub unsafe fn operator_le_4(mut a: i64, b: *const S) -> bool {
    return ((a) <= ((*b).v as i64));
}
pub unsafe fn operator_ge_5(mut a: *const libc::c_char, b: *const S) -> bool {
    return ((((*a) as i32) - (('0' as libc::c_char) as i32)) >= ((*b).v));
}
pub unsafe fn operator_lt_6(a: *mut S, mut b: i32) -> bool {
    return ((((*a).v) + (1)) < (b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 5 };
    let cs: *const S = &s;
    (if (unsafe { S::operator_eq(&(*cs), 5) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"cs == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_ne(&(*cs), 4) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"cs != 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_lt(&(*cs), 6) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"cs < 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_gt(&(*cs), 4.5E+0) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"cs > 4.5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_le(&(*cs), 5_i64) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"cs <= 5L".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_ge(&(*cs), c"3".as_ptr()) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"cs >= \"3\"".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { operator_eq_0(5, &s) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"5 == s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { operator_ne_1(4, &s) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"4 != s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { operator_lt_2(4, &s) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"4 < s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { operator_gt_3(5.5E+0, &s) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"5.5 > s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { operator_le_4(5_i64, &s) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"5L <= s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { operator_ge_5(c"7".as_ptr(), &s) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"\"7\" >= s".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let _a: *mut S = &mut s;
        operator_lt_6(_a, 7)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"s < 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe {
        let _a: *mut S = &mut s;
        operator_lt_6(_a, 6)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_7(
                c"!(s < 6)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_7(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
