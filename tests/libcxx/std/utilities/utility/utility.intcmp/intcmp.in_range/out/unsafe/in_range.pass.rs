extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_char_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_char_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_char_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_char_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_signed_char_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_signed_char_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_signed_char_ {
    fn default() -> Self {
        unsafe { Tuple_signed_char_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_short_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_short_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_short_ {
    fn default() -> Self {
        unsafe { Tuple_short_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_short_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_short_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_short_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_short_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_int_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_int_ {
    fn default() -> Self {
        unsafe { Tuple_int_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_int_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_int_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_int_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_long_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_long_ {
    fn default() -> Self {
        unsafe { Tuple_long_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_long_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_long_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_long_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_long_long_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_long_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_long_long_ {
    fn default() -> Self {
        unsafe { Tuple_long_long_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_long_long_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_long_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_long_long_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_long_long_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned___int128_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned___int128_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned___int128_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned___int128_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple___int128_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple___int128_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = unimplemented!();
        this.max = unimplemented!();
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple___int128_ {
    fn default() -> Self {
        unsafe { Tuple___int128_::new() }
    }
}
pub unsafe fn test_in_range1_0() {
    let tup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_1() {
    let tup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_2() {
    let tup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_3() {
    let tup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_4() {
    let tup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_5() {
    let tup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_6() {
    let tup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_7() {
    let tup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_8() {
    let tup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_9() {
    let tup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_10() {
    let tup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range1_11() {
    let tup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_in_range_12() {
    let utup8: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let stup8: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test1_13(_a0: *const ()) {
    {
        (unsafe { test_in_range1_11() });
        {
            (unsafe { test_in_range1_10() });
            {
                (unsafe { test_in_range1_9() });
                {
                    (unsafe { test_in_range1_8() });
                    {
                        (unsafe { test_in_range1_7() });
                        {
                            (unsafe { test_in_range1_6() });
                            {
                                (unsafe { test_in_range1_5() });
                                {
                                    (unsafe { test_in_range1_4() });
                                    {
                                        (unsafe { test_in_range1_3() });
                                        {
                                            (unsafe { test_in_range1_2() });
                                            {
                                                (unsafe { test_in_range1_1() });
                                                (unsafe { test_in_range1_0() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test_14() -> () {
    let mut types: () = unimplemented!();
    (unsafe { test1_13(&types) });
    (unsafe { test_in_range_12() });
    return true;
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
    const _: () = assert!(true, "");;
    (unsafe { test_14() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
