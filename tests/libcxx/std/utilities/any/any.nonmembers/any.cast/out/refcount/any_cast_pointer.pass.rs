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
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub fn operator_eq_2(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_3(LHS: Ptr<TypeID>, RHS: Ptr<TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } != { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn makeTypeIDImp_4() -> Ptr<TypeID> {
    return id_5.with(|v| v.as_pointer());
}
pub fn makeTypeIDImp_6() -> Ptr<TypeID> {
    return id_7.with(|v| v.as_pointer());
}
pub fn makeTypeIDImp_8() -> Ptr<TypeID> {
    return id_9.with(|v| v.as_pointer());
}
pub fn makeTypeIDImp_10() -> Ptr<TypeID> {
    return id_11.with(|v| v.as_pointer());
}
pub fn makeTypeIDImp_12() -> Ptr<TypeID> {
    return id_13.with(|v| v.as_pointer());
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
            operator_eq_2(
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
thread_local!(
    static id_5: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<>]")
    })));
);
thread_local!(
    static id_7: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(
            b"const TypeID &makeTypeIDImp() [T = ArgumentListID<const small_tracked_t &>]",
        )
    })));
);
thread_local!(
    static id_9: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(
            b"const TypeID &makeTypeIDImp() [T = ArgumentListID<small_tracked_t &&>]",
        )
    })));
);
thread_local!(
    static id_11: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(
            b"const TypeID &makeTypeIDImp() [T = ArgumentListID<const large_tracked_t &>]",
        )
    })));
);
thread_local!(
    static id_13: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(
            b"const TypeID &makeTypeIDImp() [T = ArgumentListID<large_tracked_t &&>]",
        )
    })));
);
pub fn makeArgumentID_14() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_4() });
}
pub fn makeArgumentID_15() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_6() });
}
pub fn makeArgumentID_16() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_8() });
}
pub fn makeArgumentID_17() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_10() });
}
pub fn makeArgumentID_18() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_12() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IsSmallObject_small_tracked_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IsSmallObject_large_tracked_t_ {}
thread_local!(
    pub static count_19: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_20: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_21: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static const_copied_22: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static non_const_copied_23: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor, Default)]
#[byte_size(4)]
pub struct small_type_0_ {
    #[offset(0)]
    pub value: (),
}
impl small_type_0_ {
    pub fn reset() {
        copied_20.with(|rc| *rc.borrow_mut() = 0_());
        moved_21.with(|rc| *rc.borrow_mut() = 0_());
        const_copied_22.with(|rc| *rc.borrow_mut() = 0_());
        non_const_copied_23.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn new_1(mut val: ()) -> Self {
        let __this: small_type_0_ = Self { value: val };
        (*count_19.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_2(other: Ptr<small_type_0_>) -> Self {
        let __this: Value<small_type_0_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_type_0_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_19.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_20.with(Value::clone).borrow_mut()).prefix_inc();
        (*const_copied_22.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<small_type_0_>) -> Self {
        let __this: Value<small_type_0_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_type_0_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        field!(other, value).write(0_());
        (*count_19.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_21.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for small_type_0_ {
    fn clone(&self) -> Self {
        let __src: Value<small_type_0_> = Rc::new(RefCell::new(small_type_0_ {
            value: self.value.clone(),
        }));
        small_type_0_::new_2(__src.as_pointer())
    }
}
thread_local!(
    pub static count_24: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_25: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_26: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static const_copied_27: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static non_const_copied_28: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(44)]
pub struct large_type_0_ {
    #[offset(0)]
    pub value: (),
    #[offset(4)]
    #[byte_size(40)]
    data: Box<[()]>,
}
impl large_type_0_ {
    pub fn reset() {
        copied_25.with(|rc| *rc.borrow_mut() = 0_());
        moved_26.with(|rc| *rc.borrow_mut() = 0_());
        const_copied_27.with(|rc| *rc.borrow_mut() = 0_());
        non_const_copied_28.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn new_1(mut val: ()) -> Self {
        let __this: Value<large_type_0_> = Rc::new(RefCell::new(Self {
            value: val,
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_type_0_> = __this.as_pointer();
        (*count_24.with(Value::clone).borrow_mut()).prefix_inc();
        array_elem!(this, data, 0_()).write(0_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(other: Ptr<large_type_0_>) -> Self {
        let __this: Value<large_type_0_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_type_0_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_24.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_25.with(Value::clone).borrow_mut()).prefix_inc();
        (*const_copied_27.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<large_type_0_>) -> Self {
        let __this: Value<large_type_0_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_type_0_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        field!(other, value).write(0_());
        (*count_24.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_26.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for large_type_0_ {
    fn clone(&self) -> Self {
        let __src: Value<large_type_0_> = Rc::new(RefCell::new(large_type_0_ {
            value: self.value.clone(),
            data: self.data.clone(),
        }));
        large_type_0_::new_2(__src.as_pointer())
    }
}
impl Default for large_type_0_ {
    fn default() -> Self {
        {
            large_type_0_ {
                value: unimplemented!(),
                data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct my_any_exception {}
pub fn throwMyAnyExpression_29() {
    unimplemented!();
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(4)]
pub struct small_throws_on_copy {
    #[offset(0)]
    pub value: (),
}
impl small_throws_on_copy {
    pub fn reset() {
        count_30.with(|rc| {
            *rc.borrow_mut() = {
                copied_31.with(|rc| {
                    *rc.borrow_mut() = {
                        moved_32.with(|rc| *rc.borrow_mut() = 0_());
                        moved_32.with(|rc| *rc.borrow())
                    }
                });
                copied_31.with(|rc| *rc.borrow())
            }
        });
    }
    pub fn new_1(val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| 0_());
        let __this: small_throws_on_copy = Self { value: val };
        (*count_30.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let __this: small_throws_on_copy = Self { value: val };
        (*count_30.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<small_throws_on_copy>) -> Self {
        let __this: small_throws_on_copy = Self {
            value: unimplemented!(),
        };
        ({ throwMyAnyExpression_29() });
        __this
    }
    pub fn move_from(other: Ptr<small_throws_on_copy>) -> Self {
        let __this: Value<small_throws_on_copy> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_throws_on_copy> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_30.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_32.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for small_throws_on_copy {
    fn clone(&self) -> Self {
        let __src: Value<small_throws_on_copy> = Rc::new(RefCell::new(small_throws_on_copy {
            value: self.value.clone(),
        }));
        small_throws_on_copy::copy_from(__src.as_pointer())
    }
}
impl Default for small_throws_on_copy {
    fn default() -> Self {
        { small_throws_on_copy::new_1(None) }
    }
}
thread_local!(
    pub static count_30: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_31: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_32: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(44)]
pub struct large_throws_on_copy {
    #[offset(0)]
    pub value: (),
    #[offset(4)]
    #[byte_size(40)]
    data: Box<[()]>,
}
impl large_throws_on_copy {
    pub fn reset() {
        count_33.with(|rc| {
            *rc.borrow_mut() = {
                copied_34.with(|rc| {
                    *rc.borrow_mut() = {
                        moved_35.with(|rc| *rc.borrow_mut() = 0_());
                        moved_35.with(|rc| *rc.borrow())
                    }
                });
                copied_34.with(|rc| *rc.borrow())
            }
        });
    }
    pub fn new_1(val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| 0_());
        let __this: Value<large_throws_on_copy> = Rc::new(RefCell::new(Self {
            value: val,
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_throws_on_copy> = __this.as_pointer();
        array_elem!(this, data, 0_()).write(0_());
        (*count_33.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let __this: Value<large_throws_on_copy> = Rc::new(RefCell::new(Self {
            value: val,
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_throws_on_copy> = __this.as_pointer();
        array_elem!(this, data, 0_()).write(0_());
        (*count_33.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(_a0: Ptr<large_throws_on_copy>) -> Self {
        let __this: large_throws_on_copy = Self {
            value: 0_(),
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        };
        ({ throwMyAnyExpression_29() });
        __this
    }
    pub fn move_from(other: Ptr<large_throws_on_copy>) -> Self {
        let __this: Value<large_throws_on_copy> = Rc::new(RefCell::new(Self {
            value: 0_(),
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_throws_on_copy> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_33.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_35.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for large_throws_on_copy {
    fn clone(&self) -> Self {
        let __src: Value<large_throws_on_copy> = Rc::new(RefCell::new(large_throws_on_copy {
            value: self.value.clone(),
            data: self.data.clone(),
        }));
        large_throws_on_copy::copy_from(__src.as_pointer())
    }
}
impl Default for large_throws_on_copy {
    fn default() -> Self {
        { large_throws_on_copy::new_1(None) }
    }
}
thread_local!(
    pub static count_33: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_34: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_35: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(4)]
pub struct throws_on_move {
    #[offset(0)]
    pub value: (),
}
impl throws_on_move {
    pub fn reset() {
        count_36.with(|rc| {
            *rc.borrow_mut() = {
                copied_37.with(|rc| {
                    *rc.borrow_mut() = {
                        moved_38.with(|rc| *rc.borrow_mut() = 0_());
                        moved_38.with(|rc| *rc.borrow())
                    }
                });
                copied_37.with(|rc| *rc.borrow())
            }
        });
    }
    pub fn new_1(val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| 0_());
        let __this: throws_on_move = Self { value: val };
        (*count_36.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let __this: throws_on_move = Self { value: val };
        (*count_36.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(other: Ptr<throws_on_move>) -> Self {
        let __this: Value<throws_on_move> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<throws_on_move> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_36.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_37.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<throws_on_move>) -> Self {
        let __this: throws_on_move = Self {
            value: unimplemented!(),
        };
        ({ throwMyAnyExpression_29() });
        __this
    }
}
impl Clone for throws_on_move {
    fn clone(&self) -> Self {
        let __src: Value<throws_on_move> = Rc::new(RefCell::new(throws_on_move {
            value: self.value.clone(),
        }));
        throws_on_move::copy_from(__src.as_pointer())
    }
}
impl Default for throws_on_move {
    fn default() -> Self {
        { throws_on_move::new_1(None) }
    }
}
thread_local!(
    pub static count_36: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_37: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_38: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(8)]
pub struct small_tracked_t {
    #[offset(0)]
    #[byte_size(8)]
    pub arg_types: Ptr<TypeID>,
}
impl small_tracked_t {
    pub fn new() -> Self {
        Self {
            arg_types: ({ makeArgumentID_14() }),
        }
    }
    pub fn copy_from(_a0: Ptr<small_tracked_t>) -> Self {
        Self {
            arg_types: ({ makeArgumentID_15() }),
        }
    }
    pub fn move_from(_a0: Ptr<small_tracked_t>) -> Self {
        Self {
            arg_types: ({ makeArgumentID_16() }),
        }
    }
}
impl Clone for small_tracked_t {
    fn clone(&self) -> Self {
        let __src: Value<small_tracked_t> = Rc::new(RefCell::new(small_tracked_t {
            arg_types: self.arg_types.clone(),
        }));
        small_tracked_t::copy_from(__src.as_pointer())
    }
}
impl Default for small_tracked_t {
    fn default() -> Self {
        { small_tracked_t::new() }
    }
}
const _: () = assert!(true, "IsSmallObject<small_tracked_t>::value");
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(48)]
pub struct large_tracked_t {
    #[offset(0)]
    #[byte_size(8)]
    pub arg_types: Ptr<TypeID>,
    #[offset(8)]
    #[byte_size(36)]
    pub dummy: Box<[()]>,
}
impl large_tracked_t {
    pub fn new() -> Self {
        let __this: Value<large_tracked_t> = Rc::new(RefCell::new(Self {
            arg_types: ({ makeArgumentID_14() }),
            dummy: (0..9).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_tracked_t> = __this.as_pointer();
        array_elem!(this, dummy, 0_()).write(42_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(_a0: Ptr<large_tracked_t>) -> Self {
        Self {
            arg_types: ({ makeArgumentID_17() }),
            dummy: (0..9).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }
    }
    pub fn move_from(_a0: Ptr<large_tracked_t>) -> Self {
        Self {
            arg_types: ({ makeArgumentID_18() }),
            dummy: (0..9).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }
    }
}
impl Clone for large_tracked_t {
    fn clone(&self) -> Self {
        let __src: Value<large_tracked_t> = Rc::new(RefCell::new(large_tracked_t {
            arg_types: self.arg_types.clone(),
            dummy: self.dummy.clone(),
        }));
        large_tracked_t::copy_from(__src.as_pointer())
    }
}
impl Default for large_tracked_t {
    fn default() -> Self {
        { large_tracked_t::new() }
    }
}
const _: () = assert!(true, "!IsSmallObject<large_tracked_t>::value");
pub fn test_cast_is_noexcept_39() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    let ca: Ptr<()> = a.as_pointer();
    const _: () = assert!(true, "");;
}
pub fn test_cast_return_type_40() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let ca: Ptr<()> = a.as_pointer();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_cast_nullptr_41() {
    let mut a: Ptr<()> = Ptr::<()>::null();
    unimplemented!();
    unimplemented!();
    let mut ca: Ptr<()> = Ptr::<()>::null();
    unimplemented!();
    unimplemented!();
}
pub fn test_cast_empty_42() {
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let ca: Ptr<()> = a.as_pointer();
        unimplemented!();
        unimplemented!();
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(42_()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let ca: Ptr<()> = a.as_pointer();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_cast_43() {
    unimplemented!();
    ({ small_type_0_::reset() });
    {
        let a: Value<()> = Rc::new(RefCell::new(small_type_0_::new_1({ 42_() })));
        let ca: Ptr<()> = a.as_pointer();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut v: Ptr<small_type_0_> = ({ any_cast_44((a.as_pointer())) });
        unimplemented!();
        unimplemented!();
        field!(v, value).write(999_());
        let mut cv: Ptr<small_type_0_> = ({ any_cast_45((a.as_pointer())) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        cv = ({ any_cast_46((ca).clone()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        cv = ({ any_cast_47((ca).clone()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_cast_48() {
    unimplemented!();
    ({ large_type_0_::reset() });
    {
        let a: Value<()> = Rc::new(RefCell::new(large_type_0_::new_1({ 42_() })));
        let ca: Ptr<()> = a.as_pointer();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut v: Ptr<large_type_0_> = ({ any_cast_49((a.as_pointer())) });
        unimplemented!();
        unimplemented!();
        field!(v, value).write(999_());
        let mut cv: Ptr<large_type_0_> = ({ any_cast_50((a.as_pointer())) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        cv = ({ any_cast_51((ca).clone()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        cv = ({ any_cast_52((ca).clone()) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_cast_non_copyable_type_53() {
    let a: Value<()> = Rc::new(RefCell::new(42_()));
    let ca: Ptr<()> = a.as_pointer();
    unimplemented!();
    unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoCopy_54 {}
pub fn test_cast_array_55() {
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let a: Value<()> = Rc::new(RefCell::new(std_any::new_1({ arr.as_pointer() })));
    unimplemented!();
    let mut p: Ptr<Box<[()]>> = ({ any_cast_56((a.as_pointer())) });
    unimplemented!();
}
pub fn test_fn_57() {}
pub fn test_cast_function_pointer_58() {
    let a: Value<()> = Rc::new(RefCell::new(std_any::new_2({
        FnPtr::<fn()>::new(test_fn_57)
    })));
    unimplemented!();
    let mut fn_ptr: FnPtr<fn()> = unimplemented!();
    unimplemented!();
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
    ({ test_cast_is_noexcept_39() });
    ({ test_cast_return_type_40() });
    ({ test_cast_nullptr_41() });
    ({ test_cast_empty_42() });
    ({ test_cast_43() });
    ({ test_cast_48() });
    ({ test_cast_non_copyable_type_53() });
    ({ test_cast_array_55() });
    ({ test_cast_function_pointer_58() });
    return 0_();
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
pub trait large_throws_on_copyImpl {
    fn destructor(&self);
}
impl large_throws_on_copyImpl for Ptr<large_throws_on_copy> {
    fn destructor(&self) {
        (*count_33.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait large_type_0_Impl {
    fn destructor(&self);
}
impl large_type_0_Impl for Ptr<large_type_0_> {
    fn destructor(&self) {
        field!((*self), value).write(0_());
        (*count_24.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait small_throws_on_copyImpl {
    fn destructor(&self);
}
impl small_throws_on_copyImpl for Ptr<small_throws_on_copy> {
    fn destructor(&self) {
        (*count_30.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait small_type_0_Impl {
    fn destructor(&self);
}
impl small_type_0_Impl for Ptr<small_type_0_> {
    fn destructor(&self) {
        field!((*self), value).write(-1_());
        (*count_19.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait throws_on_moveImpl {
    fn destructor(&self);
}
impl throws_on_moveImpl for Ptr<throws_on_move> {
    fn destructor(&self) {
        (*count_36.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_19.with(|_| ());
    let _ = copied_20.with(|_| ());
    let _ = moved_21.with(|_| ());
    let _ = const_copied_22.with(|_| ());
    let _ = non_const_copied_23.with(|_| ());
    let _ = count_24.with(|_| ());
    let _ = copied_25.with(|_| ());
    let _ = moved_26.with(|_| ());
    let _ = const_copied_27.with(|_| ());
    let _ = non_const_copied_28.with(|_| ());
    let _ = count_30.with(|_| ());
    let _ = copied_31.with(|_| ());
    let _ = moved_32.with(|_| ());
    let _ = count_33.with(|_| ());
    let _ = copied_34.with(|_| ());
    let _ = moved_35.with(|_| ());
    let _ = count_36.with(|_| ());
    let _ = copied_37.with(|_| ());
    let _ = moved_38.with(|_| ());
}
