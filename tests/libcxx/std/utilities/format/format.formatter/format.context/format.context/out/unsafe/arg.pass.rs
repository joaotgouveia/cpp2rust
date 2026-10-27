extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_basic_format_arg_0(mut arg: (), mut expected: ()) -> () {
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let expected: () = expected;
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return ((a) == (expected));
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: *const ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        }
    );
    return unimplemented!();
}
pub unsafe fn test_basic_format_arg_1(mut arg: (), mut expected: ()) -> () {
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let expected: () = expected;
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return ((a) == (expected));
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: *const ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        }
    );
    return unimplemented!();
}
pub unsafe fn test_basic_format_arg_2(mut arg: (), mut expected: ()) -> () {
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let expected: () = expected;
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return ((a) == (expected));
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: *const ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        }
    );
    return unimplemented!();
}
pub unsafe fn test_basic_format_arg_3(mut arg: (), mut expected: ()) -> () {
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let expected: () = unimplemented!();
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: *const ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return unimplemented!();
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        }
    );
    return unimplemented!();
}
pub unsafe fn test_basic_format_arg_4(mut arg: (), mut expected: ()) -> () {
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let expected: () = expected;
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return ((a) == (expected));
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: *const ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        }
    );
    return unimplemented!();
}
pub unsafe fn test_basic_format_arg_5(mut arg: (), mut expected: ()) -> () {
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let expected: () = expected;
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return ((a) == (expected));
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: *const ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        }
    );
    return unimplemented!();
}
pub unsafe fn test_basic_format_arg_6(mut arg: (), mut expected: ()) -> () {
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let expected: () = expected;
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return ((a) == (expected));
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: *const ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        }
    );
    return unimplemented!();
}
pub unsafe fn test_basic_format_arg_7(mut arg: (), mut expected: ()) -> () {
    let mut visitor: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let expected: () = unimplemented!();
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: *const ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return unimplemented!();
            }
            panic!("ub: non-void function does not return a value")
        },
        |mut a: ()| -> () {
            {
                return false;
            }
            panic!("ub: non-void function does not return a value")
        }
    );
    return unimplemented!();
}
pub unsafe fn test_format_context_create_8(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_9(out_it, args.clone(), None) });
}
pub unsafe fn test_format_context_create_10(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_11(out_it, args.clone(), None) });
}
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
pub unsafe fn test_12() {
    let mut b: () = true;
    let mut c: () = ('a' as ());
    let mut a: () = 42_();
    let mut string: () = unimplemented!();
    let mut store: () = unimplemented!();
    let mut args: () = unimplemented!();
    let mut output: () = unimplemented!();
    let context: () = (unsafe { test_format_context_create_8(unimplemented!(), args.clone()) });
    unimplemented!();
    const _: () = assert!(true, "");;
    let mut i: () = 0_();
    let mut e: () = (unsafe { args.__size() });
    'loop_: while ((i) != (e)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_13() {
    let mut b: () = true;
    let mut c: () = ('a' as ());
    let mut a: () = 42_();
    let mut string: () = unimplemented!();
    let mut store: () = unimplemented!();
    let mut args: () = unimplemented!();
    let mut output: () = unimplemented!();
    let context: () = (unsafe { test_format_context_create_10(unimplemented!(), args.clone()) });
    unimplemented!();
    const _: () = assert!(true, "");;
    let mut i: () = 0_();
    let mut e: () = (unsafe { args.__size() });
    'loop_: while ((i) != (e)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
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
    (unsafe { test_12() });
    (unsafe { test_13() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
