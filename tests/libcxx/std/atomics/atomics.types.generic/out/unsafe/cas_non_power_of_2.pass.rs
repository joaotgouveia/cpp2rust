extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(lhs: *const S_1_, rhs: *const S_1_) -> bool {
    return (({
        let sa = core::slice::from_raw_parts(
            ((&(*lhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 1]>() as usize,
        );
        let sb = core::slice::from_raw_parts(
            ((&(*rhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 1]>() as usize,
        );
        let mut diff = 0_i32;
        for (x, y) in sa.iter().zip(sb.iter()) {
            if x != y {
                diff = (*x as i32) - (*y as i32);
                break;
            }
        }
        diff
    }) == (0));
}
pub unsafe fn operator_ne_1(lhs: *const S_1_, rhs: *const S_1_) -> bool {
    return !(unsafe {
        let _lhs: *const S_1_ = lhs;
        let _rhs: *const S_1_ = rhs;
        operator_eq_0(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S_1_ {
    pub data: [libc::c_char; 1],
}
impl S_1_ {
    pub unsafe fn new(mut v: Option<libc::c_char>) -> Self {
        let mut v: libc::c_char = v.unwrap_or_else(|| unsafe { (0 as libc::c_char) });
        let mut this = Self {
            data: [(0 as libc::c_char); 1],
        };
        {
            let byte_0 = ((&mut this.data[(0) as usize] as *mut libc::c_char)
                as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 1]>() {
                *byte_0.offset(offset as isize) = (v as i32) as u8;
            }
            ((&mut this.data[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void)
        };
        this
    }
}
impl std::cmp::PartialEq for S_1_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const S_1_, other as *const S_1_) }
    }
}
impl std::cmp::Eq for S_1_ {}
impl Default for S_1_ {
    fn default() -> Self {
        unsafe { S_1_::new(None) }
    }
}
pub unsafe fn operator_eq_2(lhs: *const S_2_, rhs: *const S_2_) -> bool {
    return (({
        let sa = core::slice::from_raw_parts(
            ((&(*lhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 2]>() as usize,
        );
        let sb = core::slice::from_raw_parts(
            ((&(*rhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 2]>() as usize,
        );
        let mut diff = 0_i32;
        for (x, y) in sa.iter().zip(sb.iter()) {
            if x != y {
                diff = (*x as i32) - (*y as i32);
                break;
            }
        }
        diff
    }) == (0));
}
pub unsafe fn operator_ne_3(lhs: *const S_2_, rhs: *const S_2_) -> bool {
    return !(unsafe {
        let _lhs: *const S_2_ = lhs;
        let _rhs: *const S_2_ = rhs;
        operator_eq_2(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S_2_ {
    pub data: [libc::c_char; 2],
}
impl S_2_ {
    pub unsafe fn new(mut v: Option<libc::c_char>) -> Self {
        let mut v: libc::c_char = v.unwrap_or_else(|| unsafe { (0 as libc::c_char) });
        let mut this = Self {
            data: [(0 as libc::c_char); 2],
        };
        {
            let byte_0 = ((&mut this.data[(0) as usize] as *mut libc::c_char)
                as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 2]>() {
                *byte_0.offset(offset as isize) = (v as i32) as u8;
            }
            ((&mut this.data[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void)
        };
        this
    }
}
impl std::cmp::PartialEq for S_2_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const S_2_, other as *const S_2_) }
    }
}
impl std::cmp::Eq for S_2_ {}
impl Default for S_2_ {
    fn default() -> Self {
        unsafe { S_2_::new(None) }
    }
}
pub unsafe fn operator_eq_4(lhs: *const S_3_, rhs: *const S_3_) -> bool {
    return (({
        let sa = core::slice::from_raw_parts(
            ((&(*lhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 3]>() as usize,
        );
        let sb = core::slice::from_raw_parts(
            ((&(*rhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 3]>() as usize,
        );
        let mut diff = 0_i32;
        for (x, y) in sa.iter().zip(sb.iter()) {
            if x != y {
                diff = (*x as i32) - (*y as i32);
                break;
            }
        }
        diff
    }) == (0));
}
pub unsafe fn operator_ne_5(lhs: *const S_3_, rhs: *const S_3_) -> bool {
    return !(unsafe {
        let _lhs: *const S_3_ = lhs;
        let _rhs: *const S_3_ = rhs;
        operator_eq_4(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S_3_ {
    pub data: [libc::c_char; 3],
}
impl S_3_ {
    pub unsafe fn new(mut v: Option<libc::c_char>) -> Self {
        let mut v: libc::c_char = v.unwrap_or_else(|| unsafe { (0 as libc::c_char) });
        let mut this = Self {
            data: [(0 as libc::c_char); 3],
        };
        {
            let byte_0 = ((&mut this.data[(0) as usize] as *mut libc::c_char)
                as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 3]>() {
                *byte_0.offset(offset as isize) = (v as i32) as u8;
            }
            ((&mut this.data[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void)
        };
        this
    }
}
impl std::cmp::PartialEq for S_3_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self as *const S_3_, other as *const S_3_) }
    }
}
impl std::cmp::Eq for S_3_ {}
impl Default for S_3_ {
    fn default() -> Self {
        unsafe { S_3_::new(None) }
    }
}
pub unsafe fn operator_eq_6(lhs: *const S_4_, rhs: *const S_4_) -> bool {
    return (({
        let sa = core::slice::from_raw_parts(
            ((&(*lhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 4]>() as usize,
        );
        let sb = core::slice::from_raw_parts(
            ((&(*rhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 4]>() as usize,
        );
        let mut diff = 0_i32;
        for (x, y) in sa.iter().zip(sb.iter()) {
            if x != y {
                diff = (*x as i32) - (*y as i32);
                break;
            }
        }
        diff
    }) == (0));
}
pub unsafe fn operator_ne_7(lhs: *const S_4_, rhs: *const S_4_) -> bool {
    return !(unsafe {
        let _lhs: *const S_4_ = lhs;
        let _rhs: *const S_4_ = rhs;
        operator_eq_6(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S_4_ {
    pub data: [libc::c_char; 4],
}
impl S_4_ {
    pub unsafe fn new(mut v: Option<libc::c_char>) -> Self {
        let mut v: libc::c_char = v.unwrap_or_else(|| unsafe { (0 as libc::c_char) });
        let mut this = Self {
            data: [(0 as libc::c_char); 4],
        };
        {
            let byte_0 = ((&mut this.data[(0) as usize] as *mut libc::c_char)
                as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 4]>() {
                *byte_0.offset(offset as isize) = (v as i32) as u8;
            }
            ((&mut this.data[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void)
        };
        this
    }
}
impl std::cmp::PartialEq for S_4_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const S_4_, other as *const S_4_) }
    }
}
impl std::cmp::Eq for S_4_ {}
impl Default for S_4_ {
    fn default() -> Self {
        unsafe { S_4_::new(None) }
    }
}
pub unsafe fn operator_eq_8(lhs: *const S_5_, rhs: *const S_5_) -> bool {
    return (({
        let sa = core::slice::from_raw_parts(
            ((&(*lhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 5]>() as usize,
        );
        let sb = core::slice::from_raw_parts(
            ((&(*rhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 5]>() as usize,
        );
        let mut diff = 0_i32;
        for (x, y) in sa.iter().zip(sb.iter()) {
            if x != y {
                diff = (*x as i32) - (*y as i32);
                break;
            }
        }
        diff
    }) == (0));
}
pub unsafe fn operator_ne_9(lhs: *const S_5_, rhs: *const S_5_) -> bool {
    return !(unsafe {
        let _lhs: *const S_5_ = lhs;
        let _rhs: *const S_5_ = rhs;
        operator_eq_8(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S_5_ {
    pub data: [libc::c_char; 5],
}
impl S_5_ {
    pub unsafe fn new(mut v: Option<libc::c_char>) -> Self {
        let mut v: libc::c_char = v.unwrap_or_else(|| unsafe { (0 as libc::c_char) });
        let mut this = Self {
            data: [(0 as libc::c_char); 5],
        };
        {
            let byte_0 = ((&mut this.data[(0) as usize] as *mut libc::c_char)
                as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 5]>() {
                *byte_0.offset(offset as isize) = (v as i32) as u8;
            }
            ((&mut this.data[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void)
        };
        this
    }
}
impl std::cmp::PartialEq for S_5_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const S_5_, other as *const S_5_) }
    }
}
impl std::cmp::Eq for S_5_ {}
impl Default for S_5_ {
    fn default() -> Self {
        unsafe { S_5_::new(None) }
    }
}
pub unsafe fn operator_eq_10(lhs: *const S_6_, rhs: *const S_6_) -> bool {
    return (({
        let sa = core::slice::from_raw_parts(
            ((&(*lhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 6]>() as usize,
        );
        let sb = core::slice::from_raw_parts(
            ((&(*rhs).data[(0) as usize] as *const libc::c_char) as *const ::libc::c_void)
                as *const u8,
            ::std::mem::size_of::<[libc::c_char; 6]>() as usize,
        );
        let mut diff = 0_i32;
        for (x, y) in sa.iter().zip(sb.iter()) {
            if x != y {
                diff = (*x as i32) - (*y as i32);
                break;
            }
        }
        diff
    }) == (0));
}
pub unsafe fn operator_ne_11(lhs: *const S_6_, rhs: *const S_6_) -> bool {
    return !(unsafe {
        let _lhs: *const S_6_ = lhs;
        let _rhs: *const S_6_ = rhs;
        operator_eq_10(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S_6_ {
    pub data: [libc::c_char; 6],
}
impl S_6_ {
    pub unsafe fn new(mut v: Option<libc::c_char>) -> Self {
        let mut v: libc::c_char = v.unwrap_or_else(|| unsafe { (0 as libc::c_char) });
        let mut this = Self {
            data: [(0 as libc::c_char); 6],
        };
        {
            let byte_0 = ((&mut this.data[(0) as usize] as *mut libc::c_char)
                as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 6]>() {
                *byte_0.offset(offset as isize) = (v as i32) as u8;
            }
            ((&mut this.data[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void)
        };
        this
    }
}
impl std::cmp::PartialEq for S_6_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self as *const S_6_, other as *const S_6_) }
    }
}
impl std::cmp::Eq for S_6_ {}
impl Default for S_6_ {
    fn default() -> Self {
        unsafe { S_6_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(S_1_, new, value)]
pub struct Expected_1_ {
    pub s: S_1_,
    pub b: bool,
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(S_2_, new, value)]
pub struct Expected_2_ {
    pub s: S_2_,
    pub b: bool,
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(S_3_, new, value)]
pub struct Expected_3_ {
    pub s: S_3_,
    pub b: bool,
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(S_4_, new, value)]
pub struct Expected_4_ {
    pub s: S_4_,
    pub b: bool,
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(S_5_, new, value)]
pub struct Expected_5_ {
    pub s: S_5_,
    pub b: bool,
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(S_6_, new, value)]
pub struct Expected_6_ {
    pub s: S_6_,
    pub b: bool,
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
pub unsafe fn test_12() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_1_ = Expected_1_::new({ S_1_::new({ Some((17 as libc::c_char)) }) });
    assert!(
        (unsafe {
            let mut _lhs: S_1_ = unimplemented!();
            operator_ne_1(&mut _lhs, &expected.s)
        })
    );
    assert!(expected.b);
    let mut r1: bool = unimplemented!();
    assert!(!(r1));
    assert!(
        (unsafe {
            let _lhs: *const S_1_ = &expected.s;
            let mut _rhs: S_1_ = S_1_::new({ Some((0 as libc::c_char)) });
            operator_eq_0(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
    assert!(
        (unsafe {
            let mut _lhs: S_1_ = unimplemented!();
            let mut _rhs: S_1_ = S_1_::new({ Some((0 as libc::c_char)) });
            operator_eq_0(&mut _lhs, &mut _rhs)
        })
    );
    let mut r2: bool = unimplemented!();
    assert!((r2 as bool));
    assert!(
        (unsafe {
            let mut _lhs: S_1_ = unimplemented!();
            let mut _rhs: S_1_ = S_1_::new({ Some((18 as libc::c_char)) });
            operator_eq_0(&mut _lhs, &mut _rhs)
        })
    );
    assert!(
        (unsafe {
            let _lhs: *const S_1_ = &expected.s;
            let mut _rhs: S_1_ = S_1_::new({ Some((0 as libc::c_char)) });
            operator_eq_0(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
}
pub unsafe fn test_13() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_2_ = Expected_2_::new({ S_2_::new({ Some((17 as libc::c_char)) }) });
    assert!(
        (unsafe {
            let mut _lhs: S_2_ = unimplemented!();
            operator_ne_3(&mut _lhs, &expected.s)
        })
    );
    assert!(expected.b);
    let mut r1: bool = unimplemented!();
    assert!(!(r1));
    assert!(
        (unsafe {
            let _lhs: *const S_2_ = &expected.s;
            let mut _rhs: S_2_ = S_2_::new({ Some((0 as libc::c_char)) });
            operator_eq_2(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
    assert!(
        (unsafe {
            let mut _lhs: S_2_ = unimplemented!();
            let mut _rhs: S_2_ = S_2_::new({ Some((0 as libc::c_char)) });
            operator_eq_2(&mut _lhs, &mut _rhs)
        })
    );
    let mut r2: bool = unimplemented!();
    assert!((r2 as bool));
    assert!(
        (unsafe {
            let mut _lhs: S_2_ = unimplemented!();
            let mut _rhs: S_2_ = S_2_::new({ Some((18 as libc::c_char)) });
            operator_eq_2(&mut _lhs, &mut _rhs)
        })
    );
    assert!(
        (unsafe {
            let _lhs: *const S_2_ = &expected.s;
            let mut _rhs: S_2_ = S_2_::new({ Some((0 as libc::c_char)) });
            operator_eq_2(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
}
pub unsafe fn test_14() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_3_ = Expected_3_::new({ S_3_::new({ Some((17 as libc::c_char)) }) });
    assert!(
        (unsafe {
            let mut _lhs: S_3_ = unimplemented!();
            operator_ne_5(&mut _lhs, &expected.s)
        })
    );
    assert!(expected.b);
    let mut r1: bool = unimplemented!();
    assert!(!(r1));
    assert!(
        (unsafe {
            let _lhs: *const S_3_ = &expected.s;
            let mut _rhs: S_3_ = S_3_::new({ Some((0 as libc::c_char)) });
            operator_eq_4(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
    assert!(
        (unsafe {
            let mut _lhs: S_3_ = unimplemented!();
            let mut _rhs: S_3_ = S_3_::new({ Some((0 as libc::c_char)) });
            operator_eq_4(&mut _lhs, &mut _rhs)
        })
    );
    let mut r2: bool = unimplemented!();
    assert!((r2 as bool));
    assert!(
        (unsafe {
            let mut _lhs: S_3_ = unimplemented!();
            let mut _rhs: S_3_ = S_3_::new({ Some((18 as libc::c_char)) });
            operator_eq_4(&mut _lhs, &mut _rhs)
        })
    );
    assert!(
        (unsafe {
            let _lhs: *const S_3_ = &expected.s;
            let mut _rhs: S_3_ = S_3_::new({ Some((0 as libc::c_char)) });
            operator_eq_4(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
}
pub unsafe fn test_15() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_4_ = Expected_4_::new({ S_4_::new({ Some((17 as libc::c_char)) }) });
    assert!(
        (unsafe {
            let mut _lhs: S_4_ = unimplemented!();
            operator_ne_7(&mut _lhs, &expected.s)
        })
    );
    assert!(expected.b);
    let mut r1: bool = unimplemented!();
    assert!(!(r1));
    assert!(
        (unsafe {
            let _lhs: *const S_4_ = &expected.s;
            let mut _rhs: S_4_ = S_4_::new({ Some((0 as libc::c_char)) });
            operator_eq_6(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
    assert!(
        (unsafe {
            let mut _lhs: S_4_ = unimplemented!();
            let mut _rhs: S_4_ = S_4_::new({ Some((0 as libc::c_char)) });
            operator_eq_6(&mut _lhs, &mut _rhs)
        })
    );
    let mut r2: bool = unimplemented!();
    assert!((r2 as bool));
    assert!(
        (unsafe {
            let mut _lhs: S_4_ = unimplemented!();
            let mut _rhs: S_4_ = S_4_::new({ Some((18 as libc::c_char)) });
            operator_eq_6(&mut _lhs, &mut _rhs)
        })
    );
    assert!(
        (unsafe {
            let _lhs: *const S_4_ = &expected.s;
            let mut _rhs: S_4_ = S_4_::new({ Some((0 as libc::c_char)) });
            operator_eq_6(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
}
pub unsafe fn test_16() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_5_ = Expected_5_::new({ S_5_::new({ Some((17 as libc::c_char)) }) });
    assert!(
        (unsafe {
            let mut _lhs: S_5_ = unimplemented!();
            operator_ne_9(&mut _lhs, &expected.s)
        })
    );
    assert!(expected.b);
    let mut r1: bool = unimplemented!();
    assert!(!(r1));
    assert!(
        (unsafe {
            let _lhs: *const S_5_ = &expected.s;
            let mut _rhs: S_5_ = S_5_::new({ Some((0 as libc::c_char)) });
            operator_eq_8(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
    assert!(
        (unsafe {
            let mut _lhs: S_5_ = unimplemented!();
            let mut _rhs: S_5_ = S_5_::new({ Some((0 as libc::c_char)) });
            operator_eq_8(&mut _lhs, &mut _rhs)
        })
    );
    let mut r2: bool = unimplemented!();
    assert!((r2 as bool));
    assert!(
        (unsafe {
            let mut _lhs: S_5_ = unimplemented!();
            let mut _rhs: S_5_ = S_5_::new({ Some((18 as libc::c_char)) });
            operator_eq_8(&mut _lhs, &mut _rhs)
        })
    );
    assert!(
        (unsafe {
            let _lhs: *const S_5_ = &expected.s;
            let mut _rhs: S_5_ = S_5_::new({ Some((0 as libc::c_char)) });
            operator_eq_8(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
}
pub unsafe fn test_17() {
    let mut a: () = unimplemented!();
    let mut expected: Expected_6_ = Expected_6_::new({ S_6_::new({ Some((17 as libc::c_char)) }) });
    assert!(
        (unsafe {
            let mut _lhs: S_6_ = unimplemented!();
            operator_ne_11(&mut _lhs, &expected.s)
        })
    );
    assert!(expected.b);
    let mut r1: bool = unimplemented!();
    assert!(!(r1));
    assert!(
        (unsafe {
            let _lhs: *const S_6_ = &expected.s;
            let mut _rhs: S_6_ = S_6_::new({ Some((0 as libc::c_char)) });
            operator_eq_10(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
    assert!(
        (unsafe {
            let mut _lhs: S_6_ = unimplemented!();
            let mut _rhs: S_6_ = S_6_::new({ Some((0 as libc::c_char)) });
            operator_eq_10(&mut _lhs, &mut _rhs)
        })
    );
    let mut r2: bool = unimplemented!();
    assert!((r2 as bool));
    assert!(
        (unsafe {
            let mut _lhs: S_6_ = unimplemented!();
            let mut _rhs: S_6_ = S_6_::new({ Some((18 as libc::c_char)) });
            operator_eq_10(&mut _lhs, &mut _rhs)
        })
    );
    assert!(
        (unsafe {
            let _lhs: *const S_6_ = &expected.s;
            let mut _rhs: S_6_ = S_6_::new({ Some((0 as libc::c_char)) });
            operator_eq_10(_lhs, &mut _rhs)
        })
    );
    assert!(expected.b);
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
    (unsafe { test_12() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_17() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
