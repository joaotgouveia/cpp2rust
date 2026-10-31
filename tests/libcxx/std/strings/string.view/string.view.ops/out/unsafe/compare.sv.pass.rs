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
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub unsafe fn lt(mut c1: libc::c_char, mut c2: libc::c_char) -> bool {
        return ((c1 as i32) < (c2 as i32));
    }
    pub unsafe fn compare(
        mut s1: *const libc::c_char,
        mut s2: *const libc::c_char,
        mut n: usize,
    ) -> i32 {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: libc::c_char = (*s1);
                let _c2: libc::c_char = (*s2);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_i32;
            }
            if (unsafe {
                let _c1: libc::c_char = (*s2);
                let _c2: libc::c_char = (*s1);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1;
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0;
    }
}
pub unsafe fn sign_0(mut x: i32) -> i32 {
    return if ((x) > (0)) {
        1
    } else {
        (if ((x) < (0)) { -1_i32 } else { 0 })
    };
}
pub unsafe fn test1_1(mut sv1: (), mut sv2: (), mut expected: i32) {
    assert!(((unsafe { sign_0(unimplemented!(),) }) == (unsafe { sign_0(expected,) })));
}
pub unsafe fn test1_2(mut sv1: (), mut sv2: (), mut expected: i32) {
    assert!(((unsafe { sign_0(unimplemented!(),) }) == (unsafe { sign_0(expected,) })));
}
pub unsafe fn test1_3(mut sv1: (), mut sv2: (), mut expected: i32) {
    assert!(((unsafe { sign_0(unimplemented!(),) }) == (unsafe { sign_0(expected,) })));
}
pub unsafe fn test1_4(mut sv1: (), mut sv2: (), mut expected: i32) {
    assert!(((unsafe { sign_0(unimplemented!(),) }) == (unsafe { sign_0(expected,) })));
}
pub unsafe fn test_5(mut s1: *const libc::c_char, mut s2: *const libc::c_char, mut expected: i32) {
    let mut sv1: () = unimplemented!();
    let mut sv2: () = unimplemented!();
    (unsafe { test1_1(unimplemented!(), unimplemented!(), expected) });
}
pub unsafe fn test_6(mut s1: *const i32, mut s2: *const i32, mut expected: i32) {
    let mut sv1: () = unimplemented!();
    let mut sv2: () = unimplemented!();
    (unsafe { test1_2(unimplemented!(), unimplemented!(), expected) });
}
pub unsafe fn test_7(mut s1: *const u16, mut s2: *const u16, mut expected: i32) {
    let mut sv1: () = unimplemented!();
    let mut sv2: () = unimplemented!();
    (unsafe { test1_3(unimplemented!(), unimplemented!(), expected) });
}
pub unsafe fn test_8(mut s1: *const u32, mut s2: *const u32, mut expected: i32) {
    let mut sv1: () = unimplemented!();
    let mut sv2: () = unimplemented!();
    (unsafe { test1_4(unimplemented!(), unimplemented!(), expected) });
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
    (unsafe { test_5(c"".as_ptr(), c"".as_ptr(), 0) });
    (unsafe { test_5(c"".as_ptr(), c"abcde".as_ptr(), -5_i32) });
    (unsafe { test_5(c"".as_ptr(), c"abcdefghij".as_ptr(), -10_i32) });
    (unsafe { test_5(c"".as_ptr(), c"abcdefghijklmnopqrst".as_ptr(), -20_i32) });
    (unsafe { test_5(c"abcde".as_ptr(), c"".as_ptr(), 5) });
    (unsafe { test_5(c"abcde".as_ptr(), c"abcde".as_ptr(), 0) });
    (unsafe { test_5(c"abcde".as_ptr(), c"abcdefghij".as_ptr(), -5_i32) });
    (unsafe { test_5(c"abcde".as_ptr(), c"abcdefghijklmnopqrst".as_ptr(), -15_i32) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), c"".as_ptr(), 10) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), c"abcde".as_ptr(), 5) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), c"abcdefghij".as_ptr(), 0) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -10_i32,
        )
    });
    (unsafe { test_5(c"abcdefghijklmnopqrst".as_ptr(), c"".as_ptr(), 20) });
    (unsafe { test_5(c"abcdefghijklmnopqrst".as_ptr(), c"abcde".as_ptr(), 15) });
    (unsafe { test_5(c"abcdefghijklmnopqrst".as_ptr(), c"abcdefghij".as_ptr(), 10) });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0,
        )
    });
    (unsafe { test_6([0 as i32].as_ptr(), [0 as i32].as_ptr(), 0) });
    (unsafe {
        test_6(
            [0 as i32].as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            -5_i32,
        )
    });
    (unsafe {
        test_6(
            [0 as i32].as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]
            .as_ptr(),
            -10_i32,
        )
    });
    (unsafe {
        test_6(
            [0 as i32].as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            -20_i32,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            [0 as i32].as_ptr(),
            5,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            0,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]
            .as_ptr(),
            -5_i32,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            -15_i32,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]
            .as_ptr(),
            [0 as i32].as_ptr(),
            10,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            5,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]
            .as_ptr(),
            0,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            -10_i32,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            [0 as i32].as_ptr(),
            20,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]
            .as_ptr(),
            15,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]
            .as_ptr(),
            10,
        )
    });
    (unsafe {
        test_6(
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            0,
        )
    });
    (unsafe { test_7([0 as u16].as_ptr(), [0 as u16].as_ptr(), 0) });
    (unsafe {
        test_7(
            [0 as u16].as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]
            .as_ptr(),
            -5_i32,
        )
    });
    (unsafe {
        test_7(
            [0 as u16].as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]
            .as_ptr(),
            -10_i32,
        )
    });
    (unsafe {
        test_7(
            [0 as u16].as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]
            .as_ptr(),
            -20_i32,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]
            .as_ptr(),
            [0 as u16].as_ptr(),
            5,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]
            .as_ptr(),
            0,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]
            .as_ptr(),
            -5_i32,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]
            .as_ptr(),
            -15_i32,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]
            .as_ptr(),
            [0 as u16].as_ptr(),
            10,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]
            .as_ptr(),
            5,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]
            .as_ptr(),
            0,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]
            .as_ptr(),
            -10_i32,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]
            .as_ptr(),
            [0 as u16].as_ptr(),
            20,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]
            .as_ptr(),
            15,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]
            .as_ptr(),
            10,
        )
    });
    (unsafe {
        test_7(
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]
            .as_ptr(),
            0,
        )
    });
    (unsafe { test_8([0 as u32].as_ptr(), [0 as u32].as_ptr(), 0) });
    (unsafe {
        test_8(
            [0 as u32].as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]
            .as_ptr(),
            -5_i32,
        )
    });
    (unsafe {
        test_8(
            [0 as u32].as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]
            .as_ptr(),
            -10_i32,
        )
    });
    (unsafe {
        test_8(
            [0 as u32].as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]
            .as_ptr(),
            -20_i32,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]
            .as_ptr(),
            [0 as u32].as_ptr(),
            5,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]
            .as_ptr(),
            0,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]
            .as_ptr(),
            -5_i32,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]
            .as_ptr(),
            -15_i32,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]
            .as_ptr(),
            [0 as u32].as_ptr(),
            10,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]
            .as_ptr(),
            5,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]
            .as_ptr(),
            0,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]
            .as_ptr(),
            -10_i32,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]
            .as_ptr(),
            [0 as u32].as_ptr(),
            20,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]
            .as_ptr(),
            15,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]
            .as_ptr(),
            10,
        )
    });
    (unsafe {
        test_8(
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]
            .as_ptr(),
            0,
        )
    });
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        let sv3: () = unimplemented!();
        const _: () = assert!(true, "sv1.compare(sv2) == 0");;
        const _: () = assert!(true, "sv2.compare(sv1) == 0");;
        const _: () = assert!(true, "sv3.compare(sv2) > 0");;
        const _: () = assert!(true, "sv2.compare(sv3) < 0");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
