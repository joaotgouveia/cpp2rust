extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_filesystem_path_iterator__std_filesystem_path_iterator_ {
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_filesystem_path__std_filesystem_path_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_filesystem_path_iterator_ref__std_filesystem_path_iterator_ref_
{}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_filesystem_path_ref__const_std_filesystem_path_ref_
{}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_filesystem_path_ptr__const_std_filesystem_path_ptr_
{}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ref__const_std_basic_string_char__std_char_traits_char___std_allocator_char___ref_
{}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub fn checkCollectionsEqual_7(
    start1: (),
    end1: (),
    mut start2: Ptr<Vec<i8>>,
    mut end2: Ptr<Vec<i8>>,
) -> bool {
    let start1: Value<()> = Rc::new(RefCell::new(start1));
    let end1: Value<()> = Rc::new(RefCell::new(end1));
    'loop_: while ((*start1.borrow()) != (*end1.borrow()))
        && ({ (start2).clone() } != { (end2).clone() })
    {
        if !(unimplemented!()) {
            return false;
        }
        (*start1.borrow_mut()).iter_inc();
        start2.prefix_inc();
    }
    return (((*start1.borrow()) == (*end1.borrow())) && ({ start2 } == { end2 }));
}
pub fn checkCollectionsEqualBackwards_8(
    start1: (),
    end1: (),
    mut start2: Ptr<Vec<i8>>,
    mut end2: Ptr<Vec<i8>>,
) -> bool {
    let start1: Value<()> = Rc::new(RefCell::new(start1));
    let end1: Value<()> = Rc::new(RefCell::new(end1));
    'loop_: while ((*start1.borrow()) != (*end1.borrow()))
        && ({ (start2).clone() } != { (end2).clone() })
    {
        (*end1.borrow_mut()).iter_dec();
        end2.prefix_dec();
        if !(unimplemented!()) {
            return false;
        }
    }
    return (((*start1.borrow()) == (*end1.borrow())) && ({ start2 } == { end2 }));
}
pub fn checkIteratorConcepts_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::bidirectional_iterator<It>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == unimplemented!());
    }
}
pub fn checkBeginEndBasic_10() {
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(unimplemented!() == unimplemented!());
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (b"foo" as Ptr<i8>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        let default_constructed: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*default_constructed.borrow_mut()) = unimplemented!().clone();
        assert!((*default_constructed.borrow()) == unimplemented!());
        assert!((*default_constructed.borrow()) != unimplemented!());
        (*default_constructed.borrow_mut()) = unimplemented!().clone();
        assert!((*default_constructed.borrow()) == unimplemented!());
        assert!((*default_constructed.borrow()) != unimplemented!());
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (b"//root_name//first_dir////second_dir" as Ptr<i8>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        let expect: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            {
                let mut __v: Vec<i8> = (b"/" as Ptr<i8>)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as i8)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<i8> = (b"root_name" as Ptr<i8>)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as i8)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<i8> = (b"first_dir" as Ptr<i8>)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as i8)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<i8> = (b"second_dir" as Ptr<i8>)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as i8)
                    .collect();
                __v.push(0);
                __v
            },
        ])));
        assert!(
            ({
                let _start1: () = unimplemented!();
                let _end1: () = unimplemented!();
                let _start2: Ptr<Vec<i8>> = unimplemented!();
                let _end2: Ptr<Vec<i8>> = (expect.as_pointer() as Ptr<Vec<i8>>).to_end();
                checkCollectionsEqual_7(_start1, _end1, _start2, _end2)
            })
        );
        assert!(
            ({
                let _start1: () = unimplemented!();
                let _end1: () = unimplemented!();
                let _start2: Ptr<Vec<i8>> = unimplemented!();
                let _end2: Ptr<Vec<i8>> = (expect.as_pointer() as Ptr<Vec<i8>>).to_end();
                checkCollectionsEqualBackwards_8(_start1, _end1, _start2, _end2)
            })
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (b"////foo/bar/baz///" as Ptr<i8>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        let expect: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
            {
                let mut __v: Vec<i8> = (b"/" as Ptr<i8>)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as i8)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<i8> = (b"foo" as Ptr<i8>)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as i8)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<i8> = (b"bar" as Ptr<i8>)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as i8)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<i8> = (b"baz" as Ptr<i8>)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as i8)
                    .collect();
                __v.push(0);
                __v
            },
            {
                let mut __v: Vec<i8> = (b"" as Ptr<i8>)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as i8)
                    .collect();
                __v.push(0);
                __v
            },
        ])));
        assert!(
            ({
                let _start1: () = unimplemented!();
                let _end1: () = unimplemented!();
                let _start2: Ptr<Vec<i8>> = unimplemented!();
                let _end2: Ptr<Vec<i8>> = (expect.as_pointer() as Ptr<Vec<i8>>).to_end();
                checkCollectionsEqual_7(_start1, _end1, _start2, _end2)
            })
        );
        assert!(
            ({
                let _start1: () = unimplemented!();
                let _end1: () = unimplemented!();
                let _start2: Ptr<Vec<i8>> = unimplemented!();
                let _end2: Ptr<Vec<i8>> = (expect.as_pointer() as Ptr<Vec<i8>>).to_end();
                checkCollectionsEqualBackwards_8(_start1, _end1, _start2, _end2)
            })
        );
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ checkIteratorConcepts_9() });
    ({ checkBeginEndBasic_10() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
