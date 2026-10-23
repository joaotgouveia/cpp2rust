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
    pub unsafe fn lt(mut c1: (), mut c2: ()) -> () {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const (), mut s2: *const (), mut n: ()) -> () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s1);
                let _c2: () = (*s2);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if (unsafe {
                let _c1: () = (*s2);
                let _c2: () = (*s1);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
}
pub unsafe fn sign_0(mut x: ()) -> () {
    return if ((x) > (0_())) {
        1_()
    } else {
        (if ((x) < (0_())) { -1_() } else { 0_() })
    };
}
pub unsafe fn test1_1(mut sv1: (), mut sv2: (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn test1_2(mut sv1: (), mut sv2: (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn test1_3(mut sv1: (), mut sv2: (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn test1_4(mut sv1: (), mut sv2: (), mut expected: ()) {
    unimplemented!();
}
pub unsafe fn test_5(mut s1: *const (), mut s2: *const (), mut expected: ()) {
    let mut sv1: () = unimplemented!();
    let mut sv2: () = unimplemented!();
    (unsafe { test1_1(unimplemented!(), unimplemented!(), expected) });
}
pub unsafe fn test_6(mut s1: *const (), mut s2: *const (), mut expected: ()) {
    let mut sv1: () = unimplemented!();
    let mut sv2: () = unimplemented!();
    (unsafe { test1_2(unimplemented!(), unimplemented!(), expected) });
}
pub unsafe fn test_7(mut s1: *const (), mut s2: *const (), mut expected: ()) {
    let mut sv1: () = unimplemented!();
    let mut sv2: () = unimplemented!();
    (unsafe { test1_3(unimplemented!(), unimplemented!(), expected) });
}
pub unsafe fn test_8(mut s1: *const (), mut s2: *const (), mut expected: ()) {
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_5(c"".as_ptr(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"".as_ptr(), c"abcde".as_ptr(), -5_()) });
    (unsafe { test_5(c"".as_ptr(), c"abcdefghij".as_ptr(), -10_()) });
    (unsafe { test_5(c"".as_ptr(), c"abcdefghijklmnopqrst".as_ptr(), -20_()) });
    (unsafe { test_5(c"abcde".as_ptr(), c"".as_ptr(), 5_()) });
    (unsafe { test_5(c"abcde".as_ptr(), c"abcde".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), c"abcdefghij".as_ptr(), -5_()) });
    (unsafe { test_5(c"abcde".as_ptr(), c"abcdefghijklmnopqrst".as_ptr(), -15_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), c"".as_ptr(), 10_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), c"abcde".as_ptr(), 5_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), c"abcdefghij".as_ptr(), 0_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -10_(),
        )
    });
    (unsafe { test_5(c"abcdefghijklmnopqrst".as_ptr(), c"".as_ptr(), 20_()) });
    (unsafe { test_5(c"abcdefghijklmnopqrst".as_ptr(), c"abcde".as_ptr(), 15_()) });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            c"abcdefghij".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
        )
    });
    (unsafe { test_6([0 as ()].as_ptr(), [0 as ()].as_ptr(), 0_()) });
    (unsafe {
        test_6(
            [0 as ()].as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_6(
            [0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_6(
            [0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            -20_(),
        )
    });
    (unsafe {
        test_6(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_6(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_6(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_6(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            -15_(),
        )
    });
    (unsafe {
        test_6(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [0 as ()].as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_6(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_6(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_6(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_6(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [0 as ()].as_ptr(),
            20_(),
        )
    });
    (unsafe {
        test_6(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            15_(),
        )
    });
    (unsafe {
        test_6(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_6(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            0_(),
        )
    });
    (unsafe { test_7([0 as ()].as_ptr(), [0 as ()].as_ptr(), 0_()) });
    (unsafe {
        test_7(
            [0 as ()].as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_7(
            [0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_7(
            [0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            -20_(),
        )
    });
    (unsafe {
        test_7(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_7(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_7(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_7(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            -15_(),
        )
    });
    (unsafe {
        test_7(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [0 as ()].as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_7(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_7(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_7(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_7(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [0 as ()].as_ptr(),
            20_(),
        )
    });
    (unsafe {
        test_7(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            15_(),
        )
    });
    (unsafe {
        test_7(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_7(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            0_(),
        )
    });
    (unsafe { test_8([0 as ()].as_ptr(), [0 as ()].as_ptr(), 0_()) });
    (unsafe {
        test_8(
            [0 as ()].as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_8(
            [0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_8(
            [0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            -20_(),
        )
    });
    (unsafe {
        test_8(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_8(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_8(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_8(
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            -15_(),
        )
    });
    (unsafe {
        test_8(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [0 as ()].as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_8(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_8(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_8(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_8(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [0 as ()].as_ptr(),
            20_(),
        )
    });
    (unsafe {
        test_8(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
            15_(),
        )
    });
    (unsafe {
        test_8(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 0 as (),
            ]
            .as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_8(
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            [
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
            ]
            .as_ptr(),
            0_(),
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
