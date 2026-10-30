extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vec_: Vec<u8> = Vec::from([195_u8, 167_u8]);
    let mut i: i32 = 27;
    let mut str: Vec<libc::c_char> =
        std::slice::from_raw_parts(c"foo.".as_ptr(), libcc2rs::c_str_len(c"foo.".as_ptr()) + 1)
            .to_vec();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*{
                        let __r: &mut ::std::fs::File = &mut (*{
                            let __r: &mut ::std::fs::File = &mut (*{
                                let __r: &mut ::std::fs::File = &mut (*{
                                    let __r: &mut ::std::fs::File = &mut (*{
                                        let __r: &mut ::std::fs::File = &mut (*{
                                            let __r: &mut ::std::fs::File = &mut (*{
                                                let __r: &mut ::std::fs::File = &mut (*{
                                                    let __r: &mut ::std::fs::File = &mut (*{
                                                        let __r: &mut ::std::fs::File = &mut (*{
                                                            let __r: &mut ::std::fs::File =
                                                                &mut (*{
                                                                    let __r: &mut ::std::fs::File =
                                                                        &mut (*{
                                                                            let __r: &mut ::std::fs::File = &mut ( * {let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe()) ;
    let __s: *mut ::std::fs::File = __r;
    libcc2rs::write_int(&mut *__s, i );
    __s}  )  ;
                                                                            let __s: *mut ::std::fs::File = __r;
                                                                            let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c" a" .as_ptr() ).to_bytes());
                                                                            __s
                                                                        });
                                                                    let __s: *mut ::std::fs::File =
                                                                        __r;
                                                                    let _ = (*__s).write_all(&[
                                                                        (*(&mut vec_)
                                                                            .as_mut_ptr()
                                                                            .add(0_usize as usize)),
                                                                    ]);
                                                                    __s
                                                                });
                                                            let __s: *mut ::std::fs::File = __r;
                                                            let _ =
                                                                (*__s).write_all(&[(*(&mut vec_)
                                                                    .as_mut_ptr()
                                                                    .add(1_usize as usize))]);
                                                            __s
                                                        });
                                                        let __s: *mut ::std::fs::File = __r;
                                                        let _ = (*__s).write_all(&[('o'
                                                            as libc::c_char)
                                                            as u8]);
                                                        __s
                                                    });
                                                    let __s: *mut ::std::fs::File = __r;
                                                    let mut __bytes = Vec::new();
                                                    for &__c in &str[..str.len().saturating_sub(1)]
                                                    {
                                                        __c.push_stream_bytes(&mut __bytes);
                                                    }
                                                    let _ = ::std::io::Write::write_all(
                                                        &mut *__s, &__bytes,
                                                    );
                                                    __s
                                                });
                                                let __s: *mut ::std::fs::File = __r;
                                                (libcc2rs::endl_unsafe
                                                    as unsafe fn(
                                                        *mut ::std::fs::File,
                                                    )
                                                        -> *mut ::std::fs::File)(
                                                    __s
                                                )
                                            });
                                            let __s: *mut ::std::fs::File = __r;
                                            (libcc2rs::hex_unsafe
                                                as unsafe fn(
                                                    *mut ::std::fs::File,
                                                )
                                                    -> *mut ::std::fs::File)(
                                                __s
                                            )
                                        });
                                        let __s: *mut ::std::fs::File = __r;
                                        let _ = (*__s).write_all(
                                            ::std::ffi::CStr::from_ptr(c"0x".as_ptr()).to_bytes(),
                                        );
                                        __s
                                    });
                                    let __s: *mut ::std::fs::File = __r;
                                    libcc2rs::write_int(&mut *__s, 27);
                                    __s
                                });
                                let __s: *mut ::std::fs::File = __r;
                                let _ = (*__s).write_all(
                                    ::std::ffi::CStr::from_ptr(c" a\xc3\xa7ordas?".as_ptr())
                                        .to_bytes(),
                                );
                                __s
                            });
                            let __s: *mut ::std::fs::File = __r;
                            let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
                            __s
                        });
                        let __s: *mut ::std::fs::File = __r;
                        let _ = (*__s)
                            .write_all(::std::ffi::CStr::from_ptr(c"Sim, 0x".as_ptr()).to_bytes());
                        __s
                    });
                    let __s: *mut ::std::fs::File = __r;
                    libcc2rs::write_int(&mut *__s, i);
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[('.' as libc::c_char) as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::dec_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*{
                        let __r: &mut ::std::fs::File = &mut (*{
                            let __r: &mut ::std::fs::File = &mut (*{
                                let __r: &mut ::std::fs::File = &mut (*{
                                    let __r: &mut ::std::fs::File = &mut (*{
                                        let __r: &mut ::std::fs::File = &mut (*{
                                            let __r: &mut ::std::fs::File = &mut (*{
                                                let __r: &mut ::std::fs::File = &mut (*{
                                                    let __r: &mut ::std::fs::File = &mut (*{
                                                        let __r: &mut ::std::fs::File = &mut (*{
                                                            let __r: &mut ::std::fs::File =
                                                                &mut (*libcc2rs::cout_unsafe());
                                                            let __s: *mut ::std::fs::File = __r;
                                                            let _ = (*__s).write_all(&[('H'
                                                                as libc::c_char)
                                                                as u8]);
                                                            __s
                                                        });
                                                        let __s: *mut ::std::fs::File = __r;
                                                        let _ = (*__s).write_all(&[('e'
                                                            as libc::c_char)
                                                            as u8]);
                                                        __s
                                                    });
                                                    let __s: *mut ::std::fs::File = __r;
                                                    let _ = (*__s)
                                                        .write_all(&[('l' as libc::c_char) as u8]);
                                                    __s
                                                });
                                                let __s: *mut ::std::fs::File = __r;
                                                let _ = (*__s)
                                                    .write_all(&[('l' as libc::c_char) as u8]);
                                                __s
                                            });
                                            let __s: *mut ::std::fs::File = __r;
                                            let _ =
                                                (*__s).write_all(&[('o' as libc::c_char) as u8]);
                                            __s
                                        });
                                        let __s: *mut ::std::fs::File = __r;
                                        let _ = (*__s).write_all(&[(',' as libc::c_char) as u8]);
                                        __s
                                    });
                                    let __s: *mut ::std::fs::File = __r;
                                    let _ = (*__s).write_all(&[(' ' as libc::c_char) as u8]);
                                    __s
                                });
                                let __s: *mut ::std::fs::File = __r;
                                let _ = (*__s).write_all(&[('W' as libc::c_char) as u8]);
                                __s
                            });
                            let __s: *mut ::std::fs::File = __r;
                            let _ = (*__s).write_all(&[('o' as libc::c_char) as u8]);
                            __s
                        });
                        let __s: *mut ::std::fs::File = __r;
                        let _ = (*__s).write_all(&[('r' as libc::c_char) as u8]);
                        __s
                    });
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(&[('l' as libc::c_char) as u8]);
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[('d' as libc::c_char) as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            let _ = (*__s).write_all(&[('!' as libc::c_char) as u8]);
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
        __s
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(&[(*(&mut vec_).as_mut_ptr().add(0_usize as usize))]);
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            let _ = (*__s).write_all(&[(*(&mut vec_).as_mut_ptr().add(1_usize as usize))]);
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
        __s
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
