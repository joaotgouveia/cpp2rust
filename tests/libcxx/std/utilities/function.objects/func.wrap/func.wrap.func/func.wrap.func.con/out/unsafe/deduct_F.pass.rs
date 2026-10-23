extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_R______std_function_R_____ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_R__A1____std_function_R__A1___ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_R__A1__A2____std_function_R__A1__A2___ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_R__A1__A2__A3____std_function_R__A1__A2__A3___ {
}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_R__A1_refref____std_function_R__A1_refref___ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct R {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A1 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A2 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A3 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_0 {}
impl f0_0 {
    pub unsafe fn operator_call(&mut self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_0 {}
impl f1_0 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_0 {}
impl f2_0 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_0 {}
impl f3_0 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_1 {}
impl f0_1 {
    pub unsafe fn operator_call(&self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_1 {}
impl f1_1 {
    pub unsafe fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_1 {}
impl f2_1 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_1 {}
impl f3_1 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_2 {}
impl f0_2 {
    pub unsafe fn operator_call(&mut self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_2 {}
impl f1_2 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_2 {}
impl f2_2 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_2 {}
impl f3_2 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_3 {}
impl f0_3 {
    pub unsafe fn operator_call(&self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_3 {}
impl f1_3 {
    pub unsafe fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_3 {}
impl f2_3 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_3 {}
impl f3_3 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_4 {}
impl f0_4 {
    pub unsafe fn operator_call(&mut self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_4 {}
impl f1_4 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_4 {}
impl f2_4 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_4 {}
impl f3_4 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_5 {}
impl f0_5 {
    pub unsafe fn operator_call(&self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_5 {}
impl f1_5 {
    pub unsafe fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_5 {}
impl f2_5 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_5 {}
impl f3_5 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_6 {}
impl f0_6 {
    pub unsafe fn operator_call(&mut self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_6 {}
impl f1_6 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_6 {}
impl f2_6 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_6 {}
impl f3_6 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_7 {}
impl f0_7 {
    pub unsafe fn operator_call(&self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_7 {}
impl f1_7 {
    pub unsafe fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_7 {}
impl f2_7 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_7 {}
impl f3_7 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_8 {}
impl f0_8 {
    pub unsafe fn operator_call(&mut self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_8 {}
impl f1_8 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_8 {}
impl f2_8 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_8 {}
impl f3_8 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_9 {}
impl f0_9 {
    pub unsafe fn operator_call(&self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_9 {}
impl f1_9 {
    pub unsafe fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_9 {}
impl f2_9 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_9 {}
impl f3_9 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_10 {}
impl f0_10 {
    pub unsafe fn operator_call(&mut self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_10 {}
impl f1_10 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_10 {}
impl f2_10 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_10 {}
impl f3_10 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_11 {}
impl f0_11 {
    pub unsafe fn operator_call(&self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_11 {}
impl f1_11 {
    pub unsafe fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_11 {}
impl f2_11 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_11 {}
impl f3_11 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_12 {}
impl f0_12 {
    pub unsafe fn operator_call(&mut self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_12 {}
impl f1_12 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_12 {}
impl f2_12 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_12 {}
impl f3_12 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_13 {}
impl f0_13 {
    pub unsafe fn operator_call(&self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_13 {}
impl f1_13 {
    pub unsafe fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_13 {}
impl f2_13 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_13 {}
impl f3_13 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_14 {}
impl f0_14 {
    pub unsafe fn operator_call(&mut self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_14 {}
impl f1_14 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_14 {}
impl f2_14 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_14 {}
impl f3_14 {
    pub unsafe fn operator_call(&mut self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f0_15 {}
impl f0_15 {
    pub unsafe fn operator_call(&self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f1_15 {}
impl f1_15 {
    pub unsafe fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f2_15 {}
impl f2_15 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct f3_15 {}
impl f3_15 {
    pub unsafe fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
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
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_1({ <f1_0>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_1({ <f2_0>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_1({ <f3_0>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_1({ <f1_0>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_1({ <f2_0>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_1({ <f3_0>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_2({ <f1_1>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_2({ <f2_1>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_2({ <f3_1>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_2({ <f1_1>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_2({ <f2_1>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_2({ <f3_1>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_3({ <f1_2>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_3({ <f2_2>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_3({ <f3_2>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_3({ <f1_2>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_3({ <f2_2>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_3({ <f3_2>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_4({ <f1_3>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_4({ <f2_3>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_4({ <f3_3>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_4({ <f1_3>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_4({ <f2_3>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_4({ <f3_3>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_5({ <f1_4>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_5({ <f2_4>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_5({ <f3_4>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_5({ <f1_4>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_5({ <f2_4>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_5({ <f3_4>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_6({ <f1_5>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_6({ <f2_5>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_6({ <f3_5>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_6({ <f1_5>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_6({ <f2_5>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_6({ <f3_5>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_7({ <f1_6>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_7({ <f2_6>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_7({ <f3_6>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_7({ <f1_6>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_7({ <f2_6>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_7({ <f3_6>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_8({ <f1_7>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_8({ <f2_7>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_8({ <f3_7>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_8({ <f1_7>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_8({ <f2_7>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_8({ <f3_7>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_9({ <f1_8>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_9({ <f2_8>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_9({ <f3_8>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_9({ <f1_8>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_9({ <f2_8>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_9({ <f3_8>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_10({ <f1_9>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_10({ <f2_9>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_10({ <f3_9>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_10({ <f1_9>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_10({ <f2_9>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_10({ <f3_9>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_11({ <f1_10>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_11({ <f2_10>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_11({ <f3_10>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_11({ <f1_10>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_11({ <f2_10>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_11({ <f3_10>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_12({ <f1_11>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_12({ <f2_11>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_12({ <f3_11>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_12({ <f1_11>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_12({ <f2_11>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_12({ <f3_11>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_13({ <f1_12>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_13({ <f2_12>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_13({ <f3_12>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_13({ <f1_12>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_13({ <f2_12>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_13({ <f3_12>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_14({ <f1_13>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_14({ <f2_13>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_14({ <f3_13>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_14({ <f1_13>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_14({ <f2_13>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_14({ <f3_13>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_15({ <f1_14>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_15({ <f2_14>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_15({ <f3_14>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_15({ <f1_14>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_15({ <f2_14>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_15({ <f3_14>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let mut g0: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g1: () = std_function_R__A1__::new_16({ <f1_15>::default() });
        const _: () = assert!(true, "");;
        let mut g2: () = std_function_R__A1__A2__::new_16({ <f2_15>::default() });
        const _: () = assert!(true, "");;
        let mut g3: () = std_function_R__A1__A2__A3__::new_16({ <f3_15>::default() });
        const _: () = assert!(true, "");;
        let mut g4: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g5: () = std_function_R__A1__::new_16({ <f1_15>::default() });
        const _: () = assert!(true, "");;
        let mut g6: () = std_function_R__A1__A2__::new_16({ <f2_15>::default() });
        const _: () = assert!(true, "");;
        let mut g7: () = std_function_R__A1__A2__A3__::new_16({ <f3_15>::default() });
        const _: () = assert!(true, "");;
        let mut unary: () = unimplemented!();
        let mut g8: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g9: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut unary_ref: () = unimplemented!();
        let mut g10: () = unimplemented!();
        const _: () = assert!(true, "");;
        let mut g11: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    return 0_();
}
pub unsafe fn can_deduce_test_5(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn can_deduce_test_6(__args: Box<[VaArg]>) -> () {
    return false;
}
pub unsafe fn can_deduce_test_7(__args: Box<[VaArg]>) -> () {
    return false;
}
pub unsafe fn can_deduce_test_8(__args: Box<[VaArg]>) -> () {
    return false;
}
pub unsafe fn can_deduce_test_9(__args: Box<[VaArg]>) -> () {
    return false;
}
pub unsafe fn can_deduce_test_10(__args: Box<[VaArg]>) -> () {
    return false;
}
pub unsafe fn can_deduce_test_11(__args: Box<[VaArg]>) -> () {
    return false;
}
pub static mut can_deduce_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { can_deduce_test_5(0_()) }) });
pub static mut can_deduce_13: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (unsafe { can_deduce_test_6(Box::new([(0_()).into()])) })
});
pub static mut can_deduce_14: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (unsafe { can_deduce_test_7(Box::new([(0_()).into()])) })
});
pub static mut can_deduce_15: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (unsafe { can_deduce_test_8(Box::new([(0_()).into()])) })
});
pub static mut can_deduce_16: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (unsafe { can_deduce_test_9(Box::new([(0_()).into()])) })
});
pub static mut can_deduce_17: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (unsafe { can_deduce_test_10(Box::new([(0_()).into()])) })
});
pub static mut can_deduce_18: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (unsafe { can_deduce_test_11(Box::new([(0_()).into()])) })
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct valid {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct invalid1 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct invalid2 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct invalid3 {}
const _: () = assert!(true, "can_deduce<valid>");
const _: () = assert!(true, "!can_deduce<invalid1>");
const _: () = assert!(true, "!can_deduce<invalid2>");
const _: () = assert!(true, "!can_deduce<invalid3>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct invalid_rvalue_ref {}
impl invalid_rvalue_ref {
    pub unsafe fn operator_call(&mut self) -> R {
        return <R>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct invalid_c_vararg {}
impl invalid_c_vararg {
    pub unsafe fn operator_call(&mut self, mut _a0: (), __args: Box<[VaArg]>) -> R {
        return <R>::default();
    }
}
const _: () = assert!(true, "!can_deduce<invalid_rvalue_ref>");
const _: () = assert!(true, "!can_deduce<invalid_c_vararg>");
const _: () = assert!(true, "!can_deduce<std::nullptr_t>");
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const can_deduce_12);
    std::cell::LazyCell::force(&*&raw const can_deduce_13);
    std::cell::LazyCell::force(&*&raw const can_deduce_14);
    std::cell::LazyCell::force(&*&raw const can_deduce_15);
    std::cell::LazyCell::force(&*&raw const can_deduce_16);
    std::cell::LazyCell::force(&*&raw const can_deduce_17);
    std::cell::LazyCell::force(&*&raw const can_deduce_18);
}
