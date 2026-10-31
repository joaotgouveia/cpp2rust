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
        let mut m: () = unimplemented!();
        let s: [libc::c_char; 12] = std::mem::transmute(*b"abcdefghijk\0");
        assert!(unimplemented!());
        let fmt: [libc::c_char; 54] =
            std::mem::transmute(*b"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2\0");
        let mut out: Vec<libc::c_char> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                c"prefix: ab, match: cdefghi, suffix: jk, m[1]: efg, m[2]: e".as_ptr(),
                libcc2rs::c_str_len(
                    c"prefix: ab, match: cdefghi, suffix: jk, m[1]: efg, m[2]: e".as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut m: () = unimplemented!();
        let s: [libc::c_char; 12] = std::mem::transmute(*b"abcdefghijk\0");
        assert!(unimplemented!());
        let fmt: [libc::c_char; 54] =
            std::mem::transmute(*b"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2\0");
        let mut out: Vec<libc::c_char> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                c"prefix: $`, match: $cdefghi, suffix: $\', m[1]: $1, m[2]: $2".as_ptr(),
                libcc2rs::c_str_len(
                    c"prefix: $`, match: $cdefghi, suffix: $\', m[1]: $1, m[2]: $2".as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut m: () = unimplemented!();
        let s: [libc::c_char; 12] = std::mem::transmute(*b"abcdefghijk\0");
        assert!(unimplemented!());
        let fmt: [libc::c_char; 29] = std::mem::transmute(*b"match: &, m[1]: \\1, m[2]: \\2\0");
        let mut out: Vec<libc::c_char> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                c"match: cdefghi, m[1]: efg, m[2]: e".as_ptr(),
                libcc2rs::c_str_len(c"match: cdefghi, m[1]: efg, m[2]: e".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    {
        let mut m: () = unimplemented!();
        let s: [i32; 12] = [
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ];
        assert!(unimplemented!());
        let fmt: [i32; 54] = [
            112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
            32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
            116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 38 as i32,
            44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32,
            120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32,
            109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
            49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32,
            58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
        ];
        let mut out: Vec<i32> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                [
                    112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                    58 as i32, 32 as i32, 97 as i32, 98 as i32, 44 as i32, 32 as i32, 109 as i32,
                    97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 99 as i32,
                    100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32, 105 as i32,
                    44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32,
                    105 as i32, 120 as i32, 58 as i32, 32 as i32, 106 as i32, 107 as i32,
                    44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32,
                    32 as i32, 101 as i32, 102 as i32, 103 as i32, 44 as i32, 32 as i32,
                    109 as i32, 91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32,
                    0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                        58 as i32, 32 as i32, 97 as i32, 98 as i32, 44 as i32, 32 as i32,
                        109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32,
                        32 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                        104 as i32, 105 as i32, 44 as i32, 32 as i32, 115 as i32, 117 as i32,
                        102 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32, 32 as i32,
                        106 as i32, 107 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                        49 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32, 102 as i32,
                        103 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32,
                        93 as i32, 58 as i32, 32 as i32, 101 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut m: () = unimplemented!();
        let s: [i32; 12] = [
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ];
        assert!(unimplemented!());
        let fmt: [i32; 54] = [
            112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
            32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
            116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 38 as i32,
            44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32,
            120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32,
            109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
            49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32,
            58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
        ];
        let mut out: Vec<i32> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                [
                    112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                    58 as i32, 32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32,
                    97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32,
                    99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32,
                    105 as i32, 44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32,
                    102 as i32, 105 as i32, 120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32,
                    44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32,
                    32 as i32, 36 as i32, 49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                    50 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32,
                        58 as i32, 32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32,
                        109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32,
                        32 as i32, 36 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                        103 as i32, 104 as i32, 105 as i32, 44 as i32, 32 as i32, 115 as i32,
                        117 as i32, 102 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                        32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32, 109 as i32,
                        91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
                        49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32,
                        93 as i32, 58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    {
        let mut m: () = unimplemented!();
        let s: [i32; 12] = [
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ];
        assert!(unimplemented!());
        let fmt: [i32; 29] = [
            109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32,
            38 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32,
            58 as i32, 32 as i32, 92 as i32, 49 as i32, 44 as i32, 32 as i32, 109 as i32,
            91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 92 as i32, 50 as i32, 0 as i32,
        ];
        let mut out: Vec<i32> = unimplemented!();
        assert!(
            out == std::slice::from_raw_parts(
                [
                    109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32,
                    99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32,
                    105 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32,
                    58 as i32, 32 as i32, 101 as i32, 102 as i32, 103 as i32, 44 as i32, 32 as i32,
                    109 as i32, 91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32,
                    0 as i32,
                ]
                .as_ptr(),
                libcc2rs::c_str_len(
                    [
                        109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32,
                        32 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                        104 as i32, 105 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                        49 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32, 102 as i32,
                        103 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32,
                        93 as i32, 58 as i32, 32 as i32, 101 as i32, 0 as i32,
                    ]
                    .as_ptr()
                ) + 1
            )
            .to_vec()
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
