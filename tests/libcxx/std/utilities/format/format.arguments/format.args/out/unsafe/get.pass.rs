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
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
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
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
    }
}
pub unsafe fn test_0(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_1(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_2(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_3(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_4(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_5(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_6(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_7(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_8(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_9(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_10(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_11(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_12(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_13(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_14(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_15(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_16(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_17(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_18(mut value: *const ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *const () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
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
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_20(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
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
            let v: () = unimplemented!();
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_22(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_23(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_24(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_25(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_26(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_27(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_28(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_29(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_30(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_31(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_32(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_33(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_34(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_35(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_36(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_37(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_38(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_39(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_40(mut value: *const ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *const () = value;
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
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
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_42(mut value: ()) {
    let mut store: () = unimplemented!();
    let format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
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
            let v: () = unimplemented!();
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: *const ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    );
    unimplemented!();
}
pub unsafe fn test_handle_44(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: *const ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        }
    );
    unimplemented!();
}
pub unsafe fn test_handle_45(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: *const ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        }
    );
    unimplemented!();
}
pub unsafe fn test_handle_46(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: *const ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        }
    );
    unimplemented!();
}
pub unsafe fn test_handle_47(mut value: ()) {
    let mut store: () = unimplemented!();
    let mut format_args: () = unimplemented!();
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: *const ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        }
    );
    unimplemented!();
}
pub unsafe fn test_48() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    (unsafe { test_0(true) });
    (unsafe { test_0(false) });
    (unsafe { test_1(('a' as ())) });
    (unsafe { test_1(('z' as ())) });
    (unsafe { test_1(('0' as ())) });
    (unsafe { test_1(('9' as ())) });
    {
        (unsafe { test_1(('a' as ())) });
        (unsafe { test_1(('z' as ())) });
        (unsafe { test_1(('0' as ())) });
        (unsafe { test_1(('9' as ())) });
    }
    (unsafe { test_5(unimplemented!()) });
    (unsafe { test_5(0_()) });
    (unsafe { test_5(unimplemented!()) });
    (unsafe { test_6(unimplemented!()) });
    (unsafe { test_6(unimplemented!()) });
    (unsafe { test_6(0_()) });
    (unsafe { test_6(unimplemented!()) });
    (unsafe { test_6(unimplemented!()) });
    (unsafe { test_7(unimplemented!()) });
    (unsafe { test_7(unimplemented!()) });
    (unsafe { test_7(unimplemented!()) });
    (unsafe { test_7(0_()) });
    (unsafe { test_7(unimplemented!()) });
    (unsafe { test_7(unimplemented!()) });
    (unsafe { test_7(unimplemented!()) });
    (unsafe { test_8(unimplemented!()) });
    (unsafe { test_8(unimplemented!()) });
    (unsafe { test_8(unimplemented!()) });
    (unsafe { test_8(unimplemented!()) });
    (unsafe { test_8(0_()) });
    (unsafe { test_8(unimplemented!()) });
    (unsafe { test_8(unimplemented!()) });
    (unsafe { test_8(unimplemented!()) });
    (unsafe { test_8(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(0_()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_handle_44(0_()) });
    (unsafe { test_10(0_()) });
    (unsafe { test_10(unimplemented!()) });
    (unsafe { test_11(0_()) });
    (unsafe { test_11(unimplemented!()) });
    (unsafe { test_11(unimplemented!()) });
    (unsafe { test_12(0_()) });
    (unsafe { test_12(unimplemented!()) });
    (unsafe { test_12(unimplemented!()) });
    (unsafe { test_12(unimplemented!()) });
    (unsafe { test_13(0_()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_14(0_()) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_14(unimplemented!()) });
    (unsafe { test_handle_45(0_()) });
    (unsafe { test_15(-unimplemented!()) });
    (unsafe { test_15(-unimplemented!()) });
    (unsafe { test_15(-0_f64) });
    (unsafe { test_15(0_f64) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_16(-unimplemented!()) });
    (unsafe { test_16(-unimplemented!()) });
    (unsafe { test_16(-0_f64) });
    (unsafe { test_16(0_f64) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_16(unimplemented!()) });
    (unsafe { test_17(-unimplemented!()) });
    (unsafe { test_17(-unimplemented!()) });
    (unsafe { test_17(-0_f64) });
    (unsafe { test_17(0_f64) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_18(unimplemented!()) });
    (unsafe { test_18(unimplemented!()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_20(unimplemented!()) });
    (unsafe { test_20(unimplemented!()) });
    (unsafe { test_20(unimplemented!()) });
    (unsafe { test_21(Default::default()) });
}
pub unsafe fn test_49() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    (unsafe { test_22(true) });
    (unsafe { test_22(false) });
    (unsafe { test_23(('a' as ())) });
    (unsafe { test_23(('z' as ())) });
    (unsafe { test_23(('0' as ())) });
    (unsafe { test_23(('9' as ())) });
    {
        {
            (unsafe { test_24(('a' as ())) });
            (unsafe { test_24(('z' as ())) });
            (unsafe { test_24(('0' as ())) });
            (unsafe { test_24(('9' as ())) });
        }
    }
    (unsafe { test_27(unimplemented!()) });
    (unsafe { test_27(0_()) });
    (unsafe { test_27(unimplemented!()) });
    (unsafe { test_28(unimplemented!()) });
    (unsafe { test_28(unimplemented!()) });
    (unsafe { test_28(0_()) });
    (unsafe { test_28(unimplemented!()) });
    (unsafe { test_28(unimplemented!()) });
    (unsafe { test_29(unimplemented!()) });
    (unsafe { test_29(unimplemented!()) });
    (unsafe { test_29(unimplemented!()) });
    (unsafe { test_29(0_()) });
    (unsafe { test_29(unimplemented!()) });
    (unsafe { test_29(unimplemented!()) });
    (unsafe { test_29(unimplemented!()) });
    (unsafe { test_30(unimplemented!()) });
    (unsafe { test_30(unimplemented!()) });
    (unsafe { test_30(unimplemented!()) });
    (unsafe { test_30(unimplemented!()) });
    (unsafe { test_30(0_()) });
    (unsafe { test_30(unimplemented!()) });
    (unsafe { test_30(unimplemented!()) });
    (unsafe { test_30(unimplemented!()) });
    (unsafe { test_30(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(0_()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_handle_46(0_()) });
    (unsafe { test_32(0_()) });
    (unsafe { test_32(unimplemented!()) });
    (unsafe { test_33(0_()) });
    (unsafe { test_33(unimplemented!()) });
    (unsafe { test_33(unimplemented!()) });
    (unsafe { test_34(0_()) });
    (unsafe { test_34(unimplemented!()) });
    (unsafe { test_34(unimplemented!()) });
    (unsafe { test_34(unimplemented!()) });
    (unsafe { test_35(0_()) });
    (unsafe { test_35(unimplemented!()) });
    (unsafe { test_35(unimplemented!()) });
    (unsafe { test_35(unimplemented!()) });
    (unsafe { test_35(unimplemented!()) });
    (unsafe { test_36(0_()) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_36(unimplemented!()) });
    (unsafe { test_handle_47(0_()) });
    (unsafe { test_37(-unimplemented!()) });
    (unsafe { test_37(-unimplemented!()) });
    (unsafe { test_37(-0_f64) });
    (unsafe { test_37(0_f64) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_38(-unimplemented!()) });
    (unsafe { test_38(-unimplemented!()) });
    (unsafe { test_38(-0_f64) });
    (unsafe { test_38(0_f64) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_38(unimplemented!()) });
    (unsafe { test_39(-unimplemented!()) });
    (unsafe { test_39(-unimplemented!()) });
    (unsafe { test_39(-0_f64) });
    (unsafe { test_39(0_f64) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_39(unimplemented!()) });
    (unsafe { test_40(unimplemented!()) });
    (unsafe { test_40(unimplemented!()) });
    (unsafe { test_41(unimplemented!()) });
    (unsafe { test_41(unimplemented!()) });
    (unsafe { test_41(unimplemented!()) });
    (unsafe { test_42(unimplemented!()) });
    (unsafe { test_42(unimplemented!()) });
    (unsafe { test_42(unimplemented!()) });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_50() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
