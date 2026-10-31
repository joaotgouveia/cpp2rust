extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_unget_0(mut fb: *mut (), mut expected: libc::c_char) {
    let mut c: () = unimplemented!();
    if ((c) != (-1_i32)) {
        assert!(((c) == (expected as i32)));
        assert!(((unimplemented!()) == (expected as i32)));
    }
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
    let mut stream_buffer: Vec<libc::c_char> = (0..(10_usize) as usize)
        .map(|_| (0 as libc::c_char))
        .collect::<Vec<_>>();
    let mut fs: ::std::fs::File = unimplemented!();
    let mut fb: *mut () = fs;
    unimplemented!();
    assert!(((unimplemented!()) == (('t' as libc::c_char) as i32)));
    let mut test_buffer: Vec<libc::c_char> = (0..(5_usize) as usize)
        .map(|_| (0 as libc::c_char))
        .collect::<Vec<_>>();
    (*(&mut test_buffer).as_mut_ptr().add(0_usize as usize)) = ('\0' as libc::c_char);
    {
        assert!(((unimplemented!()) == (5_())));
        assert!(
            std::slice::from_raw_parts((test_buffer.as_mut_ptr()).cast_const(), 5_usize as usize)
                .to_vec()
                .iter()
                .copied()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect()
                == std::slice::from_raw_parts(
                    c"this ".as_ptr(),
                    libcc2rs::c_str_len(c"this ".as_ptr()) + 1
                )
                .to_vec()
        );
        (unsafe { check_unget_0(fb, (' ' as libc::c_char)) });
    }
    {
        assert!(((unimplemented!()) == (5_())));
        assert!(
            std::slice::from_raw_parts((test_buffer.as_mut_ptr()).cast_const(), 5_usize as usize)
                .to_vec()
                .iter()
                .copied()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect()
                == std::slice::from_raw_parts(
                    c"is so".as_ptr(),
                    libcc2rs::c_str_len(c"is so".as_ptr()) + 1
                )
                .to_vec()
        );
        (unsafe { check_unget_0(fb, ('o' as libc::c_char)) });
    }
    {
        {
            let __a0 = 12_usize as usize;
            test_buffer.resize_with(__a0, || (0 as libc::c_char))
        };
        assert!(((unimplemented!()) == (12_())));
        assert!(
            std::slice::from_raw_parts((test_buffer.as_mut_ptr()).cast_const(), 12_usize as usize)
                .to_vec()
                .iter()
                .copied()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect()
                == std::slice::from_raw_parts(
                    c"me random da".as_ptr(),
                    libcc2rs::c_str_len(c"me random da".as_ptr()) + 1
                )
                .to_vec()
        );
        (unsafe { check_unget_0(fb, ('a' as libc::c_char)) });
    }
    {
        {
            let __a0 = 2_usize as usize;
            test_buffer.resize_with(__a0, || (0 as libc::c_char))
        };
        assert!(((unimplemented!()) == (2_())));
        assert!(
            std::slice::from_raw_parts((test_buffer.as_mut_ptr()).cast_const(), 2_usize as usize)
                .to_vec()
                .iter()
                .copied()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect()
                == std::slice::from_raw_parts(
                    c"ta".as_ptr(),
                    libcc2rs::c_str_len(c"ta".as_ptr()) + 1
                )
                .to_vec()
        );
        (unsafe { check_unget_0(fb, ('a' as libc::c_char)) });
        {
            let __a0 = 12_usize as usize;
            test_buffer.resize_with(__a0, || (0 as libc::c_char))
        };
        assert!(((unimplemented!()) == (12_())));
        assert!(
            std::slice::from_raw_parts((test_buffer.as_mut_ptr()).cast_const(), 12_usize as usize)
                .to_vec()
                .iter()
                .copied()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect()
                == std::slice::from_raw_parts(
                    c" to be able ".as_ptr(),
                    libcc2rs::c_str_len(c" to be able ".as_ptr()) + 1
                )
                .to_vec()
        );
        (unsafe { check_unget_0(fb, (' ' as libc::c_char)) });
    }
    {
        {
            let __a0 = 30_usize as usize;
            test_buffer.resize_with(__a0, || (0 as libc::c_char))
        };
        assert!(((unimplemented!()) == (24_())));
        {
            let __a0 = 24_usize as usize;
            test_buffer.resize_with(__a0, || (0 as libc::c_char))
        };
        assert!(
            std::slice::from_raw_parts((test_buffer.as_mut_ptr()).cast_const(), 24_usize as usize)
                .to_vec()
                .iter()
                .copied()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect()
                == std::slice::from_raw_parts(
                    c"to test buffer behaviour".as_ptr(),
                    libcc2rs::c_str_len(c"to test buffer behaviour".as_ptr()) + 1
                )
                .to_vec()
        );
        (unsafe { check_unget_0(fb, ('r' as libc::c_char)) });
    }
    {
        let mut buf: [libc::c_char; 10] = [(0 as libc::c_char); 10];
        let mut ifs: ::std::fs::File = unimplemented!();
        let mut filebuf: *mut () = ifs;
        assert!(!(unimplemented!()));
        assert!(((unimplemented!()) == (0_())));
        assert!(!(unimplemented!()));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
