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
pub unsafe fn test_0(s: *const (), mut str: *const (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1))) {
        let mut n: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_2(s: *const (), mut str: *const (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1))) {
        let mut n: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test0_3() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            6_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            6_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            6_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            6_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            11_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            11_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            11_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            11_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcde".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abcdeabcde".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 19_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 20_(), 20_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            21_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcde".as_ptr(),
            21_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            21_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            21_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
}
pub unsafe fn test1_4() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(
            &mut _s,
            c"abcde".as_ptr(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"abcde".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(
            &mut _s,
            c"abcdeabcde".as_ptr(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"abcde".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"abcdeabcde".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(
            &mut _s,
            c"abcdeabcdeabcdeabcde".as_ptr(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"abcde".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"abcdeabcde".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_2(&mut _s, c"abcdeabcdeabcdeabcde".as_ptr(), 0_())
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
        (unsafe { test0_3() });
        (unsafe { test1_4() });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.find(\"\") == 0");;
        const _: () = assert!(true, "sv1.find(\"abcde\") == SV::npos");;
        const _: () = assert!(true, "sv2.find(\"\") == 0");;
        const _: () = assert!(true, "sv2.find(\"abcde\") == 0");;
        const _: () = assert!(true, "sv2.find(\"abcde\", 1) == SV::npos");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
