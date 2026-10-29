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
pub struct S {
    pub as_: (),
    pub async_: (),
    pub await_: (),
    pub crate_: (),
    pub dyn_: (),
    pub fn_: (),
    pub impl_: (),
    pub in_: (),
    pub let_: (),
    pub loop_: (),
    pub match_: (),
    pub mod_: (),
    pub move_: (),
    pub mut_: (),
    pub pub_: (),
    pub ref_: (),
    pub self_: (),
    pub Self_: (),
    pub super_: (),
    pub trait_: (),
    pub type_: (),
    pub unsafe_: (),
    pub use_: (),
    pub where_: (),
    pub abstract_: (),
    pub become_: (),
    pub box_: (),
    pub final_: (),
    pub gen_: (),
    pub macro_: (),
    pub override_: (),
    pub priv_: (),
    pub unsized_: (),
    pub yield_: (),
    pub macro_rules_: (),
    pub raw_: (),
    pub safe_: (),
    pub vec_: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct type_ {
    pub v: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct loop_ {
    pub t: type_,
}
pub unsafe fn foo_0(
    mut as_: (),
    mut async_: (),
    mut await_: (),
    mut crate_: (),
    mut dyn_: (),
    mut fn_: (),
    mut impl_: (),
    mut in_: (),
    mut let_: (),
    mut loop_: (),
    mut match_: (),
    mut mod_: (),
    mut move_: (),
    mut mut_: (),
    mut pub_: (),
    mut ref_: (),
    mut self_: (),
    mut Self_: (),
    mut super_: (),
    mut trait_: (),
    mut type_: (),
    mut unsafe_: (),
    mut use_: (),
    mut where_: (),
    mut abstract_: (),
    mut become_: (),
    mut box_: (),
    mut final_: (),
    mut gen_: (),
    mut macro_: (),
    mut override_: (),
    mut priv_: (),
    mut unsized_: (),
    mut yield_: (),
    mut macro_rules_: (),
    mut raw_: (),
    mut safe_: (),
    mut vec_: (),
    mut dummy: (),
) -> () {
    return 0_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S {
        as_: 0_(),
        async_: 0_(),
        await_: 0_(),
        crate_: 0_(),
        dyn_: 0_(),
        fn_: 0_(),
        impl_: 0_(),
        in_: 0_(),
        let_: 0_(),
        loop_: 0_(),
        match_: 0_(),
        mod_: 0_(),
        move_: 0_(),
        mut_: 0_(),
        pub_: 0_(),
        ref_: 0_(),
        self_: 0_(),
        Self_: 0_(),
        super_: 0_(),
        trait_: 0_(),
        type_: 0_(),
        unsafe_: 0_(),
        use_: 0_(),
        where_: 0_(),
        abstract_: 0_(),
        become_: 0_(),
        box_: 0_(),
        final_: 0_(),
        gen_: 0_(),
        macro_: 0_(),
        override_: 0_(),
        priv_: 0_(),
        unsized_: 0_(),
        yield_: 0_(),
        macro_rules_: 0_(),
        raw_: 0_(),
        safe_: 0_(),
        vec_: 0_(),
    };
    let mut as_: () = 0_();
    let mut async_: () = 0_();
    let mut await_: () = 0_();
    let mut crate_: () = 0_();
    let mut dyn_: () = 0_();
    let mut fn_: () = 0_();
    let mut impl_: () = 0_();
    let mut in_: () = 0_();
    let mut let_: () = 0_();
    let mut loop_: () = 0_();
    let mut match_: () = 0_();
    let mut mod_: () = 0_();
    let mut move_: () = 0_();
    let mut mut_: () = 0_();
    let mut pub_: () = 0_();
    let mut ref_: () = 0_();
    let mut self_: () = 0_();
    let mut Self_: () = 0_();
    let mut super_: () = 0_();
    let mut trait_: () = 0_();
    let mut type_: () = 0_();
    let mut unsafe_: () = 0_();
    let mut use_: () = 0_();
    let mut where_: () = 0_();
    let mut abstract_: () = 0_();
    let mut become_: () = 0_();
    let mut box_: () = 0_();
    let mut final_: () = 0_();
    let mut gen_: () = 0_();
    let mut macro_: () = 0_();
    let mut override_: () = 0_();
    let mut priv_: () = 0_();
    let mut unsized_: () = 0_();
    let mut yield_: () = 0_();
    let mut macro_rules_: () = 0_();
    let mut raw_: () = 0_();
    let mut safe_: () = 0_();
    let mut vec_: () = 0_();
    let mut l: loop_ = loop_ {
        t: type_ { v: 3_() },
    };
    let mut t: type_ = l.t;
    return (unsafe {
        foo_0(
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            0_(),
            ((t.v) - (3_())),
        )
    });
}
pub unsafe fn __cpp2rust_init_globals() {}
