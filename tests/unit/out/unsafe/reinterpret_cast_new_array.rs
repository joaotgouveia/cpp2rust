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
    let mut arr: *mut u32 =
        Box::leak((0..2_usize).map(|_| 0_u32).collect::<Box<[u32]>>()).as_mut_ptr();
    (*arr.offset((0) as isize)) = 67305985_u32;
    (*arr.offset((1) as isize)) = 134678021_u32;
    let mut bytes: *mut u8 = (arr as *mut u8);
    (if (((*bytes.offset((0) as isize)) as i32) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bytes[0] == 0x01".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*bytes.offset((4) as isize)) as i32) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bytes[4] == 0x05".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*bytes.offset((7) as isize)) as i32) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"bytes[7] == 0x08".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*bytes.offset((0) as isize)) = 170_u8;
    (if ((*arr.offset((0) as isize)) == (67306154_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[0] == 0x040302AA".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*bytes.offset((5) as isize)) = 187_u8;
    (if ((*arr.offset((1) as isize)) == (134724357_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"arr[1] == 0x0807BB05".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    {
        let __p = arr;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<u32>(),
            )))
        }
    };
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
