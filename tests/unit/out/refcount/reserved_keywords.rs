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
    return unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
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
    let mut t: type_ = l.t.clone();
    return ({
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
            (t.v - 3_()),
        )
    });
}
pub fn __cpp2rust_init_globals() {}
