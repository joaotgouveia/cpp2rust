extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_1(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } != { RHS.with(|__s| __s.m_id.clone()) });
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(8)]
pub struct TypeID {
    #[offset(0)]
    #[byte_size(8)]
    m_id: Ptr<()>,
}
impl TypeID {
    fn new(mut xid: Ptr<()>) -> Self {
        Self {
            m_id: (xid).clone(),
        }
    }
}
impl std::cmp::PartialEq for TypeID {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(TypeID {
                    m_id: self.m_id.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TypeID {
                    m_id: other.m_id.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TypeID {}
pub type CallType = ();
pub const CallType_CT_None: CallType = 0;
pub const CallType_CT_NonConst: CallType = 1;
pub const CallType_CT_Const: CallType = 2;
pub const CallType_CT_LValue: CallType = 4;
pub const CallType_CT_RValue: CallType = 8;
pub fn operator_bitor_2(mut LHS: CallType, mut RHS: CallType) -> CallType {
    return (((LHS as ()) | (RHS as ())) as CallType);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ForwardingCallObject {}
thread_local!(
    pub static last_call_type_3: Value<CallType> = Rc::new(RefCell::new(CallType_CT_None));
);
thread_local!(
    pub static last_call_args_4: Value<Ptr<TypeID>> = Rc::new(RefCell::new(Ptr::<TypeID>::null()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnFirst {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnArity {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Dummy {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ThrowsT {}
impl ThrowsT {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoThrowT {}
impl NoThrowT {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
}
impl AnyConstructible {
    pub fn new(_a0: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct AnyConstructible {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoConstructible {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RValueConvertibleFrom_int_ {}
impl RValueConvertibleFrom_int_ {
    pub fn new(_a0: Ptr<()>) -> Self {
        Self {}
    }
}
pub fn test_T_ctor_noexcept_5() {
    {
        const _: () = assert!(true, "std::is_nothrow_constructible<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_constructible<V, int>::value");;
    }
}
pub fn test_T_ctor_sfinae_6() {
    {
        const _: () = assert!(true, "!std::is_constructible<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<V, const char*>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<V, int>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<V, int>::value");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::unique_ptr<char>>::value"
        );;
        const _: () = assert!(true, "!std::is_constructible<V, X>::value");;
        const _: () = assert!(true, "std::is_constructible<V, std::false_type>::value");;
    }
    {
        const _: () = assert!(true, "std::is_constructible<V, Y>::value");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_type_t<NoConstructible>>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<1>>::value"
        );;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X_7 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X_8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Y_9 {}
pub fn test_T_ctor_basic_10() {
    {
        let v: Value<()> = Rc::new(RefCell::new(42_()));
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v) == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(42_()));
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(42_()));
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(std_variant_std_string__const_bool_::new({
            Ptr::<()>::from_string_literal(b"foo")
        })));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(Default::default()));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(true));
        unimplemented!();
        unimplemented!();
    }
    {
        let v1: Value<()> = Rc::new(RefCell::new(42_()));
        unimplemented!();
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let v2: Value<()> = Rc::new(RefCell::new(
            std_variant_RValueConvertibleFrom_int___AnyConstructible_::new({ x.as_pointer() }),
        ));
        unimplemented!();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BoomOnAnything {}
pub fn test_no_narrowing_check_for_class_types_11() {
    let v: Value<()> = Rc::new(RefCell::new(42_()));
    unimplemented!();
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Bar {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Baz {}
pub fn test_construction_with_repeated_types_12() {
    const _: () = assert!(true, "!std::is_constructible<V, int>::value");;
    const _: () = assert!(true, "!std::is_constructible<V, Baz>::value");;
    const _: () = assert!(true, "std::is_constructible<V, Bar>::value");;
}
pub fn test_vector_bool_13() {
    let mut vec_: () = unimplemented!();
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleFromAny {}
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
    ({ test_T_ctor_basic_10() });
    ({ test_T_ctor_noexcept_5() });
    ({ test_T_ctor_sfinae_6() });
    ({ test_no_narrowing_check_for_class_types_11() });
    ({ test_construction_with_repeated_types_12() });
    ({ test_vector_bool_13() });
    {
        let vec_: Value<Matcher_14> = Rc::new(RefCell::new(Matcher_14::new_1()));
        let mut m: Matcher_14 = (*vec_.borrow()).clone();
    }
    return 0_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Matcher_14 {}
impl Matcher_14 {
    pub fn new_1() -> Self {
        Self {}
    }
    pub fn new_2(_a0: ()) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {}
    }
}
impl Default for Matcher_14 {
    fn default() -> Self {
        { Matcher_14::new_1() }
    }
}
pub trait ForwardingCallObjectImpl {
    fn to_(&self) -> ();
}
impl ForwardingCallObjectImpl for Ptr<ForwardingCallObject> {
    fn to_(&self) -> () {
        return 0_();
    }
}
pub trait TypeIDImpl {
    fn name(&self) -> ();
}
impl TypeIDImpl for Ptr<TypeID> {
    fn name(&self) -> () {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { (*self).with(|__s| __s.m_id.clone()) },
            None,
        );
    }
}
pub trait X_7Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl X_7Impl for Ptr<X_7> {}
pub trait Y_9Impl {
    fn to_X_8(&self) -> X_8 {
        unimplemented!()
    }
}
impl Y_9Impl for Ptr<Y_9> {}
pub fn __cpp2rust_init_globals() {
    let _ = last_call_type_3.with(|_| ());
    let _ = last_call_args_4.with(|_| ());
}
