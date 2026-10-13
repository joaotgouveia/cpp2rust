extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, Default)]
pub struct Counted {
    pub x: i32,
    pub y: i32,
}
impl Counted {
    pub unsafe fn new(mut x: i32, mut y: i32) -> Self {
        let mut this = Self { x: x, y: y };
        this
    }
    pub unsafe fn copy_from(other: *const Counted) -> Self {
        let mut this = Self {
            x: (((*other).x) * (2)),
            y: (((*other).y) * (2)),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
pub unsafe fn make_counted_1() -> Counted {
    return Counted::new({ 5 }, { 6 });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: Counted = Counted::new({ 1 }, { 2 });
    let mut __decomp_2: Counted = Counted::copy_from({ &s });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"copies == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((__decomp_2.x) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"a == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((__decomp_2.y) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"b == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let __decomp_4: *mut Counted = &mut s;
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"copies == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*__decomp_4).x) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"ra == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*__decomp_4).y) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"rb == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let __decomp_5: *const Counted = &s;
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"copies == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*__decomp_5).x) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"ca == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut __decomp_6: Counted = (unsafe { make_counted_1() });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"copies == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((__decomp_6.x) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"pa == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((__decomp_6.y) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_3(
                c"pb == 6".as_ptr(),
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
    std::cell::LazyCell::force(&*&raw const copies_0);
}
