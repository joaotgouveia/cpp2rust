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
    pub unsafe fn operator_not(&self) -> bool {
        return ((self.v) == (0));
    }
    pub unsafe fn operator_and(&self, o: *const S) -> bool {
        return ((self.v) != (0)) && (((*o).v) != (0));
    }
    pub unsafe fn operator_or(&self, o: *const S) -> bool {
        return ((self.v) != (0)) || (((*o).v) != (0));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: S = S { v: 1 };
    let mut f: S = S { v: 0 };
    (if (unsafe { S::operator_not(&f) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"!f".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe { S::operator_not(&t) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"!!t".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe {
        let _o: *const S = &t;
        S::operator_and(&t, _o)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"t && t".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe { S::operator_and(&t, &f) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"!(t && f)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { S::operator_or(&t, &f) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"t || f".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe {
        let _o: *const S = &f;
        S::operator_or(&f, _o)
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"!(f || f)".as_ptr(),
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
