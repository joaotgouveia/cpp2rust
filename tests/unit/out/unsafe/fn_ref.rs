extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn square_0(mut x: i32) -> i32 {
    return ((x) * (x));
}
pub unsafe fn twice_1(mut x: i32) -> i32 {
    return ((2) * (x));
}
pub unsafe fn call_ref_2(f: Option<unsafe fn(i32) -> i32>, mut x: i32) -> i32 {
    return (unsafe { (f).unwrap()(x) });
}
pub unsafe fn call_deduced_3(f: Option<unsafe fn(i32) -> i32>, mut x: i32) -> i32 {
    return (unsafe { (f).unwrap()(x) });
}
pub unsafe fn call_forwarded_4(f: Option<unsafe fn(i32) -> i32>, mut x: i32) -> i32 {
    return (unsafe { (f).unwrap()(x) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub f: Option<unsafe fn(i32) -> i32>,
}
impl Holder {
    pub unsafe fn run(&self, mut x: i32) -> i32 {
        return (unsafe { (self.f).unwrap()(x) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((unsafe { call_ref_2(Some(square_0), 3) }) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"call_ref(square, 3) == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { call_ref_2(Some(twice_1), 3) }) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"call_ref(twice, 3) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let r: Option<unsafe fn(i32) -> i32> = Some(square_0);
    (if ((unsafe { (r).unwrap()(4) }) == (16)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"r(4) == 16".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { call_ref_2(r, 5) }) == (25)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"call_ref(r, 5) == 25".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { call_deduced_3(Some(twice_1), 7) }) == (14)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"call_deduced(twice, 7) == 14".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { call_forwarded_4(Some(square_0), 6) }) == (36)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"call_forwarded(square, 6) == 36".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut h: Holder = Holder { f: Some(twice_1) };
    (if ((unsafe { Holder::run(&h, 8) }) == (16)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_5(
                c"h.run(8) == 16".as_ptr(),
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
