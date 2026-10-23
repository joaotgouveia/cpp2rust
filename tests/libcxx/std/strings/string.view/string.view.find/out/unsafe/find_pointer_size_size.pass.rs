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
    pub unsafe fn find(mut s: *const (), mut n: (), a: *const ()) -> *const () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s);
                let _c2: () = (*a);
                constexpr_char_traits_char_::eq(_c1, _c2)
            }) {
                return s;
            }
            s.prefix_inc();
            n.prefix_dec();
        }
        return std::ptr::null();
    }
}
pub unsafe fn test_0(s: *const (), mut str: *const (), mut pos: (), mut n: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test0_1() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 2_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 2_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 2_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 2_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            2_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            2_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            2_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            2_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            2_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            2_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 4_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 4_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 4_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 4_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            4_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            4_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            4_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            4_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            4_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            4_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 2_(), unimplemented!())
    });
}
pub unsafe fn test1_2() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            5_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            5_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            5_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            5_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 6_(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 6_(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 6_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 6_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 6_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 6_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            6_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            6_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            6_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            6_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            6_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            6_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 1_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 5_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 5_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            5_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 5_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            5_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            5_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 9_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 9_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 9_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 9_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            9_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            9_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            9_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            9_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            9_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            9_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            9_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            9_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            10_(),
            1_(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test2_3() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            10_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            10_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            10_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 11_(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 11_(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 11_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 11_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 11_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 11_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            11_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            11_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            11_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            11_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            11_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            11_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            11_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            11_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 19_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 20_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 1_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 1_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 10_(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 10_(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 10_(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 10_(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 19_(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 19_(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 19_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 19_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 19_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 19_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 19_(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            19_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            19_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            19_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            19_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            19_(),
            0_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            19_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            19_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            19_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            19_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 20_(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 20_(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 20_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 20_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 20_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 20_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 20_(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            20_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            20_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            20_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            20_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            20_(),
            0_(),
            20_(),
        )
    });
}
pub unsafe fn test3_4() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            20_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            20_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            20_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            20_(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 21_(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 21_(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 21_(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 21_(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 21_(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 21_(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            21_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            21_(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            21_(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            21_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            21_(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            21_(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            21_(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            21_(),
            20_(),
            unimplemented!(),
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
    {
        (unsafe { test0_1() });
        (unsafe { test1_2() });
        (unsafe { test2_3() });
        (unsafe { test3_4() });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.find(\"\", 0, 0) == 0");;
        const _: () = assert!(true, "sv1.find(\"abcde\", 0, 0) == 0");;
        const _: () = assert!(true, "sv1.find(\"abcde\", 0, 1) == SV::npos");;
        const _: () = assert!(true, "sv2.find(\"\", 0, 0) == 0");;
        const _: () = assert!(true, "sv2.find(\"abcde\", 0, 0) == 0");;
        const _: () = assert!(true, "sv2.find(\"abcde\", 0, 1) == 0");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
