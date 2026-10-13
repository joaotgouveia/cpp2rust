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
    let N: i32 = 10000;
    let mut arr: *mut u32 =
        Box::leak((0..(N as usize)).map(|_| 0_u32).collect::<Box<[u32]>>()).as_mut_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*arr.offset((i) as isize)) = 0_u32;
        i.postfix_inc();
    }
    (*arr.offset(((N) - (1)) as isize)) = 3148519816_u32;
    let mut words: *mut u16 = (arr as *mut u16);
    (if (((*words.offset((((N) * (2)) - (1)) as isize)) as i32) == (48042)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[N * 2 - 1] == 0xBBAA".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*words.offset((((N) * (2)) - (2)) as isize)) as i32) == (39304)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"words[N * 2 - 2] == 0x9988".as_ptr(),
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
