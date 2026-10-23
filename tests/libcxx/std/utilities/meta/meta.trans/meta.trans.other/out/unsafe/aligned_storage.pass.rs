extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 1");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_10__1__type>()) == (10_())),
            "sizeof(T1) == 10"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_10__2__type>()) == (10_())),
            "sizeof(T1) == 10"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_10__4__type>()) == (12_())),
            "sizeof(T1) == 12"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_10__8__type>()) == (16_())),
            "sizeof(T1) == 16"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 16");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_10__16__type>()) == (16_())),
            "sizeof(T1) == 16"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 32");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_10__32__type>()) == (32_())),
            "sizeof(T1) == 32"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 32");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_20__32__type>()) == (32_())),
            "sizeof(T1) == 32"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 32");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_40__32__type>()) == (64_())),
            "sizeof(T1) == 64"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 16");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_12__16__type>()) == (16_())),
            "sizeof(T1) == 16"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 1");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_1__type>()) == (1_())),
            "sizeof(T1) == 1"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_2__type>()) == (2_())),
            "sizeof(T1) == 2"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_3__type>()) == (4_())),
            "sizeof(T1) == 4"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_4__type>()) == (4_())),
            "sizeof(T1) == 4"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_5__type>()) == (8_())),
            "sizeof(T1) == 8"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_7__type>()) == (8_())),
            "sizeof(T1) == 8"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_8__type>()) == (8_())),
            "sizeof(T1) == 8"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_9__type>()) == (16_())),
            "sizeof(T1) == 16"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_15__type>()) == (16_())),
            "sizeof(T1) == 16"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        let alignment: () = if ((::std::mem::align_of::<()>()) > (16_())) {
            16_()
        } else {
            ::std::mem::align_of::<()>()
        };
        const _: () = assert!(true, "std::alignment_of<T1>::value == alignment");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_16__type>()) == (16_())),
            "sizeof(T1) == 16"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        let alignment: () = if ((::std::mem::align_of::<()>()) > (16_())) {
            16_()
        } else {
            ::std::mem::align_of::<()>()
        };
        const _: () = assert!(true, "std::alignment_of<T1>::value == alignment");;
        const _: () = assert!(true, "sizeof(T1) == 16 + alignment");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!(
            ((::std::mem::size_of::<std_aligned_storage_10__8__type>()) == (16_())),
            "sizeof(T1) == 16"
        );;
    }
    {
        let Align: () = 8192_();
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == Align");;
        const _: () = assert!(true, "sizeof(T1) == Align");;
    }
    {
        let Align: () = 65536_();
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == Align");;
        const _: () = assert!(true, "sizeof(T1) == Align");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
