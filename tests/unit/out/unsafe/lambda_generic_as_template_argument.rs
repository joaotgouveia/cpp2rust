extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_int_0(mut fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    return (unsafe { fn_.spec::<fn(i32) -> i32>(0).call(x) });
}
pub unsafe fn apply_int_2(mut fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    return (unsafe { fn_.spec::<fn(i32) -> i32>(0).call(x) });
}
pub unsafe fn apply_int_4(mut fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    return (unsafe { fn_.spec::<fn(i32) -> i32>(0).call(x) });
}
pub unsafe fn apply_double_6(mut fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    return (unsafe { fn_.spec::<fn(f64) -> f64>(1).call(x) });
}
pub unsafe fn apply_double_7(mut fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    return (unsafe { fn_.spec::<fn(f64) -> f64>(1).call(x) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut factor: i32 = 3;
    let mut scale: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let factor: i32 = factor;
        },
        |mut x: i32| -> i32 {
            return ((x) * (factor));
        },
        |mut x: f64| -> f64 {
            return ((x) * (factor as f64));
        }
    );
    (if ((unsafe { apply_int_0(scale.copy_from(), 4) }) == (12)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"apply_int(scale, 4) == 12".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { apply_double_6(scale.copy_from(), 1.5E+0) }) == (4.5E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"apply_double(scale, 1.5) == 4.5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        apply_int_2(
            lambda_unsafe!(Generic, {}, |mut x: i32| -> i32 {
                return -x;
            }),
            9,
        )
    }) == (-9_i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"apply_int([](auto x) { return -x; }, 9) == -9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut offset: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let factor: i32 = factor;
        },
        |mut x: i32| -> i32 {
            return ((x) + (factor));
        },
        |mut x: f64| -> f64 {
            return ((x) + (factor as f64));
        }
    );
    (if ((unsafe { apply_int_4(offset.copy_from(), 4) }) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"apply_int(offset, 4) == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { apply_double_7(offset.copy_from(), 1.5E+0) }) == (4.5E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"apply_double(offset, 1.5) == 4.5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_8(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
