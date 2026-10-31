extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_octal_0(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::oct_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u16));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_octal_1(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::oct_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i16));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_octal_2(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::oct_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u32));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_octal_3(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::oct_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i32));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_octal_4(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::oct_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u64));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_octal_5(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::oct_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i64));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_octal_6(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::oct_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u64));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_octal_7(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::oct_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i64));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_dec_8(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::dec_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u16));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_dec_9(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::dec_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i16));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_dec_10(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::dec_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u32));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_dec_11(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::dec_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i32));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_dec_12(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::dec_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u64));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_dec_13(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::dec_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i64));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_dec_14(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::dec_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u64));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_dec_15(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::dec_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i64));
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_hex_16(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::hex_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u16));
        __s
    };
    let mut str: Vec<libc::c_char> = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (str.len() - 1)) {
        (*(&mut str).as_mut_ptr().add(i as usize)) = (if (0..=255)
            .contains(&((*(&mut str).as_mut_ptr().add(i as usize)) as i32))
        {
            (((*(&mut str).as_mut_ptr().add(i as usize)) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            ((*(&mut str).as_mut_ptr().add(i as usize)) as i32)
        } as libc::c_char);
        i.prefix_inc();
    }
    assert!(
        str == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_hex_17(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::hex_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i16));
        __s
    };
    let mut str: Vec<libc::c_char> = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (str.len() - 1)) {
        (*(&mut str).as_mut_ptr().add(i as usize)) = (if (0..=255)
            .contains(&((*(&mut str).as_mut_ptr().add(i as usize)) as i32))
        {
            (((*(&mut str).as_mut_ptr().add(i as usize)) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            ((*(&mut str).as_mut_ptr().add(i as usize)) as i32)
        } as libc::c_char);
        i.prefix_inc();
    }
    assert!(
        str == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_hex_18(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::hex_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u32));
        __s
    };
    let mut str: Vec<libc::c_char> = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (str.len() - 1)) {
        (*(&mut str).as_mut_ptr().add(i as usize)) = (if (0..=255)
            .contains(&((*(&mut str).as_mut_ptr().add(i as usize)) as i32))
        {
            (((*(&mut str).as_mut_ptr().add(i as usize)) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            ((*(&mut str).as_mut_ptr().add(i as usize)) as i32)
        } as libc::c_char);
        i.prefix_inc();
    }
    assert!(
        str == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_hex_19(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::hex_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i32));
        __s
    };
    let mut str: Vec<libc::c_char> = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (str.len() - 1)) {
        (*(&mut str).as_mut_ptr().add(i as usize)) = (if (0..=255)
            .contains(&((*(&mut str).as_mut_ptr().add(i as usize)) as i32))
        {
            (((*(&mut str).as_mut_ptr().add(i as usize)) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            ((*(&mut str).as_mut_ptr().add(i as usize)) as i32)
        } as libc::c_char);
        i.prefix_inc();
    }
    assert!(
        str == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_hex_20(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::hex_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u64));
        __s
    };
    let mut str: Vec<libc::c_char> = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (str.len() - 1)) {
        (*(&mut str).as_mut_ptr().add(i as usize)) = (if (0..=255)
            .contains(&((*(&mut str).as_mut_ptr().add(i as usize)) as i32))
        {
            (((*(&mut str).as_mut_ptr().add(i as usize)) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            ((*(&mut str).as_mut_ptr().add(i as usize)) as i32)
        } as libc::c_char);
        i.prefix_inc();
    }
    assert!(
        str == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_hex_21(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::hex_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i64));
        __s
    };
    let mut str: Vec<libc::c_char> = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (str.len() - 1)) {
        (*(&mut str).as_mut_ptr().add(i as usize)) = (if (0..=255)
            .contains(&((*(&mut str).as_mut_ptr().add(i as usize)) as i32))
        {
            (((*(&mut str).as_mut_ptr().add(i as usize)) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            ((*(&mut str).as_mut_ptr().add(i as usize)) as i32)
        } as libc::c_char);
        i.prefix_inc();
    }
    assert!(
        str == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_hex_22(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::hex_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as u64));
        __s
    };
    let mut str: Vec<libc::c_char> = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (str.len() - 1)) {
        (*(&mut str).as_mut_ptr().add(i as usize)) = (if (0..=255)
            .contains(&((*(&mut str).as_mut_ptr().add(i as usize)) as i32))
        {
            (((*(&mut str).as_mut_ptr().add(i as usize)) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            ((*(&mut str).as_mut_ptr().add(i as usize)) as i32)
        } as libc::c_char);
        i.prefix_inc();
    }
    assert!(
        str == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_hex_23(mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::hex_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, (-1_i32 as i64));
        __s
    };
    let mut str: Vec<libc::c_char> = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (str.len() - 1)) {
        (*(&mut str).as_mut_ptr().add(i as usize)) = (if (0..=255)
            .contains(&((*(&mut str).as_mut_ptr().add(i as usize)) as i32))
        {
            (((*(&mut str).as_mut_ptr().add(i as usize)) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            ((*(&mut str).as_mut_ptr().add(i as usize)) as i32)
        } as libc::c_char);
        i.prefix_inc();
    }
    assert!(
        str == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
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
    (unsafe { test_octal_0(c"177777".as_ptr()) });
    (unsafe { test_octal_1(c"177777".as_ptr()) });
    (unsafe { test_octal_2(c"37777777777".as_ptr()) });
    (unsafe { test_octal_3(c"37777777777".as_ptr()) });
    (unsafe { test_octal_4(c"1777777777777777777777".as_ptr()) });
    (unsafe { test_octal_5(c"1777777777777777777777".as_ptr()) });
    (unsafe { test_octal_4(c"1777777777777777777777".as_ptr()) });
    let long_is_64: bool = unimplemented!();
    let long_long_is_64: bool = unimplemented!();
    {
        (unsafe { test_octal_4(c"1777777777777777777777".as_ptr()) });
        (unsafe { test_octal_5(c"1777777777777777777777".as_ptr()) });
    }
    {
        (unsafe { test_octal_6(c"1777777777777777777777".as_ptr()) });
        (unsafe { test_octal_7(c"1777777777777777777777".as_ptr()) });
    }
    (unsafe { test_dec_8(c"65535".as_ptr()) });
    (unsafe { test_dec_9(c"-1".as_ptr()) });
    (unsafe { test_dec_10(c"4294967295".as_ptr()) });
    (unsafe { test_dec_11(c"-1".as_ptr()) });
    (unsafe { test_dec_12(c"18446744073709551615".as_ptr()) });
    (unsafe { test_dec_13(c"-1".as_ptr()) });
    {
        (unsafe { test_dec_12(c"18446744073709551615".as_ptr()) });
        (unsafe { test_dec_13(c"-1".as_ptr()) });
    }
    {
        (unsafe { test_dec_14(c"18446744073709551615".as_ptr()) });
        (unsafe { test_dec_15(c"-1".as_ptr()) });
    }
    (unsafe { test_hex_16(c"FFFF".as_ptr()) });
    (unsafe { test_hex_17(c"FFFF".as_ptr()) });
    (unsafe { test_hex_18(c"FFFFFFFF".as_ptr()) });
    (unsafe { test_hex_19(c"FFFFFFFF".as_ptr()) });
    (unsafe { test_hex_20(c"FFFFFFFFFFFFFFFF".as_ptr()) });
    (unsafe { test_hex_21(c"FFFFFFFFFFFFFFFF".as_ptr()) });
    {
        (unsafe { test_hex_20(c"FFFFFFFFFFFFFFFF".as_ptr()) });
        (unsafe { test_hex_21(c"FFFFFFFFFFFFFFFF".as_ptr()) });
    }
    {
        (unsafe { test_hex_22(c"FFFFFFFFFFFFFFFF".as_ptr()) });
        (unsafe { test_hex_23(c"FFFFFFFFFFFFFFFF".as_ptr()) });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
