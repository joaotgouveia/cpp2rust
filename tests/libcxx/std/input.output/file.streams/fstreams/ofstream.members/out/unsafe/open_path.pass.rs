extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_temp_file_name_0() -> Vec<libc::c_char> {
    let mut Name: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"libcxx.XXXXXX".as_ptr(),
        libcc2rs::c_str_len(c"libcxx.XXXXXX".as_ptr()) + 1,
    )
    .to_vec();
    let mut FD: i32 = unimplemented!();
    if ((FD) == (-1_i32)) {
        unimplemented!();
        std::process::abort();
    }
    libc::close(FD);
    return std::mem::take(&mut Name);
}
pub unsafe fn glibc_version_less_than_1(mut version: *const libc::c_char) -> bool {
    let mut test_version: Vec<libc::c_char> = {
        let mut __tmp2 = std::slice::from_raw_parts(
            c"glibc ".as_ptr(),
            libcc2rs::c_str_len(c"glibc ".as_ptr()) + 1,
        )
        .to_vec()
        .clone();
        __tmp2.pop();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            version,
            libcc2rs::c_str_len(version),
        ));
        __tmp2.push((0 as libc::c_char));
        __tmp2
    };
    let mut n: usize = unimplemented!();
    let mut current_version: *mut libc::c_char = Box::leak(
        (0..n)
            .map(|_| (0 as libc::c_char))
            .collect::<Box<[libc::c_char]>>(),
    )
    .as_mut_ptr();
    unimplemented!();
    let mut result: bool = ((unimplemented!()) < (0));
    {
        let __p = current_version;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<libc::c_char>(),
            )))
        }
    };
    return result;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    let mut p: Vec<libc::c_char> = unimplemented!();
    {
        let mut fs: ::std::fs::File = unimplemented!();
        assert!(!(unimplemented!()));
        let mut c: libc::c_char = ('a' as libc::c_char);
        {
            let __r: &mut ::std::fs::File = &mut fs;
            let __s: *mut ::std::fs::File = __r;
            let _ = (*__s).write_all(&[c as u8]);
            __s
        };
        assert!(unimplemented!());
        unimplemented!();
        assert!(unimplemented!());
        {
            let __r: &mut ::std::fs::File = &mut fs;
            let __s: *mut ::std::fs::File = __r;
            let _ = (*__s).write_all(&[c as u8]);
            __s
        };
    }
    {
        let mut fs: ::std::fs::File = unimplemented!();
        let mut c: libc::c_char = (0 as libc::c_char);
        unimplemented!();
        assert!(((c as i32) == (('a' as libc::c_char) as i32)));
    }
    unimplemented!();
    {
        let mut fs: () = unimplemented!();
        assert!(!(unimplemented!()));
        let mut c: i32 = (97 as i32);
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
    }
    {
        let mut fs: () = unimplemented!();
        let mut c: i32 = 0_i32;
        unimplemented!();
        assert!(((c) == (97 as i32)));
    }
    unimplemented!();
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
