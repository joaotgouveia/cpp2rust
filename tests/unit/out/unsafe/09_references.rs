extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut counter_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn inc_1() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) += 1;
}
pub unsafe fn dec_2() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) -= 1;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: i32 = 15;
    let h_ref1: *mut i32 = &mut h;
    (*h_ref1) = 16;
    let mut h_ptr: *mut i32 = (h_ref1);
    let h_ref2: *mut i32 = &mut (*h_ptr);
    (*h_ref2) = 17;
    (if (((*h_ref1) + (*h_ref2)) == (34)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"h_ref1 + h_ref2 == 34".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let r: *mut i32 = if ((a) < (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    };
    (*r) = 10;
    (if ((a) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"a == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let cr: *const i32 = if ((a) > (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    };
    (if ((*cr) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"cr == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let x: i32 = 1;
    let y: i32 = 2;
    let cx: *const i32 = if ((x) < (y)) {
        (&x as *const i32)
    } else {
        (&y as *const i32)
    };
    (if ((*cx) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"cx == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut cp: *const i32 = (if ((a) > (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    })
    .cast_const();
    (if ((*cp) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"*cp == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut mp: *mut i32 = if ((a) < (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    };
    (*mp) = 20;
    (if ((b) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"b == 20".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*if ((a) < (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    }) = 30;
    (if ((a) == (30)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"a == 30".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((b) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"b == 20".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*if ((a) < (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    }) += 5;
    (if ((b) == (25)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"b == 25".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ap: *mut i32 = (if ((a) > (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    });
    (*ap) = 40;
    (if ((a) == (40)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"a == 40".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    if ((a) < (b)) {
        (unsafe { inc_1() });
    } else {
        (unsafe { dec_2() });
    };
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) == (-1_i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"counter == -1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    if ((a) > (b)) {
        (unsafe { inc_1() });
    } else {
        (unsafe { dec_2() });
    };
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"counter == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_3(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const counter_0);
}
