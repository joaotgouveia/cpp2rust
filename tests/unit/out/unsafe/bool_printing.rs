extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0() -> bool {
    return true;
}
pub unsafe fn bar_1() -> bool {
    return true;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 0;
    let mut i2: i32 = 1;
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_bool(&mut *__s, true);
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
        __s
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_bool(&mut *__s, false);
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
        __s
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_bool(&mut *__s, ((i1) != (i2)));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
        __s
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_bool(&mut *__s, ((i1) == (i2)));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
        __s
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_bool(&mut *__s, (unsafe { foo_0() }));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
        __s
    };
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_bool(&mut *__s, (unsafe { bar_1() }));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(&[('\n' as libc::c_char) as u8]);
        __s
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
