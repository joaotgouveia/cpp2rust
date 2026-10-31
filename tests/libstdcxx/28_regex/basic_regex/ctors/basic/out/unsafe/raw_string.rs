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
pub struct __gnu_test_regex_expected_fail {}
pub static mut regex_error_internal_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((-1_i32) as ()) });
pub static mut regex_error_codes_1: std::cell::LazyCell<[*const libc::c_char; 13]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            c"error_collate".as_ptr(),
            c"error_ctype".as_ptr(),
            c"error_escape".as_ptr(),
            c"error_backref".as_ptr(),
            c"error_brack".as_ptr(),
            c"error_paren".as_ptr(),
            c"error_brace".as_ptr(),
            c"error_badbrace".as_ptr(),
            c"error_range".as_ptr(),
            c"error_space".as_ptr(),
            c"error_badrepeat".as_ptr(),
            c"error_complexity".as_ptr(),
            c"error_stack".as_ptr(),
        ]
    });
pub unsafe fn show_regex_error_codes_2() {
    let tab: libc::c_char = ('\t' as libc::c_char);
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_collate =   ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_ctype =     ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_escape =    ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_backref =   ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_brack =     ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_paren =     ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_brace =     ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_badbrace =  ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_range =     ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_space =     ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_badrepeat = ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_complexity =".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(
                        ::std::ffi::CStr::from_ptr(c"error_stack =     ".as_ptr()).to_bytes(),
                    );
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[tab as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i32));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
}
pub unsafe fn regex_sanity_check_3(
    __res: *const Vec<libc::c_char>,
    mut __f: Option<()>,
    mut __error: Option<()>,
) {
    let mut __f: () = __f.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut __error: () = __error.unwrap_or_else(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut regex_error_internal_0))
    });
    let mut reo: () = unimplemented!();
    let mut n: u32 = unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(
                    ::std::ffi::CStr::from_ptr(c"regex_type::mark_count ".as_ptr()).to_bytes(),
                );
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, n);
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
}
pub unsafe fn test01_4() {
    let mut sre0: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"\\d{3}-\\d{3}-\\d{4}".as_ptr(),
        libcc2rs::c_str_len(c"\\d{3}-\\d{3}-\\d{4}".as_ptr()) + 1,
    )
    .to_vec();
    let mut sre1: Vec<libc::c_char> = std::slice::from_raw_parts(
        c" this\n  and new : forms\n  ".as_ptr(),
        libcc2rs::c_str_len(c" this\n  and new : forms\n  ".as_ptr()) + 1,
    )
    .to_vec();
    let mut sre2: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"[:num:]{3}-[:num:]{3}-[:num:]{4}".as_ptr(),
        libcc2rs::c_str_len(c"[:num:]{3}-[:num:]{3}-[:num:]{4}".as_ptr()) + 1,
    )
    .to_vec();
    let mut re0: () = unimplemented!();
    let mut re1: () = unimplemented!();
    let mut re2: () = unimplemented!();
    (unsafe { regex_sanity_check_3(&sre0, Some(unimplemented!()), None) });
    (unsafe { regex_sanity_check_3(&sre1, None, None) });
    (unsafe { regex_sanity_check_3(&sre2, None, None) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const regex_error_internal_0);
    std::cell::LazyCell::force(&*&raw const regex_error_codes_1);
}
