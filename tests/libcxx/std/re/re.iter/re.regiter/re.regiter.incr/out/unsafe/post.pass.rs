extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn validate_prefixes_0(empty_matching_pattern: *const ()) {
    let source: [libc::c_char; 4] = std::mem::transmute(*b"abc\0");
    let mut i: () = unimplemented!();
    assert!(!((*unimplemented!()).matched));
    assert!(((unimplemented!()) == (0_())));
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).0)
            == (source.as_ptr()))
    );
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).1)
            == (source.as_ptr()))
    );
    unimplemented!();
    assert!((*unimplemented!()).matched);
    assert!(((unimplemented!()) == (1_())));
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).0)
            == (source.as_ptr()))
    );
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).1)
            == (source.as_ptr().offset((1) as isize)))
    );
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(c"a".as_ptr(), libcc2rs::c_str_len(c"a".as_ptr()) + 1)
                .to_vec()
    );
    unimplemented!();
    assert!((*unimplemented!()).matched);
    assert!(((unimplemented!()) == (1_())));
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).0)
            == (source.as_ptr().offset((1) as isize)))
    );
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).1)
            == (source.as_ptr().offset((2) as isize)))
    );
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(c"b".as_ptr(), libcc2rs::c_str_len(c"b".as_ptr()) + 1)
                .to_vec()
    );
    unimplemented!();
    assert!((*unimplemented!()).matched);
    assert!(((unimplemented!()) == (1_())));
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).0)
            == (source.as_ptr().offset((2) as isize)))
    );
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).1)
            == (source.as_ptr().offset((3) as isize)))
    );
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(c"c".as_ptr(), libcc2rs::c_str_len(c"c".as_ptr()) + 1)
                .to_vec()
    );
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_prefix_adjustment_1() {
    (unsafe {
        let mut _empty_matching_pattern: () = unimplemented!();
        validate_prefixes_0(&mut _empty_matching_pattern)
    });
    (unsafe {
        let mut _empty_matching_pattern: () = unimplemented!();
        validate_prefixes_0(&mut _empty_matching_pattern)
    });
    (unsafe {
        let mut _empty_matching_pattern: () = unimplemented!();
        validate_prefixes_0(&mut _empty_matching_pattern)
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
        let mut phone_numbers: () = unimplemented!();
        let phone_book: [libc::c_char; 29] =
            std::mem::transmute(*b"555-1234, 555-2345, 555-3456\0");
        let mut i: () = unimplemented!();
        let mut i2: () = unimplemented!();
        assert!(i != unimplemented!());
        assert!(i2 != unimplemented!());
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(i != unimplemented!());
        assert!(i2 != unimplemented!());
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (10_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-2345".as_ptr(),
                    libcc2rs::c_str_len(c"555-2345".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(i != unimplemented!());
        assert!(i2 != unimplemented!());
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (20_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-3456".as_ptr(),
                    libcc2rs::c_str_len(c"555-3456".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(unimplemented!());
        assert!(i2 != unimplemented!());
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut phone_numbers: () = unimplemented!();
        let phone_book: [libc::c_char; 29] =
            std::mem::transmute(*b"555-1234, 555-2345, 555-3456\0");
        let mut i: () = unimplemented!();
        let mut i2: () = unimplemented!();
        assert!(i != unimplemented!());
        assert!(i2 != unimplemented!());
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(i != unimplemented!());
        assert!(i2 != unimplemented!());
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (10_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-2345".as_ptr(),
                    libcc2rs::c_str_len(c"555-2345".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(i != unimplemented!());
        assert!(i2 != unimplemented!());
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (20_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-3456".as_ptr(),
                    libcc2rs::c_str_len(c"555-3456".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(unimplemented!());
        assert!(i2 != unimplemented!());
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (0_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"555-1234".as_ptr(),
                    libcc2rs::c_str_len(c"555-1234".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut rex: () = unimplemented!();
        let foo: [libc::c_char; 4] = std::mem::transmute(*b"foo\0");
        let mut i: () = unimplemented!();
        let mut e: () = unimplemented!();
        assert!(i != e);
        assert!(((unimplemented!()) == (1_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"foo".as_ptr(),
                    libcc2rs::c_str_len(c"foo".as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(i != e);
        assert!(((unimplemented!()) == (1_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(c"".as_ptr(), libcc2rs::c_str_len(c"".as_ptr()) + 1)
                    .to_vec()
        );
        unimplemented!();
        assert!(unimplemented!());
    }
    (unsafe { test_prefix_adjustment_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
