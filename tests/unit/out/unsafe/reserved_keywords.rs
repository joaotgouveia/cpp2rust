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
    return unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S {
        as_: unimplemented!(),
        async_: unimplemented!(),
        await_: unimplemented!(),
        crate_: unimplemented!(),
        dyn_: unimplemented!(),
        fn_: unimplemented!(),
        impl_: unimplemented!(),
        in_: unimplemented!(),
        let_: unimplemented!(),
        loop_: unimplemented!(),
        match_: unimplemented!(),
        mod_: unimplemented!(),
        move_: unimplemented!(),
        mut_: unimplemented!(),
        pub_: unimplemented!(),
        ref_: unimplemented!(),
        self_: unimplemented!(),
        Self_: unimplemented!(),
        super_: unimplemented!(),
        trait_: unimplemented!(),
        type_: unimplemented!(),
        unsafe_: unimplemented!(),
        use_: unimplemented!(),
        where_: unimplemented!(),
        abstract_: unimplemented!(),
        become_: unimplemented!(),
        box_: unimplemented!(),
        final_: unimplemented!(),
        gen_: unimplemented!(),
        macro_: unimplemented!(),
        override_: unimplemented!(),
        priv_: unimplemented!(),
        unsized_: unimplemented!(),
        yield_: unimplemented!(),
        macro_rules_: unimplemented!(),
        raw_: unimplemented!(),
        safe_: unimplemented!(),
        vec_: unimplemented!(),
    };
    let mut as_: () = unimplemented!();
    let mut async_: () = unimplemented!();
    let mut await_: () = unimplemented!();
    let mut crate_: () = unimplemented!();
    let mut dyn_: () = unimplemented!();
    let mut fn_: () = unimplemented!();
    let mut impl_: () = unimplemented!();
    let mut in_: () = unimplemented!();
    let mut let_: () = unimplemented!();
    let mut loop_: () = unimplemented!();
    let mut match_: () = unimplemented!();
    let mut mod_: () = unimplemented!();
    let mut move_: () = unimplemented!();
    let mut mut_: () = unimplemented!();
    let mut pub_: () = unimplemented!();
    let mut ref_: () = unimplemented!();
    let mut self_: () = unimplemented!();
    let mut Self_: () = unimplemented!();
    let mut super_: () = unimplemented!();
    let mut trait_: () = unimplemented!();
    let mut type_: () = unimplemented!();
    let mut unsafe_: () = unimplemented!();
    let mut use_: () = unimplemented!();
    let mut where_: () = unimplemented!();
    let mut abstract_: () = unimplemented!();
    let mut become_: () = unimplemented!();
    let mut box_: () = unimplemented!();
    let mut final_: () = unimplemented!();
    let mut gen_: () = unimplemented!();
    let mut macro_: () = unimplemented!();
    let mut override_: () = unimplemented!();
    let mut priv_: () = unimplemented!();
    let mut unsized_: () = unimplemented!();
    let mut yield_: () = unimplemented!();
    let mut macro_rules_: () = unimplemented!();
    let mut raw_: () = unimplemented!();
    let mut safe_: () = unimplemented!();
    let mut vec_: () = unimplemented!();
    let mut l: loop_ = loop_ {
        t: type_ { v: 3_() },
    };
    let mut t: type_ = l.t;
    return (unsafe {
        foo_0(
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            ((t.v) - (3_())),
        )
    });
}
pub unsafe fn __cpp2rust_init_globals() {}
