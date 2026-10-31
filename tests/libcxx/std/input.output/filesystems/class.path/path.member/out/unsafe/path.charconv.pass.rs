extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_latin_unicode_0() {
    let u16str: [u16; 4] = [229_u16, 228_u16, 246_u16, 0_u16];
    let u32str: [u32; 4] = [229_u32, 228_u32, 246_u32, 0_u32];
    let str: [libc::c_char; 7] = [
        (-61 as libc::c_char),
        (-91 as libc::c_char),
        (-61 as libc::c_char),
        (-92 as libc::c_char),
        (-61 as libc::c_char),
        (-74 as libc::c_char),
        (0 as libc::c_char),
    ];
    let u8str: [u8; 7] = [195_u8, 165_u8, 195_u8, 164_u8, 195_u8, 182_u8, 0_u8];
    let wstr: [i32; 4] = [229_i32, 228_i32, 246_i32, 0_i32];
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u16str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u32str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u8str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u16str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u32str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u16str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = wstr
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(str.as_ptr(), libcc2rs::c_str_len(str.as_ptr()) + 1)
                    .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(str.as_ptr(), libcc2rs::c_str_len(str.as_ptr()) + 1)
                    .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u16str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(str.as_ptr(), libcc2rs::c_str_len(str.as_ptr()) + 1)
                    .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(str.as_ptr(), libcc2rs::c_str_len(str.as_ptr()) + 1)
                    .to_vec()
        );
    }
}
pub unsafe fn test_wide_unicode_1() {
    let u16str: [u16; 3] = [55297_u16, 56375_u16, 0_u16];
    let u32str: [u32; 2] = [66615_u32, 0_u32];
    let u8str: [u8; 5] = [240_u8, 144_u8, 144_u8, 183_u8, 0_u8];
    let str: [libc::c_char; 5] = [
        (-16 as libc::c_char),
        (-112 as libc::c_char),
        (-112 as libc::c_char),
        (-73 as libc::c_char),
        (0 as libc::c_char),
    ];
    {
        let p: Vec<libc::c_char> = unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u16str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u32str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    let wstr: [i32; 2] = [66615_i32, 0_i32];
    {
        let p: Vec<libc::c_char> = unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u16str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u32str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = wstr
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u8str.as_ptr(),
                    libcc2rs::c_str_len(u8str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u16str.as_ptr(),
                    libcc2rs::c_str_len(u16str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32str.as_ptr(),
                    libcc2rs::c_str_len(u32str.as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    wstr.as_ptr(),
                    libcc2rs::c_str_len(wstr.as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test_append_2() {
    let u16str: [u16; 3] = [55297_u16, 56375_u16, 0_u16];
    let u32str: [u32; 2] = [66615_u32, 0_u32];
    let u32ref: [u32; 6] = [
        66615_u32,
        (unimplemented!() as u32),
        66615_u32,
        (unimplemented!() as u32),
        66615_u32,
        0_u32,
    ];
    let str: [libc::c_char; 5] = [
        (-16 as libc::c_char),
        (-112 as libc::c_char),
        (-112 as libc::c_char),
        (-73 as libc::c_char),
        (0 as libc::c_char),
    ];
    {
        let mut p: Vec<libc::c_char> = unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    let wstr: [i32; 2] = [66615_i32, 0_i32];
    {
        let mut p: Vec<libc::c_char> = unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test_concat_3() {
    let u16str: [u16; 3] = [55297_u16, 56375_u16, 0_u16];
    let u32str: [u32; 2] = [66615_u32, 0_u32];
    let u32ref: [u32; 4] = [66615_u32, 66615_u32, 66615_u32, 0_u32];
    let str: [libc::c_char; 5] = [
        (-16 as libc::c_char),
        (-112 as libc::c_char),
        (-112 as libc::c_char),
        (-73 as libc::c_char),
        (0 as libc::c_char),
    ];
    {
        let mut p: Vec<libc::c_char> = unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    let wstr: [i32; 2] = [66615_i32, 0_i32];
    {
        let mut p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = u16str
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref.as_ptr(),
                    libcc2rs::c_str_len(u32ref.as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test_append_concat_narrow_4() {
    let u16str: [u16; 2] = [229_u16, 0_u16];
    let u32ref_append: [u32; 4] = [229_u32, (unimplemented!() as u32), 229_u32, 0_u32];
    let u32ref_concat: [u32; 3] = [229_u32, 229_u32, 0_u32];
    {
        let u8str: [u8; 3] = [195_u8, 165_u8, 0_u8];
        let mut p: Vec<libc::c_char> = unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_append.as_ptr(),
                    libcc2rs::c_str_len(u32ref_append.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_append.as_ptr(),
                    libcc2rs::c_str_len(u32ref_append.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_append.as_ptr(),
                    libcc2rs::c_str_len(u32ref_append.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_concat.as_ptr(),
                    libcc2rs::c_str_len(u32ref_concat.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_concat.as_ptr(),
                    libcc2rs::c_str_len(u32ref_concat.as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let str: [libc::c_char; 3] = [
            (-61 as libc::c_char),
            (-91 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut p: Vec<libc::c_char> = unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_append.as_ptr(),
                    libcc2rs::c_str_len(u32ref_append.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_append.as_ptr(),
                    libcc2rs::c_str_len(u32ref_append.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_append.as_ptr(),
                    libcc2rs::c_str_len(u32ref_append.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_concat.as_ptr(),
                    libcc2rs::c_str_len(u32ref_concat.as_ptr()) + 1
                )
                .to_vec()
        );
        unimplemented!();
        unimplemented!();
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    u32ref_concat.as_ptr(),
                    libcc2rs::c_str_len(u32ref_concat.as_ptr()) + 1
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
    (unsafe { test_latin_unicode_0() });
    (unsafe { test_wide_unicode_1() });
    (unsafe { test_append_2() });
    (unsafe { test_concat_3() });
    (unsafe { test_append_concat_narrow_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
