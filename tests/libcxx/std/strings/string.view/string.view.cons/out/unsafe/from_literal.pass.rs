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
    pub unsafe fn eq(mut c1: libc::c_char, mut c2: libc::c_char) -> bool {
        return ((c1 as i32) == (c2 as i32));
    }
    pub unsafe fn length(mut s: *const libc::c_char) -> usize {
        let mut len: usize = 0_usize;
        'loop_: while !(unsafe {
            let _c1: libc::c_char = (*s);
            let _c2: libc::c_char = (0 as libc::c_char);
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
}
pub unsafe fn StrLen_0(mut s: *const libc::c_char) -> usize {
    let mut retVal: usize = 0_usize;
    'loop_: while (((*s) as i32) != (0)) {
        retVal.prefix_inc();
        s.prefix_inc();
    }
    return retVal;
}
pub unsafe fn StrLen_1(mut s: *const i32) -> usize {
    let mut retVal: usize = 0_usize;
    'loop_: while ((*s) != (0)) {
        retVal.prefix_inc();
        s.prefix_inc();
    }
    return retVal;
}
pub unsafe fn StrLen_2(mut s: *const u16) -> usize {
    let mut retVal: usize = 0_usize;
    'loop_: while (((*s) as i32) != (0)) {
        retVal.prefix_inc();
        s.prefix_inc();
    }
    return retVal;
}
pub unsafe fn StrLen_3(mut s: *const u32) -> usize {
    let mut retVal: usize = 0_usize;
    'loop_: while ((*s) != (0_u32)) {
        retVal.prefix_inc();
        s.prefix_inc();
    }
    return retVal;
}
pub unsafe fn test_4(mut s: *const libc::c_char) {
    let mut sv1: () = unimplemented!();
    assert!(((unimplemented!() as usize) == (unsafe { StrLen_0(s,) })));
    assert!(((unimplemented!()) == (s)));
}
pub unsafe fn test_5(mut s: *const i32) {
    let mut sv1: () = unimplemented!();
    assert!(((unimplemented!() as usize) == (unsafe { StrLen_1(s,) })));
    assert!(((unimplemented!()) == (s)));
}
pub unsafe fn test_6(mut s: *const u16) {
    let mut sv1: () = unimplemented!();
    assert!(((unimplemented!() as usize) == (unsafe { StrLen_2(s,) })));
    assert!(((unimplemented!()) == (s)));
}
pub unsafe fn test_7(mut s: *const u32) {
    let mut sv1: () = unimplemented!();
    assert!(((unimplemented!() as usize) == (unsafe { StrLen_3(s,) })));
    assert!(((unimplemented!()) == (s)));
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
    (unsafe { test_4(c"QBCDE".as_ptr()) });
    (unsafe { test_4(c"A".as_ptr()) });
    (unsafe { test_4(c"".as_ptr()) });
    (unsafe {
        test_5(
            [
                81 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]
            .as_ptr(),
        )
    });
    (unsafe { test_5([65 as i32, 0 as i32].as_ptr()) });
    (unsafe { test_5([0 as i32].as_ptr()) });
    (unsafe {
        test_6(
            [
                81 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]
            .as_ptr(),
        )
    });
    (unsafe { test_6([65 as u16, 0 as u16].as_ptr()) });
    (unsafe { test_6([0 as u16].as_ptr()) });
    (unsafe {
        test_7(
            [
                81 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]
            .as_ptr(),
        )
    });
    (unsafe { test_7([65 as u32, 0 as u32].as_ptr()) });
    (unsafe { test_7([0 as u32].as_ptr()) });
    {
        let sv1: () = unimplemented!();
        const _: () = assert!(true, "sv1.size() == 5");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
