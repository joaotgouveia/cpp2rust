extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_latin_unicode_0() {
    let u16str: [(); 4] = [229_(), 228_(), 246_(), 0_()];
    let u32str: [(); 4] = [229_(), 228_(), 246_(), 0_()];
    let str: [(); 7] = [-61_(), -91_(), -61_(), -92_(), -61_(), -74_(), 0_()];
    let u8str: [(); 7] = [195_(), 165_(), 195_(), 164_(), 195_(), 182_(), 0_()];
    let wstr: [(); 4] = [229_(), 228_(), 246_(), 0_()];
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_wide_unicode_1() {
    let u16str: [(); 3] = [55297_(), 56375_(), 0_()];
    let u32str: [(); 2] = [66615_(), 0_()];
    let u8str: [(); 5] = [240_(), 144_(), 144_(), 183_(), 0_()];
    let str: [(); 5] = [-16_(), -112_(), -112_(), -73_(), 0_()];
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let wstr: [(); 2] = [66615_(), 0_()];
    {
        let p: () = unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_append_2() {
    let u16str: [(); 3] = [55297_(), 56375_(), 0_()];
    let u32str: [(); 2] = [66615_(), 0_()];
    let u32ref: [(); 6] = [
        66615_(),
        unimplemented!(),
        66615_(),
        unimplemented!(),
        66615_(),
        0_(),
    ];
    let str: [(); 5] = [-16_(), -112_(), -112_(), -73_(), 0_()];
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let wstr: [(); 2] = [66615_(), 0_()];
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_concat_3() {
    let u16str: [(); 3] = [55297_(), 56375_(), 0_()];
    let u32str: [(); 2] = [66615_(), 0_()];
    let u32ref: [(); 4] = [66615_(), 66615_(), 66615_(), 0_()];
    let str: [(); 5] = [-16_(), -112_(), -112_(), -73_(), 0_()];
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let wstr: [(); 2] = [66615_(), 0_()];
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_append_concat_narrow_4() {
    let u16str: [(); 2] = [229_(), 0_()];
    let u32ref_append: [(); 4] = [229_(), unimplemented!(), 229_(), 0_()];
    let u32ref_concat: [(); 3] = [229_(), 229_(), 0_()];
    {
        let u8str: [(); 3] = [195_(), 165_(), 0_()];
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let str: [(); 3] = [-61_(), -91_(), 0_()];
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_latin_unicode_0() });
    (unsafe { test_wide_unicode_1() });
    (unsafe { test_append_2() });
    (unsafe { test_concat_3() });
    (unsafe { test_append_concat_narrow_4() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
