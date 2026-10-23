extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntType {
    #[offset(0)]
    pub val: (),
}
impl IntType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        {
            IntTypeImpl::operator_eq(
                &Rc::new(RefCell::new(IntType {
                    val: self.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(IntType {
                    val: other.val.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for IntType {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntTypeNC {
    #[offset(0)]
    pub val: (),
}
impl IntTypeNC {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long___std_extents_unsigned_long__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__18446744073709551615___std_extents_unsigned_long__18446744073709551615__
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__18446744073709551615__18446744073709551615___std_extents_unsigned_long__18446744073709551615__18446744073709551615__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615___std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__5___std_extents_unsigned_long__5__
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__5__18446744073709551615___std_extents_unsigned_long__5__18446744073709551615__
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_extents_unsigned_long__5__18446744073709551615__7___std_extents_unsigned_long__5__18446744073709551615__7__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct NoDefaultCtorIndex {
    #[offset(0)]
    pub value: (),
}
impl NoDefaultCtorIndex {
    pub fn new_1(mut val: ()) -> Self {
        Self { value: val }
    }
}
pub fn test_7(e: (), expected: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_8(e: (), expected: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_9(e: (), expected: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_10(e: (), expected: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_11(e: (), expected: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_12(e: (), expected: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_13(e: (), expected: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_14() -> () {
    let mut D: () = dynamic_extent_15.with(|rc| *rc.borrow());
    ({ test_7(unimplemented!(), unimplemented!()) });
    ({
        test_8(
            std_extents_unsigned_long__18446744073709551615_::new_1({ 1_() }),
            std_extents_unsigned_long__18446744073709551615_::new_1({ 1_() }),
        )
    });
    ({
        test_9(
            std_extents_unsigned_long__18446744073709551615__18446744073709551615_::new_1(
                { 1_() },
                { 2_() },
            ),
            std_extents_unsigned_long__18446744073709551615__18446744073709551615_::new_1(
                { 1_() },
                { 2_() },
            ),
        )
    });
    ({
        test_10 ( std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615_ :: new_1 ( { 1_()  } , { 2_()  } , { 3_()  } , { 4_()  } , { 5_()  } , { 6_()  } , { 7_()  } , { 8_()  } , { 9_()  } , )   , std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615_ :: new_1 ( { 1_()  } , { 2_()  } , { 3_()  } , { 4_()  } , { 5_()  } , { 6_()  } , { 7_()  } , { 8_()  } , { 9_()  } , )   , )
    });
    ({
        test_9(
            std_extents_unsigned_long__18446744073709551615__18446744073709551615_::new_2(
                { NoDefaultCtorIndex::new_1({ 1_() }) },
                { NoDefaultCtorIndex::new_1({ 2_() }) },
            ),
            std_extents_unsigned_long__18446744073709551615__18446744073709551615_::new_3(
                { 1_() },
                { 2_() },
            ),
        )
    });
    ({
        test_11(
            std_extents_unsigned_long__5_::new_1({ unimplemented!() }),
            unimplemented!(),
        )
    });
    ({
        test_12(
            std_extents_unsigned_long__5__18446744073709551615_::new_1({ unimplemented!() }, {
                6_()
            }),
            std_extents_unsigned_long__5__18446744073709551615_::new_2({ 6_() }),
        )
    });
    ({
        test_13(
            std_extents_unsigned_long__5__18446744073709551615__7_::new_1(
                { unimplemented!() },
                { 6_() },
                { unimplemented!() },
            ),
            std_extents_unsigned_long__5__18446744073709551615__7_::new_2({ 6_() }),
        )
    });
    ({
        test_11(
            std_extents_unsigned_long__5_::new_2({ <()>::default() }),
            unimplemented!(),
        )
    });
    ({
        test_12(
            std_extents_unsigned_long__5__18446744073709551615_::new_3({ <()>::default() }, {
                6_()
            }),
            std_extents_unsigned_long__5__18446744073709551615_::new_2({ 6_() }),
        )
    });
    ({
        test_13(
            std_extents_unsigned_long__5__18446744073709551615__7_::new_3(
                { unimplemented!() },
                { 6_() },
                { <()>::default() },
            ),
            std_extents_unsigned_long__5__18446744073709551615__7_::new_2({ 6_() }),
        )
    });
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
    ({ test_14() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait IntTypeImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeImpl for Ptr<IntType> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait IntTypeNCImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeNCImpl for Ptr<IntTypeNC> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait NoDefaultCtorIndexImpl {
    fn to_(&self) -> ();
}
impl NoDefaultCtorIndexImpl for Ptr<NoDefaultCtorIndex> {
    fn to_(&self) -> () {
        return (*self).with(|__s| __s.value);
    }
}
pub fn __cpp2rust_init_globals() {}
