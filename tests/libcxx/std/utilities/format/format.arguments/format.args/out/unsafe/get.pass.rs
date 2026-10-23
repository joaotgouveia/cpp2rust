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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___bool_ =
        (unsafe { make_format_args_1(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_1({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___char_ =
        (unsafe { make_format_args_3(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___char_ =
        (unsafe { make_format_args_3(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___char_ =
        (unsafe { make_format_args_3(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___char_ =
        (unsafe { make_format_args_3(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___signed_char_ =
        (unsafe { make_format_args_8(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_3({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___short_ =
        (unsafe { make_format_args_10(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_4({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___int_ =
        (unsafe { make_format_args_12(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_5({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___long_ =
        (unsafe { make_format_args_14(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_6({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___long_long_ =
        (unsafe { make_format_args_16(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_7({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_char_ =
        (unsafe { make_format_args_18(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_8({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_short_ = ( unsafe { make_format_args_20 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_9({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_int_ =
        (unsafe { make_format_args_22(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_10({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_long_ =
        (unsafe { make_format_args_24(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_11({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_long_long_ = ( unsafe { make_format_args_26 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_12({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___float_ =
        (unsafe { make_format_args_28(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_13({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___double_ =
        (unsafe { make_format_args_30(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_14({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___long_double_ =
        (unsafe { make_format_args_32(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_15({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_33(mut value: *const ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___const_char_ptr_ = ( unsafe { make_format_args_34 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_16({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *const () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___std_string_view_ = ( unsafe { make_format_args_36 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_17({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___std_string_ =
        (unsafe { make_format_args_38(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_18({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = (unsafe { value() });
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char___std_nullptr_t_ =
        (unsafe { make_format_args_40(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_19({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___bool_ =
        (unsafe { make_format_args_42(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_1({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___wchar_t_ =
        (unsafe { make_format_args_44(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_2({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_45(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_ =
        (unsafe { make_format_args_46(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_47(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_ =
        (unsafe { make_format_args_46(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_48(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_ =
        (unsafe { make_format_args_46(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_49(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___signed_char_ = ( unsafe { make_format_args_50 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_4({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_51(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___short_ =
        (unsafe { make_format_args_52(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_5({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_53(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int_ =
        (unsafe { make_format_args_54(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_6({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_55(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_ =
        (unsafe { make_format_args_56(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_7({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_57(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_long_ = ( unsafe { make_format_args_58 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_8({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_59(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_char_ = ( unsafe { make_format_args_60 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_9({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_61(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_short_ = ( unsafe { make_format_args_62 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_10({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_63(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_int_ = ( unsafe { make_format_args_64 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_11({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_65(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_long_ = ( unsafe { make_format_args_66 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_12({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_67(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_long_long_ = ( unsafe { make_format_args_68 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_13({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_69(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___float_ =
        (unsafe { make_format_args_70(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_14({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_71(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___double_ =
        (unsafe { make_format_args_72(&mut value) });
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_15({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_73(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_double_ = ( unsafe { make_format_args_74 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_16({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_75(mut value: *const ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___const_wchar_t_ptr_ = ( unsafe { make_format_args_76 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_17({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *const () = value;
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_77(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_wstring_view_ = ( unsafe { make_format_args_78 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_18({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_79(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_wstring_ = ( unsafe { make_format_args_80 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_19({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = (unsafe { value() });
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_81(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_nullptr_t_ = ( unsafe { make_format_args_82 ( & mut value , ) } )  ;
    let format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_20({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: () = unimplemented!();
        },
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
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
pub unsafe fn test_handle_83(mut value: ()) {
    let mut store: std___format_arg_store_std_basic_format_context_char_ptr__char_____int128_ =
        (unsafe { make_format_args_84(&mut value) });
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_20({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
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
pub unsafe fn test_handle_85(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned___int128_ = ( unsafe { make_format_args_86 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_21({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
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
pub unsafe fn test_handle_87(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t_____int128_ = ( unsafe { make_format_args_88 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_21({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
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
pub unsafe fn test_handle_89(mut value: ()) {
    let mut store : std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned___int128_ = ( unsafe { make_format_args_90 ( & mut value , ) } )  ;
    let mut format_args: () =
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_22({ &store });
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
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
pub unsafe fn test_91() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    (unsafe { test_0(true) });
    (unsafe { test_0(false) });
    (unsafe { test_2(('a' as ())) });
    (unsafe { test_2(('z' as ())) });
    (unsafe { test_2(('0' as ())) });
    (unsafe { test_2(('9' as ())) });
    {
        (unsafe { test_2(('a' as ())) });
        (unsafe { test_2(('z' as ())) });
        (unsafe { test_2(('0' as ())) });
        (unsafe { test_2(('9' as ())) });
    }
    (unsafe { test_7(unimplemented!()) });
    (unsafe { test_7(0_()) });
    (unsafe { test_7(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(0_()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_9(unimplemented!()) });
    (unsafe { test_11(unimplemented!()) });
    (unsafe { test_11(unimplemented!()) });
    (unsafe { test_11(unimplemented!()) });
    (unsafe { test_11(0_()) });
    (unsafe { test_11(unimplemented!()) });
    (unsafe { test_11(unimplemented!()) });
    (unsafe { test_11(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(0_()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_13(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(0_()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_15(unimplemented!()) });
    (unsafe { test_handle_83(0_()) });
    (unsafe { test_17(0_()) });
    (unsafe { test_17(unimplemented!()) });
    (unsafe { test_19(0_()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_19(unimplemented!()) });
    (unsafe { test_21(0_()) });
    (unsafe { test_21(unimplemented!()) });
    (unsafe { test_21(unimplemented!()) });
    (unsafe { test_21(unimplemented!()) });
    (unsafe { test_23(0_()) });
    (unsafe { test_23(unimplemented!()) });
    (unsafe { test_23(unimplemented!()) });
    (unsafe { test_23(unimplemented!()) });
    (unsafe { test_23(unimplemented!()) });
    (unsafe { test_25(0_()) });
    (unsafe { test_25(unimplemented!()) });
    (unsafe { test_25(unimplemented!()) });
    (unsafe { test_25(unimplemented!()) });
    (unsafe { test_25(unimplemented!()) });
    (unsafe { test_25(unimplemented!()) });
    (unsafe { test_handle_85(0_()) });
    (unsafe { test_27(-unimplemented!()) });
    (unsafe { test_27(-unimplemented!()) });
    (unsafe { test_27(-0_f64) });
    (unsafe { test_27(0_f64) });
    (unsafe { test_27(unimplemented!()) });
    (unsafe { test_27(unimplemented!()) });
    (unsafe { test_29(-unimplemented!()) });
    (unsafe { test_29(-unimplemented!()) });
    (unsafe { test_29(-0_f64) });
    (unsafe { test_29(0_f64) });
    (unsafe { test_29(unimplemented!()) });
    (unsafe { test_29(unimplemented!()) });
    (unsafe { test_31(-unimplemented!()) });
    (unsafe { test_31(-unimplemented!()) });
    (unsafe { test_31(-0_f64) });
    (unsafe { test_31(0_f64) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_31(unimplemented!()) });
    (unsafe { test_33(unimplemented!()) });
    (unsafe { test_33(unimplemented!()) });
    (unsafe { test_35(unimplemented!()) });
    (unsafe { test_35((unsafe { empty() })) });
    (unsafe { test_35((unsafe { str() })) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_37(unimplemented!()) });
    (unsafe { test_39(Default::default()) });
}
pub unsafe fn test_92() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    (unsafe { test_41(true) });
    (unsafe { test_41(false) });
    (unsafe { test_43(('a' as ())) });
    (unsafe { test_43(('z' as ())) });
    (unsafe { test_43(('0' as ())) });
    (unsafe { test_43(('9' as ())) });
    {
        {
            (unsafe { test_45(('a' as ())) });
            (unsafe { test_45(('z' as ())) });
            (unsafe { test_45(('0' as ())) });
            (unsafe { test_45(('9' as ())) });
        }
    }
    (unsafe { test_49(unimplemented!()) });
    (unsafe { test_49(0_()) });
    (unsafe { test_49(unimplemented!()) });
    (unsafe { test_51(unimplemented!()) });
    (unsafe { test_51(unimplemented!()) });
    (unsafe { test_51(0_()) });
    (unsafe { test_51(unimplemented!()) });
    (unsafe { test_51(unimplemented!()) });
    (unsafe { test_53(unimplemented!()) });
    (unsafe { test_53(unimplemented!()) });
    (unsafe { test_53(unimplemented!()) });
    (unsafe { test_53(0_()) });
    (unsafe { test_53(unimplemented!()) });
    (unsafe { test_53(unimplemented!()) });
    (unsafe { test_53(unimplemented!()) });
    (unsafe { test_55(unimplemented!()) });
    (unsafe { test_55(unimplemented!()) });
    (unsafe { test_55(unimplemented!()) });
    (unsafe { test_55(unimplemented!()) });
    (unsafe { test_55(0_()) });
    (unsafe { test_55(unimplemented!()) });
    (unsafe { test_55(unimplemented!()) });
    (unsafe { test_55(unimplemented!()) });
    (unsafe { test_55(unimplemented!()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_57(0_()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_57(unimplemented!()) });
    (unsafe { test_handle_87(0_()) });
    (unsafe { test_59(0_()) });
    (unsafe { test_59(unimplemented!()) });
    (unsafe { test_61(0_()) });
    (unsafe { test_61(unimplemented!()) });
    (unsafe { test_61(unimplemented!()) });
    (unsafe { test_63(0_()) });
    (unsafe { test_63(unimplemented!()) });
    (unsafe { test_63(unimplemented!()) });
    (unsafe { test_63(unimplemented!()) });
    (unsafe { test_65(0_()) });
    (unsafe { test_65(unimplemented!()) });
    (unsafe { test_65(unimplemented!()) });
    (unsafe { test_65(unimplemented!()) });
    (unsafe { test_65(unimplemented!()) });
    (unsafe { test_67(0_()) });
    (unsafe { test_67(unimplemented!()) });
    (unsafe { test_67(unimplemented!()) });
    (unsafe { test_67(unimplemented!()) });
    (unsafe { test_67(unimplemented!()) });
    (unsafe { test_67(unimplemented!()) });
    (unsafe { test_handle_89(0_()) });
    (unsafe { test_69(-unimplemented!()) });
    (unsafe { test_69(-unimplemented!()) });
    (unsafe { test_69(-0_f64) });
    (unsafe { test_69(0_f64) });
    (unsafe { test_69(unimplemented!()) });
    (unsafe { test_69(unimplemented!()) });
    (unsafe { test_71(-unimplemented!()) });
    (unsafe { test_71(-unimplemented!()) });
    (unsafe { test_71(-0_f64) });
    (unsafe { test_71(0_f64) });
    (unsafe { test_71(unimplemented!()) });
    (unsafe { test_71(unimplemented!()) });
    (unsafe { test_73(-unimplemented!()) });
    (unsafe { test_73(-unimplemented!()) });
    (unsafe { test_73(-0_f64) });
    (unsafe { test_73(0_f64) });
    (unsafe { test_73(unimplemented!()) });
    (unsafe { test_73(unimplemented!()) });
    (unsafe { test_75(unimplemented!()) });
    (unsafe { test_75(unimplemented!()) });
    (unsafe { test_77(unimplemented!()) });
    (unsafe { test_77((unsafe { empty() })) });
    (unsafe { test_77((unsafe { str() })) });
    (unsafe { test_79(unimplemented!()) });
    (unsafe { test_79(unimplemented!()) });
    (unsafe { test_79(unimplemented!()) });
    (unsafe { test_81(Default::default()) });
}
pub unsafe fn test_93() {
    (unsafe { test_91() });
    (unsafe { test_92() });
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
    (unsafe { test_93() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
