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
pub unsafe fn operator_add_0(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) + ((*b).v)),
    };
}
pub unsafe fn operator_sub_1(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) - ((*b).v)),
    };
}
pub unsafe fn operator_mul_2(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) * ((*b).v)),
    };
}
pub unsafe fn operator_div_3(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) / ((*b).v)),
    };
}
pub unsafe fn operator_rem_4(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) % ((*b).v)),
    };
}
pub unsafe fn operator_pos_5(a: *const S) -> S {
    return S { v: (*a).v };
}
pub unsafe fn operator_neg_6(a: *const S) -> S {
    return S { v: -(*a).v };
}
pub unsafe fn operator_inc_7(a: *mut S) -> *mut S {
    (*a).v.prefix_inc();
    return a;
}
pub unsafe fn operator_post_inc_8(a: *mut S, mut _a1: i32) -> S {
    let mut old: S = (*a);
    (*a).v.prefix_inc();
    return old;
}
pub unsafe fn operator_dec_9(a: *mut S) -> *mut S {
    (*a).v.prefix_dec();
    return a;
}
pub unsafe fn operator_post_dec_10(a: *mut S, mut _a1: i32) -> S {
    let mut old: S = (*a);
    (*a).v.prefix_dec();
    return old;
}
pub unsafe fn operator_add_11(a: *const S, mut b: i32) -> S {
    return S {
        v: (((*a).v) + (b)),
    };
}
pub unsafe fn operator_add_12(mut a: i32, b: *const S) -> S {
    return S {
        v: ((a) + ((*b).v)),
    };
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
    (if (((unsafe {
        let _a: *const S = &a;
        operator_add_0(_a, &b)
    })
    .v) == (9))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(a + b).v == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _a: *const S = &a;
        operator_sub_1(_a, &b)
    })
    .v) == (5))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(a - b).v == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _a: *const S = &a;
        operator_mul_2(_a, &b)
    })
    .v) == (14))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(a * b).v == 14".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _a: *const S = &a;
        operator_div_3(_a, &b)
    })
    .v) == (3))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(a / b).v == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _a: *const S = &a;
        operator_rem_4(_a, &b)
    })
    .v) == (1))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(a % b).v == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _a: *const S = &a;
        operator_pos_5(_a)
    })
    .v) == (7))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(+a).v == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _a: *const S = &a;
        operator_neg_6(_a)
    })
    .v) == (-7_i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(-a).v == -7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe {
        let _a: *mut S = &mut a;
        operator_inc_7(_a)
    }))
    .v) == (8))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(++a).v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _a: *mut S = &mut a;
        operator_post_inc_8(_a, 0)
    })
    .v) == (8))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
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
            __assert_fail_13(
                c"a.v == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*(unsafe {
        let _a: *mut S = &mut a;
        operator_dec_9(_a)
    }))
    .v) == (8))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(--a).v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _a: *mut S = &mut a;
        operator_post_dec_10(_a, 0)
    })
    .v) == (8))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
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
            __assert_fail_13(
                c"a.v == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe {
        let _a: *const S = &a;
        operator_add_11(_a, 1)
    })
    .v) == (8))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(a + 1).v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { operator_add_12(1, &a) }).v) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"(1 + a).v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_13(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
