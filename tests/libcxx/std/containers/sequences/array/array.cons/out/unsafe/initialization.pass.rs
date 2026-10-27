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
pub struct NoDefault {}
impl NoDefault {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_initialization {}
impl test_initialization {
    pub unsafe fn operator_call_1_char(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
    pub unsafe fn operator_call_1_int(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
    pub unsafe fn operator_call_1_long(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
    pub unsafe fn operator_call_1_float(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
    pub unsafe fn operator_call_1_double(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
    pub unsafe fn operator_call_1_long_double(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
    pub unsafe fn operator_call_1_struct_Empty(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
    pub unsafe fn operator_call_1_struct_Trivial(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
    pub unsafe fn operator_call_1_struct_NonTrivial(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
    pub unsafe fn operator_call_1_struct_NonEmptyNonTrivial(&self) {
        {
            let mut a0: () = unimplemented!();
            &(a0);
            {
                let mut a1: () = unimplemented!();
                &(a1);
                let mut a2: () = unimplemented!();
                &(a2);
                let mut a3: () = unimplemented!();
                &(a3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            let a0: () = unimplemented!();
            &(a0);
            let nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
        {
            {
                let mut a0_0: () = unimplemented!();
                &(a0_0);
            }
            {
                let mut a1_0: () = unimplemented!();
                &(a1_0);
                let mut a1_1: () = unimplemented!();
                &(a1_1);
            }
            {
                let mut a2_0: () = unimplemented!();
                &(a2_0);
                let mut a2_1: () = unimplemented!();
                &(a2_1);
                let mut a2_2: () = unimplemented!();
                &(a2_2);
            }
            {
                let mut a3_0: () = unimplemented!();
                &(a3_0);
                let mut a3_1: () = unimplemented!();
                &(a3_1);
                let mut a3_2: () = unimplemented!();
                &(a3_2);
                let mut a3_3: () = unimplemented!();
                &(a3_3);
            }
            let mut nodefault: () = unimplemented!();
            &(nodefault);
        }
    }
}
pub unsafe fn test_initializer_list_0() -> () {
    {
        let a3_0: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a3_1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a3_2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a3_3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Trivial {
    pub i: (),
    pub j: (),
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct NonTrivial {}
impl NonTrivial {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const NonTrivial) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NonTrivial) -> *mut NonTrivial {
        return &mut (*(self as *mut NonTrivial));
    }
}
impl Clone for NonTrivial {
    fn clone(&self) -> Self {
        unsafe { NonTrivial::copy_from(self as *const NonTrivial) }
    }
}
impl Default for NonTrivial {
    fn default() -> Self {
        unsafe { NonTrivial::new() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct NonEmptyNonTrivial {
    pub i: (),
    pub j: (),
}
impl NonEmptyNonTrivial {
    pub unsafe fn new() -> Self {
        let mut this = Self { i: 22_(), j: 33_() };
        this
    }
    pub unsafe fn copy_from(_a0: *const NonEmptyNonTrivial) -> Self {
        let mut this = Self { i: 22_(), j: 33_() };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const NonEmptyNonTrivial,
    ) -> *mut NonEmptyNonTrivial {
        self.i = (*_a0).i;
        self.j = (*_a0).j;
        return &mut (*(self as *mut NonEmptyNonTrivial));
    }
}
impl Clone for NonEmptyNonTrivial {
    fn clone(&self) -> Self {
        unsafe { NonEmptyNonTrivial::copy_from(self as *const NonEmptyNonTrivial) }
    }
}
impl Default for NonEmptyNonTrivial {
    fn default() -> Self {
        unsafe { NonEmptyNonTrivial::new() }
    }
}
pub unsafe fn with_all_types_1() -> () {
    (unsafe { test_initialization::operator_call_1_char(&<test_initialization>::default()) });
    (unsafe { test_initialization::operator_call_1_int(&<test_initialization>::default()) });
    (unsafe { test_initialization::operator_call_1_long(&<test_initialization>::default()) });
    (unsafe { test_initialization::operator_call_1_float(&<test_initialization>::default()) });
    (unsafe { test_initialization::operator_call_1_double(&<test_initialization>::default()) });
    (unsafe {
        test_initialization::operator_call_1_long_double(&<test_initialization>::default())
    });
    (unsafe {
        test_initialization::operator_call_1_struct_Empty(&<test_initialization>::default())
    });
    (unsafe {
        test_initialization::operator_call_1_struct_Trivial(&<test_initialization>::default())
    });
    (unsafe {
        test_initialization::operator_call_1_struct_NonTrivial(&<test_initialization>::default())
    });
    (unsafe {
        test_initialization::operator_call_1_struct_NonEmptyNonTrivial(
            &<test_initialization>::default(),
        )
    });
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {}
const _: () = assert!(!(false), "!is_list_initializable_int<std::array<Foo, 0>>");
const _: () = assert!(!(false), "!is_list_initializable_int<std::array<Foo, 1>>");
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
    (unsafe { with_all_types_1() });
    (unsafe { test_initializer_list_0() });
    const _: () = assert!(true, "with_all_types<test_initialization>()");;
    const _: () = assert!(true, "test_initializer_list()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
