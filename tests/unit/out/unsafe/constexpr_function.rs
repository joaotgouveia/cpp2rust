extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn runtime_only_0(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn first_1(mut p: *const i32) -> i32 {
    return (*p);
}
pub unsafe fn scaled_2(mut x: i32) -> i32 {
    if ((x) < (0)) {
        return (unsafe { runtime_only_0(-x) });
    }
    return x;
}
pub unsafe fn half_3(mut x: f64) -> f64 {
    return ((x) / (2.0E+0));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Flag {
    pub v: i32,
}
impl Flag {
    pub unsafe fn to_bool(&self) -> bool {
        return ((self.v) != (0));
    }
}
pub unsafe fn use_4(mut f: Flag) -> i32 {
    (if (unsafe { Flag::to_bool(&f) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"f".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int use(Flag)".as_ptr(),
            )
        });
    });
    return f.v;
}
pub unsafe fn checked_6(mut x: i32) -> i32 {
    (if ((x) > (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"x > 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int checked(int)".as_ptr(),
            )
        });
    });
    return ((x) + (1));
}
pub unsafe fn in_constant_context_7() -> bool {
    {}
    return ((unsafe { runtime_only_0(1) }) == (0));
}
const _: () = assert!(true, "in_constant_context()");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct P {
    pub v: i32,
}
impl P {
    pub unsafe fn get(&self) -> i32 {
        return self.v;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut runtime_context: bool = (unsafe { in_constant_context_7() });
    (if !(runtime_context) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"!runtime_context".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut arr: [i32; 2] = [7, 8];
    (if ((unsafe { first_1((arr.as_mut_ptr()).cast_const()) }) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"first(arr) == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { first_1((arr.as_mut_ptr().offset((1) as isize)).cast_const()) }) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"first(arr + 1) == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { scaled_2(3) }) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"scaled(3) == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { scaled_2(-3_i32) }) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"scaled(-3) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { half_3(5.0E+0) }) == (2.5E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"half(5.0) == 2.5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { checked_6(1) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"checked(1) == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let c: i32 = (unsafe { checked_6(4) });
    (if ((c) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"c == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { use_4(Flag { v: 2 }) }) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"use(Flag{2}) == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let u: i32 = (unsafe { use_4(Flag { v: 3 }) });
    (if ((u) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"u == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ptr: *mut i32 = arr.as_mut_ptr();
    (if !(ptr).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"ptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: P = P { v: 9 };
    (if ((unsafe { P::get(&p) }) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"p.get() == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let k: i32 = (unsafe { scaled_2(4) });
    (if ((k) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"k == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_5(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
