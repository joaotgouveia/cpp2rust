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
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
    }
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
    pub unsafe fn length(mut s: *const ()) -> () {
        let mut len: () = 0_();
        'loop_: while !(unsafe {
            let _c1: () = (*s);
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
}
pub unsafe fn sign_0(mut x: ()) -> () {
    return if ((x) > (0_())) {
        1_()
    } else {
        (if ((x) < (0_())) { -1_() } else { 0_() })
    };
}
pub unsafe fn test1_1(mut sv1: (), mut pos1: (), mut n1: (), mut s: *const (), mut expected: ()) {
    if ((pos1) <= (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test1_2(mut sv1: (), mut pos1: (), mut n1: (), mut s: *const (), mut expected: ()) {
    if ((pos1) <= (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test1_3(mut sv1: (), mut pos1: (), mut n1: (), mut s: *const (), mut expected: ()) {
    if ((pos1) <= (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test1_4(mut sv1: (), mut pos1: (), mut n1: (), mut s: *const (), mut expected: ()) {
    if ((pos1) <= (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test_5(
    mut s1: *const (),
    mut pos1: (),
    mut n1: (),
    mut s2: *const (),
    mut expected: (),
) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_1(unimplemented!(), pos1, n1, s2, expected) });
}
pub unsafe fn test_6(
    mut s1: *const (),
    mut pos1: (),
    mut n1: (),
    mut s2: *const (),
    mut expected: (),
) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_2(unimplemented!(), pos1, n1, s2, expected) });
}
pub unsafe fn test_7(
    mut s1: *const (),
    mut pos1: (),
    mut n1: (),
    mut s2: *const (),
    mut expected: (),
) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_3(unimplemented!(), pos1, n1, s2, expected) });
}
pub unsafe fn test_8(
    mut s1: *const (),
    mut pos1: (),
    mut n1: (),
    mut s2: *const (),
    mut expected: (),
) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_4(unimplemented!(), pos1, n1, s2, expected) });
}
pub unsafe fn test0_9() {
    (unsafe { test_5(c"".as_ptr(), 0_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"".as_ptr(), 0_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe { test_5(c"".as_ptr(), 0_(), 0_(), c"abcdefghij".as_ptr(), -10_()) });
    (unsafe {
        test_5(
            c"".as_ptr(),
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"".as_ptr(), 0_(), 1_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"".as_ptr(), 0_(), 1_(), c"abcde".as_ptr(), -5_()) });
    (unsafe { test_5(c"".as_ptr(), 0_(), 1_(), c"abcdefghij".as_ptr(), -10_()) });
    (unsafe {
        test_5(
            c"".as_ptr(),
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"".as_ptr(), 1_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"".as_ptr(), 1_(), 0_(), c"abcde".as_ptr(), 0_()) });
    (unsafe { test_5(c"".as_ptr(), 1_(), 0_(), c"abcdefghij".as_ptr(), 0_()) });
    (unsafe {
        test_5(
            c"".as_ptr(),
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            0_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 1_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 1_(), c"abcde".as_ptr(), -4_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 1_(), c"abcdefghij".as_ptr(), -9_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -19_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 2_(), c"".as_ptr(), 2_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 2_(), c"abcde".as_ptr(), -3_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 2_(), c"abcdefghij".as_ptr(), -8_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -18_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 4_(), c"".as_ptr(), 4_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 4_(), c"abcde".as_ptr(), -1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 4_(), c"abcdefghij".as_ptr(), -6_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -16_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 5_(), c"".as_ptr(), 5_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 5_(), c"abcde".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 5_(), c"abcdefghij".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -15_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 6_(), c"".as_ptr(), 5_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 6_(), c"abcde".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 0_(), 6_(), c"abcdefghij".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -15_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            1_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 1_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 1_(), c"abcde".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 1_(), c"abcdefghij".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 2_(), c"".as_ptr(), 2_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 2_(), c"abcde".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 2_(), c"abcdefghij".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 3_(), c"".as_ptr(), 3_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 3_(), c"abcde".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 3_(), c"abcdefghij".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 4_(), c"".as_ptr(), 4_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 4_(), c"abcde".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 4_(), c"abcdefghij".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 5_(), c"".as_ptr(), 4_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 5_(), c"abcde".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 1_(), 5_(), c"abcdefghij".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            2_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 1_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 1_(), c"abcde".as_ptr(), 2_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 1_(), c"abcdefghij".as_ptr(), 2_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            2_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 2_(), c"".as_ptr(), 2_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 2_(), c"abcde".as_ptr(), 2_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 2_(), c"abcdefghij".as_ptr(), 2_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            2_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 3_(), c"".as_ptr(), 3_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 3_(), c"abcde".as_ptr(), 2_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 3_(), c"abcdefghij".as_ptr(), 2_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            2_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 4_(), c"".as_ptr(), 3_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 4_(), c"abcde".as_ptr(), 2_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 2_(), 4_(), c"abcdefghij".as_ptr(), 2_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            2_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 4_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 4_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            4_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 4_(), 1_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 4_(), 1_(), c"abcde".as_ptr(), 4_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 4_(), 1_(), c"abcdefghij".as_ptr(), 4_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            4_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 4_(), 2_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 4_(), 2_(), c"abcde".as_ptr(), 4_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 4_(), 2_(), c"abcdefghij".as_ptr(), 4_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            4_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 5_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 5_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            5_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcde".as_ptr(), 5_(), 1_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 5_(), 1_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            5_(),
            1_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
}
pub unsafe fn test1_10() {
    (unsafe { test_5(c"abcde".as_ptr(), 6_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 6_(), 0_(), c"abcde".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcde".as_ptr(), 6_(), 0_(), c"abcdefghij".as_ptr(), 0_()) });
    (unsafe {
        test_5(
            c"abcde".as_ptr(),
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 1_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 1_(), c"abcde".as_ptr(), -4_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            1_(),
            c"abcdefghij".as_ptr(),
            -9_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -19_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 5_(), c"".as_ptr(), 5_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 5_(), c"abcde".as_ptr(), 0_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            5_(),
            c"abcdefghij".as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -15_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 9_(), c"".as_ptr(), 9_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 9_(), c"abcde".as_ptr(), 4_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            9_(),
            c"abcdefghij".as_ptr(),
            -1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -11_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 10_(), c"".as_ptr(), 10_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 10_(), c"abcde".as_ptr(), 5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            10_(),
            c"abcdefghij".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -10_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 11_(), c"".as_ptr(), 10_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 0_(), 11_(), c"abcde".as_ptr(), 5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            11_(),
            c"abcdefghij".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -10_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 1_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 1_(), c"abcde".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            1_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 4_(), c"".as_ptr(), 4_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 4_(), c"abcde".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            4_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 8_(), c"".as_ptr(), 8_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 8_(), c"abcde".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            8_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 9_(), c"".as_ptr(), 9_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 9_(), c"abcde".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            9_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 10_(), c"".as_ptr(), 9_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 1_(), 10_(), c"abcde".as_ptr(), 1_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            10_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 1_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 1_(), c"abcde".as_ptr(), 5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            1_(),
            c"abcdefghij".as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            5_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 2_(), c"".as_ptr(), 2_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 2_(), c"abcde".as_ptr(), 5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            2_(),
            c"abcdefghij".as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            5_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 4_(), c"".as_ptr(), 4_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 4_(), c"abcde".as_ptr(), 5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            4_(),
            c"abcdefghij".as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            5_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 5_(), c"".as_ptr(), 5_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 5_(), c"abcde".as_ptr(), 5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            5_(),
            c"abcdefghij".as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            5_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 6_(), c"".as_ptr(), 5_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 5_(), 6_(), c"abcde".as_ptr(), 5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            6_(),
            c"abcdefghij".as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            5_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 9_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 9_(), 0_(), c"abcde".as_ptr(), -5_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            9_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 9_(), 1_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 9_(), 1_(), c"abcde".as_ptr(), 9_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            9_(),
            1_(),
            c"abcdefghij".as_ptr(),
            9_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            9_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 9_(), 2_(), c"".as_ptr(), 1_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 9_(), 2_(), c"abcde".as_ptr(), 9_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            9_(),
            2_(),
            c"abcdefghij".as_ptr(),
            9_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            9_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 10_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            10_(),
            0_(),
            c"abcde".as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            10_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 10_(), 1_(), c"".as_ptr(), 0_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            10_(),
            1_(),
            c"abcde".as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            10_(),
            1_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 11_(), 0_(), c"".as_ptr(), 0_()) });
    (unsafe { test_5(c"abcdefghij".as_ptr(), 11_(), 0_(), c"abcde".as_ptr(), 0_()) });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            11_(),
            0_(),
            c"abcdefghij".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghij".as_ptr(),
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
        )
    });
}
pub unsafe fn test2_11() {
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
            c"abcde".as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
            c"".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
            c"abcde".as_ptr(),
            -4_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
            c"abcdefghij".as_ptr(),
            -9_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
            c"".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
            c"abcde".as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
            c"abcdefghij".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
            c"".as_ptr(),
            19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
            c"abcde".as_ptr(),
            14_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
            c"abcdefghij".as_ptr(),
            9_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            20_(),
            c"".as_ptr(),
            20_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            20_(),
            c"abcde".as_ptr(),
            15_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            20_(),
            c"abcdefghij".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            21_(),
            c"".as_ptr(),
            20_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            21_(),
            c"abcde".as_ptr(),
            15_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            21_(),
            c"abcdefghij".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
            c"abcde".as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
            c"".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
            c"abcde".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
            c"".as_ptr(),
            9_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
            c"abcde".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            18_(),
            c"".as_ptr(),
            18_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            18_(),
            c"abcde".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            18_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
            c"".as_ptr(),
            19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
            c"abcde".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            20_(),
            c"".as_ptr(),
            19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            20_(),
            c"abcde".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            20_(),
            c"abcdefghij".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
            c"".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
            c"abcde".as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
            c"".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
            c"abcde".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
            c"abcdefghij".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
            c"".as_ptr(),
            5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
            c"abcde".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
            c"abcdefghij".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
            c"".as_ptr(),
            9_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
            c"abcde".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
            c"abcdefghij".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
            c"".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
            c"abcde".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
            c"abcdefghij".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            11_(),
            c"".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            11_(),
            c"abcde".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            11_(),
            c"abcdefghij".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
            c"".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
            c"abcde".as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
            c"".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
            c"abcde".as_ptr(),
            19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
            c"abcdefghij".as_ptr(),
            19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
            c"".as_ptr(),
            1_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
            c"abcde".as_ptr(),
            19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
            c"abcdefghij".as_ptr(),
            19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
            c"".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
            c"abcde".as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
            c"".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
            c"abcde".as_ptr(),
            -5_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
            c"abcdefghij".as_ptr(),
            -10_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            -20_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_(),
            0_(),
            c"".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_(),
            0_(),
            c"abcde".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_(),
            0_(),
            c"abcdefghij".as_ptr(),
            0_(),
        )
    });
    (unsafe {
        test_5(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
        )
    });
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
    (unsafe { test0_9() });
    (unsafe { test1_10() });
    (unsafe { test2_11() });
    {
        (unsafe { test_5(c"".as_ptr(), 0_(), 0_(), c"".as_ptr(), 0_()) });
        (unsafe { test_5(c"".as_ptr(), 0_(), 0_(), c"abcde".as_ptr(), -5_()) });
        (unsafe { test_5(c"".as_ptr(), 0_(), 0_(), c"abcdefghij".as_ptr(), -10_()) });
        (unsafe {
            test_5(
                c"".as_ptr(),
                0_(),
                0_(),
                c"abcdefghijklmnopqrst".as_ptr(),
                -20_(),
            )
        });
        (unsafe { test_5(c"abcde".as_ptr(), 0_(), 2_(), c"".as_ptr(), 5_()) });
        (unsafe { test_5(c"abcde".as_ptr(), 0_(), 6_(), c"abcde".as_ptr(), 0_()) });
        (unsafe { test_5(c"abcde".as_ptr(), 0_(), 6_(), c"abcdefghij".as_ptr(), -5_()) });
        (unsafe {
            test_5(
                c"abcde".as_ptr(),
                0_(),
                6_(),
                c"abcdefghijklmnopqrst".as_ptr(),
                -15_(),
            )
        });
        (unsafe { test_5(c"abcdefghij".as_ptr(), 3_(), 3_(), c"".as_ptr(), 10_()) });
        (unsafe { test_5(c"abcdefghij".as_ptr(), 3_(), 3_(), c"abcde".as_ptr(), 5_()) });
        (unsafe { test_5(c"abcdefghij".as_ptr(), 3_(), 3_(), c"def".as_ptr(), 0_()) });
        (unsafe {
            test_5(
                c"abcdefghij".as_ptr(),
                0_(),
                4_(),
                c"abcdefghijklmnopqrst".as_ptr(),
                -10_(),
            )
        });
        (unsafe {
            test_5(
                c"abcdefghijklmnopqrst".as_ptr(),
                5_(),
                5_(),
                c"".as_ptr(),
                20_(),
            )
        });
        (unsafe {
            test_5(
                c"abcdefghijklmnopqrst".as_ptr(),
                0_(),
                8_(),
                c"abcde".as_ptr(),
                15_(),
            )
        });
        (unsafe {
            test_5(
                c"abcdefghijklmnopqrst".as_ptr(),
                0_(),
                12_(),
                c"abcdefghij".as_ptr(),
                10_(),
            )
        });
        (unsafe {
            test_5(
                c"abcdefghijklmnopqrst".as_ptr(),
                0_(),
                -1_(),
                c"abcdefghijklmnopqrst".as_ptr(),
                0_(),
            )
        });
    }
    {
        (unsafe { test_6([0 as ()].as_ptr(), 0_(), 0_(), [0 as ()].as_ptr(), 0_()) });
        (unsafe {
            test_6(
                [0 as ()].as_ptr(),
                0_(),
                0_(),
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                -5_(),
            )
        });
        (unsafe {
            test_6(
                [0 as ()].as_ptr(),
                0_(),
                0_(),
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
                0_(),
                0_(),
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
                0_(),
                2_(),
                [0 as ()].as_ptr(),
                5_(),
            )
        });
        (unsafe {
            test_6(
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                0_(),
                6_(),
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                0_(),
            )
        });
        (unsafe {
            test_6(
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                0_(),
                6_(),
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
                0_(),
                6_(),
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
                3_(),
                3_(),
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
                3_(),
                3_(),
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
                3_(),
                3_(),
                [100 as (), 101 as (), 102 as (), 0 as ()].as_ptr(),
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
                0_(),
                4_(),
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
                5_(),
                5_(),
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
                0_(),
                8_(),
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
                0_(),
                12_(),
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
                0_(),
                -1_(),
                [
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]
                .as_ptr(),
                0_(),
            )
        });
    }
    {
        (unsafe { test_7([0 as ()].as_ptr(), 0_(), 0_(), [0 as ()].as_ptr(), 0_()) });
        (unsafe {
            test_7(
                [0 as ()].as_ptr(),
                0_(),
                0_(),
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                -5_(),
            )
        });
        (unsafe {
            test_7(
                [0 as ()].as_ptr(),
                0_(),
                0_(),
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
                0_(),
                0_(),
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
                0_(),
                2_(),
                [0 as ()].as_ptr(),
                5_(),
            )
        });
        (unsafe {
            test_7(
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                0_(),
                6_(),
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                0_(),
            )
        });
        (unsafe {
            test_7(
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                0_(),
                6_(),
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
                0_(),
                6_(),
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
                3_(),
                3_(),
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
                3_(),
                3_(),
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
                3_(),
                3_(),
                [100 as (), 101 as (), 102 as (), 0 as ()].as_ptr(),
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
                0_(),
                4_(),
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
                5_(),
                5_(),
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
                0_(),
                8_(),
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
                0_(),
                12_(),
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
                0_(),
                -1_(),
                [
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]
                .as_ptr(),
                0_(),
            )
        });
    }
    {
        (unsafe { test_8([0 as ()].as_ptr(), 0_(), 0_(), [0 as ()].as_ptr(), 0_()) });
        (unsafe {
            test_8(
                [0 as ()].as_ptr(),
                0_(),
                0_(),
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                -5_(),
            )
        });
        (unsafe {
            test_8(
                [0 as ()].as_ptr(),
                0_(),
                0_(),
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
                0_(),
                0_(),
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
                0_(),
                2_(),
                [0 as ()].as_ptr(),
                5_(),
            )
        });
        (unsafe {
            test_8(
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                0_(),
                6_(),
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                0_(),
            )
        });
        (unsafe {
            test_8(
                [97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as ()].as_ptr(),
                0_(),
                6_(),
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
                0_(),
                6_(),
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
                3_(),
                3_(),
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
                3_(),
                3_(),
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
                3_(),
                3_(),
                [100 as (), 101 as (), 102 as (), 0 as ()].as_ptr(),
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
                0_(),
                4_(),
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
                5_(),
                5_(),
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
                0_(),
                8_(),
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
                0_(),
                12_(),
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
                0_(),
                -1_(),
                [
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]
                .as_ptr(),
                0_(),
            )
        });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.compare(0, 0, \"\") == 0");;
        const _: () = assert!(true, "sv1.compare(0, 0, \"abcde\") < 0");;
        const _: () = assert!(true, "sv2.compare(0, 2, \"\") > 0");;
        const _: () = assert!(true, "sv2.compare(0, 6, \"abcde\") == 0");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
