extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reference_wrapper_0 {}
impl Clone for reference_wrapper_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reference_wrapper_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reference_wrapper_0 {
    pub unsafe fn copy_from(_a0: *const reference_wrapper_0) -> Self {
        unimplemented!()
    }
}
impl reference_wrapper_0 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reference_wrapper_0,
    ) -> *mut reference_wrapper_0 {
        unimplemented!()
    }
}
impl reference_wrapper_0 {
    pub unsafe fn to_pmuti32(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl reference_wrapper_0 {
    pub unsafe fn get(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl reference_wrapper_0 {
    pub unsafe fn new(__uref: *mut i32) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reference_wrapper_1 {}
impl Clone for reference_wrapper_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reference_wrapper_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reference_wrapper_1 {
    pub unsafe fn copy_from(_a0: *const reference_wrapper_1) -> Self {
        unimplemented!()
    }
}
impl reference_wrapper_1 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reference_wrapper_1,
    ) -> *mut reference_wrapper_1 {
        unimplemented!()
    }
}
impl reference_wrapper_1 {
    pub unsafe fn to_pmutPoint(&self) -> *mut Point {
        unimplemented!()
    }
}
impl reference_wrapper_1 {
    pub unsafe fn get(&self) -> *mut Point {
        unimplemented!()
    }
}
impl reference_wrapper_1 {
    pub unsafe fn new(__uref: *mut Point) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_2 {}
impl Clone for integral_constant_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_2 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_integral_constant_bool__true__value_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_2 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_2 {}
impl integral_constant_2 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_4 {}
impl Clone for integral_constant_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_4 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_integral_constant_bool__false__value_5: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_4 {}
impl integral_constant_4 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_6 {}
impl Clone for integral_constant_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_6 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_7 {}
impl Clone for integral_constant_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_8 {}
impl Clone for integral_constant_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_9 {}
impl Clone for enable_if_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_10 {}
impl Clone for enable_if_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_11 {}
impl Clone for enable_if_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_12 {}
impl Clone for enable_if_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_13 {}
impl Clone for enable_if_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_14 {}
impl Clone for enable_if_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_15 {}
impl Clone for enable_if_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_15 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_16 {}
impl Clone for enable_if_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_17 {}
impl Clone for enable_if_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_18 {}
impl Clone for __conditional_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_19 {}
impl Clone for __conditional_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_20 {}
impl Clone for __type_identity_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_21 {}
impl Clone for __type_identity_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_22 {}
impl Clone for __type_identity_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_23 {}
impl Clone for __type_identity_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_24 {}
impl Clone for __type_identity_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_25 {}
impl Clone for __type_identity_25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_26 {}
impl Clone for __type_identity_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_27 {}
impl Clone for __type_identity_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_28 {}
impl Clone for __type_identity_28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_29 {}
impl Clone for __type_identity_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_30 {}
impl Clone for __type_identity_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_31 {}
impl Clone for __type_identity_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_32 {}
impl Clone for __type_identity_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_33 {}
impl Clone for __type_identity_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_34 {}
impl Clone for __type_identity_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_35 {}
impl Clone for __type_identity_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__36 {}
impl Clone for __or__36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__37 {}
impl Clone for __or__37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__38 {}
impl Clone for __or__38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__39 {}
impl Clone for __or__39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__40 {}
impl Clone for __or__40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__41 {}
impl Clone for __or__41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__42 {}
impl Clone for __or__42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__43 {}
impl Clone for __or__43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__44 {}
impl Clone for __or__44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__45 {}
impl Clone for __or__45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__46 {}
impl Clone for __or__46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__47 {}
impl Clone for __or__47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__48 {}
impl Clone for __or__48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__49 {}
impl Clone for __or__49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__50 {}
impl Clone for __or__50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__51 {}
impl Clone for __or__51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__52 {}
impl Clone for __or__52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__53 {}
impl Clone for __or__53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__54 {}
impl Clone for __or__54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__55 {}
impl Clone for __or__55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__56 {}
impl Clone for __or__56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__57 {}
impl Clone for __or__57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__58 {}
impl Clone for __and__58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__59 {}
impl Clone for __and__59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__60 {}
impl Clone for __and__60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__61 {}
impl Clone for __and__61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__62 {}
impl Clone for __and__62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__63 {}
impl Clone for __and__63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__64 {}
impl Clone for __and__64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__65 {}
impl Clone for __and__65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__66 {}
impl Clone for __and__66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__67 {}
impl Clone for __and__67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__68 {}
impl Clone for __and__68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__69 {}
impl Clone for __and__69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__70 {}
impl Clone for __and__70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__71 {}
impl Clone for __and__71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__72 {}
impl Clone for __and__72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__73 {}
impl Clone for __and__73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__74 {}
impl Clone for __and__74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__75 {}
impl Clone for __and__75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__76 {}
impl Clone for __and__76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__77 {}
impl Clone for __and__77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__78 {}
impl Clone for __and__78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__79 {}
impl Clone for __and__79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__80 {}
impl Clone for __and__80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__81 {}
impl Clone for __and__81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__82 {}
impl Clone for __and__82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__83 {}
impl Clone for __and__83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__84 {}
impl Clone for __and__84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__85 {}
impl Clone for __and__85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__86 {}
impl Clone for __and__86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__87 {}
impl Clone for __not__87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__88 {}
impl Clone for __not__88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__89 {}
impl Clone for __not__89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__90 {}
impl Clone for __not__90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__91 {}
impl Clone for __not__91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__92 {}
impl Clone for __not__92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__93 {}
impl Clone for __not__93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__94 {}
impl Clone for __not__94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__95 {}
impl Clone for __not__95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__96 {}
impl Clone for __not__96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_97 {}
impl Clone for is_reference_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_98 {}
impl Clone for is_reference_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_99 {}
impl Clone for is_reference_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_100 {}
impl Clone for is_reference_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_101 {}
impl Clone for is_reference_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_102 {}
impl Clone for is_reference_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_103 {}
impl Clone for is_reference_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_104 {}
impl Clone for is_reference_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_105 {}
impl Clone for is_reference_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_106 {}
impl Clone for is_reference_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_107 {}
impl Clone for is_reference_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_108 {}
impl Clone for is_reference_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_109 {}
impl Clone for is_reference_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_110 {}
impl Clone for is_reference_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_111 {}
impl Clone for is_reference_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_112 {}
impl Clone for is_reference_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_113 {}
impl Clone for is_function_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_114 {}
impl Clone for is_function_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_115 {}
impl Clone for is_function_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_116 {}
impl Clone for is_function_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_117 {}
impl Clone for is_function_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_118 {}
impl Clone for is_function_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_119 {}
impl Clone for is_function_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_120 {}
impl Clone for is_function_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_121 {}
impl Clone for is_function_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_122 {}
impl Clone for is_function_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_123 {}
impl Clone for is_function_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_124 {}
impl Clone for is_function_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_125 {}
impl Clone for is_function_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_126 {}
impl Clone for is_function_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_127 {}
impl Clone for is_function_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_128 {}
impl Clone for is_function_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_129 {}
impl Clone for is_void_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_130 {}
impl Clone for is_void_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_131 {}
impl Clone for is_void_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_132 {}
impl Clone for is_void_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_133 {}
impl Clone for is_void_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_134 {}
impl Clone for is_void_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_135 {}
impl Clone for is_void_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_136 {}
impl Clone for is_void_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_137 {}
impl Clone for is_void_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_138 {}
impl Clone for is_void_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_139 {}
impl Clone for is_void_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_140 {}
impl Clone for is_void_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_141 {}
impl Clone for is_void_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_142 {}
impl Clone for is_void_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_143 {}
impl Clone for is_void_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_144 {}
impl Clone for is_void_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_145 {}
impl Clone for is_void_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_146 {}
impl Clone for is_void_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_147 {}
impl Clone for is_void_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_148 {}
impl Clone for is_void_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_149 {}
impl Clone for is_void_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_150 {}
impl Clone for is_void_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_151 {}
impl Clone for is_void_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_152 {}
impl Clone for is_void_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_153 {}
impl Clone for is_void_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_154 {}
impl Clone for remove_cv_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_155 {}
impl Clone for remove_cv_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_156 {}
impl Clone for remove_cv_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_157 {}
impl Clone for remove_cv_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_158 {}
impl Clone for remove_cv_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_159 {}
impl Clone for remove_cv_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_160 {}
impl Clone for remove_cv_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_161 {}
impl Clone for remove_cv_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_162 {}
impl Clone for remove_cv_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_163 {}
impl Clone for remove_cv_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_164 {}
impl Clone for remove_cv_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_165 {}
impl Clone for remove_cv_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_166 {}
impl Clone for is_const_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_167 {}
impl Clone for is_const_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_168 {}
impl Clone for is_const_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_169 {}
impl Clone for is_const_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_170 {}
impl Clone for is_const_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_171 {}
impl Clone for __is_array_unknown_bounds_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_172 {}
impl Clone for __is_array_unknown_bounds_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_173 {}
impl Clone for __is_array_unknown_bounds_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_174 {}
impl Clone for __is_array_unknown_bounds_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_175 {}
impl Clone for __is_array_unknown_bounds_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_176 {}
impl Clone for __is_array_unknown_bounds_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_177 {}
impl Clone for __is_array_unknown_bounds_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_178 {}
impl Clone for __is_array_unknown_bounds_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_179 {}
impl Clone for __is_array_unknown_bounds_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_180 {}
impl Clone for __is_array_unknown_bounds_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_181 {}
impl Clone for __is_array_unknown_bounds_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_182 {}
impl Clone for __is_array_unknown_bounds_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_183 {}
impl Clone for __is_array_unknown_bounds_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_184 {}
impl Clone for __is_array_unknown_bounds_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_185 {}
impl Clone for __is_array_unknown_bounds_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_186 {}
impl Clone for __is_array_unknown_bounds_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_187 {}
impl Clone for __is_integral_helper_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_188 {}
impl Clone for __is_integral_helper_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_189 {}
impl Clone for __is_integral_helper_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_190 {}
impl Clone for __is_integral_helper_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_191 {}
impl Clone for __is_integral_helper_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_192 {}
impl Clone for is_integral_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_193 {}
impl Clone for is_integral_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_194 {}
impl Clone for is_integral_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_195 {}
impl Clone for is_integral_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_196 {}
impl Clone for is_integral_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_197 {}
impl Clone for is_pointer_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_198 {}
impl Clone for is_pointer_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_object_pointer_199 {}
impl Clone for is_member_object_pointer_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_object_pointer_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_function_pointer_200 {}
impl Clone for is_member_function_pointer_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_function_pointer_201 {}
impl Clone for is_member_function_pointer_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_function_pointer_202 {}
impl Clone for is_member_function_pointer_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_class_203 {}
impl Clone for is_class_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_class_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_204 {}
impl Clone for is_same_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_205 {}
impl Clone for is_same_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_206 {}
impl Clone for is_same_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_207 {}
impl Clone for is_same_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_208 {}
impl Clone for is_same_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_209 {}
impl Clone for is_same_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_210 {}
impl Clone for is_same_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_211 {}
impl Clone for is_same_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_212 {}
impl Clone for is_same_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_213 {}
impl Clone for is_same_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_214 {}
impl Clone for is_same_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_215 {}
impl Clone for is_same_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_216 {}
impl Clone for is_same_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_217 {}
impl Clone for is_same_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_218 {}
impl Clone for is_same_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_219 {}
impl Clone for is_same_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_220 {}
impl Clone for is_same_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_221 {}
impl Clone for is_same_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_222 {}
impl Clone for is_same_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_223 {}
impl Clone for is_same_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_224 {}
impl Clone for is_same_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_225 {}
impl Clone for is_same_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_226 {}
impl Clone for is_same_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_227 {}
impl Clone for is_same_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_228 {}
impl Clone for is_same_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_229 {}
impl Clone for is_volatile_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_230 {}
impl Clone for is_volatile_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_231 {}
impl Clone for is_volatile_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_232 {}
impl Clone for is_volatile_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_233 {}
impl Clone for is_volatile_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___do_is_destructible_impl {}
impl Clone for std___do_is_destructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_destructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___do_is_nt_destructible_impl {}
impl Clone for std___do_is_nt_destructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_nt_destructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_234 {}
impl Clone for is_constructible_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_235 {}
impl Clone for is_constructible_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_236 {}
impl Clone for is_constructible_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_237 {}
impl Clone for is_constructible_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_238 {}
impl Clone for is_constructible_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_239 {}
impl Clone for is_default_constructible_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_240 {}
impl Clone for is_default_constructible_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_241 {}
impl Clone for is_default_constructible_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_copy_constructible_242 {}
impl Clone for is_copy_constructible_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_constructible_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_243 {}
impl Clone for is_move_constructible_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_244 {}
impl Clone for is_move_constructible_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_245 {}
impl Clone for is_move_constructible_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_246 {}
impl Clone for is_nothrow_default_constructible_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_247 {}
impl Clone for is_nothrow_default_constructible_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_248 {}
impl Clone for is_nothrow_default_constructible_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_249 {}
impl Clone for is_nothrow_default_constructible_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_250 {}
impl Clone for is_nothrow_copy_constructible_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_251 {}
impl Clone for is_nothrow_copy_constructible_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_252 {}
impl Clone for is_nothrow_copy_constructible_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_253 {}
impl Clone for is_nothrow_copy_constructible_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_254 {}
impl Clone for is_nothrow_copy_constructible_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_255 {}
impl Clone for is_nothrow_move_constructible_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_256 {}
impl Clone for is_nothrow_move_constructible_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_257 {}
impl Clone for is_nothrow_move_constructible_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_copy_assignable_258 {}
impl Clone for is_copy_assignable_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_copy_assignable_259 {}
impl Clone for is_copy_assignable_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_260 {}
impl Clone for is_move_assignable_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_261 {}
impl Clone for is_move_assignable_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_262 {}
impl Clone for is_move_assignable_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_263 {}
impl Clone for is_move_assignable_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_264 {}
impl Clone for is_nothrow_move_assignable_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_265 {}
impl Clone for is_nothrow_move_assignable_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_266 {}
impl Clone for is_nothrow_move_assignable_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___do_is_implicitly_default_constructible_impl {}
impl Clone for std___do_is_implicitly_default_constructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_implicitly_default_constructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_267 {}
impl Clone for is_convertible_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_268 {}
impl Clone for is_convertible_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_269 {}
impl Clone for is_convertible_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_270 {}
impl Clone for is_convertible_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_271 {}
impl Clone for is_convertible_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_272 {}
impl Clone for is_convertible_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_273 {}
impl Clone for is_convertible_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_274 {}
impl Clone for is_convertible_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_275 {}
impl Clone for is_convertible_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_276 {}
impl Clone for is_convertible_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_277 {}
impl Clone for is_convertible_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_278 {}
impl Clone for is_convertible_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_279 {}
impl Clone for is_convertible_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_280 {}
impl Clone for is_convertible_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_281 {}
impl Clone for is_convertible_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_282 {}
impl Clone for is_convertible_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_283 {}
impl Clone for remove_reference_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_284 {}
impl Clone for remove_reference_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_285 {}
impl Clone for remove_reference_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_286 {}
impl Clone for remove_reference_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_287 {}
impl Clone for remove_reference_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_288 {}
impl Clone for remove_reference_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_289 {}
impl Clone for remove_reference_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_290 {}
impl Clone for remove_reference_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_291 {}
impl Clone for remove_reference_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_292 {}
impl Clone for remove_reference_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_293 {}
impl Clone for remove_reference_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_294 {}
impl Clone for remove_reference_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_295 {}
impl Clone for remove_reference_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_296 {}
impl Clone for remove_reference_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_297 {}
impl Clone for remove_reference_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_298 {}
impl Clone for remove_reference_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_299 {}
impl Clone for remove_reference_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_300 {}
impl Clone for __cv_selector_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_301 {}
impl Clone for __cv_selector_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_302 {}
impl Clone for __cv_selector_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_303 {}
impl Clone for __cv_selector_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_304 {}
impl Clone for __match_cv_qualifiers_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_305 {}
impl Clone for __match_cv_qualifiers_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_306 {}
impl Clone for __match_cv_qualifiers_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_307 {}
impl Clone for __match_cv_qualifiers_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_308 {}
impl Clone for __match_cv_qualifiers_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_309 {}
impl Clone for __make_unsigned_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_310 {}
impl Clone for __make_unsigned_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_311 {}
impl Clone for __make_unsigned_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_312 {}
impl Clone for __make_unsigned_selector_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_313 {}
impl Clone for __make_unsigned_selector_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_314 {}
impl Clone for __make_unsigned_selector_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_315 {}
impl Clone for __make_unsigned_selector_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_316 {}
impl Clone for __make_unsigned_selector_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_317 {}
impl Clone for __make_unsigned_selector_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___make_unsigned_selector_base {}
impl Clone for std___make_unsigned_selector_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___make_unsigned_selector_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_321 {}
impl Clone for _List_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_322 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_323 {}
impl Clone for _List_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_324 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_325 {}
impl Clone for _List_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_326 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_327 {}
impl Clone for _List_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_328 {}
impl Clone for _List_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_329 {}
impl Clone for _List_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_330 {}
impl Clone for __select_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_331 {}
impl Clone for __select_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_332 {}
impl Clone for __select_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_333 {}
impl Clone for __select_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_334 {}
impl Clone for __select_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_335 {}
impl Clone for __make_signed_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_336 {}
impl Clone for __make_signed_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_337 {}
impl Clone for __make_signed_selector_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_338 {}
impl Clone for __make_signed_selector_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_339 {}
impl Clone for __make_signed_selector_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_340 {}
impl Clone for __make_signed_selector_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_341 {}
impl Clone for __make_signed_selector_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct add_pointer_342 {}
impl Clone for add_pointer_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_pointer_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___aligned_storage_max_align_t {}
impl Clone for std___aligned_storage_max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___aligned_storage_max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct common_type_343 {}
impl Clone for common_type_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __success_type_344 {}
impl Clone for __success_type_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __success_type_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___failure_type {}
impl Clone for std___failure_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___failure_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___do_common_type_impl {}
impl Clone for std___do_common_type_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_common_type_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_memfun_ref {}
impl Clone for std___invoke_memfun_ref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memfun_ref {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_memfun_ref {}
impl std___invoke_memfun_ref {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_memfun_deref {}
impl Clone for std___invoke_memfun_deref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memfun_deref {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_memfun_deref {}
impl std___invoke_memfun_deref {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_memobj_ref {}
impl Clone for std___invoke_memobj_ref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memobj_ref {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_memobj_ref {}
impl std___invoke_memobj_ref {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_memobj_deref {}
impl Clone for std___invoke_memobj_deref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memobj_deref {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_memobj_deref {}
impl std___invoke_memobj_deref {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_other {}
impl Clone for std___invoke_other {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_other {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___invoke_other {}
impl std___invoke_other {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __result_of_success_345 {}
impl Clone for __result_of_success_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_success_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_memfun_ref_impl {}
impl Clone for std___result_of_memfun_ref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memfun_ref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_memfun_deref_impl {}
impl Clone for std___result_of_memfun_deref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memfun_deref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_memobj_ref_impl {}
impl Clone for std___result_of_memobj_ref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memobj_ref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_memobj_deref_impl {}
impl Clone for std___result_of_memobj_deref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memobj_deref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __result_of_impl_346 {}
impl Clone for __result_of_impl_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_impl_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_other_impl {}
impl Clone for std___result_of_other_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_other_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __invoke_result_347 {}
impl Clone for __invoke_result_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __invoke_result_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __detector_348 {}
impl Clone for __detector_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __detector_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_impl_349 {}
impl Clone for __is_tuple_like_impl_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_impl_350 {}
impl Clone for __is_tuple_like_impl_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_impl_351 {}
impl Clone for __is_tuple_like_impl_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_352 {}
impl Clone for __is_tuple_like_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_353 {}
impl Clone for __is_tuple_like_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_tuple_like_354 {}
impl Clone for __is_tuple_like_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___swappable_details___do_is_swappable_impl {}
impl Clone for std___swappable_details___do_is_swappable_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_details___do_is_swappable_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___swappable_details___do_is_nothrow_swappable_impl {}
impl Clone for std___swappable_details___do_is_nothrow_swappable_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_details___do_is_nothrow_swappable_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___swappable_with_details___do_is_swappable_with_impl {}
impl Clone for std___swappable_with_details___do_is_swappable_with_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_with_details___do_is_swappable_with_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___swappable_with_details___do_is_nothrow_swappable_with_impl {}
impl Clone for std___swappable_with_details___do_is_nothrow_swappable_with_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_with_details___do_is_nothrow_swappable_with_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_invocable_impl_355 {}
impl Clone for __is_invocable_impl_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_impl_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_invocable_356 {}
impl Clone for __is_invocable_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __call_is_nothrow_357 {}
impl Clone for __call_is_nothrow_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __call_is_nothrow_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_invocable_358 {}
impl Clone for __is_nothrow_invocable_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_invocable_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___nonesuchbase {}
impl Clone for std___nonesuchbase {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___nonesuchbase {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___nonesuchbase {}
impl std___nonesuchbase {}
impl std___nonesuchbase {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std___nonesuchbase,
    ) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
impl std___nonesuchbase {
    pub unsafe fn move_assign(&mut self, _a0: *mut std___nonesuchbase) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___nonesuch {}
impl Clone for std___nonesuch {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___nonesuch {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut is_array_v_359: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_360: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_361: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_362: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_363: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_364: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_365: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_366: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_367: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_368: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_369: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_370: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_371: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_372: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_373: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_array_v_374: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_copyable_v_375: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_trivially_default_constructible_v_376: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_standard_layout_v_377: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_378: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_379 {}
impl Clone for binary_function_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_380 {}
impl Clone for binary_function_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_380 {}
impl binary_function_380 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_381 {}
impl Clone for binary_function_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_381 {}
impl binary_function_381 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_382 {}
impl Clone for binary_function_382 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_382 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl binary_function_382 {}
impl binary_function_382 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct equal_to_383 {}
impl Clone for equal_to_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for equal_to_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct equal_to_384 {}
impl Clone for equal_to_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for equal_to_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl equal_to_384 {}
impl equal_to_384 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_385 {}
impl Clone for greater_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_386 {}
impl Clone for less_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_387 {}
impl Clone for less_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_387 {
    pub unsafe fn operator_call(
        &self,
        mut __x: *const std_error_category,
        mut __y: *const std_error_category,
    ) -> bool {
        unimplemented!()
    }
}
impl less_387 {}
impl less_387 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_388 {}
impl Clone for less_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl less_388 {
    pub unsafe fn operator_call(
        &self,
        mut __x: *const libc::c_char,
        mut __y: *const libc::c_char,
    ) -> bool {
        unimplemented!()
    }
}
impl less_388 {}
impl less_388 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_389 {}
impl Clone for greater_equal_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_390 {}
impl Clone for less_equal_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_in_place_t {}
impl Clone for std_in_place_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_in_place_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_in_place_t {}
impl std_in_place_t {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Swallow_assign {}
impl Clone for std__Swallow_assign {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Swallow_assign {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_piecewise_construct_t {}
impl Clone for std_piecewise_construct_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_piecewise_construct_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_piecewise_construct_t {}
impl std_piecewise_construct_t {}
pub static mut piecewise_construct_391: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg)]
pub struct pair_392 {
    pub first: bool,
    pub second: u64,
}
impl Clone for pair_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pair_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl pair_392 {
    pub unsafe fn copy_from(_a0: *const pair_392) -> Self {
        unimplemented!()
    }
}
impl pair_392 {
    pub unsafe fn move_from(_a0: *mut pair_392) -> Self {
        unimplemented!()
    }
}
impl pair_392 {
    pub unsafe fn copy_assign(&mut self, __p: *const pair_392) -> *mut pair_392 {
        unimplemented!()
    }
}
impl pair_392 {
    pub unsafe fn move_assign(&mut self, __p: *mut pair_392) -> *mut pair_392 {
        unimplemented!()
    }
}
impl pair_392 {
    pub unsafe fn new_1(__x: *mut bool, __y: *mut u64) -> Self {
        unimplemented!()
    }
}
impl pair_392 {
    pub unsafe fn new_2(__x: *mut bool, __y: *mut i32) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_393 {}
impl Clone for tuple_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_393 {}
impl tuple_393 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _PCC_394 {}
impl Clone for _PCC_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _PCC_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __pair_base_395 {}
impl Clone for __pair_base_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pair_base_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___erased_type {}
impl Clone for std___erased_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___erased_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_allocator_arg_t {}
impl Clone for std_allocator_arg_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_allocator_arg_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_allocator_arg_t {}
impl std_allocator_arg_t {}
pub static mut allocator_arg_396: std::cell::LazyCell<std_allocator_arg_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___uses_alloc_base {}
impl Clone for std___uses_alloc_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___uses_alloc_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std___uses_alloc0 {
    pub _M_a: std___uses_alloc0__Sink,
}
impl Clone for std___uses_alloc0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___uses_alloc0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_397 {}
impl Clone for __hash_base_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_398 {}
impl Clone for __hash_base_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_399 {}
impl Clone for __hash_base_399 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_399 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_400 {}
impl Clone for __hash_base_400 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_400 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_401 {}
impl Clone for __hash_base_401 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_401 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_402 {}
impl Clone for __hash_base_402 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_402 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_403 {}
impl Clone for __hash_base_403 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_403 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_404 {}
impl Clone for __hash_base_404 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_404 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_405 {}
impl Clone for __hash_base_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __hash_base_405 {}
impl __hash_base_405 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_406 {}
impl Clone for __hash_base_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_407 {}
impl Clone for __hash_base_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_408 {}
impl Clone for __hash_base_408 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_408 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_409 {}
impl Clone for __hash_base_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_410 {}
impl Clone for __hash_base_410 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_410 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_411 {}
impl Clone for __hash_base_411 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_411 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_412 {}
impl Clone for __hash_base_412 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_412 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_413 {}
impl Clone for __hash_base_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_414 {}
impl Clone for __hash_base_414 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_414 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_415 {}
impl Clone for __hash_base_415 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_415 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_416 {}
impl Clone for __hash_base_416 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_416 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_417 {}
impl Clone for __hash_base_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_418 {}
impl Clone for __hash_base_418 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_418 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_419 {}
impl Clone for __hash_base_419 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_419 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_420 {}
impl Clone for __hash_base_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_421 {}
impl Clone for __hash_base_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_421 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_422 {}
impl Clone for __hash_base_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_422 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_423 {}
impl Clone for __hash_base_423 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_423 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_424 {}
impl Clone for hash_424 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_424 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl hash_424 {}
impl hash_424 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_425 {}
impl Clone for hash_425 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_425 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_426 {}
impl Clone for hash_426 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_426 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_427 {}
impl Clone for hash_427 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_427 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_428 {}
impl Clone for hash_428 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_428 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_429 {}
impl Clone for hash_429 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_429 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Hash_impl {}
impl Clone for std__Hash_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Hash_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash(
        mut __ptr: *const ::libc::c_void,
        mut __clength: usize,
        mut __seed: Option<usize>,
    ) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_430(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_431(__val: *const f64) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_432(__val: *const i32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn __hash_combine_433(
        __val: *const *const std_error_category,
        mut __hash: usize,
    ) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_fast_hash_434 {}
impl Clone for __is_fast_hash_434 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_fast_hash_434 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Maybe_get_result_type_435 {}
impl Clone for _Maybe_get_result_type_435 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Maybe_get_result_type_435 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_435 {}
impl _Maybe_get_result_type_435 {}
impl _Maybe_get_result_type_435 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Maybe_get_result_type_435,
    ) -> *mut _Maybe_get_result_type_435 {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_435 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Maybe_get_result_type_435,
    ) -> *mut _Maybe_get_result_type_435 {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_435 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Maybe_get_result_type_436 {}
impl Clone for _Maybe_get_result_type_436 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Maybe_get_result_type_436 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_436 {}
impl _Maybe_get_result_type_436 {}
impl _Maybe_get_result_type_436 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Maybe_get_result_type_436,
    ) -> *mut _Maybe_get_result_type_436 {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_436 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Maybe_get_result_type_436,
    ) -> *mut _Maybe_get_result_type_436 {
        unimplemented!()
    }
}
impl _Maybe_get_result_type_436 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Weak_result_type_impl_437 {}
impl Clone for _Weak_result_type_impl_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_impl_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_437 {}
impl _Weak_result_type_impl_437 {}
impl _Weak_result_type_impl_437 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Weak_result_type_impl_437,
    ) -> *mut _Weak_result_type_impl_437 {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_437 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Weak_result_type_impl_437,
    ) -> *mut _Weak_result_type_impl_437 {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_437 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Weak_result_type_impl_438 {}
impl Clone for _Weak_result_type_impl_438 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_impl_438 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_438 {}
impl _Weak_result_type_impl_438 {}
impl _Weak_result_type_impl_438 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Weak_result_type_impl_438,
    ) -> *mut _Weak_result_type_impl_438 {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_438 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Weak_result_type_impl_438,
    ) -> *mut _Weak_result_type_impl_438 {
        unimplemented!()
    }
}
impl _Weak_result_type_impl_438 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Weak_result_type_memfun_439 {}
impl Clone for _Weak_result_type_memfun_439 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_memfun_439 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_439 {}
impl _Weak_result_type_memfun_439 {}
impl _Weak_result_type_memfun_439 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Weak_result_type_memfun_439,
    ) -> *mut _Weak_result_type_memfun_439 {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_439 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Weak_result_type_memfun_439,
    ) -> *mut _Weak_result_type_memfun_439 {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_439 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Weak_result_type_memfun_440 {}
impl Clone for _Weak_result_type_memfun_440 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_memfun_440 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_440 {}
impl _Weak_result_type_memfun_440 {}
impl _Weak_result_type_memfun_440 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Weak_result_type_memfun_440,
    ) -> *mut _Weak_result_type_memfun_440 {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_440 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Weak_result_type_memfun_440,
    ) -> *mut _Weak_result_type_memfun_440 {
        unimplemented!()
    }
}
impl _Weak_result_type_memfun_440 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Weak_result_type_441 {}
impl Clone for _Weak_result_type_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_441 {}
impl _Weak_result_type_441 {}
impl _Weak_result_type_441 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Weak_result_type_441,
    ) -> *mut _Weak_result_type_441 {
        unimplemented!()
    }
}
impl _Weak_result_type_441 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Weak_result_type_441,
    ) -> *mut _Weak_result_type_441 {
        unimplemented!()
    }
}
impl _Weak_result_type_441 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Weak_result_type_442 {}
impl Clone for _Weak_result_type_442 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Weak_result_type_442 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Weak_result_type_442 {}
impl _Weak_result_type_442 {}
impl _Weak_result_type_442 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Weak_result_type_442,
    ) -> *mut _Weak_result_type_442 {
        unimplemented!()
    }
}
impl _Weak_result_type_442 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Weak_result_type_442,
    ) -> *mut _Weak_result_type_442 {
        unimplemented!()
    }
}
impl _Weak_result_type_442 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Refwrap_base_arg1_443 {}
impl Clone for _Refwrap_base_arg1_443 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Refwrap_base_arg1_443 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_443 {}
impl _Refwrap_base_arg1_443 {}
impl _Refwrap_base_arg1_443 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Refwrap_base_arg1_443,
    ) -> *mut _Refwrap_base_arg1_443 {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_443 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Refwrap_base_arg1_443,
    ) -> *mut _Refwrap_base_arg1_443 {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_443 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Refwrap_base_arg1_444 {}
impl Clone for _Refwrap_base_arg1_444 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Refwrap_base_arg1_444 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_444 {}
impl _Refwrap_base_arg1_444 {}
impl _Refwrap_base_arg1_444 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Refwrap_base_arg1_444,
    ) -> *mut _Refwrap_base_arg1_444 {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_444 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Refwrap_base_arg1_444,
    ) -> *mut _Refwrap_base_arg1_444 {
        unimplemented!()
    }
}
impl _Refwrap_base_arg1_444 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Refwrap_base_arg2_445 {}
impl Clone for _Refwrap_base_arg2_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Refwrap_base_arg2_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_445 {}
impl _Refwrap_base_arg2_445 {}
impl _Refwrap_base_arg2_445 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Refwrap_base_arg2_445,
    ) -> *mut _Refwrap_base_arg2_445 {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_445 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Refwrap_base_arg2_445,
    ) -> *mut _Refwrap_base_arg2_445 {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_445 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Refwrap_base_arg2_446 {}
impl Clone for _Refwrap_base_arg2_446 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Refwrap_base_arg2_446 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_446 {}
impl _Refwrap_base_arg2_446 {}
impl _Refwrap_base_arg2_446 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Refwrap_base_arg2_446,
    ) -> *mut _Refwrap_base_arg2_446 {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_446 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Refwrap_base_arg2_446,
    ) -> *mut _Refwrap_base_arg2_446 {
        unimplemented!()
    }
}
impl _Refwrap_base_arg2_446 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Reference_wrapper_base_447 {}
impl Clone for _Reference_wrapper_base_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Reference_wrapper_base_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_447 {}
impl _Reference_wrapper_base_447 {}
impl _Reference_wrapper_base_447 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Reference_wrapper_base_447,
    ) -> *mut _Reference_wrapper_base_447 {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_447 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Reference_wrapper_base_447,
    ) -> *mut _Reference_wrapper_base_447 {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_447 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Reference_wrapper_base_448 {}
impl Clone for _Reference_wrapper_base_448 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Reference_wrapper_base_448 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_448 {}
impl _Reference_wrapper_base_448 {}
impl _Reference_wrapper_base_448 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Reference_wrapper_base_448,
    ) -> *mut _Reference_wrapper_base_448 {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_448 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Reference_wrapper_base_448,
    ) -> *mut _Reference_wrapper_base_448 {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_448 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Reference_wrapper_base_memfun_449 {}
impl Clone for _Reference_wrapper_base_memfun_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Reference_wrapper_base_memfun_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_449 {}
impl _Reference_wrapper_base_memfun_449 {}
impl _Reference_wrapper_base_memfun_449 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Reference_wrapper_base_memfun_449,
    ) -> *mut _Reference_wrapper_base_memfun_449 {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_449 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Reference_wrapper_base_memfun_449,
    ) -> *mut _Reference_wrapper_base_memfun_449 {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_449 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Reference_wrapper_base_memfun_450 {}
impl Clone for _Reference_wrapper_base_memfun_450 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Reference_wrapper_base_memfun_450 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_450 {}
impl _Reference_wrapper_base_memfun_450 {}
impl _Reference_wrapper_base_memfun_450 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Reference_wrapper_base_memfun_450,
    ) -> *mut _Reference_wrapper_base_memfun_450 {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_450 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Reference_wrapper_base_memfun_450,
    ) -> *mut _Reference_wrapper_base_memfun_450 {
        unimplemented!()
    }
}
impl _Reference_wrapper_base_memfun_450 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_exception {}
impl Clone for std_exception {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_exception {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_from(_a0: *const std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_from(_a0: *mut std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_bad_alloc {}
impl Clone for std_bad_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bad_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_from(_a0: *const std_bad_alloc) -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_bad_alloc) -> *mut std_bad_alloc {
        unimplemented!()
    }
}
pub type std_align_val_t = usize;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_nothrow_t {}
impl Clone for std_nothrow_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_nothrow_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_nothrow_t {}
impl std_nothrow_t {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_type_info {}
impl Clone for std_type_info {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_type_info {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_type_info {
    pub unsafe fn name(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl std_type_info {
    pub unsafe fn operator_eq(&self, __arg: *const std_type_info) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Nocopy_types {
    pub _M_object: *mut ::libc::c_void,
    pub _M_const_object: *const ::libc::c_void,
    pub _M_function_pointer: Option<unsafe fn()>,
}
impl Clone for std__Nocopy_types {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Nocopy_types {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Nocopy_types {}
impl std__Nocopy_types {}
#[derive(VaArg, FnPtrArg)]
pub struct std__Any_data {
    pub _M_unused: std__Nocopy_types,
    pub _M_pod_data: [libc::c_char; 16],
}
impl Clone for std__Any_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Any_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Any_data {}
impl std__Any_data {}
impl std__Any_data {}
pub type std__Manager_operation = u32;
pub const std__Manager_operation___get_type_info: std__Manager_operation = 0;
pub const std__Manager_operation___get_functor_ptr: std__Manager_operation = 1;
pub const std__Manager_operation___clone_functor: std__Manager_operation = 2;
pub const std__Manager_operation___destroy_functor: std__Manager_operation = 3;
#[derive(VaArg, FnPtrArg)]
pub struct std__Function_base {
    pub _M_functor: std__Any_data,
    pub _M_manager:
        Option<unsafe fn(*mut std__Any_data, *const std__Any_data, std__Manager_operation) -> bool>,
}
impl Clone for std__Function_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Function_base {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std__Function_base__M_max_size_451: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std__Function_base__M_max_align_452: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl std__Function_base {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Function_base {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Function_handler_453 {}
impl Clone for _Function_handler_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Function_handler_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_454 {}
impl Clone for initializer_list_454 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_454 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_454 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl initializer_list_454 {
    pub unsafe fn begin(&self) -> *const bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_455 {}
impl Clone for initializer_list_455 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_455 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_455 {}
impl initializer_list_455 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_456 {}
impl Clone for initializer_list_456 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_456 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_456 {}
impl initializer_list_456 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_457 {}
impl Clone for initializer_list_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_457 {}
impl initializer_list_457 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_458 {}
impl Clone for initializer_list_458 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_458 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_458 {}
impl initializer_list_458 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___true_type {}
impl Clone for std___true_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___true_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___false_type {}
impl Clone for std___false_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___false_type {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___false_type {}
impl std___false_type {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_459 {}
impl Clone for __are_same_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_460 = u32;
pub const anon_460___value: anon_460 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_461 {}
impl Clone for __are_same_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_462 = u32;
pub const anon_462___value: anon_462 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_463 {}
impl Clone for __are_same_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_464 = u32;
pub const anon_464___value: anon_464 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_465 {}
impl Clone for __are_same_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_466 = u32;
pub const anon_466___value: anon_466 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_467 {}
impl Clone for __are_same_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_468 = u32;
pub const anon_468___value: anon_468 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_469 {}
impl Clone for __are_same_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_470 = u32;
pub const anon_470___value: anon_470 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_471 {}
impl Clone for __is_integer_471 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_471 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_472 = u32;
pub const anon_472___value: anon_472 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_473 {}
impl Clone for __is_integer_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_473 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_474 = u32;
pub const anon_474___value: anon_474 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_475 {}
impl Clone for __is_integer_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_476 = u32;
pub const anon_476___value: anon_476 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_477 {}
impl Clone for __is_integer_477 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_477 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_478 = u32;
pub const anon_478___value: anon_478 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_479 {}
impl Clone for __is_integer_479 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_479 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_480 = u32;
pub const anon_480___value: anon_480 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_481 {}
impl Clone for __is_integer_481 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_481 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_482 = u32;
pub const anon_482___value: anon_482 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_483 {}
impl Clone for __is_integer_483 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_483 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_484 = u32;
pub const anon_484___value: anon_484 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_485 {}
impl Clone for __is_integer_485 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_485 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_486 = u32;
pub const anon_486___value: anon_486 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_487 {}
impl Clone for __is_integer_487 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_487 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_488 = u32;
pub const anon_488___value: anon_488 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_489 {}
impl Clone for __is_integer_489 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_489 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_490 = u32;
pub const anon_490___value: anon_490 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_491 {}
impl Clone for __memcpyable_integer_491 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_491 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_type_492 {}
impl Clone for __conditional_type_492 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_492 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_493 {}
impl Clone for __is_integer_nonstrict_493 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_493 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_494 = u32;
pub const anon_494___width: anon_494 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_495 {}
impl Clone for __is_integer_nonstrict_495 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_495 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_496 = u32;
pub const anon_496___width: anon_496 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_497 {}
impl Clone for __is_integer_nonstrict_497 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_497 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_498 = u32;
pub const anon_498___width: anon_498 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_499 {}
impl Clone for __is_integer_nonstrict_499 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_499 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_500 = u32;
pub const anon_500___width: anon_500 = 32;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_501 {}
impl Clone for __is_integer_nonstrict_501 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_501 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_502 = u32;
pub const anon_502___width: anon_502 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_503 {}
impl Clone for __is_integer_nonstrict_503 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_503 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_504 = u32;
pub const anon_504___width: anon_504 = 8;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_505 {}
impl Clone for __is_integer_nonstrict_505 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_505 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_506 = u32;
pub const anon_506___width: anon_506 = 16;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_507 {}
impl Clone for __numeric_traits_integer_507 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_507 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_508:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_509: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_510: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_511 {}
impl Clone for __numeric_traits_integer_511 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_511 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_512: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_513: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_514: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____min_515: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_516 {}
impl Clone for __numeric_traits_integer_516 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_516 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_int____is_signed_517: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____digits_518: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____max_519: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_int____min_520: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_521 {}
impl Clone for __numeric_traits_integer_521 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_521 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____is_signed_522:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____digits_523: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_int____max_524: std::cell::LazyCell<
    u32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_525 {}
impl Clone for __numeric_traits_integer_525 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_525 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____is_signed_526:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____digits_527:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long_long____max_528:
    std::cell::LazyCell<u64> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_529 {}
impl Clone for __numeric_traits_integer_529 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_529 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_char____is_signed_530: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_char____digits_531: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_char____max_532: std::cell::LazyCell<
    libc::c_char,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_533 {}
impl Clone for __numeric_traits_integer_533 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_533 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_short____is_signed_534: std::cell::LazyCell<
    bool,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____digits_535: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____max_536: std::cell::LazyCell<i16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_short____min_537: std::cell::LazyCell<i16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_538 {}
impl Clone for __numeric_traits_floating_538 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_538 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_float____max_exponent10_539:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_540 {}
impl Clone for __numeric_traits_floating_540 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_540 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_double____max_exponent10_541:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_542 {}
impl Clone for __numeric_traits_floating_542 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_542 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_floating_long_double____max_exponent10_543:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_544 {}
impl Clone for __numeric_traits_544 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_544 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_545 {}
impl Clone for __numeric_traits_545 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_545 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_546 {}
impl Clone for __numeric_traits_546 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_546 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_547 {}
impl Clone for __numeric_traits_547 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_547 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_548 {}
impl Clone for __numeric_traits_548 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_548 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_549 {}
impl Clone for __numeric_traits_549 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_549 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_550 {}
impl Clone for __numeric_traits_550 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_550 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_551 {}
impl Clone for __numeric_traits_551 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_551 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_input_iterator_tag {}
impl Clone for std_input_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_input_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_input_iterator_tag {}
impl std_input_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_output_iterator_tag {}
impl Clone for std_output_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_output_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_output_iterator_tag {}
impl std_output_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_forward_iterator_tag {}
impl Clone for std_forward_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_forward_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_forward_iterator_tag {}
impl std_forward_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_bidirectional_iterator_tag {}
impl Clone for std_bidirectional_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bidirectional_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bidirectional_iterator_tag {}
impl std_bidirectional_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_random_access_iterator_tag {}
impl Clone for std_random_access_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_random_access_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_random_access_iterator_tag {}
impl std_random_access_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_552 {}
impl Clone for iterator_552 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_552 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_553 {}
impl Clone for iterator_553 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_553 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_553 {}
impl iterator_553 {}
impl iterator_553 {}
impl iterator_553 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_553) -> *mut iterator_553 {
        unimplemented!()
    }
}
impl iterator_553 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_553) -> *mut iterator_553 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_554 {}
impl Clone for iterator_554 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_554 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_554 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_554) -> *mut iterator_554 {
        unimplemented!()
    }
}
impl iterator_554 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_554) -> *mut iterator_554 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_555 {}
impl Clone for iterator_555 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_555 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_555 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_555) -> *mut iterator_555 {
        unimplemented!()
    }
}
impl iterator_555 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_555) -> *mut iterator_555 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_556 {}
impl Clone for iterator_traits_556 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_556 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_557 {}
impl Clone for iterator_traits_557 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_557 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_558 {}
impl Clone for iterator_traits_558 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_558 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_559 {}
impl Clone for iterator_traits_559 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_559 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_560 {}
impl Clone for iterator_traits_560 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_560 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_561 {}
impl Clone for iterator_traits_561 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_561 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_562 {}
impl Clone for iterator_traits_562 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_562 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_563 {}
impl Clone for __iterator_traits_563 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_563 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_564 {}
impl Clone for __iterator_traits_564 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_564 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_565 {}
impl Clone for __ptr_traits_ptr_to_565 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_565 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_565 {
    pub unsafe fn pointer_to(__r: *mut libc::c_char) -> *mut libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_566 {}
impl Clone for __ptr_traits_ptr_to_566 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_566 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_566 {
    pub unsafe fn pointer_to(__r: *const libc::c_char) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_567 {}
impl Clone for __ptr_traits_ptr_to_567 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_567 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_567 {
    pub unsafe fn pointer_to(__r: *mut i32) -> *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_568 {}
impl Clone for __ptr_traits_ptr_to_568 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_568 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_568 {
    pub unsafe fn pointer_to(__r: *const i32) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_569 {}
impl Clone for __ptr_traits_ptr_to_569 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_569 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_569 {
    pub unsafe fn pointer_to(__r: *mut u16) -> *mut u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_570 {}
impl Clone for __ptr_traits_ptr_to_570 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_570 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_570 {
    pub unsafe fn pointer_to(__r: *const u16) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_571 {}
impl Clone for __ptr_traits_ptr_to_571 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_571 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_571 {
    pub unsafe fn pointer_to(__r: *mut u32) -> *mut u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_572 {}
impl Clone for __ptr_traits_ptr_to_572 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_572 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __ptr_traits_ptr_to_572 {
    pub unsafe fn pointer_to(__r: *const u32) -> *const u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_573 {}
impl Clone for pointer_traits_573 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_573 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_574 {}
impl Clone for pointer_traits_574 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_574 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_575 {}
impl Clone for pointer_traits_575 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_575 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_576 {}
impl Clone for pointer_traits_576 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_576 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_577 {}
impl Clone for pointer_traits_577 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_577 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_578 {}
impl Clone for pointer_traits_578 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_578 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_579 {}
impl Clone for pointer_traits_579 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_579 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_580 {}
impl Clone for pointer_traits_580 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_580 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_581 {}
impl Clone for reverse_iterator_581 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_581 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_581 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_581) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_581 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_581,
    ) -> *mut reverse_iterator_581 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_582 {}
impl Clone for reverse_iterator_582 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_582 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_582 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_582) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_582 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_582,
    ) -> *mut reverse_iterator_582 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_583 {}
impl Clone for __normal_iterator_583 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_583 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_583 {}
impl __normal_iterator_583 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Iter_less_iter {}
impl Clone for __gnu_cxx___ops__Iter_less_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_less_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_iter {}
impl __gnu_cxx___ops__Iter_less_iter {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Iter_less_val {}
impl Clone for __gnu_cxx___ops__Iter_less_val {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_less_val {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_val {}
impl __gnu_cxx___ops__Iter_less_val {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Val_less_iter {}
impl Clone for __gnu_cxx___ops__Val_less_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Val_less_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Val_less_iter {}
impl __gnu_cxx___ops__Val_less_iter {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Iter_equal_to_iter {}
impl Clone for __gnu_cxx___ops__Iter_equal_to_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_equal_to_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_equal_to_iter {}
impl __gnu_cxx___ops__Iter_equal_to_iter {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Iter_equal_to_val {}
impl Clone for __gnu_cxx___ops__Iter_equal_to_val {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_equal_to_val {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_equal_to_val {}
impl __gnu_cxx___ops__Iter_equal_to_val {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_584 {}
impl Clone for char_traits_584 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_584 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_584 {
    pub unsafe fn assign_1(__c1: *mut libc::c_char, __c2: *const libc::c_char) {
        unimplemented!()
    }
}
impl char_traits_584 {
    pub unsafe fn lt(__c1: *const libc::c_char, __c2: *const libc::c_char) -> bool {
        unimplemented!()
    }
}
impl char_traits_584 {
    pub unsafe fn length(mut __s: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_584 {
    pub unsafe fn move_(
        mut __s1: *mut libc::c_char,
        mut __s2: *const libc::c_char,
        mut __n: usize,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl char_traits_584 {
    pub unsafe fn copy(
        mut __s1: *mut libc::c_char,
        mut __s2: *const libc::c_char,
        mut __n: usize,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl char_traits_584 {
    pub unsafe fn eof() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_585 {}
impl Clone for char_traits_585 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_585 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_585 {
    pub unsafe fn assign_1(__c1: *mut i32, __c2: *const i32) {
        unimplemented!()
    }
}
impl char_traits_585 {
    pub unsafe fn copy(mut __s1: *mut i32, mut __s2: *const i32, mut __n: usize) -> *mut i32 {
        unimplemented!()
    }
}
impl char_traits_585 {
    pub unsafe fn eq_int_type(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_585 {
    pub unsafe fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_586 {}
impl Clone for char_traits_586 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_586 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_586 {
    pub unsafe fn assign_1(__c1: *mut u16, __c2: *const u16) {
        unimplemented!()
    }
}
impl char_traits_586 {
    pub unsafe fn eq(__c1: *const u16, __c2: *const u16) -> bool {
        unimplemented!()
    }
}
impl char_traits_586 {
    pub unsafe fn lt(__c1: *const u16, __c2: *const u16) -> bool {
        unimplemented!()
    }
}
impl char_traits_586 {
    pub unsafe fn copy(mut __s1: *mut u16, mut __s2: *const u16, mut __n: usize) -> *mut u16 {
        unimplemented!()
    }
}
impl char_traits_586 {
    pub unsafe fn eq_int_type(__c1: *const u16, __c2: *const u16) -> bool {
        unimplemented!()
    }
}
impl char_traits_586 {
    pub unsafe fn eof() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_587 {}
impl Clone for char_traits_587 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_587 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_587 {
    pub unsafe fn assign_1(__c1: *mut u32, __c2: *const u32) {
        unimplemented!()
    }
}
impl char_traits_587 {
    pub unsafe fn eq(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_587 {
    pub unsafe fn lt(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_587 {
    pub unsafe fn copy(mut __s1: *mut u32, mut __s2: *const u32, mut __n: usize) -> *mut u32 {
        unimplemented!()
    }
}
impl char_traits_587 {
    pub unsafe fn eq_int_type(__c1: *const u32, __c2: *const u32) -> bool {
        unimplemented!()
    }
}
impl char_traits_587 {
    pub unsafe fn eof() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_588 {}
impl Clone for allocator_588 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_588 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_588 {}
impl allocator_588 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_589 {}
impl Clone for allocator_589 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_589 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_589 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_589 {
    pub unsafe fn copy_from(__a: *const allocator_589) -> Self {
        unimplemented!()
    }
}
impl allocator_589 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_589) -> *mut allocator_589 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_590 {}
impl Clone for allocator_590 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_590 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_590 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_590 {
    pub unsafe fn copy_from(__a: *const allocator_590) -> Self {
        unimplemented!()
    }
}
impl allocator_590 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_590) -> *mut allocator_590 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_591 {}
impl Clone for allocator_591 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_591 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_592 {}
impl Clone for rebind_592 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_592 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_593 {}
impl Clone for rebind_593 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_593 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_591 {
    pub unsafe fn copy_from(__a: *const allocator_591) -> Self {
        unimplemented!()
    }
}
impl allocator_591 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_591) -> *mut allocator_591 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_594 {}
impl Clone for allocator_594 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_594 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_595 {}
impl Clone for rebind_595 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_595 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_596 {}
impl Clone for rebind_596 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_596 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_594 {
    pub unsafe fn copy_from(__a: *const allocator_594) -> Self {
        unimplemented!()
    }
}
impl allocator_594 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_594) -> *mut allocator_594 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_597 {}
impl Clone for allocator_597 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_597 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_597 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_597 {
    pub unsafe fn copy_from(__a: *const allocator_597) -> Self {
        unimplemented!()
    }
}
impl allocator_597 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_597) -> *mut allocator_597 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_598 {}
impl Clone for allocator_598 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_598 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_598 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_598 {
    pub unsafe fn copy_from(__a: *const allocator_598) -> Self {
        unimplemented!()
    }
}
impl allocator_598 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_598) -> *mut allocator_598 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_599 {}
impl Clone for allocator_traits_599 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_599 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_600 {}
impl Clone for allocator_traits_600 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_600 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_601 {}
impl Clone for allocator_traits_601 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_601 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_601 {
    pub unsafe fn allocate_1(__a: *mut allocator_589, mut __n: u64) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl allocator_traits_601 {
    pub unsafe fn deallocate(__a: *mut allocator_589, mut __p: *mut libc::c_char, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_601 {
    pub unsafe fn max_size(__a: *const allocator_589) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_602 {}
impl Clone for allocator_traits_602 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_602 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_602 {
    pub unsafe fn allocate_1(__a: *mut allocator_590, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl allocator_traits_602 {
    pub unsafe fn deallocate(__a: *mut allocator_590, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_602 {
    pub unsafe fn max_size(__a: *const allocator_590) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_603 {}
impl Clone for allocator_traits_603 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_603 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_603 {
    pub unsafe fn allocate_1(__a: *mut allocator_597, mut __n: u64) -> *mut u16 {
        unimplemented!()
    }
}
impl allocator_traits_603 {
    pub unsafe fn deallocate(__a: *mut allocator_597, mut __p: *mut u16, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_603 {
    pub unsafe fn max_size(__a: *const allocator_597) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_604 {}
impl Clone for allocator_traits_604 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_604 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_604 {
    pub unsafe fn allocate_1(__a: *mut allocator_598, mut __n: u64) -> *mut u32 {
        unimplemented!()
    }
}
impl allocator_traits_604 {
    pub unsafe fn deallocate(__a: *mut allocator_598, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_604 {
    pub unsafe fn max_size(__a: *const allocator_598) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_605 {}
impl Clone for __new_allocator_605 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_605 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_605 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_605 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_605) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_605 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_605,
    ) -> *mut __new_allocator_605 {
        unimplemented!()
    }
}
impl __new_allocator_605 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        unimplemented!()
    }
}
impl __new_allocator_605 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut libc::c_char, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_605 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_606 {}
impl Clone for __new_allocator_606 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_606 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_606 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_606 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_606) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_606 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_606,
    ) -> *mut __new_allocator_606 {
        unimplemented!()
    }
}
impl __new_allocator_606 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_606 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_606 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_607 {}
impl Clone for __new_allocator_607 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_607 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_607 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_607) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_607 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_607,
    ) -> *mut __new_allocator_607 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_608 {}
impl Clone for __new_allocator_608 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_608 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_608 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_608) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_608 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_608,
    ) -> *mut __new_allocator_608 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_609 {}
impl Clone for __new_allocator_609 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_609 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_609 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_609 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_609) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_609 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_609,
    ) -> *mut __new_allocator_609 {
        unimplemented!()
    }
}
impl __new_allocator_609 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u16 {
        unimplemented!()
    }
}
impl __new_allocator_609 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u16, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_609 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_610 {}
impl Clone for __new_allocator_610 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_610 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_610 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_610 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_610) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_610 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_610,
    ) -> *mut __new_allocator_610 {
        unimplemented!()
    }
}
impl __new_allocator_610 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut u32 {
        unimplemented!()
    }
}
impl __new_allocator_610 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut u32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_610 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___allocator_traits_base {}
impl Clone for std___allocator_traits_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___allocator_traits_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __rebind_612 {}
impl Clone for __rebind_612 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __rebind_612 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __rebind_613 {}
impl Clone for __rebind_613 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __rebind_613 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_614 {}
impl Clone for __is_allocator_614 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_614 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_615 {}
impl Clone for __is_allocator_615 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_615 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_616 {}
impl Clone for __is_allocator_616 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_616 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_allocator_617 {}
impl Clone for __is_allocator_617 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_allocator_617 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_618 {}
impl Clone for __alloc_traits_618 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_618 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_619 {}
impl Clone for __alloc_traits_619 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_619 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_620 {}
impl Clone for rebind_620 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_620 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_621 {}
impl Clone for __alloc_traits_621 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_621 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_622 {}
impl Clone for rebind_622 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_622 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_623 {}
impl Clone for __alloc_traits_623 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_623 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_624 {}
impl Clone for rebind_624 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_624 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_625 {}
impl Clone for __alloc_traits_625 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_625 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_626 {}
impl Clone for rebind_626 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_626 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Hashtable_627 {}
impl Clone for _Hashtable_627 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_627 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Hashtable_627 {
    pub unsafe fn copy_from(_a0: *const _Hashtable_627) -> Self {
        unimplemented!()
    }
}
impl _Hashtable_627 {
    pub unsafe fn move_from(__ht: *mut _Hashtable_627) -> Self {
        unimplemented!()
    }
}
impl _Hashtable_627 {
    pub unsafe fn copy_assign(&mut self, __ht: *const _Hashtable_627) -> *mut _Hashtable_627 {
        unimplemented!()
    }
}
impl _Hashtable_627 {
    pub unsafe fn move_assign(&mut self, __ht: *mut _Hashtable_627) -> *mut _Hashtable_627 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Hashtable_628 {}
impl Clone for _Hashtable_628 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_628 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Hashtable_628 {
    pub unsafe fn copy_from(_a0: *const _Hashtable_628) -> Self {
        unimplemented!()
    }
}
impl _Hashtable_628 {
    pub unsafe fn move_from(__ht: *mut _Hashtable_628) -> Self {
        unimplemented!()
    }
}
impl _Hashtable_628 {
    pub unsafe fn copy_assign(&mut self, __ht: *const _Hashtable_628) -> *mut _Hashtable_628 {
        unimplemented!()
    }
}
impl _Hashtable_628 {
    pub unsafe fn move_assign(&mut self, __ht: *mut _Hashtable_628) -> *mut _Hashtable_628 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Hashtable_base_629 {}
impl Clone for _Hashtable_base_629 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_base_629 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Hashtable_base_630 {}
impl Clone for _Hashtable_base_630 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_base_630 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___detail__Identity {}
impl Clone for std___detail__Identity {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Identity {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___detail__Select1st {}
impl Clone for std___detail__Select1st {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Select1st {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __1st_type_634 {}
impl Clone for __1st_type_634 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __1st_type_634 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___detail__Select1st {}
impl std___detail__Select1st {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _NodeBuilder_635 {}
impl Clone for _NodeBuilder_635 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _NodeBuilder_635 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Hashtable_alloc_636 {}
impl Clone for _Hashtable_alloc_636 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_alloc_636 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __get_value_type_637 {}
impl Clone for __get_value_type_637 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __get_value_type_637 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Hashtable_alloc_636 {
    pub unsafe fn copy_from(_a0: *const _Hashtable_alloc_636) -> Self {
        unimplemented!()
    }
}
impl _Hashtable_alloc_636 {
    pub unsafe fn move_from(_a0: *mut _Hashtable_alloc_636) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Hashtable_traits_638 {}
impl Clone for _Hashtable_traits_638 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_traits_638 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Hashtable_traits_639 {}
impl Clone for _Hashtable_traits_639 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_traits_639 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std___detail__Hash_node_base {
    pub _M_nxt: *mut std___detail__Hash_node_base,
}
impl Clone for std___detail__Hash_node_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Hash_node_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Hash_node_code_cache_640 {
    pub _M_hash_code: usize,
}
impl Clone for _Hash_node_code_cache_640 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hash_node_code_cache_640 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___detail__Mod_range_hashing {}
impl Clone for std___detail__Mod_range_hashing {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Mod_range_hashing {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___detail__Mod_range_hashing {}
impl std___detail__Mod_range_hashing {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___detail__Default_ranged_hash {}
impl Clone for std___detail__Default_ranged_hash {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Default_ranged_hash {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std___detail__Prime_rehash_policy {
    pub _M_max_load_factor: f32,
    pub _M_next_resize: usize,
}
impl Clone for std___detail__Prime_rehash_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Prime_rehash_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std___detail__Power2_rehash_policy {
    pub _M_max_load_factor: f32,
    pub _M_next_resize: usize,
}
impl Clone for std___detail__Power2_rehash_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Power2_rehash_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___detail__Power2_rehash_policy__S_growth_factor_641: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl std___detail__Power2_rehash_policy {
    pub unsafe fn _M_next_bkt(&mut self, mut __n: usize) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Map_base_642 {}
impl Clone for _Map_base_642 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Map_base_642 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Map_base_643 {}
impl Clone for _Map_base_643 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Map_base_643 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Rehash_base_644 {}
impl Clone for _Rehash_base_644 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Rehash_base_644 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Rehash_base_645 {}
impl Clone for _Rehash_base_645 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Rehash_base_645 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Hashtable_ebo_helper_646 {
    pub _M_obj: hash_424,
}
impl Clone for _Hashtable_ebo_helper_646 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_ebo_helper_646 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Hashtable_ebo_helper_647 {
    pub _M_obj: equal_to_384,
}
impl Clone for _Hashtable_ebo_helper_647 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_ebo_helper_647 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Hashtable_ebo_helper_648 {
    pub _M_obj: allocator_594,
}
impl Clone for _Hashtable_ebo_helper_648 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hashtable_ebo_helper_648 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Hashtable_ebo_helper_648 {}
impl _Hashtable_ebo_helper_648 {}
impl _Hashtable_ebo_helper_648 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Hashtable_ebo_helper_648,
    ) -> *mut _Hashtable_ebo_helper_648 {
        unimplemented!()
    }
}
impl _Hashtable_ebo_helper_648 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Hashtable_ebo_helper_648,
    ) -> *mut _Hashtable_ebo_helper_648 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Hash_code_base_649 {}
impl Clone for _Hash_code_base_649 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Hash_code_base_649 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Enable_default_constructor_tag {}
impl Clone for std__Enable_default_constructor_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Enable_default_constructor_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Enable_default_constructor_tag {}
impl std__Enable_default_constructor_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Enable_default_constructor_650 {}
impl Clone for _Enable_default_constructor_650 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Enable_default_constructor_650 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Enable_default_constructor_650 {
    pub unsafe fn copy_from(_a0: *const _Enable_default_constructor_650) -> Self {
        unimplemented!()
    }
}
impl _Enable_default_constructor_650 {
    pub unsafe fn move_from(_a0: *mut _Enable_default_constructor_650) -> Self {
        unimplemented!()
    }
}
impl _Enable_default_constructor_650 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const _Enable_default_constructor_650,
    ) -> *mut _Enable_default_constructor_650 {
        unimplemented!()
    }
}
impl _Enable_default_constructor_650 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut _Enable_default_constructor_650,
    ) -> *mut _Enable_default_constructor_650 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct unordered_multimap_651 {}
impl Clone for unordered_multimap_651 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unordered_multimap_651 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unordered_multimap_651 {
    pub unsafe fn copy_from(_a0: *const unordered_multimap_651) -> Self {
        unimplemented!()
    }
}
impl unordered_multimap_651 {
    pub unsafe fn move_from(_a0: *mut unordered_multimap_651) -> Self {
        unimplemented!()
    }
}
impl unordered_multimap_651 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const unordered_multimap_651,
    ) -> *mut unordered_multimap_651 {
        unimplemented!()
    }
}
impl unordered_multimap_651 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut unordered_multimap_651,
    ) -> *mut unordered_multimap_651 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct unordered_map_652 {}
impl Clone for unordered_map_652 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unordered_map_652 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unordered_map_652 {
    pub unsafe fn copy_from(_a0: *const unordered_map_652) -> Self {
        unimplemented!()
    }
}
impl unordered_map_652 {
    pub unsafe fn move_from(_a0: *mut unordered_map_652) -> Self {
        unimplemented!()
    }
}
impl unordered_map_652 {
    pub unsafe fn copy_assign(&mut self, _a0: *const unordered_map_652) -> *mut unordered_map_652 {
        unimplemented!()
    }
}
impl unordered_map_652 {
    pub unsafe fn move_assign(&mut self, _a0: *mut unordered_map_652) -> *mut unordered_map_652 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
pub type anon_653 = u32;
pub const anon_653__S_word_bit: anon_653 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Bit_reference {}
impl Clone for std__Bit_reference {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_reference {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn copy_from(_a0: *const std__Bit_reference) -> Self {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn operator_assign_5(&mut self, mut __x: bool) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn copy_assign(
        &mut self,
        __x: *const std__Bit_reference,
    ) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Bit_iterator_base {
    pub _M_p: *mut u64,
    pub _M_offset: u32,
}
impl Clone for std__Bit_iterator_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_iterator_base {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_assume_normalized(&self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn new(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_up(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_down(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_incr(&mut self, mut __i: i64) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Bit_iterator {}
impl Clone for std__Bit_iterator {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_iterator {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn new_1(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_deref(&self) -> std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {}
impl std__Bit_iterator {}
impl std__Bit_iterator {
    pub unsafe fn copy_assign(&mut self, _a0: *const std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn move_assign(&mut self, _a0: *mut std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Bit_const_iterator {}
impl Clone for std__Bit_const_iterator {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_const_iterator {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_deref(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_654 {}
impl Clone for _Placeholder_654 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_654 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_654 {}
impl _Placeholder_654 {}
impl _Placeholder_654 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_655 {}
impl Clone for _Placeholder_655 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_655 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_655 {}
impl _Placeholder_655 {}
impl _Placeholder_655 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_656 {}
impl Clone for _Placeholder_656 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_656 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_656 {}
impl _Placeholder_656 {}
impl _Placeholder_656 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_657 {}
impl Clone for _Placeholder_657 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_657 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_657 {}
impl _Placeholder_657 {}
impl _Placeholder_657 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_658 {}
impl Clone for _Placeholder_658 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_658 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_658 {}
impl _Placeholder_658 {}
impl _Placeholder_658 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_659 {}
impl Clone for _Placeholder_659 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_659 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_659 {}
impl _Placeholder_659 {}
impl _Placeholder_659 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_660 {}
impl Clone for _Placeholder_660 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_660 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_660 {}
impl _Placeholder_660 {}
impl _Placeholder_660 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_661 {}
impl Clone for _Placeholder_661 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_661 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_661 {}
impl _Placeholder_661 {}
impl _Placeholder_661 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_662 {}
impl Clone for _Placeholder_662 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_662 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_662 {}
impl _Placeholder_662 {}
impl _Placeholder_662 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_663 {}
impl Clone for _Placeholder_663 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_663 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_663 {}
impl _Placeholder_663 {}
impl _Placeholder_663 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_664 {}
impl Clone for _Placeholder_664 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_664 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_664 {}
impl _Placeholder_664 {}
impl _Placeholder_664 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_665 {}
impl Clone for _Placeholder_665 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_665 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_665 {}
impl _Placeholder_665 {}
impl _Placeholder_665 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_666 {}
impl Clone for _Placeholder_666 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_666 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_666 {}
impl _Placeholder_666 {}
impl _Placeholder_666 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_667 {}
impl Clone for _Placeholder_667 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_667 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_667 {}
impl _Placeholder_667 {}
impl _Placeholder_667 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_668 {}
impl Clone for _Placeholder_668 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_668 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_668 {}
impl _Placeholder_668 {}
impl _Placeholder_668 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_669 {}
impl Clone for _Placeholder_669 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_669 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_669 {}
impl _Placeholder_669 {}
impl _Placeholder_669 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_670 {}
impl Clone for _Placeholder_670 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_670 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_670 {}
impl _Placeholder_670 {}
impl _Placeholder_670 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_671 {}
impl Clone for _Placeholder_671 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_671 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_671 {}
impl _Placeholder_671 {}
impl _Placeholder_671 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_672 {}
impl Clone for _Placeholder_672 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_672 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_672 {}
impl _Placeholder_672 {}
impl _Placeholder_672 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_673 {}
impl Clone for _Placeholder_673 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_673 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_673 {}
impl _Placeholder_673 {}
impl _Placeholder_673 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_674 {}
impl Clone for _Placeholder_674 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_674 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_674 {}
impl _Placeholder_674 {}
impl _Placeholder_674 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_675 {}
impl Clone for _Placeholder_675 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_675 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_675 {}
impl _Placeholder_675 {}
impl _Placeholder_675 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_676 {}
impl Clone for _Placeholder_676 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_676 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_676 {}
impl _Placeholder_676 {}
impl _Placeholder_676 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_677 {}
impl Clone for _Placeholder_677 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_677 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_677 {}
impl _Placeholder_677 {}
impl _Placeholder_677 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_678 {}
impl Clone for _Placeholder_678 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_678 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_678 {}
impl _Placeholder_678 {}
impl _Placeholder_678 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_679 {}
impl Clone for _Placeholder_679 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_679 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_679 {}
impl _Placeholder_679 {}
impl _Placeholder_679 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_680 {}
impl Clone for _Placeholder_680 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_680 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_680 {}
impl _Placeholder_680 {}
impl _Placeholder_680 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_681 {}
impl Clone for _Placeholder_681 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_681 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_681 {}
impl _Placeholder_681 {}
impl _Placeholder_681 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Placeholder_682 {}
impl Clone for _Placeholder_682 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Placeholder_682 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Placeholder_682 {}
impl _Placeholder_682 {}
impl _Placeholder_682 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_683 {}
impl Clone for basic_string_683 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_683 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_basic_string_char___Alloc_hider {
    pub _M_p: *mut libc::c_char,
}
impl Clone for std_basic_string_char___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_char___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub unsafe fn new_1(mut __dat: *mut libc::c_char, __a: *const allocator_589) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub unsafe fn new_2(mut __dat: *mut libc::c_char, __a: Option<*mut allocator_589>) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {}
impl std_basic_string_char___Alloc_hider {}
impl std_basic_string_char___Alloc_hider {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_basic_string_char___Alloc_hider,
    ) -> *mut std_basic_string_char___Alloc_hider {
        unimplemented!()
    }
}
impl std_basic_string_char___Alloc_hider {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_basic_string_char___Alloc_hider,
    ) -> *mut std_basic_string_char___Alloc_hider {
        unimplemented!()
    }
}
pub type anon_685 = u32;
pub const anon_685__S_local_capacity: anon_685 = 15;
#[derive(VaArg, FnPtrArg)]
pub struct anon_684 {
    pub _M_local_buf: [libc::c_char; 16],
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_684 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_684 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn copy_from(__str: *const basic_string_683) -> Self {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn new_2(
        mut __s: *const libc::c_char,
        mut __n: u64,
        __a: Option<*const allocator_589>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn move_from(__str: *mut basic_string_683) -> Self {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_683) -> *mut basic_string_683 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_683) -> *mut basic_string_683 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn to_basic_string_view_686(&self) -> basic_string_view_686 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn capacity(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn reserve_79(&mut self, mut __res: u64) {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn insert_119(
        &mut self,
        mut __pos1: u64,
        __str: *const basic_string_683,
    ) -> *mut basic_string_683 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn insert_122(
        &mut self,
        mut __pos: u64,
        mut __s: *const libc::c_char,
    ) -> *mut basic_string_683 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn replace_133(
        &mut self,
        mut __pos: u64,
        mut __n1: u64,
        mut __s: *const libc::c_char,
        mut __n2: u64,
    ) -> *mut basic_string_683 {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn c_str(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn data_158(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn new_3(
        mut __beg: *mut libc::c_char,
        mut __end: *mut libc::c_char,
        __a: Option<*const allocator_589>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn __resize_and_overwrite_687(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn __resize_and_overwrite_689(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn __resize_and_overwrite_691(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn __resize_and_overwrite_693(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn __resize_and_overwrite_695(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
impl basic_string_683 {
    pub unsafe fn __resize_and_overwrite_697(
        &mut self,
        mut __n: u64,
        mut __op: FnPtr<fn(*mut libc::c_char, usize) -> usize>,
    ) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_699 {}
impl Clone for basic_string_699 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_699 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_basic_string_wchar_t___Alloc_hider {
    pub _M_p: *mut i32,
}
impl Clone for std_basic_string_wchar_t___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_wchar_t___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {
    pub unsafe fn new_1(mut __dat: *mut i32, __a: *const allocator_590) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {}
impl std_basic_string_wchar_t___Alloc_hider {}
impl std_basic_string_wchar_t___Alloc_hider {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_basic_string_wchar_t___Alloc_hider,
    ) -> *mut std_basic_string_wchar_t___Alloc_hider {
        unimplemented!()
    }
}
impl std_basic_string_wchar_t___Alloc_hider {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_basic_string_wchar_t___Alloc_hider,
    ) -> *mut std_basic_string_wchar_t___Alloc_hider {
        unimplemented!()
    }
}
pub type anon_701 = u32;
pub const anon_701__S_local_capacity: anon_701 = 3;
#[derive(VaArg, FnPtrArg)]
pub struct anon_700 {
    pub _M_local_buf: [i32; 4],
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_700 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_700 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_699 {
    pub unsafe fn copy_from(__str: *const basic_string_699) -> Self {
        unimplemented!()
    }
}
impl basic_string_699 {
    pub unsafe fn new_1(
        mut __s: *const i32,
        mut __n: u64,
        __a: Option<*const allocator_590>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_699 {
    pub unsafe fn move_from(__str: *mut basic_string_699) -> Self {
        unimplemented!()
    }
}
impl basic_string_699 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_699) -> *mut basic_string_699 {
        unimplemented!()
    }
}
impl basic_string_699 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_699) -> *mut basic_string_699 {
        unimplemented!()
    }
}
impl basic_string_699 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_699 {
    pub unsafe fn c_str(&self) -> *const i32 {
        unimplemented!()
    }
}
impl basic_string_699 {
    pub unsafe fn new_2(
        mut __beg: *const libc::c_char,
        mut __end: *const libc::c_char,
        __a: Option<*const allocator_590>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_702 {}
impl Clone for basic_string_702 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_702 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_basic_string_char16_t___Alloc_hider {
    pub _M_p: *mut u16,
}
impl Clone for std_basic_string_char16_t___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_char16_t___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {
    pub unsafe fn new_1(mut __dat: *mut u16, __a: *const allocator_597) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {}
impl std_basic_string_char16_t___Alloc_hider {}
impl std_basic_string_char16_t___Alloc_hider {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_basic_string_char16_t___Alloc_hider,
    ) -> *mut std_basic_string_char16_t___Alloc_hider {
        unimplemented!()
    }
}
impl std_basic_string_char16_t___Alloc_hider {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_basic_string_char16_t___Alloc_hider,
    ) -> *mut std_basic_string_char16_t___Alloc_hider {
        unimplemented!()
    }
}
pub type anon_704 = u32;
pub const anon_704__S_local_capacity: anon_704 = 7;
#[derive(VaArg, FnPtrArg)]
pub struct anon_703 {
    pub _M_local_buf: [u16; 8],
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_703 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_703 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_702 {
    pub unsafe fn copy_from(__str: *const basic_string_702) -> Self {
        unimplemented!()
    }
}
impl basic_string_702 {
    pub unsafe fn new_1(
        mut __s: *const u16,
        mut __n: u64,
        __a: Option<*const allocator_597>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_702 {
    pub unsafe fn move_from(__str: *mut basic_string_702) -> Self {
        unimplemented!()
    }
}
impl basic_string_702 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_702) -> *mut basic_string_702 {
        unimplemented!()
    }
}
impl basic_string_702 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_702) -> *mut basic_string_702 {
        unimplemented!()
    }
}
impl basic_string_702 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_705 {}
impl Clone for basic_string_705 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_705 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_basic_string_char32_t___Alloc_hider {
    pub _M_p: *mut u32,
}
impl Clone for std_basic_string_char32_t___Alloc_hider {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_basic_string_char32_t___Alloc_hider {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {
    pub unsafe fn new_1(mut __dat: *mut u32, __a: *const allocator_598) -> Self {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {}
impl std_basic_string_char32_t___Alloc_hider {}
impl std_basic_string_char32_t___Alloc_hider {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_basic_string_char32_t___Alloc_hider,
    ) -> *mut std_basic_string_char32_t___Alloc_hider {
        unimplemented!()
    }
}
impl std_basic_string_char32_t___Alloc_hider {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_basic_string_char32_t___Alloc_hider,
    ) -> *mut std_basic_string_char32_t___Alloc_hider {
        unimplemented!()
    }
}
pub type anon_707 = u32;
pub const anon_707__S_local_capacity: anon_707 = 3;
#[derive(VaArg, FnPtrArg)]
pub struct anon_706 {
    pub _M_local_buf: [u32; 4],
    pub _M_allocated_capacity: u64,
}
impl Clone for anon_706 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_706 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_705 {
    pub unsafe fn copy_from(__str: *const basic_string_705) -> Self {
        unimplemented!()
    }
}
impl basic_string_705 {
    pub unsafe fn new_1(
        mut __s: *const u32,
        mut __n: u64,
        __a: Option<*const allocator_598>,
    ) -> Self {
        unimplemented!()
    }
}
impl basic_string_705 {
    pub unsafe fn move_from(__str: *mut basic_string_705) -> Self {
        unimplemented!()
    }
}
impl basic_string_705 {
    pub unsafe fn copy_assign(&mut self, __str: *const basic_string_705) -> *mut basic_string_705 {
        unimplemented!()
    }
}
impl basic_string_705 {
    pub unsafe fn move_assign(&mut self, __str: *mut basic_string_705) -> *mut basic_string_705 {
        unimplemented!()
    }
}
impl basic_string_705 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: anon_708,
}
impl Clone for __mbstate_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_708 {
    pub __wch: u32,
    pub __wchb: [libc::c_char; 4],
}
impl Clone for anon_708 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_708 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __locale_struct {}
impl __locale_struct {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_ios_709 {}
impl Clone for basic_ios_709 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_709 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_ios_710 {}
impl Clone for basic_ios_710 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ios_710 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_streambuf_711 {}
impl Clone for basic_streambuf_711 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_711 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_streambuf_712 {}
impl Clone for basic_streambuf_712 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_streambuf_712 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_istream_713 {}
impl Clone for basic_istream_713 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_713 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_istream_714 {}
impl Clone for basic_istream_714 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_istream_714 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_ostream_715 {}
impl Clone for basic_ostream_715 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_715 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_ostream_715 {
    pub unsafe fn operator_shl_11(&mut self, mut __n: i32) -> *mut basic_ostream_715 {
        unimplemented!()
    }
}
impl basic_ostream_715 {
    pub unsafe fn put(&mut self, mut __c: libc::c_char) -> *mut basic_ostream_715 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_ostream_716 {}
impl Clone for basic_ostream_716 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_ostream_716 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_iostream_717 {}
impl Clone for basic_iostream_717 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_717 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_iostream_718 {}
impl Clone for basic_iostream_718 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_iostream_718 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___exception_ptr_exception_ptr {}
impl Clone for std___exception_ptr_exception_ptr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___exception_ptr_exception_ptr {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn copy_from(__other: *const std___exception_ptr_exception_ptr) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn move_from(__o: *mut std___exception_ptr_exception_ptr) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn copy_assign(
        &mut self,
        __other: *const std___exception_ptr_exception_ptr,
    ) -> *mut std___exception_ptr_exception_ptr {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn move_assign(
        &mut self,
        __o: *mut std___exception_ptr_exception_ptr,
    ) -> *mut std___exception_ptr_exception_ptr {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn swap(&mut self, __other: *mut std___exception_ptr_exception_ptr) {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_nested_exception {}
impl Clone for std_nested_exception {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_nested_exception {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub unsafe fn copy_from(_a0: *const std_nested_exception) -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_nested_exception,
    ) -> *mut std_nested_exception {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Char_types_719 {}
impl Clone for _Char_types_719 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_719 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Char_types_720 {}
impl Clone for _Char_types_720 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Char_types_720 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_721 {}
impl Clone for char_traits_721 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_721 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_721 {
    pub unsafe fn eq(__c1: *const libc::c_char, __c2: *const libc::c_char) -> bool {
        unimplemented!()
    }
}
impl char_traits_721 {
    pub unsafe fn length(mut __p: *const libc::c_char) -> usize {
        unimplemented!()
    }
}
impl char_traits_721 {
    pub unsafe fn find(
        mut __s: *const libc::c_char,
        mut __n: usize,
        __a: *const libc::c_char,
    ) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct char_traits_722 {}
impl Clone for char_traits_722 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for char_traits_722 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl char_traits_722 {
    pub unsafe fn eq(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_722 {
    pub unsafe fn lt(__c1: *const i32, __c2: *const i32) -> bool {
        unimplemented!()
    }
}
impl char_traits_722 {
    pub unsafe fn compare(mut __s1: *const i32, mut __s2: *const i32, mut __n: usize) -> i32 {
        unimplemented!()
    }
}
impl char_traits_722 {
    pub unsafe fn length(mut __p: *const i32) -> usize {
        unimplemented!()
    }
}
impl char_traits_722 {
    pub unsafe fn find(mut __s: *const i32, mut __n: usize, __a: *const i32) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
impl Clone for lconv {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for lconv {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_723 = u32;
pub const anon_723__ISupper: anon_723 = 256;
pub const anon_723__ISlower: anon_723 = 512;
pub const anon_723__ISalpha: anon_723 = 1024;
pub const anon_723__ISdigit: anon_723 = 2048;
pub const anon_723__ISxdigit: anon_723 = 4096;
pub const anon_723__ISspace: anon_723 = 8192;
pub const anon_723__ISprint: anon_723 = 16384;
pub const anon_723__ISgraph: anon_723 = 32768;
pub const anon_723__ISblank: anon_723 = 1;
pub const anon_723__IScntrl: anon_723 = 2;
pub const anon_723__ISpunct: anon_723 = 4;
pub const anon_723__ISalnum: anon_723 = 8;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct ctype_724 {}
impl Clone for ctype_724 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_724 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct ctype_725 {}
impl Clone for ctype_725 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ctype_725 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_get_726 {}
impl Clone for num_get_726 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_726 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_get_727 {}
impl Clone for num_get_727 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_get_727 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_put_728 {}
impl Clone for num_put_728 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_728 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct num_put_729 {}
impl Clone for num_put_729 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for num_put_729 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numpunct_730 {}
impl Clone for numpunct_730 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_730 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numpunct_731 {}
impl Clone for numpunct_731 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numpunct_731 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct collate_732 {}
impl Clone for collate_732 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_732 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct collate_733 {}
impl Clone for collate_733 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for collate_733 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct sched_param {
    pub sched_priority: i32,
}
impl Clone for sched_param {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sched_param {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct cpu_set_t {
    pub __bits: [u64; 16],
}
impl Clone for cpu_set_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for cpu_set_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timex {
    pub modes: u32,
    pub offset: i64,
    pub freq: i64,
    pub maxerror: i64,
    pub esterror: i64,
    pub status: i32,
    pub constant: i64,
    pub precision: i64,
    pub tolerance: i64,
    pub time: timeval,
    pub tick: i64,
    pub ppsfreq: i64,
    pub jitter: i64,
    pub shift: i32,
    pub stabil: i64,
    pub jitcnt: i64,
    pub calcnt: i64,
    pub errcnt: i64,
    pub stbcnt: i64,
    pub tai: i32,
    pub anon_734: i32,
    pub anon_735: i32,
    pub anon_736: i32,
    pub anon_737: i32,
    pub anon_738: i32,
    pub anon_739: i32,
    pub anon_740: i32,
    pub anon_741: i32,
    pub anon_742: i32,
    pub anon_743: i32,
    pub anon_744: i32,
}
impl Clone for timex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timex {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const libc::c_char,
}
impl Clone for tm {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tm {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
impl Clone for itimerspec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for itimerspec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: anon_745,
}
impl Clone for __atomic_wide_counter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_745 {
    pub __low: u32,
    pub __high: u32,
}
impl Clone for anon_745 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_745 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
impl Clone for __pthread_internal_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
impl Clone for __pthread_internal_slist {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_slist {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
    pub __list: __pthread_internal_list,
}
impl Clone for __pthread_mutex_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: i8,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
    pub __flags: u32,
}
impl Clone for __pthread_rwlock_arch_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_rwlock_arch_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
}
impl Clone for __pthread_cond_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __once_flag {
    pub __data: i32,
}
impl Clone for __once_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __once_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: i32,
}
impl Clone for pthread_mutexattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: i32,
}
impl Clone for pthread_condattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: i64,
}
impl Clone for pthread_attr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: i64,
}
impl Clone for pthread_mutex_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: i64,
}
impl Clone for pthread_cond_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: i64,
}
impl Clone for pthread_rwlock_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: i64,
}
impl Clone for pthread_rwlockattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrier_t {
    pub __size: [libc::c_char; 32],
    pub __align: i64,
}
impl Clone for pthread_barrier_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrierattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: i32,
}
impl Clone for pthread_barrierattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: [i64; 8],
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
impl Clone for __jmp_buf_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __jmp_buf_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_746 = u32;
pub const anon_746_PTHREAD_MUTEX_TIMED_NP: anon_746 = 0;
pub const anon_746_PTHREAD_MUTEX_RECURSIVE_NP: anon_746 = 1;
pub const anon_746_PTHREAD_MUTEX_ERRORCHECK_NP: anon_746 = 2;
pub const anon_746_PTHREAD_MUTEX_ADAPTIVE_NP: anon_746 = 3;
pub const anon_746_PTHREAD_MUTEX_NORMAL: anon_746 = 0;
pub const anon_746_PTHREAD_MUTEX_RECURSIVE: anon_746 = 1;
pub const anon_746_PTHREAD_MUTEX_ERRORCHECK: anon_746 = 2;
pub const anon_746_PTHREAD_MUTEX_DEFAULT: anon_746 = 0;
pub const anon_746_PTHREAD_MUTEX_FAST_NP: anon_746 = 0;
pub type anon_747 = u32;
pub const anon_747_PTHREAD_MUTEX_STALLED: anon_747 = 0;
pub const anon_747_PTHREAD_MUTEX_STALLED_NP: anon_747 = 0;
pub const anon_747_PTHREAD_MUTEX_ROBUST: anon_747 = 1;
pub const anon_747_PTHREAD_MUTEX_ROBUST_NP: anon_747 = 1;
pub type anon_748 = u32;
pub const anon_748_PTHREAD_RWLOCK_PREFER_READER_NP: anon_748 = 0;
pub const anon_748_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_748 = 1;
pub const anon_748_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_748 = 2;
pub const anon_748_PTHREAD_RWLOCK_DEFAULT_NP: anon_748 = 0;
#[derive(VaArg, FnPtrArg)]
pub struct _pthread_cleanup_buffer {
    pub __routine: Option<unsafe fn(*mut ::libc::c_void)>,
    pub __arg: *mut ::libc::c_void,
    pub __canceltype: i32,
    pub __prev: *mut _pthread_cleanup_buffer,
}
impl Clone for _pthread_cleanup_buffer {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _pthread_cleanup_buffer {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_749 = u32;
pub const anon_749_PTHREAD_CANCEL_DEFERRED: anon_749 = 0;
pub const anon_749_PTHREAD_CANCEL_ASYNCHRONOUS: anon_749 = 1;
#[derive(VaArg, FnPtrArg)]
pub struct __cancel_jmp_buf_tag {
    pub __cancel_jmp_buf: [i64; 8],
    pub __mask_was_saved: i32,
}
impl Clone for __cancel_jmp_buf_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cancel_jmp_buf_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__cancel_jmp_buf_tag; 1],
    pub __pad: [*mut ::libc::c_void; 4],
}
impl Clone for __pthread_unwind_buf_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_unwind_buf_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine: Option<unsafe fn(*mut ::libc::c_void)>,
    pub __cancel_arg: *mut ::libc::c_void,
    pub __do_it: i32,
    pub __cancel_type: i32,
}
impl Clone for __pthread_cleanup_frame {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cleanup_frame {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __pthread_cleanup_class {}
impl Clone for __pthread_cleanup_class {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cleanup_class {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __libc_single_threaded_750: std::cell::LazyCell<libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_686 {}
impl Clone for basic_string_view_686 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_686 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_686 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_686) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_686 {
    pub unsafe fn new_1(mut __str: *const libc::c_char, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_686 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_686,
    ) -> *mut basic_string_view_686 {
        unimplemented!()
    }
}
impl basic_string_view_686 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_686 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_686 {
    pub unsafe fn data(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_751 {}
impl Clone for basic_string_view_751 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_751 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_751 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_751) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_751 {
    pub unsafe fn new_1(mut __str: *const i32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_751 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_751,
    ) -> *mut basic_string_view_751 {
        unimplemented!()
    }
}
impl basic_string_view_751 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_751 {
    pub unsafe fn data(&self) -> *const i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_752 {}
impl Clone for basic_string_view_752 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_752 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_752 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_752) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_752 {
    pub unsafe fn new_1(mut __str: *const u16, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_752 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_752,
    ) -> *mut basic_string_view_752 {
        unimplemented!()
    }
}
impl basic_string_view_752 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_752 {
    pub unsafe fn data(&self) -> *const u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct basic_string_view_753 {}
impl Clone for basic_string_view_753 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for basic_string_view_753 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl basic_string_view_753 {
    pub unsafe fn copy_from(_a0: *const basic_string_view_753) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_753 {
    pub unsafe fn new_1(mut __str: *const u32, mut __len: u64) -> Self {
        unimplemented!()
    }
}
impl basic_string_view_753 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const basic_string_view_753,
    ) -> *mut basic_string_view_753 {
        unimplemented!()
    }
}
impl basic_string_view_753 {
    pub unsafe fn length(&self) -> u64 {
        unimplemented!()
    }
}
impl basic_string_view_753 {
    pub unsafe fn data(&self) -> *const u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct div_t {
    pub quot: i32,
    pub rem: i32,
}
impl Clone for div_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for div_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ldiv_t {
    pub quot: i64,
    pub rem: i64,
}
impl Clone for ldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct lldiv_t {
    pub quot: i64,
    pub rem: i64,
}
impl Clone for lldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for lldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl lldiv_t {}
impl lldiv_t {}
impl lldiv_t {}
#[derive(VaArg, FnPtrArg)]
pub struct fd_set {
    pub fds_bits: [i64; 16],
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: i32,
    pub rand_deg: i32,
    pub rand_sep: i32,
    pub end_ptr: *mut i32,
}
impl Clone for random_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for random_data {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct drand48_data {
    pub __x: [u16; 3],
    pub __old_x: [u16; 3],
    pub __c: u16,
    pub __init: u16,
    pub __a: u64,
}
impl Clone for drand48_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for drand48_data {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _G_fpos_t {
    pub __pos: i64,
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _G_fpos64_t {
    pub __pos: i64,
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos64_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos64_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut ::libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::libc::c_void,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
impl Clone for _IO_FILE {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_FILE {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _IO_cookie_io_functions_t {
    pub read: Option<unsafe fn(*mut ::libc::c_void, *mut libc::c_char, usize) -> i64>,
    pub write: Option<unsafe fn(*mut ::libc::c_void, *const libc::c_char, usize) -> i64>,
    pub seek: Option<unsafe fn(*mut ::libc::c_void, *mut i64, i32) -> i32>,
    pub close: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
}
impl Clone for _IO_cookie_io_functions_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_cookie_io_functions_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __integer_to_chars_is_unsigned_754: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_755: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __integer_to_chars_is_unsigned_756: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_locale {}
impl Clone for std_locale {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_locale_ctype_757: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_numeric_758: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_collate_759: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_time_760: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_monetary_761: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_locale_messages_762: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type anon_763 = u32;
pub const anon_763__S_categories_size: anon_763 = 12;
impl std_locale {
    pub unsafe fn copy_from(__other: *const std_locale) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub unsafe fn new_1(mut __s: *const libc::c_char) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub unsafe fn new_2(
        __base: *const std_locale,
        mut __s: *const libc::c_char,
        mut __cat: i32,
    ) -> Self {
        unimplemented!()
    }
}
impl std_locale {
    pub unsafe fn copy_assign(&mut self, __other: *const std_locale) -> *const std_locale {
        unimplemented!()
    }
}
impl std_locale {
    pub unsafe fn operator_eq(&self, __other: *const std_locale) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_locale_facet {}
impl Clone for std_locale_facet {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale_facet {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_locale_id {}
impl Clone for std_locale_id {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale_id {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_locale__Impl {}
impl Clone for std_locale__Impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_locale__Impl {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_errc = i32;
pub const std_errc_address_family_not_supported: std_errc = 97;
pub const std_errc_address_in_use: std_errc = 98;
pub const std_errc_address_not_available: std_errc = 99;
pub const std_errc_already_connected: std_errc = 106;
pub const std_errc_argument_list_too_long: std_errc = 7;
pub const std_errc_argument_out_of_domain: std_errc = 33;
pub const std_errc_bad_address: std_errc = 14;
pub const std_errc_bad_file_descriptor: std_errc = 9;
pub const std_errc_bad_message: std_errc = 74;
pub const std_errc_broken_pipe: std_errc = 32;
pub const std_errc_connection_aborted: std_errc = 103;
pub const std_errc_connection_already_in_progress: std_errc = 114;
pub const std_errc_connection_refused: std_errc = 111;
pub const std_errc_connection_reset: std_errc = 104;
pub const std_errc_cross_device_link: std_errc = 18;
pub const std_errc_destination_address_required: std_errc = 89;
pub const std_errc_device_or_resource_busy: std_errc = 16;
pub const std_errc_directory_not_empty: std_errc = 39;
pub const std_errc_executable_format_error: std_errc = 8;
pub const std_errc_file_exists: std_errc = 17;
pub const std_errc_file_too_large: std_errc = 27;
pub const std_errc_filename_too_long: std_errc = 36;
pub const std_errc_function_not_supported: std_errc = 38;
pub const std_errc_host_unreachable: std_errc = 113;
pub const std_errc_identifier_removed: std_errc = 43;
pub const std_errc_illegal_byte_sequence: std_errc = 84;
pub const std_errc_inappropriate_io_control_operation: std_errc = 25;
pub const std_errc_interrupted: std_errc = 4;
pub const std_errc_invalid_argument: std_errc = 22;
pub const std_errc_invalid_seek: std_errc = 29;
pub const std_errc_io_error: std_errc = 5;
pub const std_errc_is_a_directory: std_errc = 21;
pub const std_errc_message_size: std_errc = 90;
pub const std_errc_network_down: std_errc = 100;
pub const std_errc_network_reset: std_errc = 102;
pub const std_errc_network_unreachable: std_errc = 101;
pub const std_errc_no_buffer_space: std_errc = 105;
pub const std_errc_no_child_process: std_errc = 10;
pub const std_errc_no_link: std_errc = 67;
pub const std_errc_no_lock_available: std_errc = 37;
pub const std_errc_no_message_available: std_errc = 61;
pub const std_errc_no_message: std_errc = 42;
pub const std_errc_no_protocol_option: std_errc = 92;
pub const std_errc_no_space_on_device: std_errc = 28;
pub const std_errc_no_stream_resources: std_errc = 63;
pub const std_errc_no_such_device_or_address: std_errc = 6;
pub const std_errc_no_such_device: std_errc = 19;
pub const std_errc_no_such_file_or_directory: std_errc = 2;
pub const std_errc_no_such_process: std_errc = 3;
pub const std_errc_not_a_directory: std_errc = 20;
pub const std_errc_not_a_socket: std_errc = 88;
pub const std_errc_not_a_stream: std_errc = 60;
pub const std_errc_not_connected: std_errc = 107;
pub const std_errc_not_enough_memory: std_errc = 12;
pub const std_errc_not_supported: std_errc = 95;
pub const std_errc_operation_canceled: std_errc = 125;
pub const std_errc_operation_in_progress: std_errc = 115;
pub const std_errc_operation_not_permitted: std_errc = 1;
pub const std_errc_operation_not_supported: std_errc = 95;
pub const std_errc_operation_would_block: std_errc = 11;
pub const std_errc_owner_dead: std_errc = 130;
pub const std_errc_permission_denied: std_errc = 13;
pub const std_errc_protocol_error: std_errc = 71;
pub const std_errc_protocol_not_supported: std_errc = 93;
pub const std_errc_read_only_file_system: std_errc = 30;
pub const std_errc_resource_deadlock_would_occur: std_errc = 35;
pub const std_errc_resource_unavailable_try_again: std_errc = 11;
pub const std_errc_result_out_of_range: std_errc = 34;
pub const std_errc_state_not_recoverable: std_errc = 131;
pub const std_errc_stream_timeout: std_errc = 62;
pub const std_errc_text_file_busy: std_errc = 26;
pub const std_errc_timed_out: std_errc = 110;
pub const std_errc_too_many_files_open_in_system: std_errc = 23;
pub const std_errc_too_many_files_open: std_errc = 24;
pub const std_errc_too_many_links: std_errc = 31;
pub const std_errc_too_many_symbolic_link_levels: std_errc = 40;
pub const std_errc_value_too_large: std_errc = 75;
pub const std_errc_wrong_protocol_type: std_errc = 91;
#[derive(VaArg, FnPtrArg)]
pub struct std___cow_string {
    pub anon_764: anon_764,
}
impl Clone for std___cow_string {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___cow_string {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_764 {
    pub _M_p: *const libc::c_char,
    pub _M_bytes: [libc::c_char; 8],
}
impl Clone for anon_764 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_764 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub unsafe fn copy_from(_a0: *const std___cow_string) -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub unsafe fn copy_assign(&mut self, _a0: *const std___cow_string) -> *mut std___cow_string {
        unimplemented!()
    }
}
impl std___cow_string {
    pub unsafe fn move_from(_a0: *mut std___cow_string) -> Self {
        unimplemented!()
    }
}
impl std___cow_string {
    pub unsafe fn move_assign(&mut self, _a0: *mut std___cow_string) -> *mut std___cow_string {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_logic_error {}
impl Clone for std_logic_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_logic_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_logic_error {
    pub unsafe fn move_from(_a0: *mut std_logic_error) -> Self {
        unimplemented!()
    }
}
impl std_logic_error {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_logic_error) -> *mut std_logic_error {
        unimplemented!()
    }
}
impl std_logic_error {
    pub unsafe fn copy_from(_a0: *const std_logic_error) -> Self {
        unimplemented!()
    }
}
impl std_logic_error {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_logic_error) -> *mut std_logic_error {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_runtime_error {}
impl Clone for std_runtime_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_runtime_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub unsafe fn new_1(__arg: *const basic_string_683) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub unsafe fn move_from(_a0: *mut std_runtime_error) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_runtime_error) -> *mut std_runtime_error {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub unsafe fn copy_from(_a0: *const std_runtime_error) -> Self {
        unimplemented!()
    }
}
impl std_runtime_error {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_runtime_error) -> *mut std_runtime_error {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_765 {}
impl Clone for is_error_code_enum_765 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_765 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_766 {}
impl Clone for is_error_code_enum_766 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_766 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_767 {}
impl Clone for is_error_code_enum_767 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_767 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_768 {}
impl Clone for is_error_code_enum_768 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_768 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_769 {}
impl Clone for is_error_code_enum_769 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_769 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_code_enum_770 {}
impl Clone for is_error_code_enum_770 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_code_enum_770 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_condition_enum_771 {}
impl Clone for is_error_condition_enum_771 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_771 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_error_condition_enum_772 {}
impl Clone for is_error_condition_enum_772 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_error_condition_enum_772 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_error_code {}
impl Clone for std_error_code {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_error_code {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn new_1(mut __v: i32, __cat: *const std_error_category) -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn copy_from(_a0: *const std_error_code) -> Self {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_error_code) -> *mut std_error_code {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn assign(&mut self, mut __v: i32, __cat: *const std_error_category) {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn value(&self) -> i32 {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn category(&self) -> *const std_error_category {
        unimplemented!()
    }
}
impl std_error_code {
    pub unsafe fn message(&self) -> basic_string_683 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_error_condition {}
impl Clone for std_error_condition {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_error_condition {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn new_1(mut __v: i32, __cat: *const std_error_category) -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn copy_from(_a0: *const std_error_condition) -> Self {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_error_condition,
    ) -> *mut std_error_condition {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn assign(&mut self, mut __v: i32, __cat: *const std_error_category) {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn value(&self) -> i32 {
        unimplemented!()
    }
}
impl std_error_condition {
    pub unsafe fn category(&self) -> *const std_error_category {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_system_error {}
impl Clone for std_system_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_system_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub unsafe fn new_1(mut __ec: std_error_code, mut __what: *const libc::c_char) -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub unsafe fn copy_from(_a0: *const std_system_error) -> Self {
        unimplemented!()
    }
}
impl std_system_error {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_system_error) -> *mut std_system_error {
        unimplemented!()
    }
}
pub type std__Ios_Fmtflags = i32;
pub const std__Ios_Fmtflags__S_boolalpha: std__Ios_Fmtflags = 1;
pub const std__Ios_Fmtflags__S_dec: std__Ios_Fmtflags = 2;
pub const std__Ios_Fmtflags__S_fixed: std__Ios_Fmtflags = 4;
pub const std__Ios_Fmtflags__S_hex: std__Ios_Fmtflags = 8;
pub const std__Ios_Fmtflags__S_internal: std__Ios_Fmtflags = 16;
pub const std__Ios_Fmtflags__S_left: std__Ios_Fmtflags = 32;
pub const std__Ios_Fmtflags__S_oct: std__Ios_Fmtflags = 64;
pub const std__Ios_Fmtflags__S_right: std__Ios_Fmtflags = 128;
pub const std__Ios_Fmtflags__S_scientific: std__Ios_Fmtflags = 256;
pub const std__Ios_Fmtflags__S_showbase: std__Ios_Fmtflags = 512;
pub const std__Ios_Fmtflags__S_showpoint: std__Ios_Fmtflags = 1024;
pub const std__Ios_Fmtflags__S_showpos: std__Ios_Fmtflags = 2048;
pub const std__Ios_Fmtflags__S_skipws: std__Ios_Fmtflags = 4096;
pub const std__Ios_Fmtflags__S_unitbuf: std__Ios_Fmtflags = 8192;
pub const std__Ios_Fmtflags__S_uppercase: std__Ios_Fmtflags = 16384;
pub const std__Ios_Fmtflags__S_adjustfield: std__Ios_Fmtflags = 176;
pub const std__Ios_Fmtflags__S_basefield: std__Ios_Fmtflags = 74;
pub const std__Ios_Fmtflags__S_floatfield: std__Ios_Fmtflags = 260;
pub const std__Ios_Fmtflags__S_ios_fmtflags_end: std__Ios_Fmtflags = 65536;
pub const std__Ios_Fmtflags__S_ios_fmtflags_max: std__Ios_Fmtflags = 2147483647;
pub const std__Ios_Fmtflags__S_ios_fmtflags_min: std__Ios_Fmtflags = -2147483648;
pub type std__Ios_Openmode = i32;
pub const std__Ios_Openmode__S_app: std__Ios_Openmode = 1;
pub const std__Ios_Openmode__S_ate: std__Ios_Openmode = 2;
pub const std__Ios_Openmode__S_bin: std__Ios_Openmode = 4;
pub const std__Ios_Openmode__S_in: std__Ios_Openmode = 8;
pub const std__Ios_Openmode__S_out: std__Ios_Openmode = 16;
pub const std__Ios_Openmode__S_trunc: std__Ios_Openmode = 32;
pub const std__Ios_Openmode__S_noreplace: std__Ios_Openmode = 64;
pub const std__Ios_Openmode__S_ios_openmode_end: std__Ios_Openmode = 65536;
pub const std__Ios_Openmode__S_ios_openmode_max: std__Ios_Openmode = 2147483647;
pub const std__Ios_Openmode__S_ios_openmode_min: std__Ios_Openmode = -2147483648;
pub type std__Ios_Iostate = i32;
pub const std__Ios_Iostate__S_goodbit: std__Ios_Iostate = 0;
pub const std__Ios_Iostate__S_badbit: std__Ios_Iostate = 1;
pub const std__Ios_Iostate__S_eofbit: std__Ios_Iostate = 2;
pub const std__Ios_Iostate__S_failbit: std__Ios_Iostate = 4;
pub const std__Ios_Iostate__S_ios_iostate_end: std__Ios_Iostate = 65536;
pub const std__Ios_Iostate__S_ios_iostate_max: std__Ios_Iostate = 2147483647;
pub const std__Ios_Iostate__S_ios_iostate_min: std__Ios_Iostate = -2147483648;
pub type std__Ios_Seekdir = u32;
pub const std__Ios_Seekdir__S_beg: std__Ios_Seekdir = 0;
pub const std__Ios_Seekdir__S_cur: std__Ios_Seekdir = 1;
pub const std__Ios_Seekdir__S_end: std__Ios_Seekdir = 2;
pub const std__Ios_Seekdir__S_ios_seekdir_end: std__Ios_Seekdir = 65536;
pub type std_io_errc = i32;
pub const std_io_errc_stream: std_io_errc = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_ios_base {}
impl Clone for std_ios_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ios_base {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_ios_base_boolalpha_773: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_dec_774: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_fixed_775: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_hex_776: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_internal_777: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_left_778: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_oct_779: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_right_780: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_scientific_781: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showbase_782: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showpoint_783: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_showpos_784: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_skipws_785: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_unitbuf_786: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_uppercase_787: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_adjustfield_788: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_basefield_789: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_floatfield_790: std::cell::LazyCell<std__Ios_Fmtflags> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_badbit_791: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_eofbit_792: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_failbit_793: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_goodbit_794: std::cell::LazyCell<std__Ios_Iostate> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_in_795: std::cell::LazyCell<std__Ios_Openmode> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_out_796: std::cell::LazyCell<std__Ios_Openmode> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ios_base_cur_797: std::cell::LazyCell<std__Ios_Seekdir> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type std_ios_base_event = u32;
pub const std_ios_base_event_erase_event: std_ios_base_event = 0;
pub const std_ios_base_event_imbue_event: std_ios_base_event = 1;
pub const std_ios_base_event_copyfmt_event: std_ios_base_event = 2;
#[derive(VaArg, FnPtrArg)]
pub struct std_ios_base__Callback_list {
    pub _M_next: *mut std_ios_base__Callback_list,
    pub _M_fn: Option<unsafe fn(std_ios_base_event, *mut std_ios_base, i32)>,
    pub _M_index: i32,
    pub _M_refcount: i32,
}
impl Clone for std_ios_base__Callback_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ios_base__Callback_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_ios_base__Words {
    pub _M_pword: *mut ::libc::c_void,
    pub _M_iword: i64,
}
impl Clone for std_ios_base__Words {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ios_base__Words {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_ios_base__Words {}
impl std_ios_base__Words {}
impl std_ios_base__Words {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_ios_base__Words,
    ) -> *mut std_ios_base__Words {
        unimplemented!()
    }
}
impl std_ios_base__Words {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_ios_base__Words,
    ) -> *mut std_ios_base__Words {
        unimplemented!()
    }
}
pub type anon_798 = u32;
pub const anon_798__S_local_word_size: anon_798 = 8;
impl std_ios_base {
    pub unsafe fn setf_8(&mut self, mut __fmtfl: std__Ios_Fmtflags) -> std__Ios_Fmtflags {
        unimplemented!()
    }
}
impl std_ios_base {
    pub unsafe fn setf_9(
        &mut self,
        mut __fmtfl: std__Ios_Fmtflags,
        mut __mask: std__Ios_Fmtflags,
    ) -> std__Ios_Fmtflags {
        unimplemented!()
    }
}
impl std_ios_base {
    pub unsafe fn unsetf(&mut self, mut __mask: std__Ios_Fmtflags) {
        unimplemented!()
    }
}
impl std_ios_base {
    pub unsafe fn width_13(&self) -> i64 {
        unimplemented!()
    }
}
pub type anon_799 = i32;
pub const anon_799___ISwupper: anon_799 = 0;
pub const anon_799___ISwlower: anon_799 = 1;
pub const anon_799___ISwalpha: anon_799 = 2;
pub const anon_799___ISwdigit: anon_799 = 3;
pub const anon_799___ISwxdigit: anon_799 = 4;
pub const anon_799___ISwspace: anon_799 = 5;
pub const anon_799___ISwprint: anon_799 = 6;
pub const anon_799___ISwgraph: anon_799 = 7;
pub const anon_799___ISwblank: anon_799 = 8;
pub const anon_799___ISwcntrl: anon_799 = 9;
pub const anon_799___ISwpunct: anon_799 = 10;
pub const anon_799___ISwalnum: anon_799 = 11;
pub const anon_799__ISwupper: anon_799 = 16777216;
pub const anon_799__ISwlower: anon_799 = 33554432;
pub const anon_799__ISwalpha: anon_799 = 67108864;
pub const anon_799__ISwdigit: anon_799 = 134217728;
pub const anon_799__ISwxdigit: anon_799 = 268435456;
pub const anon_799__ISwspace: anon_799 = 536870912;
pub const anon_799__ISwprint: anon_799 = 1073741824;
pub const anon_799__ISwgraph: anon_799 = -2147483648;
pub const anon_799__ISwblank: anon_799 = 65536;
pub const anon_799__ISwcntrl: anon_799 = 131072;
pub const anon_799__ISwpunct: anon_799 = 262144;
pub const anon_799__ISwalnum: anon_799 = 524288;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_ctype_base {}
impl Clone for std_ctype_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ctype_base {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_ctype_base_upper_800: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_lower_801: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_alpha_802: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_digit_803: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_xdigit_804: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_space_805: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_print_806: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_graph_807: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_cntrl_808: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_punct_809: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_alnum_810: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_ctype_base_blank_811: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___num_base {}
impl Clone for std___num_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___num_base {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_812 = u32;
pub const anon_812__S_ominus: anon_812 = 0;
pub const anon_812__S_oplus: anon_812 = 1;
pub const anon_812__S_ox: anon_812 = 2;
pub const anon_812__S_oX: anon_812 = 3;
pub const anon_812__S_odigits: anon_812 = 4;
pub const anon_812__S_odigits_end: anon_812 = 20;
pub const anon_812__S_oudigits: anon_812 = 20;
pub const anon_812__S_oudigits_end: anon_812 = 36;
pub const anon_812__S_oe: anon_812 = 18;
pub const anon_812__S_oE: anon_812 = 34;
pub const anon_812__S_oend: anon_812 = 36;
pub static mut std___num_base__S_atoms_out_813: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std___num_base__S_atoms_in_814: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type anon_815 = u32;
pub const anon_815__S_iminus: anon_815 = 0;
pub const anon_815__S_iplus: anon_815 = 1;
pub const anon_815__S_ix: anon_815 = 2;
pub const anon_815__S_iX: anon_815 = 3;
pub const anon_815__S_izero: anon_815 = 4;
pub const anon_815__S_ie: anon_815 = 18;
pub const anon_815__S_iE: anon_815 = 24;
pub const anon_815__S_iend: anon_815 = 26;
pub static mut cout_816: std::cell::LazyCell<basic_ostream_715> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn set_817(mut ref_: reference_wrapper_0, mut val: i32) {
    (*(unsafe { reference_wrapper_0::get(&ref_) })) = val;
}
pub unsafe fn read_818(mut ref_: reference_wrapper_0) -> i32 {
    let r: *mut i32 = (unsafe { reference_wrapper_0::to_pmuti32(&ref_) });
    return (*r);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 10;
    let mut ref_1: reference_wrapper_0 = reference_wrapper_0::new({ &mut i1 });
    (*(unsafe { reference_wrapper_0::get(&ref_1) })) = 20;
    let i2: *mut i32 = (unsafe { reference_wrapper_0::to_pmuti32(&ref_1) });
    (*i2) += 5;
    (unsafe {
        let ___out: *mut basic_ostream_715 = &mut (*(unsafe {
            basic_ostream_715::operator_shl_11(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_816)),
                i1,
            )
        }));
        operator_shl_819(___out, ('\n' as libc::c_char))
    });
    let mut i3: i32 = 1;
    let mut i4: i32 = 2;
    let mut ref_3: reference_wrapper_0 = reference_wrapper_0::new({ &mut i3 });
    let mut ref_4: reference_wrapper_0 = reference_wrapper_0::new({ &mut i4 });
    (*(unsafe { reference_wrapper_0::get(&ref_3) })) =
        (*(unsafe { reference_wrapper_0::get(&ref_4) }));
    (unsafe {
        let ___out: *mut basic_ostream_715 = &mut (*(unsafe {
            basic_ostream_715::operator_shl_11(
                &mut (*(unsafe {
                    let ___out: *mut basic_ostream_715 = &mut (*(unsafe {
                        basic_ostream_715::operator_shl_11(
                            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_816)),
                            i3,
                        )
                    }));
                    operator_shl_819(___out, (' ' as libc::c_char))
                })),
                i4,
            )
        }));
        operator_shl_819(___out, ('\n' as libc::c_char))
    });
    (unsafe { set_817(reference_wrapper_0::copy_from({ &ref_1 }), 99) });
    (unsafe {
        let ___out: *mut basic_ostream_715 = &mut (*(unsafe {
            basic_ostream_715::operator_shl_11(
                &mut (*(unsafe {
                    let ___out: *mut basic_ostream_715 = &mut (*(unsafe {
                        basic_ostream_715::operator_shl_11(
                            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_816)),
                            i1,
                        )
                    }));
                    operator_shl_819(___out, (' ' as libc::c_char))
                })),
                (unsafe { read_818(reference_wrapper_0::copy_from({ &ref_1 })) }),
            )
        }));
        operator_shl_819(___out, ('\n' as libc::c_char))
    });
    let mut point: Point = Point { x: 3, y: 4 };
    let mut point_ref: reference_wrapper_1 = reference_wrapper_1::new({ &mut point });
    (*(unsafe { reference_wrapper_1::get(&point_ref) })).x = 30;
    (*(unsafe { reference_wrapper_1::to_pmutPoint(&point_ref) })).y = 40;
    (unsafe {
        let ___out: *mut basic_ostream_715 = &mut (*(unsafe {
            let ___n: i32 = point.y;
            basic_ostream_715::operator_shl_11(
                &mut (*(unsafe {
                    let ___out: *mut basic_ostream_715 = &mut (*(unsafe {
                        basic_ostream_715::operator_shl_11(
                            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut cout_816)),
                            point.x,
                        )
                    }));
                    operator_shl_819(___out, (' ' as libc::c_char))
                })),
                ___n,
            )
        }));
        operator_shl_819(___out, ('\n' as libc::c_char))
    });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std_error_category;
pub unsafe fn operator_shl_819(
    __out: *mut basic_ostream_715,
    mut __c: libc::c_char,
) -> *mut basic_ostream_715 {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
