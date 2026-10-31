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
pub struct test_macros_detail_is_same_std_optional_int___std_optional_int__ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_std_unique_ptr_int__std_default_delete_int_____std_optional_std_unique_ptr_int__std_default_delete_int____
{}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_int_ptr___std_optional_int_ptr__ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_optional_int___const_std_optional_int__ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
thread_local!(
    pub static value_6: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_int___std_optional_int__ {}
thread_local!(
    pub static value_7: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_std_unique_ptr_int__std_default_delete_int_____std_optional_std_unique_ptr_int__std_default_delete_int____
{}
thread_local!(
    pub static value_8: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_int_ptr___std_optional_int_ptr__ {}
thread_local!(
    pub static value_9: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
thread_local!(
    pub static can_make_optional_implicit_10: Value<bool> =
        Rc::new(RefCell::new(value_6.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_11: Value<bool> =
        Rc::new(RefCell::new(value_6.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_12: Value<bool> =
        Rc::new(RefCell::new(value_6.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_13: Value<bool> =
        Rc::new(RefCell::new(value_6.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_14: Value<bool> =
        Rc::new(RefCell::new(value_7.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_15: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_implicit_16: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_implicit_17: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_implicit_18: Value<bool> =
        Rc::new(RefCell::new(value_8.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_implicit_19: Value<bool> =
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
pub fn test_20() -> bool {
    {
        let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        ])));
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(((unimplemented!().read()) == (arr.as_pointer() as Ptr::<i32>)));
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "opt.value() == 2");;
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(((unimplemented!().read()) == 2));
    }
    let test_string: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        let mut s: Vec<i8> = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"123").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(Ptr::<i8>::from_string_literal(b"123").with_c_str(|__s| {
            (*unimplemented!().upgrade().deref())[..(*unimplemented!().upgrade().deref())
                .len()
                .saturating_sub(1)]
                == *__s
        }));
    })));
    {
        ({ (*test_string.borrow()).call() });
    }
    let test_unique_ptr: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        let mut s: Option<Value<i32>> = Some(Rc::new(RefCell::new(3)));
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!((((*unimplemented!().upgrade().deref()).as_pointer().read()) == 3));
        assert!(s.is_none());
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_20() });
    const _: () = assert!(true, "test()");;
    return 0;
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
