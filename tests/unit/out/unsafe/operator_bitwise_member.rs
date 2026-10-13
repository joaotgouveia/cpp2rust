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
    pub v: u32,
}
impl S {
    pub unsafe fn operator_bitnot(&self) -> S {
        return S { v: !self.v };
    }
    pub unsafe fn operator_bitand(&self, o: *const S) -> S {
        return S {
            v: ((self.v) & ((*o).v)),
        };
    }
    pub unsafe fn operator_bitor(&self, o: *const S) -> S {
        return S {
            v: ((self.v) | ((*o).v)),
        };
    }
    pub unsafe fn operator_bitxor(&self, o: *const S) -> S {
        return S {
            v: ((self.v) ^ ((*o).v)),
        };
    }
    pub unsafe fn operator_shl(&self, mut n: i32) -> S {
        return S {
            v: ((self.v) << (n)),
        };
    }
    pub unsafe fn operator_shr(&self, mut n: i32) -> S {
        return S {
            v: ((self.v) >> (n)),
        };
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 12_u32 };
    let mut b: S = S { v: 10_u32 };
    (if (((unsafe { S::operator_bitnot(&a) }).v) == (!12_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(~a).v == ~0b1100u".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_bitand(&a, &b) }).v) == (8_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a & b).v == 0b1000".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_bitor(&a, &b) }).v) == (14_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a | b).v == 0b1110".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_bitxor(&a, &b) }).v) == (6_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a ^ b).v == 0b0110".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_shl(&a, 2) }).v) == (48_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a << 2).v == 0b110000".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_shr(&a, 2) }).v) == (3_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a >> 2).v == 0b11".as_ptr(),
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
