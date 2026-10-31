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
    {
        let s1: [libc::c_char; 4] = [
            ('1' as libc::c_char),
            ('2' as libc::c_char),
            ('3' as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut sm1: () = unimplemented!();
        (sm1 as (*const libc::c_char, *const libc::c_char)).0 = s1.as_ptr();
        (sm1 as (*const libc::c_char, *const libc::c_char)).1 = s1.as_ptr().offset((3) as isize);
        sm1.matched = true;
        let mut sm2: () = unimplemented!();
        let s2: [libc::c_char; 4] = [
            ('c' as libc::c_char),
            ('a' as libc::c_char),
            ('t' as libc::c_char),
            (0 as libc::c_char),
        ];
        (sm2 as (*const libc::c_char, *const libc::c_char)).0 = s2.as_ptr();
        (sm2 as (*const libc::c_char, *const libc::c_char)).1 = s2.as_ptr().offset((3) as isize);
        sm2.matched = false;
        unimplemented!();
        assert!((((sm1 as (*const libc::c_char, *const libc::c_char)).0) == (s2.as_ptr())));
        assert!(
            (((sm1 as (*const libc::c_char, *const libc::c_char)).1)
                == (s2.as_ptr().offset((3) as isize)))
        );
        assert!(!(sm1.matched));
        assert!((((sm2 as (*const libc::c_char, *const libc::c_char)).0) == (s1.as_ptr())));
        assert!(
            (((sm2 as (*const libc::c_char, *const libc::c_char)).1)
                == (s1.as_ptr().offset((3) as isize)))
        );
        assert!(sm2.matched);
        const _: () = assert!(true, "");;
    }
    {
        let s1: [i32; 4] = [(49 as i32), (50 as i32), (51 as i32), 0_i32];
        let mut sm1: () = unimplemented!();
        (sm1 as (*const i32, *const i32)).0 = s1.as_ptr();
        (sm1 as (*const i32, *const i32)).1 = s1.as_ptr().offset((3) as isize);
        sm1.matched = true;
        let mut sm2: () = unimplemented!();
        let s2: [i32; 4] = [(99 as i32), (97 as i32), (116 as i32), 0_i32];
        (sm2 as (*const i32, *const i32)).0 = s2.as_ptr();
        (sm2 as (*const i32, *const i32)).1 = s2.as_ptr().offset((3) as isize);
        sm2.matched = false;
        unimplemented!();
        assert!((((sm1 as (*const i32, *const i32)).0) == (s2.as_ptr())));
        assert!((((sm1 as (*const i32, *const i32)).1) == (s2.as_ptr().offset((3) as isize))));
        assert!(!(sm1.matched));
        assert!((((sm2 as (*const i32, *const i32)).0) == (s1.as_ptr())));
        assert!((((sm2 as (*const i32, *const i32)).1) == (s1.as_ptr().offset((3) as isize))));
        assert!(sm2.matched);
        const _: () = assert!(true, "");;
    }
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
