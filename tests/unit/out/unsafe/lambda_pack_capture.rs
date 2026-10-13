extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_0() -> i32 {
    return 0;
}
pub unsafe fn sum_1(mut t: i32, mut u_1: i32, mut u_2: i32) -> i32 {
    return ((t) + (unsafe { sum_2(u_1, u_2) }));
}
pub unsafe fn sum_2(mut t: i32, mut u: i32) -> i32 {
    return ((t) + (unsafe { sum_3(u) }));
}
pub unsafe fn sum_3(mut t: i32) -> i32 {
    return ((t) + (unsafe { sum_0() }));
}
pub unsafe fn by_value_4() -> i32 {
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return (unsafe { sum_0() });
        }
    });
    ();
    return (unsafe { f.call() });
}
pub unsafe fn by_value_6(mut t_0: i32, mut t_1: i32, mut t_2: i32) -> i32 {
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let t_0: i32 = t_0;
            let t_1: i32 = t_1;
            let t_2: i32 = t_2;
        },
        || -> i32 {
            return (unsafe { sum_1(t_0, t_1, t_2) });
        }
    );
    {
        t_0 = 0;
        {
            t_1 = 0;
            t_2 = 0
        }
    };
    return (unsafe { f.call() });
}
pub unsafe fn by_ref_8(mut t_0: i32, mut t_1: i32, mut t_2: i32) -> i32 {
    let mut f: FnPtr<fn()> = lambda_unsafe!(
        {
            let t_0: *mut i32 = &mut t_0;
            let t_1: *mut i32 = &mut t_1;
            let t_2: *mut i32 = &mut t_2;
        },
        || {
            {
                (*t_0) *= 2;
                {
                    (*t_1) *= 2;
                    (*t_2) *= 2
                }
            };
        }
    );
    (unsafe { f.call() });
    return (unsafe { sum_1(t_0, t_1, t_2) });
}
pub unsafe fn init_pack_10(mut t_0: i32, mut t_1: i32, mut t_2: i32) -> i32 {
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let xs_0: i32 = ((t_0) + (1));
            let xs_1: i32 = ((t_1) + (1));
            let xs_2: i32 = ((t_2) + (1));
        },
        || -> i32 {
            return (unsafe { sum_1(xs_0, xs_1, xs_2) });
        }
    );
    return (unsafe { f.call() });
}
pub unsafe fn implicit_12(mut t_0: i32, mut t_1: i32) -> i32 {
    return (unsafe {
        lambda_unsafe!(
            {
                let t_0: i32 = t_0;
                let t_1: i32 = t_1;
            },
            || -> i32 {
                return (unsafe { sum_2(t_0, t_1) });
            }
        )
        .call()
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { by_value_4() }) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"by_value() == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { by_value_6(1, 2, 3) }) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"by_value(1, 2, 3) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { by_ref_8(1, 2, 3) }) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"by_ref(1, 2, 3) == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { init_pack_10(1, 2, 3) }) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"init_pack(1, 2, 3) == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { implicit_12(4, 5) }) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_13(
                c"implicit(4, 5) == 9".as_ptr(),
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
