extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(
    mut bad: libc::c_char,
    mut args_1: (libc::c_char, i32),
    mut args_2: (libc::c_char, i32),
    mut args_3: (libc::c_char, i32),
    mut args_4: (libc::c_char, i32),
) -> bool {
    let mut map: UnsafeMap<libc::c_char, (libc::c_char, i32)> = UnsafeMap::new();
    let mut pairs: [(libc::c_char, i32); 4] = [
        {
            let __a0 = (args_1).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_2).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_3).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_4).clone();
            ((*&__a0.0), (*&__a0.1))
        },
    ];
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (libc::c_char, i32) = &mut (*p);
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (libc::c_char, i32) = &mut (*p);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub unsafe fn test_1(
    mut bad: libc::c_char,
    mut args_1: (libc::c_char, libc::c_char),
    mut args_2: (libc::c_char, libc::c_char),
    mut args_3: (libc::c_char, libc::c_char),
    mut args_4: (libc::c_char, libc::c_char),
) -> bool {
    let mut map: UnsafeMap<libc::c_char, (libc::c_char, libc::c_char)> = UnsafeMap::new();
    let mut pairs: [(libc::c_char, libc::c_char); 4] = [
        {
            let __a0 = (args_1).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_2).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_3).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_4).clone();
            ((*&__a0.0), (*&__a0.1))
        },
    ];
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (libc::c_char, libc::c_char) = &mut (*p);
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (libc::c_char, libc::c_char) = &mut (*p);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub unsafe fn test_2(
    mut bad: i32,
    mut args_1: (i32, E),
    mut args_2: (i32, E),
    mut args_3: (i32, E),
    mut args_4: (i32, E),
) -> bool {
    let mut map: UnsafeMap<i32, (i32, E)> = UnsafeMap::new();
    let mut pairs: [(i32, E); 4] = [
        {
            let __a0 = (args_1).clone();
            ((*&__a0.0), <E as CppFromUnsafe<E>>::cpp_from(&__a0.1))
        },
        {
            let __a0 = (args_2).clone();
            ((*&__a0.0), <E as CppFromUnsafe<E>>::cpp_from(&__a0.1))
        },
        {
            let __a0 = (args_3).clone();
            ((*&__a0.0), <E as CppFromUnsafe<E>>::cpp_from(&__a0.1))
        },
        {
            let __a0 = (args_4).clone();
            ((*&__a0.0), <E as CppFromUnsafe<E>>::cpp_from(&__a0.1))
        },
    ];
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (i32, E) = &mut (*p);
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (i32, E) = &mut (*p);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub unsafe fn test_3(
    mut bad: libc::c_char,
    mut args_1: (libc::c_char, i32),
    mut args_2: (libc::c_char, i32),
    mut args_3: (libc::c_char, i32),
    mut args_4: (libc::c_char, i32),
) -> bool {
    let mut map: () = unimplemented!();
    let mut pairs: [(libc::c_char, i32); 4] = [
        {
            let __a0 = (args_1).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_2).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_3).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_4).clone();
            ((*&__a0.0), (*&__a0.1))
        },
    ];
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (libc::c_char, i32) = &mut (*p);
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (libc::c_char, i32) = &mut (*p);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub unsafe fn test_4(
    mut bad: libc::c_char,
    mut args_1: (libc::c_char, libc::c_char),
    mut args_2: (libc::c_char, libc::c_char),
    mut args_3: (libc::c_char, libc::c_char),
    mut args_4: (libc::c_char, libc::c_char),
) -> bool {
    let mut map: () = unimplemented!();
    let mut pairs: [(libc::c_char, libc::c_char); 4] = [
        {
            let __a0 = (args_1).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_2).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_3).clone();
            ((*&__a0.0), (*&__a0.1))
        },
        {
            let __a0 = (args_4).clone();
            ((*&__a0.0), (*&__a0.1))
        },
    ];
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (libc::c_char, libc::c_char) = &mut (*p);
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (libc::c_char, libc::c_char) = &mut (*p);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub unsafe fn test_5(
    mut bad: i32,
    mut args_1: (i32, E),
    mut args_2: (i32, E),
    mut args_3: (i32, E),
    mut args_4: (i32, E),
) -> bool {
    let mut map: () = unimplemented!();
    let mut pairs: [(i32, E); 4] = [
        {
            let __a0 = (args_1).clone();
            ((*&__a0.0), <E as CppFromUnsafe<E>>::cpp_from(&__a0.1))
        },
        {
            let __a0 = (args_2).clone();
            ((*&__a0.0), <E as CppFromUnsafe<E>>::cpp_from(&__a0.1))
        },
        {
            let __a0 = (args_3).clone();
            ((*&__a0.0), <E as CppFromUnsafe<E>>::cpp_from(&__a0.1))
        },
        {
            let __a0 = (args_4).clone();
            ((*&__a0.0), <E as CppFromUnsafe<E>>::cpp_from(&__a0.1))
        },
    ];
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (i32, E) = &mut (*p);
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        pairs.as_mut_ptr(),
        pairs.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let p: *mut (i32, E) = &mut (*p);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct E {
    pub a: i32,
    pub b: f64,
    pub c: libc::c_char,
}
impl Default for E {
    fn default() -> Self {
        unsafe {
            E {
                a: 1,
                b: 1_f64,
                c: (1 as libc::c_char),
            }
        }
    }
}
pub unsafe fn test_6() -> bool {
    {
        (unsafe {
            let _args_1: (libc::c_char, i32) = (('a' as libc::c_char), 10);
            let _args_2: (libc::c_char, i32) = (('b' as libc::c_char), 11);
            let _args_3: (libc::c_char, i32) = (('c' as libc::c_char), 12);
            let _args_4: (libc::c_char, i32) = (('d' as libc::c_char), 13);
            test_0(('e' as libc::c_char), _args_1, _args_2, _args_3, _args_4)
        });
        (unsafe {
            let _args_1: (libc::c_char, libc::c_char) =
                (('a' as libc::c_char), ('a' as libc::c_char));
            let _args_2: (libc::c_char, libc::c_char) =
                (('b' as libc::c_char), ('a' as libc::c_char));
            let _args_3: (libc::c_char, libc::c_char) =
                (('c' as libc::c_char), ('a' as libc::c_char));
            let _args_4: (libc::c_char, libc::c_char) =
                (('d' as libc::c_char), ('b' as libc::c_char));
            test_1(('e' as libc::c_char), _args_1, _args_2, _args_3, _args_4)
        });
        (unsafe {
            let _args_1: (i32, E) = (1, <E>::default());
            let _args_2: (i32, E) = (2, <E>::default());
            let _args_3: (i32, E) = (3, <E>::default());
            let _args_4: (i32, E) = (4, <E>::default());
            test_2(-1_i32, _args_1, _args_2, _args_3, _args_4)
        });
    }
    {
        (unsafe {
            let _args_1: (libc::c_char, i32) = (('a' as libc::c_char), 10);
            let _args_2: (libc::c_char, i32) = (('b' as libc::c_char), 11);
            let _args_3: (libc::c_char, i32) = (('c' as libc::c_char), 12);
            let _args_4: (libc::c_char, i32) = (('d' as libc::c_char), 13);
            test_3(('e' as libc::c_char), _args_1, _args_2, _args_3, _args_4)
        });
        (unsafe {
            let _args_1: (libc::c_char, libc::c_char) =
                (('a' as libc::c_char), ('a' as libc::c_char));
            let _args_2: (libc::c_char, libc::c_char) =
                (('b' as libc::c_char), ('a' as libc::c_char));
            let _args_3: (libc::c_char, libc::c_char) =
                (('c' as libc::c_char), ('a' as libc::c_char));
            let _args_4: (libc::c_char, libc::c_char) =
                (('d' as libc::c_char), ('b' as libc::c_char));
            test_4(('e' as libc::c_char), _args_1, _args_2, _args_3, _args_4)
        });
        (unsafe {
            let _args_1: (i32, E) = (1, <E>::default());
            let _args_2: (i32, E) = (2, <E>::default());
            let _args_3: (i32, E) = (3, <E>::default());
            let _args_4: (i32, E) = (4, <E>::default());
            test_5(-1_i32, _args_1, _args_2, _args_3, _args_4)
        });
    }
    return true;
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
    (unsafe { test_6() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
