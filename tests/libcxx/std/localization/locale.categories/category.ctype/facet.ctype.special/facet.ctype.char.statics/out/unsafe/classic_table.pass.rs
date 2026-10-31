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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    assert!(!((unimplemented!()).is_null()));
    assert!(((unimplemented!()) >= (256_usize)));
    let mut p: *const () = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (128_usize)) {
        let mut expect_cntrl: bool = (((i) < (32_usize)) || ((126_usize) < (i)));
        let mut expect_print: bool = (((32_usize) <= (i)) && ((i) <= (126_usize)));
        let mut expect_space: bool = (((9_usize) <= (i)) && ((i) <= (13_usize)))
            || ((i) == ((' ' as libc::c_char) as usize));
        let mut expect_blank: bool = (((i) == (('\t' as libc::c_char) as usize))
            || ((i) == ((' ' as libc::c_char) as usize)));
        let mut expect_upper: bool = (((('A' as libc::c_char) as usize) <= (i))
            && ((i) <= (('Z' as libc::c_char) as usize)));
        let mut expect_lower: bool = (((('a' as libc::c_char) as usize) <= (i))
            && ((i) <= (('z' as libc::c_char) as usize)));
        let mut expect_alpha: bool = (expect_upper) || (expect_lower);
        let mut expect_digit: bool = (((('0' as libc::c_char) as usize) <= (i))
            && ((i) <= (('9' as libc::c_char) as usize)));
        let mut expect_xdigit: bool = ((((('A' as libc::c_char) as usize) <= (i))
            && ((i) <= (('F' as libc::c_char) as usize)))
            || (((('a' as libc::c_char) as usize) <= (i))
                && ((i) <= (('f' as libc::c_char) as usize))))
            || (expect_digit);
        let mut expect_punct: bool = (((((33_usize) <= (i)) && ((i) <= (47_usize)))
            || (((58_usize) <= (i)) && ((i) <= (64_usize))))
            || (((91_usize) <= (i)) && ((i) <= (96_usize))))
            || (((123_usize) <= (i)) && ((i) <= (126_usize)));
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_cntrl as i32))
        );
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_print as i32))
        );
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_space as i32))
        );
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_blank as i32))
        );
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_lower as i32))
        );
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_upper as i32))
        );
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_alpha as i32))
        );
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_digit as i32))
        );
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_xdigit as i32))
        );
        assert!(
            ((((((*p.offset((i) as isize)) as i32) & (unimplemented!() as i32)) != 0) as i32)
                == (expect_punct as i32))
        );
        i.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
