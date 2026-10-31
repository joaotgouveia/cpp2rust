extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_skipws_0(is: *const ()) -> bool {
    return (((unimplemented!()) as i32) != (0));
}
pub unsafe fn is_skipws_1(is: *const ()) -> bool {
    return (((unimplemented!()) as i32) != (0));
}
pub unsafe fn both_ways_2(mut p: *const libc::c_char) {
    let mut str: Vec<libc::c_char> =
        std::slice::from_raw_parts(p, libcc2rs::c_str_len(p) + 1).to_vec();
    let mut q: () = unimplemented!();
    let mut ss: () = unimplemented!();
    let mut skippingws: bool = (unsafe { is_skipws_0(&ss) });
    (&(skippingws));
    unimplemented!();
    unimplemented!();
}
pub unsafe fn round_trip_3(mut p: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    let mut skippingws: bool = (unsafe { is_skipws_0(&ss) });
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    assert!(s == std::slice::from_raw_parts(p, libcc2rs::c_str_len(p) + 1).to_vec());
    assert!(((skippingws as i32) == ((unsafe { is_skipws_0(&ss,) }) as i32)));
}
pub unsafe fn round_trip_4(mut p: *const i32) {
    let mut ss: () = unimplemented!();
    let mut skippingws: bool = (unsafe { is_skipws_1(&ss) });
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(s == std::slice::from_raw_parts(p, libcc2rs::c_str_len(p) + 1).to_vec());
    assert!(((skippingws as i32) == ((unsafe { is_skipws_1(&ss,) }) as i32)));
}
pub unsafe fn round_trip_ws_5(mut p: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut ss;
        libcc2rs::noskipws_unsafe(__r)
    };
    let mut skippingws: bool = (unsafe { is_skipws_0(&ss) });
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    assert!(s == std::slice::from_raw_parts(p, libcc2rs::c_str_len(p) + 1).to_vec());
    assert!(((skippingws as i32) == ((unsafe { is_skipws_0(&ss,) }) as i32)));
}
pub unsafe fn round_trip_ws_6(mut p: *const i32) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut ss;
        libcc2rs::noskipws_unsafe(__r)
    };
    let mut skippingws: bool = (unsafe { is_skipws_1(&ss) });
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(s == std::slice::from_raw_parts(p, libcc2rs::c_str_len(p) + 1).to_vec());
    assert!(((skippingws as i32) == ((unsafe { is_skipws_1(&ss,) }) as i32)));
}
pub unsafe fn round_trip_d_7(mut p: *const libc::c_char, mut delim: libc::c_char) {
    let mut ss: () = unimplemented!();
    let mut d: libc::c_char = delim;
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    assert!(s == std::slice::from_raw_parts(p, libcc2rs::c_str_len(p) + 1).to_vec());
}
pub unsafe fn round_trip_d_8(mut p: *const i32, mut delim: libc::c_char) {
    let mut ss: () = unimplemented!();
    let mut d: i32 = (delim as i32);
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(s == std::slice::from_raw_parts(p, libcc2rs::c_str_len(p) + 1).to_vec());
}
pub unsafe fn round_trip_e_9(mut p: *const libc::c_char, mut escape: libc::c_char) {
    let mut ss: () = unimplemented!();
    let mut e: libc::c_char = escape;
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    assert!(s == std::slice::from_raw_parts(p, libcc2rs::c_str_len(p) + 1).to_vec());
}
pub unsafe fn round_trip_e_10(mut p: *const i32, mut escape: libc::c_char) {
    let mut ss: () = unimplemented!();
    let mut e: i32 = (escape as i32);
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(s == std::slice::from_raw_parts(p, libcc2rs::c_str_len(p) + 1).to_vec());
}
pub unsafe fn quote_11(
    mut p: *const libc::c_char,
    mut delim: Option<libc::c_char>,
    mut escape: Option<libc::c_char>,
) -> Vec<libc::c_char> {
    let mut delim: libc::c_char = delim.unwrap_or_else(|| unsafe { ('\"' as libc::c_char) });
    let mut escape: libc::c_char = escape.unwrap_or_else(|| unsafe { ('\\' as libc::c_char) });
    let mut ss: () = unimplemented!();
    let mut d: libc::c_char = delim;
    let mut e: libc::c_char = escape;
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub unsafe fn quote_12(
    mut p: *const i32,
    mut delim: Option<libc::c_char>,
    mut escape: Option<libc::c_char>,
) -> Vec<i32> {
    let mut delim: libc::c_char = delim.unwrap_or_else(|| unsafe { ('\"' as libc::c_char) });
    let mut escape: libc::c_char = escape.unwrap_or_else(|| unsafe { ('\\' as libc::c_char) });
    let mut ss: () = unimplemented!();
    let mut d: i32 = (delim as i32);
    let mut e: i32 = (escape as i32);
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub unsafe fn unquote_13(
    mut p: *const libc::c_char,
    mut delim: Option<libc::c_char>,
    mut escape: Option<libc::c_char>,
) -> Vec<libc::c_char> {
    let mut delim: libc::c_char = delim.unwrap_or_else(|| unsafe { ('\"' as libc::c_char) });
    let mut escape: libc::c_char = escape.unwrap_or_else(|| unsafe { ('\\' as libc::c_char) });
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut ss;
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(p).to_bytes());
        __s
    };
    let mut d: libc::c_char = delim;
    let mut e: libc::c_char = escape;
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub unsafe fn unquote_14(
    mut p: *const i32,
    mut delim: Option<libc::c_char>,
    mut escape: Option<libc::c_char>,
) -> Vec<i32> {
    let mut delim: libc::c_char = delim.unwrap_or_else(|| unsafe { ('\"' as libc::c_char) });
    let mut escape: libc::c_char = escape.unwrap_or_else(|| unsafe { ('\\' as libc::c_char) });
    let mut ss: () = unimplemented!();
    unimplemented!();
    let mut d: i32 = (delim as i32);
    let mut e: i32 = (escape as i32);
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub unsafe fn test_padding_15() {
    {
        let mut ss: () = unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"`abc`!!!!!".as_ptr(),
                    libcc2rs::c_str_len(c"`abc`!!!!!".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut ss: () = unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"!!!!!`abc`".as_ptr(),
                    libcc2rs::c_str_len(c"!!!!!`abc`".as_ptr()) + 1
                )
                .to_vec()
        );
    }
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
    (unsafe { both_ways_2(c"".as_ptr()) });
    (unsafe { round_trip_3(c"".as_ptr()) });
    (unsafe { round_trip_ws_5(c"".as_ptr()) });
    (unsafe { round_trip_d_7(c"".as_ptr(), ('q' as libc::c_char)) });
    (unsafe { round_trip_e_9(c"".as_ptr(), ('q' as libc::c_char)) });
    (unsafe { round_trip_4([0 as i32].as_ptr()) });
    (unsafe { round_trip_ws_6([0 as i32].as_ptr()) });
    (unsafe { round_trip_d_8([0 as i32].as_ptr(), ('q' as libc::c_char)) });
    (unsafe { round_trip_e_10([0 as i32].as_ptr(), ('q' as libc::c_char)) });
    (unsafe { round_trip_3(c"Hi".as_ptr()) });
    (unsafe { round_trip_ws_5(c"Hi".as_ptr()) });
    (unsafe { round_trip_d_7(c"Hi".as_ptr(), ('!' as libc::c_char)) });
    (unsafe { round_trip_e_9(c"Hi".as_ptr(), ('!' as libc::c_char)) });
    assert!(
        (unsafe { quote_11(c"Hi".as_ptr(), Some(('!' as libc::c_char)), None,) })
            == std::slice::from_raw_parts(
                c"!Hi!".as_ptr(),
                libcc2rs::c_str_len(c"!Hi!".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { quote_11(c"Hi!".as_ptr(), Some(('!' as libc::c_char)), None,) })
            == std::slice::from_raw_parts(
                c"!Hi\\!!".as_ptr(),
                libcc2rs::c_str_len(c"!Hi\\!!".as_ptr()) + 1
            )
            .to_vec()
    );
    (unsafe { round_trip_4([72 as i32, 105 as i32, 0 as i32].as_ptr()) });
    (unsafe { round_trip_ws_6([72 as i32, 105 as i32, 0 as i32].as_ptr()) });
    (unsafe {
        round_trip_d_8(
            [72 as i32, 105 as i32, 0 as i32].as_ptr(),
            ('!' as libc::c_char),
        )
    });
    (unsafe {
        round_trip_e_10(
            [72 as i32, 105 as i32, 0 as i32].as_ptr(),
            ('!' as libc::c_char),
        )
    });
    assert!(
        (unsafe {
            quote_12(
                [72 as i32, 105 as i32, 0 as i32].as_ptr(),
                Some(('!' as libc::c_char)),
                None,
            )
        }) == std::slice::from_raw_parts(
            [33 as i32, 72 as i32, 105 as i32, 33 as i32, 0 as i32,].as_ptr(),
            libcc2rs::c_str_len([33 as i32, 72 as i32, 105 as i32, 33 as i32, 0 as i32,].as_ptr())
                + 1
        )
        .to_vec()
    );
    assert!(
        (unsafe {
            quote_12(
                [72 as i32, 105 as i32, 33 as i32, 0 as i32].as_ptr(),
                Some(('!' as libc::c_char)),
                None,
            )
        }) == std::slice::from_raw_parts(
            [
                33 as i32, 72 as i32, 105 as i32, 92 as i32, 33 as i32, 33 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    33 as i32, 72 as i32, 105 as i32, 92 as i32, 33 as i32, 33 as i32, 0 as i32,
                ]
                .as_ptr()
            ) + 1
        )
        .to_vec()
    );
    (unsafe { round_trip_3(c"Hi Mom".as_ptr()) });
    (unsafe { round_trip_ws_5(c"Hi Mom".as_ptr()) });
    (unsafe {
        round_trip_4(
            [
                72 as i32, 105 as i32, 32 as i32, 77 as i32, 111 as i32, 109 as i32, 0 as i32,
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        round_trip_ws_6(
            [
                72 as i32, 105 as i32, 32 as i32, 77 as i32, 111 as i32, 109 as i32, 0 as i32,
            ]
            .as_ptr(),
        )
    });
    assert!(
        (unsafe { quote_11(c"".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                c"\"\"".as_ptr(),
                libcc2rs::c_str_len(c"\"\"".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { quote_11(c"a".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                c"\"a\"".as_ptr(),
                libcc2rs::c_str_len(c"\"a\"".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { quote_12([0 as i32,].as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                [34 as i32, 34 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([34 as i32, 34 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { quote_12([97 as i32, 0 as i32,].as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                [34 as i32, 97 as i32, 34 as i32, 0 as i32,].as_ptr(),
                libcc2rs::c_str_len([34 as i32, 97 as i32, 34 as i32, 0 as i32,].as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { unquote_13(c"\"abc".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe {
            unquote_14(
                [34 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32].as_ptr(),
                None,
                None,
            )
        }) == std::slice::from_raw_parts(
            [97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr(),
            libcc2rs::c_str_len([97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr()) + 1
        )
        .to_vec()
    );
    assert!(
        (unsafe { unquote_13(c"abc".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { unquote_13(c"abc def".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe {
            unquote_14(
                [97 as i32, 98 as i32, 99 as i32, 0 as i32].as_ptr(),
                None,
                None,
            )
        }) == std::slice::from_raw_parts(
            [97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr(),
            libcc2rs::c_str_len([97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr()) + 1
        )
        .to_vec()
    );
    assert!(
        (unsafe {
            unquote_14(
                [
                    97 as i32, 98 as i32, 99 as i32, 32 as i32, 100 as i32, 101 as i32, 102 as i32,
                    0 as i32,
                ]
                .as_ptr(),
                None,
                None,
            )
        }) == std::slice::from_raw_parts(
            [97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr(),
            libcc2rs::c_str_len([97 as i32, 98 as i32, 99 as i32, 0 as i32,].as_ptr()) + 1
        )
        .to_vec()
    );
    assert!(
        (unsafe { unquote_13(c"".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(c"".as_ptr(), libcc2rs::c_str_len(c"".as_ptr()) + 1)
                .to_vec()
    );
    assert!(
        (unsafe { unquote_14([0 as i32,].as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                [0 as i32,].as_ptr(),
                libcc2rs::c_str_len([0 as i32,].as_ptr()) + 1
            )
            .to_vec()
    );
    (unsafe { test_padding_15() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
