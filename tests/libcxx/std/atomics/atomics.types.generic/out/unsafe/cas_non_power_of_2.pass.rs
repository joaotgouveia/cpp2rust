extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(lhs: *const S_1_, rhs: *const S_1_) -> () {
    return ((unsafe {
        let ___s1: () = ((&(*lhs).data[(0_()) as usize] as *const ()) as ());
        let ___s2: () = ((&(*rhs).data[(0_()) as usize] as *const ()) as ());
        let ___n: () = ::std::mem::size_of::<[(); 1]>();
        memcmp_1(___s1, ___s2, ___n)
    }) == (0_()));
}
pub unsafe fn operator_ne_2(lhs: *const S_1_, rhs: *const S_1_) -> () {
    return !(unsafe {
        let _lhs: *const S_1_ = lhs;
        let _rhs: *const S_1_ = rhs;
        operator_eq_0(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_1_ {
    pub data: [(); 1],
}
impl S_1_ {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            data: [unimplemented!(); 1],
        };
        (unsafe {
            let ___s: () = ((&mut this.data[(0_()) as usize] as *mut ()) as ());
            let ___n: () = ::std::mem::size_of::<[(); 1]>();
            memset_3(___s, v, ___n)
        });
        this
    }
}
impl std::cmp::PartialEq for S_1_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const S_1_, other as *const S_1_) }
    }
}
impl std::cmp::Eq for S_1_ {}
pub unsafe fn operator_eq_4(lhs: *const S_2_, rhs: *const S_2_) -> () {
    return ((unsafe {
        let ___s1: () = ((&(*lhs).data[(0_()) as usize] as *const ()) as ());
        let ___s2: () = ((&(*rhs).data[(0_()) as usize] as *const ()) as ());
        let ___n: () = ::std::mem::size_of::<[(); 2]>();
        memcmp_1(___s1, ___s2, ___n)
    }) == (0_()));
}
pub unsafe fn operator_ne_5(lhs: *const S_2_, rhs: *const S_2_) -> () {
    return !(unsafe {
        let _lhs: *const S_2_ = lhs;
        let _rhs: *const S_2_ = rhs;
        operator_eq_4(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_2_ {
    pub data: [(); 2],
}
impl S_2_ {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            data: [unimplemented!(); 2],
        };
        (unsafe {
            let ___s: () = ((&mut this.data[(0_()) as usize] as *mut ()) as ());
            let ___n: () = ::std::mem::size_of::<[(); 2]>();
            memset_3(___s, v, ___n)
        });
        this
    }
}
impl std::cmp::PartialEq for S_2_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self as *const S_2_, other as *const S_2_) }
    }
}
impl std::cmp::Eq for S_2_ {}
pub unsafe fn operator_eq_6(lhs: *const S_3_, rhs: *const S_3_) -> () {
    return ((unsafe {
        let ___s1: () = ((&(*lhs).data[(0_()) as usize] as *const ()) as ());
        let ___s2: () = ((&(*rhs).data[(0_()) as usize] as *const ()) as ());
        let ___n: () = ::std::mem::size_of::<[(); 3]>();
        memcmp_1(___s1, ___s2, ___n)
    }) == (0_()));
}
pub unsafe fn operator_ne_7(lhs: *const S_3_, rhs: *const S_3_) -> () {
    return !(unsafe {
        let _lhs: *const S_3_ = lhs;
        let _rhs: *const S_3_ = rhs;
        operator_eq_6(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_3_ {
    pub data: [(); 3],
}
impl S_3_ {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            data: [unimplemented!(); 3],
        };
        (unsafe {
            let ___s: () = ((&mut this.data[(0_()) as usize] as *mut ()) as ());
            let ___n: () = ::std::mem::size_of::<[(); 3]>();
            memset_3(___s, v, ___n)
        });
        this
    }
}
impl std::cmp::PartialEq for S_3_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const S_3_, other as *const S_3_) }
    }
}
impl std::cmp::Eq for S_3_ {}
pub unsafe fn operator_eq_8(lhs: *const S_4_, rhs: *const S_4_) -> () {
    return ((unsafe {
        let ___s1: () = ((&(*lhs).data[(0_()) as usize] as *const ()) as ());
        let ___s2: () = ((&(*rhs).data[(0_()) as usize] as *const ()) as ());
        let ___n: () = ::std::mem::size_of::<[(); 4]>();
        memcmp_1(___s1, ___s2, ___n)
    }) == (0_()));
}
pub unsafe fn operator_ne_9(lhs: *const S_4_, rhs: *const S_4_) -> () {
    return !(unsafe {
        let _lhs: *const S_4_ = lhs;
        let _rhs: *const S_4_ = rhs;
        operator_eq_8(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_4_ {
    pub data: [(); 4],
}
impl S_4_ {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            data: [unimplemented!(); 4],
        };
        (unsafe {
            let ___s: () = ((&mut this.data[(0_()) as usize] as *mut ()) as ());
            let ___n: () = ::std::mem::size_of::<[(); 4]>();
            memset_3(___s, v, ___n)
        });
        this
    }
}
impl std::cmp::PartialEq for S_4_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const S_4_, other as *const S_4_) }
    }
}
impl std::cmp::Eq for S_4_ {}
pub unsafe fn operator_eq_10(lhs: *const S_5_, rhs: *const S_5_) -> () {
    return ((unsafe {
        let ___s1: () = ((&(*lhs).data[(0_()) as usize] as *const ()) as ());
        let ___s2: () = ((&(*rhs).data[(0_()) as usize] as *const ()) as ());
        let ___n: () = ::std::mem::size_of::<[(); 5]>();
        memcmp_1(___s1, ___s2, ___n)
    }) == (0_()));
}
pub unsafe fn operator_ne_11(lhs: *const S_5_, rhs: *const S_5_) -> () {
    return !(unsafe {
        let _lhs: *const S_5_ = lhs;
        let _rhs: *const S_5_ = rhs;
        operator_eq_10(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_5_ {
    pub data: [(); 5],
}
impl S_5_ {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            data: [unimplemented!(); 5],
        };
        (unsafe {
            let ___s: () = ((&mut this.data[(0_()) as usize] as *mut ()) as ());
            let ___n: () = ::std::mem::size_of::<[(); 5]>();
            memset_3(___s, v, ___n)
        });
        this
    }
}
impl std::cmp::PartialEq for S_5_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self as *const S_5_, other as *const S_5_) }
    }
}
impl std::cmp::Eq for S_5_ {}
pub unsafe fn operator_eq_12(lhs: *const S_6_, rhs: *const S_6_) -> () {
    return ((unsafe {
        let ___s1: () = ((&(*lhs).data[(0_()) as usize] as *const ()) as ());
        let ___s2: () = ((&(*rhs).data[(0_()) as usize] as *const ()) as ());
        let ___n: () = ::std::mem::size_of::<[(); 6]>();
        memcmp_1(___s1, ___s2, ___n)
    }) == (0_()));
}
pub unsafe fn operator_ne_13(lhs: *const S_6_, rhs: *const S_6_) -> () {
    return !(unsafe {
        let _lhs: *const S_6_ = lhs;
        let _rhs: *const S_6_ = rhs;
        operator_eq_12(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_6_ {
    pub data: [(); 6],
}
impl S_6_ {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            data: [unimplemented!(); 6],
        };
        (unsafe {
            let ___s: () = ((&mut this.data[(0_()) as usize] as *mut ()) as ());
            let ___n: () = ::std::mem::size_of::<[(); 6]>();
            memset_3(___s, v, ___n)
        });
        this
    }
}
impl std::cmp::PartialEq for S_6_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self as *const S_6_, other as *const S_6_) }
    }
}
impl std::cmp::Eq for S_6_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Expected_1_ {
    pub s: S_1_,
    pub b: (),
}
impl Expected_1_ {
    pub unsafe fn new(mut ss: S_1_) -> Self {
        let mut this = Self { s: ss, b: true };
        this
    }
}
impl Default for Expected_1_ {
    fn default() -> Self {
        unsafe {
            Expected_1_ {
                s: <S_1_>::default(),
                b: true,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Expected_2_ {
    pub s: S_2_,
    pub b: (),
}
impl Expected_2_ {
    pub unsafe fn new(mut ss: S_2_) -> Self {
        let mut this = Self { s: ss, b: true };
        this
    }
}
impl Default for Expected_2_ {
    fn default() -> Self {
        unsafe {
            Expected_2_ {
                s: <S_2_>::default(),
                b: true,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Expected_3_ {
    pub s: S_3_,
    pub b: (),
}
impl Expected_3_ {
    pub unsafe fn new(mut ss: S_3_) -> Self {
        let mut this = Self { s: ss, b: true };
        this
    }
}
impl Default for Expected_3_ {
    fn default() -> Self {
        unsafe {
            Expected_3_ {
                s: <S_3_>::default(),
                b: true,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Expected_4_ {
    pub s: S_4_,
    pub b: (),
}
impl Expected_4_ {
    pub unsafe fn new(mut ss: S_4_) -> Self {
        let mut this = Self { s: ss, b: true };
        this
    }
}
impl Default for Expected_4_ {
    fn default() -> Self {
        unsafe {
            Expected_4_ {
                s: <S_4_>::default(),
                b: true,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Expected_5_ {
    pub s: S_5_,
    pub b: (),
}
impl Expected_5_ {
    pub unsafe fn new(mut ss: S_5_) -> Self {
        let mut this = Self { s: ss, b: true };
        this
    }
}
impl Default for Expected_5_ {
    fn default() -> Self {
        unsafe {
            Expected_5_ {
                s: <S_5_>::default(),
                b: true,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Expected_6_ {
    pub s: S_6_,
    pub b: (),
}
impl Expected_6_ {
    pub unsafe fn new(mut ss: S_6_) -> Self {
        let mut this = Self { s: ss, b: true };
        this
    }
}
impl Default for Expected_6_ {
    fn default() -> Self {
        unsafe {
            Expected_6_ {
                s: <S_6_>::default(),
                b: true,
            }
        }
    }
}
pub unsafe fn test_14() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_1_ = Expected_1_::new({ S_1_::new({ 17_() }) });
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_15() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_2_ = Expected_2_::new({ S_2_::new({ 17_() }) });
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_16() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_3_ = Expected_3_::new({ S_3_::new({ 17_() }) });
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_17() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_4_ = Expected_4_::new({ S_4_::new({ 17_() }) });
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_18() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_5_ = Expected_5_::new({ S_5_::new({ 17_() }) });
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_19() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_6_ = Expected_6_::new({ S_6_::new({ 17_() }) });
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_17() });
    (unsafe { test_18() });
    (unsafe { test_19() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
