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
    pub unsafe fn eq(mut c1: libc::c_char, mut c2: libc::c_char) -> bool {
        return ((c1 as i32) == (c2 as i32));
    }
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
    pub unsafe fn length(mut s: *const libc::c_char) -> usize {
        let mut len: usize = 0_usize;
        'loop_: while !(unsafe {
            let _c1: libc::c_char = (*s);
            let _c2: libc::c_char = (0 as libc::c_char);
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
}
pub unsafe fn sign_0(mut x: i32) -> i32 {
    return if ((x) > (0)) {
        1
    } else {
        (if ((x) < (0)) { -1_i32 } else { 0 })
    };
}
pub unsafe fn test1_1(
    mut sv1: (),
    mut pos1: usize,
    mut n1: usize,
    mut s2: *const libc::c_char,
    mut n2: usize,
    mut expected: i32,
) {
    if ((pos1) <= (unimplemented!() as usize)) {
        assert!(((unsafe { sign_0(unimplemented!(),) }) == (unsafe { sign_0(expected,) })));
    }
}
pub unsafe fn test1_2(
    mut sv1: (),
    mut pos1: usize,
    mut n1: usize,
    mut s2: *const i32,
    mut n2: usize,
    mut expected: i32,
) {
    if ((pos1) <= (unimplemented!() as usize)) {
        assert!(((unsafe { sign_0(unimplemented!(),) }) == (unsafe { sign_0(expected,) })));
    }
}
pub unsafe fn test1_3(
    mut sv1: (),
    mut pos1: usize,
    mut n1: usize,
    mut s2: *const u32,
    mut n2: usize,
    mut expected: i32,
) {
    if ((pos1) <= (unimplemented!() as usize)) {
        assert!(((unsafe { sign_0(unimplemented!(),) }) == (unsafe { sign_0(expected,) })));
    }
}
pub unsafe fn test_4(
    mut s1: *const libc::c_char,
    mut pos1: usize,
    mut n1: usize,
    mut s2: *const libc::c_char,
    mut n2: usize,
    mut expected: i32,
) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_1(unimplemented!(), pos1, n1, s2, n2, expected) });
}
pub unsafe fn test_5(
    mut s1: *const i32,
    mut pos1: usize,
    mut n1: usize,
    mut s2: *const i32,
    mut n2: usize,
    mut expected: i32,
) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_2(unimplemented!(), pos1, n1, s2, n2, expected) });
}
pub unsafe fn test_6(
    mut s1: *const u32,
    mut pos1: usize,
    mut n1: usize,
    mut s2: *const u32,
    mut n2: usize,
    mut expected: i32,
) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_3(unimplemented!(), pos1, n1, s2, n2, expected) });
}
pub unsafe fn test0_7() {
    (unsafe { test_4(c"".as_ptr(), 0_usize, 0_usize, c"".as_ptr(), 0_usize, 0) });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe { test_4(c"".as_ptr(), 0_usize, 1_usize, c"".as_ptr(), 0_usize, 0) });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe { test_4(c"".as_ptr(), 1_usize, 0_usize, c"".as_ptr(), 0_usize, 0) });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            -3_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -8_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -18_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcde".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcde".as_ptr(),
            2_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcde".as_ptr(),
            4_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcde".as_ptr(),
            5_usize,
            -3_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -3_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -7_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -8_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -8_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -17_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -18_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcde".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcde".as_ptr(),
            1_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcde".as_ptr(),
            2_usize,
            2,
        )
    });
}
pub unsafe fn test1_8() {
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcde".as_ptr(),
            4_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcde".as_ptr(),
            5_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -6_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -6_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -15_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -16_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            2_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -14_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -15_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcde".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcde".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcde".as_ptr(),
            2_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcde".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -14_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            0_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -15_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcde".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcde".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
}
pub unsafe fn test2_9() {
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcde".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            1_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcde".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcde".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcde".as_ptr(),
            2_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcde".as_ptr(),
            4_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcde".as_ptr(),
            5_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcde".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcde".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcde".as_ptr(),
            2_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcde".as_ptr(),
            4_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcde".as_ptr(),
            5_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            3,
        )
    });
}
pub unsafe fn test3_10() {
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            3_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcde".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcde".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcde".as_ptr(),
            2_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcde".as_ptr(),
            4_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcde".as_ptr(),
            5_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            2_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcde".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcde".as_ptr(),
            2_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcde".as_ptr(),
            4_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcde".as_ptr(),
            5_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            4_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
}
pub unsafe fn test4_11() {
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcde".as_ptr(),
            6_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            -3_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -8_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -18_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            2_usize,
            3,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcde".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -14_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -15_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcde".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcde".as_ptr(),
            1_usize,
            8,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcde".as_ptr(),
            2_usize,
            7,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcde".as_ptr(),
            4_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcde".as_ptr(),
            5_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            8,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            8,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -11_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            2_usize,
            8,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            4_usize,
            6,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcde".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcde".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcde".as_ptr(),
            2_usize,
            8,
        )
    });
}
pub unsafe fn test5_12() {
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcde".as_ptr(),
            4_usize,
            6,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcde".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            0_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"".as_ptr(),
            0_usize,
            8,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcde".as_ptr(),
            0_usize,
            8,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            8,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            8,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            8_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcde".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
}
pub unsafe fn test6_13() {
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            1_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcde".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcde".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcde".as_ptr(),
            2_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcde".as_ptr(),
            4_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcde".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            2,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcde".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcde".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcde".as_ptr(),
            2_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcde".as_ptr(),
            4_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcde".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            4,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            4_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcde".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcde".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcde".as_ptr(),
            2_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcde".as_ptr(),
            4_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcde".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcde".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcde".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcde".as_ptr(),
            2_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcde".as_ptr(),
            4_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcde".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            5,
        )
    });
}
pub unsafe fn test7_14() {
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            5_usize,
            6_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcde".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcde".as_ptr(),
            2_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcde".as_ptr(),
            4_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcde".as_ptr(),
            5_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            9_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghij".as_ptr(),
            11_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0,
        )
    });
}
pub unsafe fn test8_15() {
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            -3_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -8_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -18_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            2_usize,
            8,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            4_usize,
            6,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcde".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcde".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcde".as_ptr(),
            1_usize,
            18,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcde".as_ptr(),
            2_usize,
            17,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcde".as_ptr(),
            4_usize,
            15,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcde".as_ptr(),
            5_usize,
            14,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            18,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            14,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"".as_ptr(),
            0_usize,
            20,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcde".as_ptr(),
            0_usize,
            20,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcde".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcde".as_ptr(),
            2_usize,
            18,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcde".as_ptr(),
            4_usize,
            16,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcde".as_ptr(),
            5_usize,
            15,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            20,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            15,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            11,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"".as_ptr(),
            0_usize,
            20,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcde".as_ptr(),
            0_usize,
            20,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcde".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcde".as_ptr(),
            2_usize,
            18,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcde".as_ptr(),
            4_usize,
            16,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcde".as_ptr(),
            5_usize,
            15,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            20,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            15,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            11,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            20,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            21_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
}
pub unsafe fn test9_16() {
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"".as_ptr(),
            0_usize,
            18,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcde".as_ptr(),
            0_usize,
            18,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            18,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            18,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            18_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcde".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcde".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcde".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcde".as_ptr(),
            2_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcde".as_ptr(),
            4_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcde".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            20_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
}
pub unsafe fn test10_17() {
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcde".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcde".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcde".as_ptr(),
            2_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcde".as_ptr(),
            4_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcde".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            5,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            5_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcde".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcde".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcde".as_ptr(),
            2_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcde".as_ptr(),
            4_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcde".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            9,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            9_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcde".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcde".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcde".as_ptr(),
            2_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcde".as_ptr(),
            4_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcde".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcde".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcde".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcde".as_ptr(),
            2_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcde".as_ptr(),
            4_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcde".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            11_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            10,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
}
pub unsafe fn test11_18() {
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcde".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcde".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcde".as_ptr(),
            2_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcde".as_ptr(),
            4_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcde".as_ptr(),
            5_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            1,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            2_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            19,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcde".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcde".as_ptr(),
            2_usize,
            -2_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcde".as_ptr(),
            4_usize,
            -4_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcde".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            -5_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            -9_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            -1_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            -10_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            -19_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            1_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            -20_i32,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcde".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcde".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcde".as_ptr(),
            2_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcde".as_ptr(),
            4_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcde".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            5_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            9_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghij".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            0_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            1_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            10_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            19_usize,
            0,
        )
    });
    (unsafe {
        test_4(
            c"abcdefghijklmnopqrst".as_ptr(),
            21_usize,
            0_usize,
            c"abcdefghijklmnopqrst".as_ptr(),
            20_usize,
            0,
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test0_7() });
    (unsafe { test1_8() });
    (unsafe { test2_9() });
    (unsafe { test3_10() });
    (unsafe { test4_11() });
    (unsafe { test5_12() });
    (unsafe { test6_13() });
    (unsafe { test7_14() });
    (unsafe { test8_15() });
    (unsafe { test9_16() });
    (unsafe { test10_17() });
    (unsafe { test11_18() });
    {
        (unsafe {
            test_4(
                c"".as_ptr(),
                0_usize,
                0_usize,
                c"abcde".as_ptr(),
                0_usize,
                0,
            )
        });
        (unsafe {
            test_4(
                c"".as_ptr(),
                0_usize,
                0_usize,
                c"abcde".as_ptr(),
                1_usize,
                -1_i32,
            )
        });
        (unsafe {
            test_4(
                c"abcdefghijklmnopqrst".as_ptr(),
                21_usize,
                0_usize,
                c"abcde".as_ptr(),
                0_usize,
                0,
            )
        });
        (unsafe {
            test_4(
                c"abcdefghijklmnopqrst".as_ptr(),
                21_usize,
                0_usize,
                c"abcde".as_ptr(),
                1_usize,
                0,
            )
        });
        (unsafe {
            test_4(
                c"abcdefghijklmnopqrst".as_ptr(),
                10_usize,
                0_usize,
                c"abcdefghij".as_ptr(),
                10_usize,
                -10_i32,
            )
        });
    }
    {
        (unsafe {
            test_5(
                [0 as i32].as_ptr(),
                0_usize,
                0_usize,
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
                ]
                .as_ptr(),
                0_usize,
                0,
            )
        });
        (unsafe {
            test_5(
                [0 as i32].as_ptr(),
                0_usize,
                0_usize,
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
                ]
                .as_ptr(),
                1_usize,
                -1_i32,
            )
        });
        (unsafe {
            test_5(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                    103 as i32, 104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32,
                    109 as i32, 110 as i32, 111 as i32, 112 as i32, 113 as i32, 114 as i32,
                    115 as i32, 116 as i32, 0 as i32,
                ]
                .as_ptr(),
                21_usize,
                0_usize,
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
                ]
                .as_ptr(),
                0_usize,
                0,
            )
        });
        (unsafe {
            test_5(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                    103 as i32, 104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32,
                    109 as i32, 110 as i32, 111 as i32, 112 as i32, 113 as i32, 114 as i32,
                    115 as i32, 116 as i32, 0 as i32,
                ]
                .as_ptr(),
                21_usize,
                0_usize,
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
                ]
                .as_ptr(),
                1_usize,
                0,
            )
        });
        (unsafe {
            test_5(
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                    103 as i32, 104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32,
                    109 as i32, 110 as i32, 111 as i32, 112 as i32, 113 as i32, 114 as i32,
                    115 as i32, 116 as i32, 0 as i32,
                ]
                .as_ptr(),
                10_usize,
                0_usize,
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                    103 as i32, 104 as i32, 105 as i32, 106 as i32, 0 as i32,
                ]
                .as_ptr(),
                10_usize,
                -10_i32,
            )
        });
    }
    {
        (unsafe {
            test_6(
                [0 as u32].as_ptr(),
                0_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
                ]
                .as_ptr(),
                0_usize,
                0,
            )
        });
        (unsafe {
            test_6(
                [0 as u32].as_ptr(),
                0_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
                ]
                .as_ptr(),
                1_usize,
                -1_i32,
            )
        });
        (unsafe {
            test_6(
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32,
                    109 as u32, 110 as u32, 111 as u32, 112 as u32, 113 as u32, 114 as u32,
                    115 as u32, 116 as u32, 0 as u32,
                ]
                .as_ptr(),
                21_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
                ]
                .as_ptr(),
                0_usize,
                0,
            )
        });
        (unsafe {
            test_6(
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32,
                    109 as u32, 110 as u32, 111 as u32, 112 as u32, 113 as u32, 114 as u32,
                    115 as u32, 116 as u32, 0 as u32,
                ]
                .as_ptr(),
                21_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
                ]
                .as_ptr(),
                1_usize,
                0,
            )
        });
        (unsafe {
            test_6(
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32,
                    109 as u32, 110 as u32, 111 as u32, 112 as u32, 113 as u32, 114 as u32,
                    115 as u32, 116 as u32, 0 as u32,
                ]
                .as_ptr(),
                10_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 104 as u32, 105 as u32, 106 as u32, 0 as u32,
                ]
                .as_ptr(),
                10_usize,
                -10_i32,
            )
        });
    }
    {
        (unsafe {
            test_6(
                [0 as u32].as_ptr(),
                0_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
                ]
                .as_ptr(),
                0_usize,
                0,
            )
        });
        (unsafe {
            test_6(
                [0 as u32].as_ptr(),
                0_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
                ]
                .as_ptr(),
                1_usize,
                -1_i32,
            )
        });
        (unsafe {
            test_6(
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32,
                    109 as u32, 110 as u32, 111 as u32, 112 as u32, 113 as u32, 114 as u32,
                    115 as u32, 116 as u32, 0 as u32,
                ]
                .as_ptr(),
                21_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
                ]
                .as_ptr(),
                0_usize,
                0,
            )
        });
        (unsafe {
            test_6(
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32,
                    109 as u32, 110 as u32, 111 as u32, 112 as u32, 113 as u32, 114 as u32,
                    115 as u32, 116 as u32, 0 as u32,
                ]
                .as_ptr(),
                21_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
                ]
                .as_ptr(),
                1_usize,
                0,
            )
        });
        (unsafe {
            test_6(
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32,
                    109 as u32, 110 as u32, 111 as u32, 112 as u32, 113 as u32, 114 as u32,
                    115 as u32, 116 as u32, 0 as u32,
                ]
                .as_ptr(),
                10_usize,
                0_usize,
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 104 as u32, 105 as u32, 106 as u32, 0 as u32,
                ]
                .as_ptr(),
                10_usize,
                -10_i32,
            )
        });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.compare(0, 0, \"abcde\", 0) == 0");;
        const _: () = assert!(true, "sv1.compare(0, 0, \"abcde\", 1) < 0");;
        const _: () = assert!(true, "sv2.compare(0, 0, \"abcde\", 1, 0) == 0");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
