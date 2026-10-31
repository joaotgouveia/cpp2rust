extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(*const libc::c_char , to_pconstlibcc_char)]
#[cpp_into(*const i32 , to_pconsti32)]
#[cpp_into(*const u8 , to_pconstu8)]
#[cpp_into(*const u16 , to_pconstu16)]
#[cpp_into(*const u32 , to_pconstu32)]
pub struct MultiStringType {
    pub w_: *const i32,
    pub wn_: usize,
    pub u8_: *const u8,
    pub u8n_: usize,
    pub u16_: *const u16,
    pub u16n_: usize,
    pub u32_: *const u32,
    pub u32n_: usize,
    pub s_: *const libc::c_char,
    pub sn_: usize,
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const i32,
        mut wn: usize,
        mut u8: *const u8,
        mut u8n: usize,
        mut u16: *const u16,
        mut u16n: usize,
        mut u32: *const u32,
        mut u32n: usize,
        mut s: *const libc::c_char,
        mut sn: usize,
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const libc::c_char) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const i32) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const u8) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const u16) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const u32) -> *const u32 {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const libc::c_char) -> usize {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const i32) -> usize {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const u8) -> usize {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const u16) -> usize {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const u32) -> usize {
        return self.u32n_;
    }
    pub unsafe fn to_pconstlibcc_char(&self) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn to_pconsti32(&self) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn to_pconstu8(&self) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn to_pconstu16(&self) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn to_pconstu32(&self) -> *const u32 {
        return self.u32_;
    }
}
pub unsafe fn test_0(mut value: bool) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: bool = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(((v as i32) == (a as i32)));
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_1(mut value: libc::c_char) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: libc::c_char = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(((v as i32) == (a as i32)));
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_2(mut value: libc::c_char) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = (value as i32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_3(mut value: libc::c_char) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = (value as i32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_4(mut value: libc::c_char) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u32 = (value as u32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_5(mut value: i8) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = (value as i32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_6(mut value: i16) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = (value as i32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_7(mut value: i32) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_8(mut value: i64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_9(mut value: i64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_10(mut value: u8) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u32 = (value as u32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_11(mut value: u16) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u32 = (value as u32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_12(mut value: u32) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u32 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_13(mut value: u64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_14(mut value: u64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_15(mut value: f32) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: f32 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_16(mut value: f64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: f64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_17(mut value: f64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: f64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_18(mut value: *const libc::c_char) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *const libc::c_char = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_19(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(unimplemented!());
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_20(mut value: Vec<libc::c_char>) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(unimplemented!());
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_21(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *const ::libc::c_void = std::ptr::null();
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const libc::c_char| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(((v) == (a)));
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_22(mut value: bool) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: bool = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(((v as i32) == (a as i32)));
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_23(mut value: i32) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_24(mut value: libc::c_char) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = (value as i32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_25(mut value: libc::c_char) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = (value as i32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_26(mut value: libc::c_char) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u32 = (value as u32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_27(mut value: i8) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = (value as i32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_28(mut value: i16) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = (value as i32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_29(mut value: i32) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i32 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_30(mut value: i64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_31(mut value: i64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: i64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_32(mut value: u8) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u32 = (value as u32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_33(mut value: u16) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u32 = (value as u32);
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_34(mut value: u32) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u32 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_35(mut value: u64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_36(mut value: u64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: u64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_37(mut value: f32) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: f32 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_38(mut value: f64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: f64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_39(mut value: f64) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: f64 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_40(mut value: *const i32) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *const i32 = value;
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(((v) == (a)));
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_41(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(unimplemented!());
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_42(mut value: Vec<i32>) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(unimplemented!());
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(false);
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_43(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *const ::libc::c_void = std::ptr::null();
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: bool| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: i32| {
            {
                assert!(false);
            }
        },
        |mut a: u32| {
            {
                assert!(false);
            }
        },
        |mut a: i64| {
            {
                assert!(false);
            }
        },
        |mut a: u64| {
            {
                assert!(false);
            }
        },
        |mut a: f32| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: f64| {
            {
                assert!(false);
            }
        },
        |mut a: *const i32| {
            {
                assert!(false);
            }
        },
        |mut a: ()| {
            {
                assert!(false);
            }
        },
        |mut a: *const ::libc::c_void| {
            {
                assert!(((v) == (a)));
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_handle_44(mut value: i128) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: bool| {
            assert!((unimplemented!()));
        },
        |mut a: libc::c_char| {
            assert!((unimplemented!()));
        },
        |mut a: i32| {
            assert!((unimplemented!()));
        },
        |mut a: u32| {
            assert!((unimplemented!()));
        },
        |mut a: i64| {
            assert!((unimplemented!()));
        },
        |mut a: u64| {
            assert!((unimplemented!()));
        },
        |mut a: f32| {
            assert!((unimplemented!()));
        },
        |mut a: f64| {
            assert!((unimplemented!()));
        },
        |mut a: f64| {
            assert!((unimplemented!()));
        },
        |mut a: *const libc::c_char| {
            assert!((unimplemented!()));
        },
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: *const ::libc::c_void| {
            assert!((unimplemented!()));
        }
    );
    unimplemented!();
}
pub unsafe fn test_handle_45(mut value: u128) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: bool| {
            assert!((unimplemented!()));
        },
        |mut a: libc::c_char| {
            assert!((unimplemented!()));
        },
        |mut a: i32| {
            assert!((unimplemented!()));
        },
        |mut a: u32| {
            assert!((unimplemented!()));
        },
        |mut a: i64| {
            assert!((unimplemented!()));
        },
        |mut a: u64| {
            assert!((unimplemented!()));
        },
        |mut a: f32| {
            assert!((unimplemented!()));
        },
        |mut a: f64| {
            assert!((unimplemented!()));
        },
        |mut a: f64| {
            assert!((unimplemented!()));
        },
        |mut a: *const libc::c_char| {
            assert!((unimplemented!()));
        },
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: *const ::libc::c_void| {
            assert!((unimplemented!()));
        }
    );
    unimplemented!();
}
pub unsafe fn test_handle_46(mut value: i128) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: bool| {
            assert!((unimplemented!()));
        },
        |mut a: i32| {
            assert!((unimplemented!()));
        },
        |mut a: i32| {
            assert!((unimplemented!()));
        },
        |mut a: u32| {
            assert!((unimplemented!()));
        },
        |mut a: i64| {
            assert!((unimplemented!()));
        },
        |mut a: u64| {
            assert!((unimplemented!()));
        },
        |mut a: f32| {
            assert!((unimplemented!()));
        },
        |mut a: f64| {
            assert!((unimplemented!()));
        },
        |mut a: f64| {
            assert!((unimplemented!()));
        },
        |mut a: *const i32| {
            assert!((unimplemented!()));
        },
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: *const ::libc::c_void| {
            assert!((unimplemented!()));
        }
    );
    unimplemented!();
}
pub unsafe fn test_handle_47(mut value: u128) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: bool| {
            assert!((unimplemented!()));
        },
        |mut a: i32| {
            assert!((unimplemented!()));
        },
        |mut a: i32| {
            assert!((unimplemented!()));
        },
        |mut a: u32| {
            assert!((unimplemented!()));
        },
        |mut a: i64| {
            assert!((unimplemented!()));
        },
        |mut a: u64| {
            assert!((unimplemented!()));
        },
        |mut a: f32| {
            assert!((unimplemented!()));
        },
        |mut a: f64| {
            assert!((unimplemented!()));
        },
        |mut a: f64| {
            assert!((unimplemented!()));
        },
        |mut a: *const i32| {
            assert!((unimplemented!()));
        },
        |mut a: ()| {
            assert!((unimplemented!()));
        },
        |mut a: *const ::libc::c_void| {
            assert!((unimplemented!()));
        }
    );
    unimplemented!();
}
pub unsafe fn test_48() {
    let mut empty: Vec<libc::c_char> = vec![libc::c_char::default()];
    let mut str: Vec<libc::c_char> = std::slice::from_raw_parts(
        (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    { [97 as i32, 98 as i32, 99 as i32, 0 as i32].as_ptr() },
                    {
                        ((::std::mem::size_of::<[i32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<i32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u8, 98 as u8, 99 as u8, 0 as u8].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u8; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u8>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u16, 98 as u16, 99 as u16, 0 as u16].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u16; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u16>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u32, 98 as u32, 99 as u32, 0 as u32].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { c"abc".as_ptr() },
                    { (::std::mem::size_of::<[libc::c_char; 4]>() as usize).wrapping_sub(1_usize) },
                ),
                (std::ptr::null() as *const ()).cast::<libc::c_char>(),
            )
        }),
        (unsafe {
            MultiStringType::length_7(
                &MultiStringType::new(
                    { [97 as i32, 98 as i32, 99 as i32, 0 as i32].as_ptr() },
                    {
                        ((::std::mem::size_of::<[i32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<i32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u8, 98 as u8, 99 as u8, 0 as u8].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u8; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u8>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u16, 98 as u16, 99 as u16, 0 as u16].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u16; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u16>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u32, 98 as u32, 99 as u32, 0 as u32].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { c"abc".as_ptr() },
                    { (::std::mem::size_of::<[libc::c_char; 4]>() as usize).wrapping_sub(1_usize) },
                ),
                (std::ptr::null() as *const ()).cast::<libc::c_char>(),
            )
        }) as usize,
    )
    .to_vec()
    .iter()
    .copied()
    .chain(std::iter::once((0 as libc::c_char)))
    .collect();
    (unsafe { test_0(true) });
    (unsafe { test_0(false) });
    (unsafe { test_1(('a' as libc::c_char)) });
    (unsafe { test_1(('z' as libc::c_char)) });
    (unsafe { test_1(('0' as libc::c_char)) });
    (unsafe { test_1(('9' as libc::c_char)) });
    {
        (unsafe { test_1(('a' as libc::c_char)) });
        (unsafe { test_1(('z' as libc::c_char)) });
        (unsafe { test_1(('0' as libc::c_char)) });
        (unsafe { test_1(('9' as libc::c_char)) });
    }
    (unsafe { test_5(i8::MIN) });
    (unsafe { test_5(0_i8) });
    (unsafe { test_5(i8::MAX) });
    (unsafe { test_6(i16::MIN) });
    (unsafe { test_6((i8::MIN as i16)) });
    (unsafe { test_6(0_i16) });
    (unsafe { test_6((i8::MAX as i16)) });
    (unsafe { test_6(i16::MAX) });
    (unsafe { test_7(i32::MIN) });
    (unsafe { test_7((i16::MIN as i32)) });
    (unsafe { test_7((i8::MIN as i32)) });
    (unsafe { test_7(0) });
    (unsafe { test_7((i8::MAX as i32)) });
    (unsafe { test_7((i16::MAX as i32)) });
    (unsafe { test_7(i32::MAX) });
    (unsafe { test_8(i64::MIN) });
    (unsafe { test_8((i32::MIN as i64)) });
    (unsafe { test_8((i16::MIN as i64)) });
    (unsafe { test_8((i8::MIN as i64)) });
    (unsafe { test_8(0_i64) });
    (unsafe { test_8((i8::MAX as i64)) });
    (unsafe { test_8((i16::MAX as i64)) });
    (unsafe { test_8((i32::MAX as i64)) });
    (unsafe { test_8(i64::MAX) });
    (unsafe { test_9(i64::MIN) });
    (unsafe { test_9(i64::MIN) });
    (unsafe { test_9((i32::MIN as i64)) });
    (unsafe { test_9((i16::MIN as i64)) });
    (unsafe { test_9((i8::MIN as i64)) });
    (unsafe { test_9(0_i64) });
    (unsafe { test_9((i8::MAX as i64)) });
    (unsafe { test_9((i16::MAX as i64)) });
    (unsafe { test_9((i32::MAX as i64)) });
    (unsafe { test_9(i64::MAX) });
    (unsafe { test_9(i64::MAX) });
    (unsafe { test_handle_44(0_i128) });
    (unsafe { test_10(0_u8) });
    (unsafe { test_10(u8::MAX) });
    (unsafe { test_11(0_u16) });
    (unsafe { test_11((u8::MAX as u16)) });
    (unsafe { test_11(u16::MAX) });
    (unsafe { test_12(0_u32) });
    (unsafe { test_12((u8::MAX as u32)) });
    (unsafe { test_12((u16::MAX as u32)) });
    (unsafe { test_12(u32::MAX) });
    (unsafe { test_13(0_u64) });
    (unsafe { test_13((u8::MAX as u64)) });
    (unsafe { test_13((u16::MAX as u64)) });
    (unsafe { test_13((u32::MAX as u64)) });
    (unsafe { test_13(u64::MAX) });
    (unsafe { test_14(0_u64) });
    (unsafe { test_14((u8::MAX as u64)) });
    (unsafe { test_14((u16::MAX as u64)) });
    (unsafe { test_14((u32::MAX as u64)) });
    (unsafe { test_14(u64::MAX) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_handle_45(0_u128) });
    (unsafe { test_15(-f32::MAX) });
    (unsafe { test_15(-f32::MIN_POSITIVE) });
    (unsafe { test_15((-0_f64 as f32)) });
    (unsafe { test_15((0_f64 as f32)) });
    (unsafe { test_15(f32::MIN_POSITIVE) });
    (unsafe { test_15(f32::MAX) });
    (unsafe { test_16(-f64::MAX) });
    (unsafe { test_16(-f64::MIN_POSITIVE) });
    (unsafe { test_16(-0_f64) });
    (unsafe { test_16(0_f64) });
    (unsafe { test_16(f64::MIN_POSITIVE) });
    (unsafe { test_16(f64::MAX) });
    (unsafe { test_17(-f64::MAX) });
    (unsafe { test_17(-f64::MIN_POSITIVE) });
    (unsafe { test_17(-0_f64) });
    (unsafe { test_17(0_f64) });
    (unsafe { test_17(f64::MIN_POSITIVE) });
    (unsafe { test_17(f64::MAX) });
    (unsafe { test_18(empty.as_ptr()) });
    (unsafe { test_18(str.as_ptr()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_20(vec![libc::c_char::default()]) });
    (unsafe { test_20(empty.clone()) });
    (unsafe { test_20(str.clone()) });
    (unsafe { test_21(Default::default()) });
}
pub unsafe fn test_49() {
    let mut empty: Vec<i32> = vec![i32::default()];
    let mut str: Vec<i32> = std::slice::from_raw_parts(
        (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    { [97 as i32, 98 as i32, 99 as i32, 0 as i32].as_ptr() },
                    {
                        ((::std::mem::size_of::<[i32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<i32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u8, 98 as u8, 99 as u8, 0 as u8].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u8; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u8>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u16, 98 as u16, 99 as u16, 0 as u16].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u16; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u16>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u32, 98 as u32, 99 as u32, 0 as u32].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { c"abc".as_ptr() },
                    { (::std::mem::size_of::<[libc::c_char; 4]>() as usize).wrapping_sub(1_usize) },
                ),
                (std::ptr::null() as *const ()).cast::<i32>(),
            )
        }),
        (unsafe {
            MultiStringType::length_8(
                &MultiStringType::new(
                    { [97 as i32, 98 as i32, 99 as i32, 0 as i32].as_ptr() },
                    {
                        ((::std::mem::size_of::<[i32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<i32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u8, 98 as u8, 99 as u8, 0 as u8].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u8; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u8>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u16, 98 as u16, 99 as u16, 0 as u16].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u16; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u16>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { [97 as u32, 98 as u32, 99 as u32, 0 as u32].as_ptr() },
                    {
                        ((::std::mem::size_of::<[u32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { c"abc".as_ptr() },
                    { (::std::mem::size_of::<[libc::c_char; 4]>() as usize).wrapping_sub(1_usize) },
                ),
                (std::ptr::null() as *const ()).cast::<i32>(),
            )
        }) as usize,
    )
    .to_vec()
    .iter()
    .copied()
    .chain(std::iter::once(0_i32))
    .collect();
    (unsafe { test_22(true) });
    (unsafe { test_22(false) });
    (unsafe { test_23((('a' as libc::c_char) as i32)) });
    (unsafe { test_23((('z' as libc::c_char) as i32)) });
    (unsafe { test_23((('0' as libc::c_char) as i32)) });
    (unsafe { test_23((('9' as libc::c_char) as i32)) });
    {
        {
            (unsafe { test_24(('a' as libc::c_char)) });
            (unsafe { test_24(('z' as libc::c_char)) });
            (unsafe { test_24(('0' as libc::c_char)) });
            (unsafe { test_24(('9' as libc::c_char)) });
        }
    }
    (unsafe { test_27(i8::MIN) });
    (unsafe { test_27(0_i8) });
    (unsafe { test_27(i8::MAX) });
    (unsafe { test_28(i16::MIN) });
    (unsafe { test_28((i8::MIN as i16)) });
    (unsafe { test_28(0_i16) });
    (unsafe { test_28((i8::MAX as i16)) });
    (unsafe { test_28(i16::MAX) });
    (unsafe { test_29(i32::MIN) });
    (unsafe { test_29((i16::MIN as i32)) });
    (unsafe { test_29((i8::MIN as i32)) });
    (unsafe { test_29(0) });
    (unsafe { test_29((i8::MAX as i32)) });
    (unsafe { test_29((i16::MAX as i32)) });
    (unsafe { test_29(i32::MAX) });
    (unsafe { test_30(i64::MIN) });
    (unsafe { test_30((i32::MIN as i64)) });
    (unsafe { test_30((i16::MIN as i64)) });
    (unsafe { test_30((i8::MIN as i64)) });
    (unsafe { test_30(0_i64) });
    (unsafe { test_30((i8::MAX as i64)) });
    (unsafe { test_30((i16::MAX as i64)) });
    (unsafe { test_30((i32::MAX as i64)) });
    (unsafe { test_30(i64::MAX) });
    (unsafe { test_31(i64::MIN) });
    (unsafe { test_31(i64::MIN) });
    (unsafe { test_31((i32::MIN as i64)) });
    (unsafe { test_31((i16::MIN as i64)) });
    (unsafe { test_31((i8::MIN as i64)) });
    (unsafe { test_31(0_i64) });
    (unsafe { test_31((i8::MAX as i64)) });
    (unsafe { test_31((i16::MAX as i64)) });
    (unsafe { test_31((i32::MAX as i64)) });
    (unsafe { test_31(i64::MAX) });
    (unsafe { test_31(i64::MAX) });
    (unsafe { test_handle_46(0_i128) });
    (unsafe { test_32(0_u8) });
    (unsafe { test_32(u8::MAX) });
    (unsafe { test_33(0_u16) });
    (unsafe { test_33((u8::MAX as u16)) });
    (unsafe { test_33(u16::MAX) });
    (unsafe { test_34(0_u32) });
    (unsafe { test_34((u8::MAX as u32)) });
    (unsafe { test_34((u16::MAX as u32)) });
    (unsafe { test_34(u32::MAX) });
    (unsafe { test_35(0_u64) });
    (unsafe { test_35((u8::MAX as u64)) });
    (unsafe { test_35((u16::MAX as u64)) });
    (unsafe { test_35((u32::MAX as u64)) });
    (unsafe { test_35(u64::MAX) });
    (unsafe { test_36(0_u64) });
    (unsafe { test_36((u8::MAX as u64)) });
    (unsafe { test_36((u16::MAX as u64)) });
    (unsafe { test_36((u32::MAX as u64)) });
    (unsafe { test_36(u64::MAX) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_handle_47(0_u128) });
    (unsafe { test_37(-f32::MAX) });
    (unsafe { test_37(-f32::MIN_POSITIVE) });
    (unsafe { test_37((-0_f64 as f32)) });
    (unsafe { test_37((0_f64 as f32)) });
    (unsafe { test_37(f32::MIN_POSITIVE) });
    (unsafe { test_37(f32::MAX) });
    (unsafe { test_38(-f64::MAX) });
    (unsafe { test_38(-f64::MIN_POSITIVE) });
    (unsafe { test_38(-0_f64) });
    (unsafe { test_38(0_f64) });
    (unsafe { test_38(f64::MIN_POSITIVE) });
    (unsafe { test_38(f64::MAX) });
    (unsafe { test_39(-f64::MAX) });
    (unsafe { test_39(-f64::MIN_POSITIVE) });
    (unsafe { test_39(-0_f64) });
    (unsafe { test_39(0_f64) });
    (unsafe { test_39(f64::MIN_POSITIVE) });
    (unsafe { test_39(f64::MAX) });
    (unsafe { test_40(empty.as_ptr()) });
    (unsafe { test_40(str.as_ptr()) });
    (unsafe { test_41(unimplemented!()) });
    (unsafe { test_41(unimplemented!()) });
    (unsafe { test_41(unimplemented!()) });
    (unsafe { test_42(vec![i32::default()]) });
    (unsafe { test_42(empty.clone()) });
    (unsafe { test_42(str.clone()) });
    (unsafe { test_43(Default::default()) });
}
pub unsafe fn test_50() {
    (unsafe { test_48() });
    (unsafe { test_49() });
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
    (unsafe { test_50() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
