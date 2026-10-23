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
pub struct test_macros_detail_is_same_std_filesystem_path_iterator__std_filesystem_path_iterator_ {
}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_filesystem_path__std_filesystem_path_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_filesystem_path_iterator_ref__std_filesystem_path_iterator_ref_
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_filesystem_path_ptr__const_std_filesystem_path_ptr_
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_string_ref__const_std_string_ref_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub fn checkCollectionsEqual_6(start1: (), end1: (), mut start2: Ptr<()>, mut end2: Ptr<()>) -> () {
    let start1: Value<()> = Rc::new(RefCell::new(start1));
    let end1: Value<()> = Rc::new(RefCell::new(end1));
    'loop_: while (unimplemented!()) && ({ (start2).clone() } != { (end2).clone() }) {
        if !(unimplemented!()) {
            return false;
        }
        unimplemented!();
        start2.prefix_inc();
    }
    return ((unimplemented!()) && ({ start2 } == { end2 }));
}
pub fn checkCollectionsEqualBackwards_7(
    start1: (),
    end1: (),
    mut start2: Ptr<()>,
    mut end2: Ptr<()>,
) -> () {
    let start1: Value<()> = Rc::new(RefCell::new(start1));
    let end1: Value<()> = Rc::new(RefCell::new(end1));
    'loop_: while (unimplemented!()) && ({ (start2).clone() } != { (end2).clone() }) {
        unimplemented!();
        end2.prefix_dec();
        if !(unimplemented!()) {
            return false;
        }
    }
    return ((unimplemented!()) && ({ start2 } == { end2 }));
}
pub fn checkIteratorConcepts_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::bidirectional_iterator<It>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "std::is_same<Traits::iterator_category, std::input_iterator_tag>::value"
    );;
    const _: () = assert!(true, "std::is_same<Traits::pointer, path const*>::value");;
    const _: () = assert!(true, "std::is_same<Traits::reference, path>::value");;
    {
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
}
pub fn checkBeginEndBasic_9() {
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(std_filesystem_path::new_1(
            { Ptr::<()>::from_string_literal(b"foo") },
            None,
        )));
        let default_constructed: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(std_filesystem_path::new_2(
            { Ptr::<()>::from_string_literal(b"//root_name//first_dir////second_dir") },
            None,
        )));
        let mut expect: [(); 4] = [
            std_filesystem_path::new_3({ Ptr::<()>::from_string_literal(b"/") }, None),
            std_filesystem_path::new_4({ Ptr::<()>::from_string_literal(b"root_name") }, None),
            std_filesystem_path::new_4({ Ptr::<()>::from_string_literal(b"first_dir") }, None),
            std_filesystem_path::new_5({ Ptr::<()>::from_string_literal(b"second_dir") }, None),
        ];
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(std_filesystem_path::new_6(
            { Ptr::<()>::from_string_literal(b"////foo/bar/baz///") },
            None,
        )));
        let mut expect: [(); 5] = [
            std_filesystem_path::new_3({ Ptr::<()>::from_string_literal(b"/") }, None),
            std_filesystem_path::new_1({ Ptr::<()>::from_string_literal(b"foo") }, None),
            std_filesystem_path::new_1({ Ptr::<()>::from_string_literal(b"bar") }, None),
            std_filesystem_path::new_1({ Ptr::<()>::from_string_literal(b"baz") }, None),
            std_filesystem_path::new_7({ Ptr::<()>::from_string_literal(b"") }, None),
        ];
        unimplemented!();
        unimplemented!();
    }
}
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
    ({ checkIteratorConcepts_8() });
    ({ checkBeginEndBasic_9() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
