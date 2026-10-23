extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut value_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_bit_and_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_bit_and_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_and_int__ {}
pub static mut value_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_bit_and_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_bit_and_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_and_std_string__ {}
pub static mut value_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_and_void__ {}
pub static mut value_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_bit_or_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_bit_or_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_or_int__ {}
pub static mut value_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_bit_or_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_bit_or_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_or_std_string__ {}
pub static mut value_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_or_void__ {}
pub static mut value_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_bit_xor_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_bit_xor_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_xor_int__ {}
pub static mut value_7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_bit_xor_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_bit_xor_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_xor_std_string__ {}
pub static mut value_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_xor_void__ {}
pub static mut value_9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_bit_not_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_bit_not_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_not_int__ {}
pub static mut value_10: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_bit_not_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_bit_not_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_not_std_string__ {}
pub static mut value_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_bit_not_void__ {}
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
    const _: () = assert!(true, "!is_transparent<std::bit_and<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::bit_and<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::bit_and<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::bit_and<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::bit_or<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::bit_or<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::bit_or<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::bit_or<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::bit_xor<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::bit_xor<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::bit_xor<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::bit_xor<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::bit_not<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::bit_not<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::bit_not<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::bit_not<>>::value");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_0);
    std::cell::LazyCell::force(&*&raw const value_1);
    std::cell::LazyCell::force(&*&raw const value_2);
    std::cell::LazyCell::force(&*&raw const value_3);
    std::cell::LazyCell::force(&*&raw const value_4);
    std::cell::LazyCell::force(&*&raw const value_5);
    std::cell::LazyCell::force(&*&raw const value_6);
    std::cell::LazyCell::force(&*&raw const value_7);
    std::cell::LazyCell::force(&*&raw const value_8);
    std::cell::LazyCell::force(&*&raw const value_9);
    std::cell::LazyCell::force(&*&raw const value_10);
    std::cell::LazyCell::force(&*&raw const value_11);
}
