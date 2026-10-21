extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(152)]
pub struct S {
    #[offset(0)]
    pub as_: (),
    #[offset(4)]
    pub async_: (),
    #[offset(8)]
    pub await_: (),
    #[offset(12)]
    pub crate_: (),
    #[offset(16)]
    pub dyn_: (),
    #[offset(20)]
    pub fn_: (),
    #[offset(24)]
    pub impl_: (),
    #[offset(28)]
    pub in_: (),
    #[offset(32)]
    pub let_: (),
    #[offset(36)]
    pub loop_: (),
    #[offset(40)]
    pub match_: (),
    #[offset(44)]
    pub mod_: (),
    #[offset(48)]
    pub move_: (),
    #[offset(52)]
    pub mut_: (),
    #[offset(56)]
    pub pub_: (),
    #[offset(60)]
    pub ref_: (),
    #[offset(64)]
    pub self_: (),
    #[offset(68)]
    pub Self_: (),
    #[offset(72)]
    pub super_: (),
    #[offset(76)]
    pub trait_: (),
    #[offset(80)]
    pub type_: (),
    #[offset(84)]
    pub unsafe_: (),
    #[offset(88)]
    pub use_: (),
    #[offset(92)]
    pub where_: (),
    #[offset(96)]
    pub abstract_: (),
    #[offset(100)]
    pub become_: (),
    #[offset(104)]
    pub box_: (),
    #[offset(108)]
    pub final_: (),
    #[offset(112)]
    pub gen_: (),
    #[offset(116)]
    pub macro_: (),
    #[offset(120)]
    pub override_: (),
    #[offset(124)]
    pub priv_: (),
    #[offset(128)]
    pub unsized_: (),
    #[offset(132)]
    pub yield_: (),
    #[offset(136)]
    pub macro_rules_: (),
    #[offset(140)]
    pub raw_: (),
    #[offset(144)]
    pub safe_: (),
    #[offset(148)]
    pub vec_: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct type_ {
    #[offset(0)]
    pub v: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct loop_ {
    #[offset(0)]
    #[byte_size(4)]
    pub t: type_,
}
pub fn foo_0(
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
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
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
    let mut t: type_ = l.t.clone();
    return ({
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
            (t.v - 3_()),
        )
    });
}
pub fn __cpp2rust_init_globals() {}
