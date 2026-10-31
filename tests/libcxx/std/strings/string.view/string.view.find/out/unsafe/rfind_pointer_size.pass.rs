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
pub unsafe fn test_0(s: *const (), mut str: *const (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (x)));
    if ((x) != (unimplemented!())) {
        let mut n: () = (unimplemented!() as ());
        assert!(((x) <= (pos)) && (((x).wrapping_add(n)) <= (unimplemented!())));
    }
}
pub unsafe fn test_1(s: *const (), mut str: *const (), mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (x)));
    if ((x) != (unimplemented!())) {
        let mut pos: () = unimplemented!();
        let mut n: () = (unimplemented!() as ());
        assert!(((x) <= (pos)) && (((x).wrapping_add(n)) <= (unimplemented!())));
    }
}
pub unsafe fn test0_2() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            2_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            2_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            4_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            4_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            6_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            6_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            6_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            6_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            9_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            9_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            9_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            11_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            11_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            11_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            11_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            19_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            19_(),
            15_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            20_(),
            20_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            20_(),
            15_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            20_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            21_(),
            20_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            21_(),
            15_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            21_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            21_(),
            0_(),
        )
    });
}
pub unsafe fn test1_3() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"".as_ptr() as *const libc::c_char).cast::<()>(),
            20_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcde".as_ptr() as *const libc::c_char).cast::<()>(),
            15_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(
            &mut _s,
            (c"abcdeabcdeabcdeabcde".as_ptr() as *const libc::c_char).cast::<()>(),
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        (unsafe { test0_2() });
        (unsafe { test1_3() });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.rfind(\"\") == 0");;
        const _: () = assert!(true, "sv1.rfind(\"abcde\") == SV::npos");;
        const _: () = assert!(true, "sv2.rfind(\"\") == 5");;
        const _: () = assert!(true, "sv2.rfind(\"abcde\") == 0");;
        const _: () = assert!(true, "sv2.rfind(\"abcde\", 1) == 0");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
