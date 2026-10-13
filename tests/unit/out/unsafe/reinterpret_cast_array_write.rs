extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let mut words: *mut u16 = (arr.as_mut_ptr() as *mut u16);
    (if (((*words.offset((0) as isize)) as i32) == (513)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[0] == 0x0201".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*words.offset((1) as isize)) as i32) == (1027)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[1] == 0x0403".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*words.offset((0) as isize)) = 48042_u16;
    (if ((arr[(0) as usize] as i32) == (170)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[0] == 0xAA".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(1) as usize] as i32) == (187)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[1] == 0xBB".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(2) as usize] as i32) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[2] == 0x03".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(3) as usize] as i32) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[3] == 0x04".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    words = (arr.as_mut_ptr().offset((1) as isize) as *mut u16);
    (if (((*words.offset((0) as isize)) as i32) == (955)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[0] == 0x03BB".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*words.offset((0) as isize)) = 0_u16;
    (if ((arr[(0) as usize] as i32) == (170)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[0] == 0xAA".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(1) as usize] as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[1] == 0x00".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(2) as usize] as i32) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[2] == 0x00".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((arr[(3) as usize] as i32) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[3] == 0x04".as_ptr(),
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
