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
pub struct Point {
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Box_0 {
    pub val: i32,
}
impl Box_0 {
    pub unsafe fn twice(&mut self) -> i32 {
        return ((self.val) + (self.val));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Box_1 {
    pub val: Point,
}
impl Box_1 {
    pub unsafe fn get(&mut self) -> Point {
        return self.val;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Deduced_2 {
    pub val: i32,
}
impl Deduced_2 {
    pub unsafe fn twice(&mut self) -> i32 {
        return ((self.val) + (self.val));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Deduced_3 {
    pub val: Point,
}
impl Deduced_3 {
    pub unsafe fn get(&mut self) -> Point {
        return self.val;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i: Box_0 = Box_0 { val: 3 };
    (if ((unsafe { Box_0::twice(&mut i) }) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"i.twice() == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: Box_1 = Box_1 {
        val: Point { x: 4 },
    };
    (if (((unsafe { Box_1::get(&mut p) }).x) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"p.get().x == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut d: Deduced_2 = Deduced_2 { val: 5 };
    (if ((unsafe { Deduced_2::twice(&mut d) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"d.twice() == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut dp: Deduced_3 = Deduced_3 {
        val: Point { x: 6 },
    };
    (if (((unsafe { Deduced_3::get(&mut dp) }).x) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"dp.get().x == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
