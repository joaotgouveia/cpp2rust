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
    pub unsafe fn operator_assign_1(&mut self, mut n: u32) -> *mut S {
        self.v = n;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_add_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_add((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_sub_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_sub((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_mul_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_mul((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_div_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_div((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_rem_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_rem((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_bitand_assign(&mut self, o: *const S) -> *mut S {
        self.v &= (*o).v;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_bitor_assign(&mut self, o: *const S) -> *mut S {
        self.v |= (*o).v;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_bitxor_assign(&mut self, o: *const S) -> *mut S {
        self.v ^= (*o).v;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_shl_assign(&mut self, mut n: i32) -> *mut S {
        self.v <<= n;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_shr_assign(&mut self, mut n: i32) -> *mut S {
        self.v >>= n;
        return &mut (*(self as *mut S));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 6_u32 };
    let mut b: S = S { v: 4_u32 };
    (unsafe { S::operator_add_assign(&mut a, &b) });
    (if ((a.v) == (10_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_sub_assign(&mut a, &b) });
    (if ((a.v) == (6_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_mul_assign(&mut a, &b) });
    (if ((a.v) == (24_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 24".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_div_assign(&mut a, &b) });
    (if ((a.v) == (6_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_rem_assign(&mut a, &b) });
    (if ((a.v) == (2_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_bitor_assign(&mut a, &b) });
    (if ((a.v) == (6_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_bitand_assign(&mut a, &b) });
    (if ((a.v) == (4_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_bitxor_assign(&mut a, &b) });
    (if ((a.v) == (0_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_assign_1(&mut a, 3_u32) });
    (if ((a.v) == (3_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_shl_assign(&mut a, 2) });
    (if ((a.v) == (12_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::operator_shr_assign(&mut a, 1) });
    (if ((a.v) == (6_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe {
        let _o: *const S = &b;
        S::operator_add_assign(&mut (*(unsafe { S::operator_add_assign(&mut a, &b) })), _o)
    });
    (if ((a.v) == (14_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 14".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut c: S = S { v: 0_u32 };
    c = (*(unsafe { S::operator_assign_1(&mut a, 1_u32) }));
    (if ((a.v) == (1_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((c.v) == (1_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"c.v == 1".as_ptr(),
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
