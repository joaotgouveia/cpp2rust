extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut value_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_less_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_less_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_less_int__ {}
pub static mut value_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_less_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_less_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_less_std_string__ {}
pub static mut value_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_less_void__ {}
pub static mut value_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_less_equal_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_less_equal_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_less_equal_int__ {}
pub static mut value_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_less_equal_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_less_equal_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_less_equal_std_string__ {}
pub static mut value_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_less_equal_void__ {}
pub static mut value_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_equal_to_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_equal_to_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_equal_to_int__ {}
pub static mut value_7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_equal_to_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_equal_to_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_equal_to_std_string__ {}
pub static mut value_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_equal_to_void__ {}
pub static mut value_9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_not_equal_to_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_not_equal_to_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_not_equal_to_int__ {}
pub static mut value_10: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_not_equal_to_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_not_equal_to_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_not_equal_to_std_string__ {}
pub static mut value_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_not_equal_to_void__ {}
pub static mut value_12: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_greater_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_greater_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_greater_int__ {}
pub static mut value_13: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_greater_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_greater_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_greater_std_string__ {}
pub static mut value_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_greater_void__ {}
pub static mut value_15: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_greater_equal_int___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_greater_equal_int___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_greater_equal_int__ {}
pub static mut value_16: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((::std::mem::size_of::<is_transparent_std_greater_equal_std_string___two>()) == (1_()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct is_transparent_std_greater_equal_std_string___two {
    pub lx: (),
    pub lxx: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_greater_equal_std_string__ {}
pub static mut value_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((::std::mem::size_of::<()>()) == (1_())) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct is_transparent_std_greater_equal_void__ {}
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
    const _: () = assert!(true, "!is_transparent<std::less<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::less<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::less<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::less<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::less_equal<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::less_equal<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::less_equal<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::less_equal<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::equal_to<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::equal_to<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::equal_to<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::equal_to<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::not_equal_to<int>>::value");;
    const _: () = assert!(
        true,
        "!is_transparent<std::not_equal_to<std::string>>::value"
    );;
    const _: () = assert!(true, "is_transparent<std::not_equal_to<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::not_equal_to<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::greater<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::greater<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::greater<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::greater<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::greater_equal<int>>::value");;
    const _: () = assert!(
        true,
        "!is_transparent<std::greater_equal<std::string>>::value"
    );;
    const _: () = assert!(true, "is_transparent<std::greater_equal<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::greater_equal<>>::value");;
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
    std::cell::LazyCell::force(&*&raw const value_12);
    std::cell::LazyCell::force(&*&raw const value_13);
    std::cell::LazyCell::force(&*&raw const value_14);
    std::cell::LazyCell::force(&*&raw const value_15);
    std::cell::LazyCell::force(&*&raw const value_16);
    std::cell::LazyCell::force(&*&raw const value_17);
}
