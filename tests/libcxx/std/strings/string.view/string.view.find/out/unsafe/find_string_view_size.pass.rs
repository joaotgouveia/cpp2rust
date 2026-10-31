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
    pub unsafe fn find(
        mut s: *const libc::c_char,
        mut n: usize,
        a: *const libc::c_char,
    ) -> *const libc::c_char {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: libc::c_char = (*s);
                let _c2: libc::c_char = (*a);
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
pub unsafe fn test_0(s: *const (), str: *const (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (x)));
    if ((x) != (unimplemented!())) {
        assert!(
            ((pos) <= (x))
                && (((x as usize).wrapping_add((unimplemented!() as usize)))
                    <= (unimplemented!() as usize))
        );
    }
}
pub unsafe fn test_1(s: *const (), str: *const (), mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (x)));
    if ((x) != (unimplemented!())) {
        assert!(
            ((0_()) <= (x))
                && (((x as usize).wrapping_add((unimplemented!() as usize)))
                    <= (unimplemented!() as usize))
        );
    }
}
pub unsafe fn test0_2() {
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 6_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 6_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 6_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 6_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 9_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 9_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 9_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 10_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 10_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 10_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 11_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 11_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 11_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 11_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 10_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 19_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 19_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 19_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 19_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 20_(), 20_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 20_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 20_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 20_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 21_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 21_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 21_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_0(&mut _s, &mut _str, 21_(), unimplemented!())
    });
}
pub unsafe fn test1_3() {
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        let mut _str: () = unimplemented!();
        test_1(&mut _s, &mut _str, 0_())
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
        const _: () = assert!(true, "sv1.find(sv1) == 0");;
        const _: () = assert!(true, "sv1.find(sv2) == SV::npos");;
        const _: () = assert!(true, "sv2.find(sv1) == 0");;
        const _: () = assert!(true, "sv2.find(sv2) == 0");;
        const _: () = assert!(true, "sv2.find(sv2, 1) == SV::npos");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
