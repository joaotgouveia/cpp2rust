extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_function_R______std_function_R_____ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_function_R__A1____std_function_R__A1___ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_function_R__A1__A2____std_function_R__A1__A2___ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_function_R__A1__A2__A3____std_function_R__A1__A2__A3___ {
}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_function_R__A1_refref____std_function_R__A1_refref___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct R {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A1 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A2 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A3 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_0 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_0 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_0 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_0 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_1 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_1 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_1 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_1 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_2 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_2 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_2 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_2 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_3 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_3 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_3 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_3 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_4 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_4 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_4 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_4 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_5 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_5 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_5 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_5 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_6 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_6 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_6 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_6 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_7 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_7 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_7 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_7 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_9 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_9 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_9 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_9 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_10 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_10 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_10 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_10 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_11 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_11 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_11 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_11 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_12 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_12 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_12 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_12 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_13 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_13 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_13 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_13 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_14 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_14 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_14 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_14 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f0_15 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f1_15 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f2_15 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct f3_15 {}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_1({
            <f0_0>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_1({
            <f1_0>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_1({
            <f2_0>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_1({
            <f3_0>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_1({
            <f0_0>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_1({
            <f1_0>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_1({
            <f2_0>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_1({
            <f3_0>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_2({
            <f0_1>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_2({
            <f1_1>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_2({
            <f2_1>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_2({
            <f3_1>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_2({
            <f0_1>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_2({
            <f1_1>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_2({
            <f2_1>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_2({
            <f3_1>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_3({
            <f0_2>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_3({
            <f1_2>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_3({
            <f2_2>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_3({
            <f3_2>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_3({
            <f0_2>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_3({
            <f1_2>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_3({
            <f2_2>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_3({
            <f3_2>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_4({
            <f0_3>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_4({
            <f1_3>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_4({
            <f2_3>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_4({
            <f3_3>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_4({
            <f0_3>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_4({
            <f1_3>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_4({
            <f2_3>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_4({
            <f3_3>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_5({
            <f0_4>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_5({
            <f1_4>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_5({
            <f2_4>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_5({
            <f3_4>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_5({
            <f0_4>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_5({
            <f1_4>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_5({
            <f2_4>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_5({
            <f3_4>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_6({
            <f0_5>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_6({
            <f1_5>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_6({
            <f2_5>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_6({
            <f3_5>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_6({
            <f0_5>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_6({
            <f1_5>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_6({
            <f2_5>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_6({
            <f3_5>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_7({
            <f0_6>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_7({
            <f1_6>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_7({
            <f2_6>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_7({
            <f3_6>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_7({
            <f0_6>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_7({
            <f1_6>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_7({
            <f2_6>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_7({
            <f3_6>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_8({
            <f0_7>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_8({
            <f1_7>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_8({
            <f2_7>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_8({
            <f3_7>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_8({
            <f0_7>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_8({
            <f1_7>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_8({
            <f2_7>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_8({
            <f3_7>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_9({
            <f0_8>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_9({
            <f1_8>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_9({
            <f2_8>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_9({
            <f3_8>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_9({
            <f0_8>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_9({
            <f1_8>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_9({
            <f2_8>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_9({
            <f3_8>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_10({
            <f0_9>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_10({
            <f1_9>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_10({
            <f2_9>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_10({
            <f3_9>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_10({
            <f0_9>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_10({
            <f1_9>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_10({
            <f2_9>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_10({
            <f3_9>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_11({
            <f0_10>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_11({
            <f1_10>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_11({
            <f2_10>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_11({
            <f3_10>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_11({
            <f0_10>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_11({
            <f1_10>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_11({
            <f2_10>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_11({
            <f3_10>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_12({
            <f0_11>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_12({
            <f1_11>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_12({
            <f2_11>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_12({
            <f3_11>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_12({
            <f0_11>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_12({
            <f1_11>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_12({
            <f2_11>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_12({
            <f3_11>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_13({
            <f0_12>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_13({
            <f1_12>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_13({
            <f2_12>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_13({
            <f3_12>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_13({
            <f0_12>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_13({
            <f1_12>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_13({
            <f2_12>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_13({
            <f3_12>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_14({
            <f0_13>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_14({
            <f1_13>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_14({
            <f2_13>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_14({
            <f3_13>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_14({
            <f0_13>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_14({
            <f1_13>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_14({
            <f2_13>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_14({
            <f3_13>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_15({
            <f0_14>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_15({
            <f1_14>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_15({
            <f2_14>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_15({
            <f3_14>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_15({
            <f0_14>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_15({
            <f1_14>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_15({
            <f2_14>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_15({
            <f3_14>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        let g0: Value<()> = Rc::new(RefCell::new(std_function_R____::new_16({
            <f0_15>::default()
        })));
        const _: () = assert!(true, "");;
        let g1: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_16({
            <f1_15>::default()
        })));
        const _: () = assert!(true, "");;
        let g2: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_16({
            <f2_15>::default()
        })));
        const _: () = assert!(true, "");;
        let g3: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_16({
            <f3_15>::default()
        })));
        const _: () = assert!(true, "");;
        let g4: Value<()> = Rc::new(RefCell::new(std_function_R____::new_16({
            <f0_15>::default()
        })));
        const _: () = assert!(true, "");;
        let g5: Value<()> = Rc::new(RefCell::new(std_function_R__A1__::new_16({
            <f1_15>::default()
        })));
        const _: () = assert!(true, "");;
        let g6: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__::new_16({
            <f2_15>::default()
        })));
        const _: () = assert!(true, "");;
        let g7: Value<()> = Rc::new(RefCell::new(std_function_R__A1__A2__A3__::new_16({
            <f3_15>::default()
        })));
        const _: () = assert!(true, "");;
        let unary: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let unary_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let g10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let g11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
    return 0_();
}
pub fn can_deduce_test_5(mut _a0: ()) -> () {
    return true;
}
pub fn can_deduce_test_6(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
pub fn can_deduce_test_7(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
pub fn can_deduce_test_8(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
pub fn can_deduce_test_9(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
pub fn can_deduce_test_10(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
pub fn can_deduce_test_11(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
thread_local!(
    pub static can_deduce_12: Value<()> = Rc::new(RefCell::new(({ can_deduce_test_5(0_()) })));
);
thread_local!(
    pub static can_deduce_13: Value<()> = Rc::new(RefCell::new(
        ({ can_deduce_test_6(Box::new([(0_()).into()])) }),
    ));
);
thread_local!(
    pub static can_deduce_14: Value<()> = Rc::new(RefCell::new(
        ({ can_deduce_test_7(Box::new([(0_()).into()])) }),
    ));
);
thread_local!(
    pub static can_deduce_15: Value<()> = Rc::new(RefCell::new(
        ({ can_deduce_test_8(Box::new([(0_()).into()])) }),
    ));
);
thread_local!(
    pub static can_deduce_16: Value<()> = Rc::new(RefCell::new(
        ({ can_deduce_test_9(Box::new([(0_()).into()])) }),
    ));
);
thread_local!(
    pub static can_deduce_17: Value<()> = Rc::new(RefCell::new(
        ({ can_deduce_test_10(Box::new([(0_()).into()])) }),
    ));
);
thread_local!(
    pub static can_deduce_18: Value<()> = Rc::new(RefCell::new(
        ({ can_deduce_test_11(Box::new([(0_()).into()])) }),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct valid {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct invalid1 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct invalid2 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct invalid3 {}
const _: () = assert!(true, "can_deduce<valid>");
const _: () = assert!(true, "!can_deduce<invalid1>");
const _: () = assert!(true, "!can_deduce<invalid2>");
const _: () = assert!(true, "!can_deduce<invalid3>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct invalid_rvalue_ref {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct invalid_c_vararg {}
const _: () = assert!(true, "!can_deduce<invalid_rvalue_ref>");
const _: () = assert!(true, "!can_deduce<invalid_c_vararg>");
const _: () = assert!(true, "!can_deduce<std::nullptr_t>");
pub trait f0_0Impl {
    fn operator_call(&self) -> R;
}
impl f0_0Impl for Ptr<f0_0> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_1Impl {
    fn operator_call(&self) -> R;
}
impl f0_1Impl for Ptr<f0_1> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_10Impl {
    fn operator_call(&self) -> R;
}
impl f0_10Impl for Ptr<f0_10> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_11Impl {
    fn operator_call(&self) -> R;
}
impl f0_11Impl for Ptr<f0_11> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_12Impl {
    fn operator_call(&self) -> R;
}
impl f0_12Impl for Ptr<f0_12> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_13Impl {
    fn operator_call(&self) -> R;
}
impl f0_13Impl for Ptr<f0_13> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_14Impl {
    fn operator_call(&self) -> R;
}
impl f0_14Impl for Ptr<f0_14> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_15Impl {
    fn operator_call(&self) -> R;
}
impl f0_15Impl for Ptr<f0_15> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_2Impl {
    fn operator_call(&self) -> R;
}
impl f0_2Impl for Ptr<f0_2> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_3Impl {
    fn operator_call(&self) -> R;
}
impl f0_3Impl for Ptr<f0_3> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_4Impl {
    fn operator_call(&self) -> R;
}
impl f0_4Impl for Ptr<f0_4> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_5Impl {
    fn operator_call(&self) -> R;
}
impl f0_5Impl for Ptr<f0_5> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_6Impl {
    fn operator_call(&self) -> R;
}
impl f0_6Impl for Ptr<f0_6> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_7Impl {
    fn operator_call(&self) -> R;
}
impl f0_7Impl for Ptr<f0_7> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_8Impl {
    fn operator_call(&self) -> R;
}
impl f0_8Impl for Ptr<f0_8> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f0_9Impl {
    fn operator_call(&self) -> R;
}
impl f0_9Impl for Ptr<f0_9> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait f1_0Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_0Impl for Ptr<f1_0> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_1Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_1Impl for Ptr<f1_1> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_10Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_10Impl for Ptr<f1_10> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_11Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_11Impl for Ptr<f1_11> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_12Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_12Impl for Ptr<f1_12> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_13Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_13Impl for Ptr<f1_13> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_14Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_14Impl for Ptr<f1_14> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_15Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_15Impl for Ptr<f1_15> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_2Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_2Impl for Ptr<f1_2> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_3Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_3Impl for Ptr<f1_3> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_4Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_4Impl for Ptr<f1_4> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_5Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_5Impl for Ptr<f1_5> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_6Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_6Impl for Ptr<f1_6> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_7Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_7Impl for Ptr<f1_7> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_8Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_8Impl for Ptr<f1_8> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f1_9Impl {
    fn operator_call(&self, _a0: A1) -> R;
}
impl f1_9Impl for Ptr<f1_9> {
    fn operator_call(&self, mut _a0: A1) -> R {
        return <R>::default();
    }
}
pub trait f2_0Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_0Impl for Ptr<f2_0> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_1Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_1Impl for Ptr<f2_1> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_10Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_10Impl for Ptr<f2_10> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_11Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_11Impl for Ptr<f2_11> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_12Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_12Impl for Ptr<f2_12> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_13Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_13Impl for Ptr<f2_13> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_14Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_14Impl for Ptr<f2_14> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_15Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_15Impl for Ptr<f2_15> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_2Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_2Impl for Ptr<f2_2> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_3Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_3Impl for Ptr<f2_3> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_4Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_4Impl for Ptr<f2_4> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_5Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_5Impl for Ptr<f2_5> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_6Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_6Impl for Ptr<f2_6> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_7Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_7Impl for Ptr<f2_7> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_8Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_8Impl for Ptr<f2_8> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f2_9Impl {
    fn operator_call(&self, _a0: A1, _a1: A2) -> R;
}
impl f2_9Impl for Ptr<f2_9> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2) -> R {
        return <R>::default();
    }
}
pub trait f3_0Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_0Impl for Ptr<f3_0> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_1Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_1Impl for Ptr<f3_1> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_10Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_10Impl for Ptr<f3_10> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_11Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_11Impl for Ptr<f3_11> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_12Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_12Impl for Ptr<f3_12> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_13Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_13Impl for Ptr<f3_13> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_14Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_14Impl for Ptr<f3_14> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_15Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_15Impl for Ptr<f3_15> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_2Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_2Impl for Ptr<f3_2> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_3Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_3Impl for Ptr<f3_3> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_4Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_4Impl for Ptr<f3_4> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_5Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_5Impl for Ptr<f3_5> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_6Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_6Impl for Ptr<f3_6> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_7Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_7Impl for Ptr<f3_7> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_8Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_8Impl for Ptr<f3_8> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait f3_9Impl {
    fn operator_call(&self, _a0: A1, _a1: A2, _a2: A3) -> R;
}
impl f3_9Impl for Ptr<f3_9> {
    fn operator_call(&self, mut _a0: A1, mut _a1: A2, mut _a2: A3) -> R {
        return <R>::default();
    }
}
pub trait invalid3Impl {
    fn operator_call_1(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_call_2(&self, mut _a0: ()) {
        unimplemented!()
    }
}
impl invalid3Impl for Ptr<invalid3> {}
pub trait invalid_c_varargImpl {
    fn operator_call(&self, _a0: (), __args: Box<[VaArg]>) -> R;
}
impl invalid_c_varargImpl for Ptr<invalid_c_vararg> {
    fn operator_call(&self, mut _a0: (), __args: Box<[VaArg]>) -> R {
        let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
        return <R>::default();
    }
}
pub trait invalid_rvalue_refImpl {
    fn operator_call(&self) -> R;
}
impl invalid_rvalue_refImpl for Ptr<invalid_rvalue_ref> {
    fn operator_call(&self) -> R {
        return <R>::default();
    }
}
pub trait validImpl {
    fn operator_call(&self) -> () {
        unimplemented!()
    }
}
impl validImpl for Ptr<valid> {}
pub fn __cpp2rust_init_globals() {
    let _ = can_deduce_12.with(|_| ());
    let _ = can_deduce_13.with(|_| ());
    let _ = can_deduce_14.with(|_| ());
    let _ = can_deduce_15.with(|_| ());
    let _ = can_deduce_16.with(|_| ());
    let _ = can_deduce_17.with(|_| ());
    let _ = can_deduce_18.with(|_| ());
}
