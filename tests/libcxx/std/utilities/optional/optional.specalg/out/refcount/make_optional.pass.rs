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
pub struct test_macros_detail_is_same_std_optional_int___std_optional_int__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_std_unique_ptr_int__std_default_delete_int_____std_optional_std_unique_ptr_int__std_default_delete_int____
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_int_ptr___std_optional_int_ptr__ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_optional_int___const_std_optional_int__ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_std_string___std_optional_std_string__ {}
thread_local!(
    pub static value_6: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_int___std_optional_int__ {}
thread_local!(
    pub static value_7: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_std_unique_ptr_int__std_default_delete_int_____std_optional_std_unique_ptr_int__std_default_delete_int____
{}
thread_local!(
    pub static value_8: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_int_ptr___std_optional_int_ptr__ {}
thread_local!(
    pub static value_9: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
thread_local!(
    pub static can_make_optional_implicit_10: Value<()> =
        Rc::new(RefCell::new(value_6.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_11: Value<()> =
        Rc::new(RefCell::new(value_6.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_12: Value<()> =
        Rc::new(RefCell::new(value_6.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_13: Value<()> =
        Rc::new(RefCell::new(value_6.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_14: Value<()> =
        Rc::new(RefCell::new(value_7.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_15: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_implicit_16: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_implicit_17: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_implicit_18: Value<()> =
        Rc::new(RefCell::new(value_8.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_19: Value<()> =
        Rc::new(RefCell::new(value_9.with(|rc| *rc.borrow())));
);
const _: () = assert!(true, "can_make_optional_implicit<int>");
const _: () = assert!(true, "can_make_optional_implicit<const int>");
const _: () = assert!(true, "can_make_optional_implicit<int&>");
const _: () = assert!(true, "can_make_optional_implicit<const int&>");
const _: () = assert!(true, "can_make_optional_implicit<std::unique_ptr<int>>");
const _: () = assert!(
    true,
    "!can_make_optional_implicit<const std::unique_ptr<int>>"
);
const _: () = assert!(true, "!can_make_optional_implicit<std::unique_ptr<int>&>");
const _: () = assert!(
    true,
    "!can_make_optional_implicit<const std::unique_ptr<int>&>"
);
const _: () = assert!(true, "can_make_optional_implicit<int[42]>");
const _: () = assert!(true, "can_make_optional_implicit<const int (&)[42]>");
pub fn test_20() -> () {
    {
        let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
        ])));
        let opt: Value<()> = Rc::new(RefCell::new(({ make_optional_21(arr.as_pointer()) })));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(
            ({
                let ___v: Value<()> = Rc::new(RefCell::new(2_()));
                make_optional_22(___v.as_pointer())
            }),
        ));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "opt.value() == 2");;
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(
            ({
                let ___v: Value<()> = Rc::new(RefCell::new(2_()));
                make_optional_22(___v.as_pointer())
            }),
        ));
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    let test_string: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        let s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"123") },
                None,
            ),
        ));
        let opt: Value<()> = Rc::new(RefCell::new(({ make_optional_23(s.as_pointer()) })));
        const _: () = assert!(true, "");;
        unimplemented!();
    })));
    {
        ({ (*test_string.borrow()).call() });
    }
    let test_unique_ptr: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        let s: Value<()> = Rc::new(RefCell::new(
            ({
                let ___args: Value<()> = Rc::new(RefCell::new(3_()));
                make_unique_24(___args.as_pointer())
            }),
        ));
        let opt: Value<()> = Rc::new(RefCell::new(
            ({ make_optional_25((s.as_pointer()).reinterpret_cast::<()>()) }),
        ));
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    })));
    {
        ({ (*test_unique_ptr.borrow()).call() });
    }
    return true;
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
    ({ test_20() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = value_6.with(|_| ());
    let _ = value_7.with(|_| ());
    let _ = value_8.with(|_| ());
    let _ = value_9.with(|_| ());
    let _ = can_make_optional_implicit_10.with(|_| ());
    let _ = can_make_optional_implicit_11.with(|_| ());
    let _ = can_make_optional_implicit_12.with(|_| ());
    let _ = can_make_optional_implicit_13.with(|_| ());
    let _ = can_make_optional_implicit_14.with(|_| ());
    let _ = can_make_optional_implicit_15.with(|_| ());
    let _ = can_make_optional_implicit_16.with(|_| ());
    let _ = can_make_optional_implicit_17.with(|_| ());
    let _ = can_make_optional_implicit_18.with(|_| ());
    let _ = can_make_optional_implicit_19.with(|_| ());
}
