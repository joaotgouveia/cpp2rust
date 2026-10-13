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
    pub unsafe fn operator_add_1(&self, o: *const S) -> S {
        return S {
            v: ((self.v) + ((*o).v)),
        };
    }
    pub unsafe fn operator_sub_2(&self, o: *const S) -> S {
        return S {
            v: ((self.v) - ((*o).v)),
        };
    }
    pub unsafe fn operator_mul(&self, o: *const S) -> S {
        return S {
            v: ((self.v) * ((*o).v)),
        };
    }
    pub unsafe fn operator_div(&self, o: *const S) -> S {
        return S {
            v: ((self.v) / ((*o).v)),
        };
    }
    pub unsafe fn operator_rem(&self, o: *const S) -> S {
        return S {
            v: ((self.v) % ((*o).v)),
        };
    }
    pub unsafe fn operator_pos_6(&self) -> S {
        return S { v: self.v };
    }
    pub unsafe fn operator_neg_7(&self) -> S {
        return S { v: -self.v };
    }
    pub unsafe fn operator_inc_8(&mut self) -> *mut S {
        self.v.prefix_inc();
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_post_inc_9(&mut self, mut _a0: i32) -> S {
        let mut old: S = (*(self as *mut S));
        self.v.prefix_inc();
        return old;
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut S {
        self.v.prefix_dec();
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_post_dec_11(&mut self, mut _a0: i32) -> S {
        let mut old: S = (*(self as *mut S));
        self.v.prefix_dec();
        return old;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 7 };
    let mut b: S = S { v: 2 };
    (if (((unsafe { S::operator_add_1(&a, &b) }).v) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a + b).v == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_sub_2(&a, &b) }).v) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a - b).v == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_mul(&a, &b) }).v) == (14)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a * b).v == 14".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_div(&a, &b) }).v) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a / b).v == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_rem(&a, &b) }).v) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a % b).v == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_pos_6(&a) }).v) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(+a).v == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_neg_7(&a) }).v) == (-7_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(-a).v == -7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { S::operator_inc_8(&mut a) })).v) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(++a).v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_post_inc_9(&mut a, 0) }).v) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a++).v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((a.v) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { S::operator_dec_10(&mut a) })).v) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(--a).v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { S::operator_post_dec_11(&mut a, 0) }).v) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(a--).v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((a.v) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"a.v == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe { S::operator_inc_8(&mut (*(unsafe { S::operator_inc_8(&mut a) }))) })).v)
        == (9))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(++(++a)).v == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let mut _o: S = S { v: 4 };
        S::operator_add_1(&S { v: 3 }, &mut _o)
    })
    .v) == (7))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"(S{3} + S{4}).v == 7".as_ptr(),
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
