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
pub struct test_macros_detail_is_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_std_unique_ptr_const_int__std_default_delete_const_int_____std_optional_std_unique_ptr_const_int__std_default_delete_const_int____
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_double___std_optional_double__ {}
thread_local!(
    pub static value_4: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_int___std_optional_int__ {}
thread_local!(
    pub static value_5: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
thread_local!(
    pub static value_6: Value<()> = Rc::new(RefCell::new(true));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct mandate_same_std_optional_std_unique_ptr_const_int__std_default_delete_const_int_____std_optional_std_unique_ptr_const_int__std_default_delete_const_int____
{}
thread_local!(
    pub static can_make_optional_explicit_impl_7: Value<()> =
        Rc::new(RefCell::new(value_4.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_explicit_impl_8: Value<()> =
        Rc::new(RefCell::new(value_4.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_explicit_impl_9: Value<()> =
        Rc::new(RefCell::new(value_5.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_explicit_impl_10: Value<()> =
        Rc::new(RefCell::new(value_5.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_explicit_impl_11: Value<()> =
        Rc::new(RefCell::new(value_6.with(|rc| *rc.borrow())));
);
thread_local!(
    pub static can_make_optional_explicit_impl_12: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_impl_13: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_impl_14: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static can_make_optional_explicit_15: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_7.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_16: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_8.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_17: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_9.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_18: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_10.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_19: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_11.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_20: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_12.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_21: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_13.with(|rc| *rc.borrow()),
    ));
);
thread_local!(
    pub static can_make_optional_explicit_22: Value<()> = Rc::new(RefCell::new(
        can_make_optional_explicit_impl_14.with(|rc| *rc.borrow()),
    ));
);
const _: () = assert!(true, "can_make_optional_explicit<int, long>");
const _: () = assert!(true, "can_make_optional_explicit<int, const long&>");
const _: () = assert!(true, "can_make_optional_explicit<std::string, const char*>");
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::string, const char* const&>"
);
const _: () = assert!(
    true,
    "can_make_optional_explicit<std::unique_ptr<const int>, std::unique_ptr<int>>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::unique_ptr<const int>, const std::unique_ptr<int>>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::unique_ptr<const int>, std::unique_ptr<int>&>"
);
const _: () = assert!(
    true,
    "!can_make_optional_explicit<std::unique_ptr<const int>, const std::unique_ptr<int>&>"
);
pub fn test_ref_23() {
    let mut i: () = 0_();
    let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_ref_24() {
    let mut i: () = 0_();
    let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_25() -> () {
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "*opt == int(\'a\')");;
    }
    let test_string: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        {
            let mut s: () = unimplemented!();
            let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
        {
            let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
    })));
    {
        ({ (*test_string.borrow()).call() });
    }
    let test_unique_ptr: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        let mut s: () = unimplemented!();
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    })));
    {
        ({ (*test_unique_ptr.borrow()).call() });
    }
    ({ test_ref_23() });
    ({ test_ref_24() });
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
    ({ test_25() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = value_4.with(|_| ());
    let _ = value_5.with(|_| ());
    let _ = value_6.with(|_| ());
    let _ = can_make_optional_explicit_impl_7.with(|_| ());
    let _ = can_make_optional_explicit_impl_8.with(|_| ());
    let _ = can_make_optional_explicit_impl_9.with(|_| ());
    let _ = can_make_optional_explicit_impl_10.with(|_| ());
    let _ = can_make_optional_explicit_impl_11.with(|_| ());
    let _ = can_make_optional_explicit_impl_12.with(|_| ());
    let _ = can_make_optional_explicit_impl_13.with(|_| ());
    let _ = can_make_optional_explicit_impl_14.with(|_| ());
    let _ = can_make_optional_explicit_15.with(|_| ());
    let _ = can_make_optional_explicit_16.with(|_| ());
    let _ = can_make_optional_explicit_17.with(|_| ());
    let _ = can_make_optional_explicit_18.with(|_| ());
    let _ = can_make_optional_explicit_19.with(|_| ());
    let _ = can_make_optional_explicit_20.with(|_| ());
    let _ = can_make_optional_explicit_21.with(|_| ());
    let _ = can_make_optional_explicit_22.with(|_| ());
}
