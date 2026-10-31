extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_skipws_0(mut is: *const ()) -> bool {
    return (((unimplemented!()) as i32) != (0));
}
pub unsafe fn is_skipws_1(mut is: *const ()) -> bool {
    return (((unimplemented!()) as i32) != (0));
}
pub unsafe fn round_trip_2(mut p: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    let mut skippingws: bool = (unsafe { is_skipws_0((&mut ss as *mut ())) });
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    assert!(unimplemented!());
    assert!(((skippingws as i32) == ((unsafe { is_skipws_0((&mut ss as *mut ()),) }) as i32)));
}
pub unsafe fn round_trip_ws_3(mut p: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut ss;
        libcc2rs::noskipws_unsafe(__r)
    };
    let mut skippingws: bool = (unsafe { is_skipws_0((&mut ss as *mut ())) });
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    assert!(unimplemented!());
    assert!(((skippingws as i32) == ((unsafe { is_skipws_0((&mut ss as *mut ()),) }) as i32)));
}
pub unsafe fn round_trip_d_4(mut p: *const libc::c_char, mut delim: libc::c_char) {
    let mut ss: () = unimplemented!();
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn round_trip_e_5(mut p: *const libc::c_char, mut escape: libc::c_char) {
    let mut ss: () = unimplemented!();
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn quote_6(
    mut p: *const libc::c_char,
    mut delim: Option<libc::c_char>,
    mut escape: Option<libc::c_char>,
) -> Vec<libc::c_char> {
    let mut delim: libc::c_char = delim.unwrap_or_else(|| unsafe { ('\"' as libc::c_char) });
    let mut escape: libc::c_char = escape.unwrap_or_else(|| unsafe { ('\\' as libc::c_char) });
    let mut ss: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub unsafe fn unquote_7(
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
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub unsafe fn round_trip_8(mut p: *const i32) {
    let mut ss: () = unimplemented!();
    let mut skippingws: bool = (unsafe { is_skipws_1((&mut ss as *mut ())) });
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(unimplemented!());
    assert!(((skippingws as i32) == ((unsafe { is_skipws_1((&mut ss as *mut ()),) }) as i32)));
}
pub unsafe fn round_trip_ws_9(mut p: *const i32) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut ss;
        libcc2rs::noskipws_unsafe(__r)
    };
    let mut skippingws: bool = (unsafe { is_skipws_1((&mut ss as *mut ())) });
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(unimplemented!());
    assert!(((skippingws as i32) == ((unsafe { is_skipws_1((&mut ss as *mut ()),) }) as i32)));
}
pub unsafe fn round_trip_d_10(mut p: *const i32, mut delim: i32) {
    let mut ss: () = unimplemented!();
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn round_trip_e_11(mut p: *const i32, mut escape: i32) {
    let mut ss: () = unimplemented!();
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn quote_12(
    mut p: *const i32,
    mut delim: Option<i32>,
    mut escape: Option<i32>,
) -> Vec<i32> {
    let mut delim: i32 = delim.unwrap_or_else(|| unsafe { (('\"' as libc::c_char) as i32) });
    let mut escape: i32 = escape.unwrap_or_else(|| unsafe { (('\\' as libc::c_char) as i32) });
    let mut ss: () = unimplemented!();
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub unsafe fn unquote_13(
    mut p: *const i32,
    mut delim: Option<i32>,
    mut escape: Option<i32>,
) -> Vec<i32> {
    let mut delim: i32 = delim.unwrap_or_else(|| unsafe { (('\"' as libc::c_char) as i32) });
    let mut escape: i32 = escape.unwrap_or_else(|| unsafe { (('\\' as libc::c_char) as i32) });
    let mut ss: () = unimplemented!();
    let mut sv: () = unimplemented!();
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    return std::mem::take(&mut s);
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
    (unsafe { round_trip_2(c"".as_ptr()) });
    (unsafe { round_trip_ws_3(c"".as_ptr()) });
    (unsafe { round_trip_d_4(c"".as_ptr(), ('q' as libc::c_char)) });
    (unsafe { round_trip_e_5(c"".as_ptr(), ('q' as libc::c_char)) });
    (unsafe { round_trip_8([0 as i32].as_ptr()) });
    (unsafe { round_trip_ws_9([0 as i32].as_ptr()) });
    (unsafe { round_trip_d_10([0 as i32].as_ptr(), (('q' as libc::c_char) as i32)) });
    (unsafe { round_trip_e_11([0 as i32].as_ptr(), (('q' as libc::c_char) as i32)) });
    (unsafe { round_trip_2(c"Hi".as_ptr()) });
    (unsafe { round_trip_ws_3(c"Hi".as_ptr()) });
    (unsafe { round_trip_d_4(c"Hi".as_ptr(), ('!' as libc::c_char)) });
    (unsafe { round_trip_e_5(c"Hi".as_ptr(), ('!' as libc::c_char)) });
    assert!(
        (unsafe { quote_6(c"Hi".as_ptr(), Some(('!' as libc::c_char)), None,) })
            == std::slice::from_raw_parts(
                c"!Hi!".as_ptr(),
                libcc2rs::c_str_len(c"!Hi!".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { quote_6(c"Hi!".as_ptr(), Some(('!' as libc::c_char)), None,) })
            == std::slice::from_raw_parts(
                c"!Hi\\!!".as_ptr(),
                libcc2rs::c_str_len(c"!Hi\\!!".as_ptr()) + 1
            )
            .to_vec()
    );
    (unsafe { round_trip_8([72 as i32, 105 as i32, 0 as i32].as_ptr()) });
    (unsafe { round_trip_ws_9([72 as i32, 105 as i32, 0 as i32].as_ptr()) });
    (unsafe {
        round_trip_d_10(
            [72 as i32, 105 as i32, 0 as i32].as_ptr(),
            (('!' as libc::c_char) as i32),
        )
    });
    (unsafe {
        round_trip_e_11(
            [72 as i32, 105 as i32, 0 as i32].as_ptr(),
            (('!' as libc::c_char) as i32),
        )
    });
    assert!(
        (unsafe {
            quote_12(
                [72 as i32, 105 as i32, 0 as i32].as_ptr(),
                Some((('!' as libc::c_char) as i32)),
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
                Some((('!' as libc::c_char) as i32)),
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
    (unsafe { round_trip_2(c"Hi Mom".as_ptr()) });
    (unsafe { round_trip_ws_3(c"Hi Mom".as_ptr()) });
    (unsafe {
        round_trip_8(
            [
                72 as i32, 105 as i32, 32 as i32, 77 as i32, 111 as i32, 109 as i32, 0 as i32,
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        round_trip_ws_9(
            [
                72 as i32, 105 as i32, 32 as i32, 77 as i32, 111 as i32, 109 as i32, 0 as i32,
            ]
            .as_ptr(),
        )
    });
    assert!(
        (unsafe { quote_6(c"".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                c"\"\"".as_ptr(),
                libcc2rs::c_str_len(c"\"\"".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { quote_6(c"a".as_ptr(), None, None,) })
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
        (unsafe { unquote_7(c"\"abc".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { unquote_7(c"abc".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { unquote_7(c"abc def".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                c"abc".as_ptr(),
                libcc2rs::c_str_len(c"abc".as_ptr()) + 1
            )
            .to_vec()
    );
    assert!(
        (unsafe { unquote_7(c"".as_ptr(), None, None,) })
            == std::slice::from_raw_parts(c"".as_ptr(), libcc2rs::c_str_len(c"".as_ptr()) + 1)
                .to_vec()
    );
    assert!(
        (unsafe {
            unquote_13(
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
        (unsafe {
            unquote_13(
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
            unquote_13(
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
        (unsafe { unquote_13([0 as i32,].as_ptr(), None, None,) })
            == std::slice::from_raw_parts(
                [0 as i32,].as_ptr(),
                libcc2rs::c_str_len([0 as i32,].as_ptr()) + 1
            )
            .to_vec()
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
