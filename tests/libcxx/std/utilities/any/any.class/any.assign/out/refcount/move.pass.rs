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
pub fn makeTypeIDImp_2() -> Ptr<TypeID> {
    return id_3.with(|v| v.as_pointer());
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
impl CppEq for TypeID {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
thread_local!(
    static id_3: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(b"const TypeID &makeTypeIDImp() [T = ArgumentListID<>]")
    })));
);
thread_local!(
    static id_5: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(
            b"const TypeID &makeTypeIDImp() [T = ArgumentListID<const small_tracked_t &>]",
        )
    })));
);
thread_local!(
    static id_7: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(
            b"const TypeID &makeTypeIDImp() [T = ArgumentListID<small_tracked_t &&>]",
        )
    })));
);
thread_local!(
    static id_9: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(
            b"const TypeID &makeTypeIDImp() [T = ArgumentListID<const large_tracked_t &>]",
        )
    })));
);
thread_local!(
    static id_11: Value<TypeID> = Rc::new(RefCell::new(TypeID::new({
        Ptr::<()>::from_string_literal(
            b"const TypeID &makeTypeIDImp() [T = ArgumentListID<large_tracked_t &&>]",
        )
    })));
);
pub fn makeArgumentID_12() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_2() });
}
pub fn makeArgumentID_13() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_4() });
}
pub fn makeArgumentID_14() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_6() });
}
pub fn makeArgumentID_15() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_8() });
}
pub fn makeArgumentID_16() -> Ptr<TypeID> {
    return ({ makeTypeIDImp_10() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IsSmallObject_small_tracked_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IsSmallObject_large_tracked_t_ {}
pub fn containsType_17(a: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn containsType_18(a: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn containsType_19(a: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn containsType_20(a: Ptr<()>) -> () {
    return unimplemented!();
}
pub fn assertEmpty_21(a: Ptr<()>) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn assertEmpty_22(a: Ptr<()>) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn assertEmpty_23(a: Ptr<()>) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn has_value_member_24(mut _a0: ()) -> () {
    return true;
}
pub fn has_value_member_25(mut _a0: ()) -> () {
    return true;
}
pub fn has_value_member_26(mut _a0: ()) -> () {
    return true;
}
pub fn has_value_member_27(mut _a0: ()) -> () {
    return true;
}
pub fn assertContains_28(a: Ptr<()>, mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn assertContains_29(a: Ptr<()>, mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn assertContains_30(a: Ptr<()>, mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn assertContains_31(a: Ptr<()>, mut value: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
thread_local!(
    pub static count_32: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_33: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_34: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static const_copied_35: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor, Default)]
#[byte_size(4)]
pub struct small_type_0_ {
    #[offset(0)]
    pub value: (),
}
impl small_type_0_ {
    pub fn new_1(mut val: ()) -> Self {
        let __this: small_type_0_ = Self { value: val };
        (*count_32.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_2(other: Ptr<small_type_0_>) -> Self {
        let __this: Value<small_type_0_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_type_0_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_32.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_33.with(Value::clone).borrow_mut()).prefix_inc();
        (*const_copied_35.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<small_type_0_>) -> Self {
        let __this: Value<small_type_0_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_type_0_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        field!(other, value).write(0_());
        (*count_32.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_34.with(Value::clone).borrow_mut()).prefix_inc();
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
    pub static count_37: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_38: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_39: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static const_copied_40: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor, Default)]
#[byte_size(4)]
pub struct small_type_1_ {
    #[offset(0)]
    pub value: (),
}
impl small_type_1_ {
    pub fn new_1(mut val: ()) -> Self {
        let __this: small_type_1_ = Self { value: val };
        (*count_37.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_2(other: Ptr<small_type_1_>) -> Self {
        let __this: Value<small_type_1_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_type_1_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_37.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_38.with(Value::clone).borrow_mut()).prefix_inc();
        (*const_copied_40.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<small_type_1_>) -> Self {
        let __this: Value<small_type_1_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_type_1_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        field!(other, value).write(0_());
        (*count_37.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_39.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for small_type_1_ {
    fn clone(&self) -> Self {
        let __src: Value<small_type_1_> = Rc::new(RefCell::new(small_type_1_ {
            value: self.value.clone(),
        }));
        small_type_1_::new_2(__src.as_pointer())
    }
}
thread_local!(
    pub static count_42: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_43: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_44: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static const_copied_45: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor, Default)]
#[byte_size(4)]
pub struct small_type_2_ {
    #[offset(0)]
    pub value: (),
}
impl small_type_2_ {
    pub fn new_1(mut val: ()) -> Self {
        let __this: small_type_2_ = Self { value: val };
        (*count_42.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_2(other: Ptr<small_type_2_>) -> Self {
        let __this: Value<small_type_2_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_type_2_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_42.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_43.with(Value::clone).borrow_mut()).prefix_inc();
        (*const_copied_45.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<small_type_2_>) -> Self {
        let __this: Value<small_type_2_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_type_2_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        field!(other, value).write(0_());
        (*count_42.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_44.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for small_type_2_ {
    fn clone(&self) -> Self {
        let __src: Value<small_type_2_> = Rc::new(RefCell::new(small_type_2_ {
            value: self.value.clone(),
        }));
        small_type_2_::new_2(__src.as_pointer())
    }
}
thread_local!(
    pub static count_47: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_48: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_49: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static const_copied_50: Value<()> = Rc::new(RefCell::new(0_()));
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
    pub fn new_1(mut val: ()) -> Self {
        let __this: Value<large_type_0_> = Rc::new(RefCell::new(Self {
            value: val,
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_type_0_> = __this.as_pointer();
        (*count_47.with(Value::clone).borrow_mut()).prefix_inc();
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
        (*count_47.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_48.with(Value::clone).borrow_mut()).prefix_inc();
        (*const_copied_50.with(Value::clone).borrow_mut()).prefix_inc();
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
        (*count_47.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_49.with(Value::clone).borrow_mut()).prefix_inc();
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
thread_local!(
    pub static count_52: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_53: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static const_copied_55: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(44)]
pub struct large_type_1_ {
    #[offset(0)]
    pub value: (),
    #[offset(4)]
    #[byte_size(40)]
    data: Box<[()]>,
}
impl large_type_1_ {
    pub fn new_1(mut val: ()) -> Self {
        let __this: Value<large_type_1_> = Rc::new(RefCell::new(Self {
            value: val,
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_type_1_> = __this.as_pointer();
        (*count_52.with(Value::clone).borrow_mut()).prefix_inc();
        array_elem!(this, data, 0_()).write(0_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(other: Ptr<large_type_1_>) -> Self {
        let __this: Value<large_type_1_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_type_1_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_52.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_53.with(Value::clone).borrow_mut()).prefix_inc();
        (*const_copied_55.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for large_type_1_ {
    fn clone(&self) -> Self {
        let __src: Value<large_type_1_> = Rc::new(RefCell::new(large_type_1_ {
            value: self.value.clone(),
            data: self.data.clone(),
        }));
        large_type_1_::new_2(__src.as_pointer())
    }
}
impl Default for large_type_1_ {
    fn default() -> Self {
        {
            large_type_1_ {
                value: unimplemented!(),
                data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
thread_local!(
    pub static count_57: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_58: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static const_copied_60: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(44)]
pub struct large_type_2_ {
    #[offset(0)]
    pub value: (),
    #[offset(4)]
    #[byte_size(40)]
    data: Box<[()]>,
}
impl large_type_2_ {
    pub fn new_1(mut val: ()) -> Self {
        let __this: Value<large_type_2_> = Rc::new(RefCell::new(Self {
            value: val,
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_type_2_> = __this.as_pointer();
        (*count_57.with(Value::clone).borrow_mut()).prefix_inc();
        array_elem!(this, data, 0_()).write(0_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(other: Ptr<large_type_2_>) -> Self {
        let __this: Value<large_type_2_> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_type_2_> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_57.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_58.with(Value::clone).borrow_mut()).prefix_inc();
        (*const_copied_60.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for large_type_2_ {
    fn clone(&self) -> Self {
        let __src: Value<large_type_2_> = Rc::new(RefCell::new(large_type_2_ {
            value: self.value.clone(),
            data: self.data.clone(),
        }));
        large_type_2_::new_2(__src.as_pointer())
    }
}
impl Default for large_type_2_ {
    fn default() -> Self {
        {
            large_type_2_ {
                value: unimplemented!(),
                data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct my_any_exception {}
pub fn throwMyAnyExpression_62() {
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
        count_63.with(|rc| {
            *rc.borrow_mut() = {
                copied_64.with(|rc| {
                    *rc.borrow_mut() = {
                        moved_65.with(|rc| *rc.borrow_mut() = 0_());
                        moved_65.with(|rc| *rc.borrow())
                    }
                });
                copied_64.with(|rc| *rc.borrow())
            }
        });
    }
    pub fn new_1(val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| 0_());
        let __this: small_throws_on_copy = Self { value: val };
        (*count_63.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let __this: small_throws_on_copy = Self { value: val };
        (*count_63.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<small_throws_on_copy>) -> Self {
        let __this: small_throws_on_copy = Self {
            value: unimplemented!(),
        };
        ({ throwMyAnyExpression_62() });
        __this
    }
    pub fn move_from(other: Ptr<small_throws_on_copy>) -> Self {
        let __this: Value<small_throws_on_copy> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<small_throws_on_copy> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_63.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_65.with(Value::clone).borrow_mut()).prefix_inc();
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
    pub static count_63: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_64: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_65: Value<()> = Rc::new(RefCell::new(0_()));
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
        count_66.with(|rc| {
            *rc.borrow_mut() = {
                copied_67.with(|rc| {
                    *rc.borrow_mut() = {
                        moved_68.with(|rc| *rc.borrow_mut() = 0_());
                        moved_68.with(|rc| *rc.borrow())
                    }
                });
                copied_67.with(|rc| *rc.borrow())
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
        (*count_66.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let __this: Value<large_throws_on_copy> = Rc::new(RefCell::new(Self {
            value: val,
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_throws_on_copy> = __this.as_pointer();
        array_elem!(this, data, 0_()).write(0_());
        (*count_66.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(_a0: Ptr<large_throws_on_copy>) -> Self {
        let __this: large_throws_on_copy = Self {
            value: 0_(),
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        };
        ({ throwMyAnyExpression_62() });
        __this
    }
    pub fn move_from(other: Ptr<large_throws_on_copy>) -> Self {
        let __this: Value<large_throws_on_copy> = Rc::new(RefCell::new(Self {
            value: 0_(),
            data: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_throws_on_copy> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_66.with(Value::clone).borrow_mut()).prefix_inc();
        (*moved_68.with(Value::clone).borrow_mut()).prefix_inc();
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
    pub static count_66: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_67: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_68: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(4)]
pub struct throws_on_move {
    #[offset(0)]
    pub value: (),
}
impl throws_on_move {
    pub fn reset() {
        count_69.with(|rc| {
            *rc.borrow_mut() = {
                copied_70.with(|rc| {
                    *rc.borrow_mut() = {
                        moved_71.with(|rc| *rc.borrow_mut() = 0_());
                        moved_71.with(|rc| *rc.borrow())
                    }
                });
                copied_70.with(|rc| *rc.borrow())
            }
        });
    }
    pub fn new_1(val: Option<()>) -> Self {
        let mut val: () = val.unwrap_or_else(|| 0_());
        let __this: throws_on_move = Self { value: val };
        (*count_69.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_2(mut _a0: (), mut val: (), mut _a2: ()) -> Self {
        let __this: throws_on_move = Self { value: val };
        (*count_69.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(other: Ptr<throws_on_move>) -> Self {
        let __this: Value<throws_on_move> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
        }));
        let this: Ptr<throws_on_move> = __this.as_pointer();
        field!(this, value).write({ other.with(|__s| __s.value) });
        (*count_69.with(Value::clone).borrow_mut()).prefix_inc();
        (*copied_70.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<throws_on_move>) -> Self {
        let __this: throws_on_move = Self {
            value: unimplemented!(),
        };
        ({ throwMyAnyExpression_62() });
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
    pub static count_69: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copied_70: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_71: Value<()> = Rc::new(RefCell::new(0_()));
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
            arg_types: ({ makeArgumentID_12() }),
        }
    }
    pub fn copy_from(_a0: Ptr<small_tracked_t>) -> Self {
        Self {
            arg_types: ({ makeArgumentID_13() }),
        }
    }
    pub fn move_from(_a0: Ptr<small_tracked_t>) -> Self {
        Self {
            arg_types: ({ makeArgumentID_14() }),
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
#[byte_size(32)]
pub struct large_tracked_t {
    #[offset(0)]
    #[byte_size(8)]
    pub arg_types: Ptr<TypeID>,
    #[offset(8)]
    #[byte_size(20)]
    pub dummy: Box<[()]>,
}
impl large_tracked_t {
    pub fn new() -> Self {
        let __this: Value<large_tracked_t> = Rc::new(RefCell::new(Self {
            arg_types: ({ makeArgumentID_12() }),
            dummy: (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<large_tracked_t> = __this.as_pointer();
        array_elem!(this, dummy, 0_()).write(42_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(_a0: Ptr<large_tracked_t>) -> Self {
        Self {
            arg_types: ({ makeArgumentID_15() }),
            dummy: (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }
    }
    pub fn move_from(_a0: Ptr<large_tracked_t>) -> Self {
        Self {
            arg_types: ({ makeArgumentID_16() }),
            dummy: (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
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
pub fn test_move_assign_72() {
    unimplemented!();
    unimplemented!();
    {
        let s1: Value<small_type_1_> = Rc::new(RefCell::new(small_type_1_::new_1({ 1_() })));
        let _dtor_s1 = ScopedDestructor::new(&s1, |__p| __p.destructor());
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s2: Value<small_type_2_> = Rc::new(RefCell::new(small_type_2_::new_1({ 2_() })));
        let _dtor_s2 = ScopedDestructor::new(&s2, |__p| __p.destructor());
        let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        ({ assertContains_28(a.as_pointer(), 2_()) });
        if unimplemented!() {
            ({ assertContains_28(a2.as_pointer(), 0_()) });
        }
        const _: () = assert!(true, "");;
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_move_assign_73() {
    unimplemented!();
    unimplemented!();
    {
        let s1: Value<large_type_1_> = Rc::new(RefCell::new(large_type_1_::new_1({ 1_() })));
        let _dtor_s1 = ScopedDestructor::new(&s1, |__p| __p.destructor());
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s2: Value<large_type_2_> = Rc::new(RefCell::new(large_type_2_::new_1({ 2_() })));
        let _dtor_s2 = ScopedDestructor::new(&s2, |__p| __p.destructor());
        let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        ({ assertContains_29(a.as_pointer(), 2_()) });
        if unimplemented!() {
            ({ assertContains_29(a2.as_pointer(), 0_()) });
        }
        const _: () = assert!(true, "");;
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_move_assign_74() {
    unimplemented!();
    unimplemented!();
    {
        let s1: Value<small_type_0_> = Rc::new(RefCell::new(small_type_0_::new_1({ 1_() })));
        let _dtor_s1 = ScopedDestructor::new(&s1, |__p| __p.destructor());
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s2: Value<large_type_0_> = Rc::new(RefCell::new(large_type_0_::new_1({ 2_() })));
        let _dtor_s2 = ScopedDestructor::new(&s2, |__p| __p.destructor());
        let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        ({ assertContains_30(a.as_pointer(), 2_()) });
        if unimplemented!() {
            ({ assertContains_30(a2.as_pointer(), 0_()) });
        }
        const _: () = assert!(true, "");;
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_move_assign_75() {
    unimplemented!();
    unimplemented!();
    {
        let s1: Value<large_type_0_> = Rc::new(RefCell::new(large_type_0_::new_1({ 1_() })));
        let _dtor_s1 = ScopedDestructor::new(&s1, |__p| __p.destructor());
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s2: Value<small_type_0_> = Rc::new(RefCell::new(small_type_0_::new_1({ 2_() })));
        let _dtor_s2 = ScopedDestructor::new(&s2, |__p| __p.destructor());
        let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        ({ assertContains_31(a.as_pointer(), 2_()) });
        if unimplemented!() {
            ({ assertContains_31(a2.as_pointer(), 0_()) });
        }
        const _: () = assert!(true, "");;
    }
    unimplemented!();
    unimplemented!();
}
pub fn test_move_assign_empty_76() {
    unimplemented!();
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        ({ assertContains_31(a.as_pointer(), 1_()) });
        if unimplemented!() {
            ({ assertContains_31(a2.as_pointer(), 0_()) });
        }
        const _: () = assert!(true, "");;
    }
    unimplemented!();
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ assertEmpty_22(a.as_pointer()) });
        ({ assertEmpty_21(a2.as_pointer()) });
    }
    unimplemented!();
}
pub fn test_move_assign_empty_77() {
    unimplemented!();
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        ({ assertContains_30(a.as_pointer(), 1_()) });
        if unimplemented!() {
            ({ assertContains_30(a2.as_pointer(), 0_()) });
        }
        const _: () = assert!(true, "");;
    }
    unimplemented!();
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ assertEmpty_23(a.as_pointer()) });
        ({ assertEmpty_21(a2.as_pointer()) });
    }
    unimplemented!();
}
pub fn test_move_assign_noexcept_78() {
    let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let a2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
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
    ({ test_move_assign_noexcept_78() });
    ({ test_move_assign_72() });
    ({ test_move_assign_73() });
    ({ test_move_assign_74() });
    ({ test_move_assign_75() });
    ({ test_move_assign_empty_76() });
    ({ test_move_assign_empty_77() });
    return 0_();
}
pub trait TypeIDImpl {
    fn name(&self) -> ();
}
impl TypeIDImpl for Ptr<TypeID> {
    fn name(&self) -> () {
        return unimplemented!();
    }
}
pub trait large_throws_on_copyImpl {
    fn destructor(&self);
}
impl large_throws_on_copyImpl for Ptr<large_throws_on_copy> {
    fn destructor(&self) {
        (*count_66.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait large_type_0_Impl {
    fn destructor(&self);
}
impl large_type_0_Impl for Ptr<large_type_0_> {
    fn destructor(&self) {
        field!((*self), value).write(0_());
        (*count_47.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait large_type_1_Impl {
    fn destructor(&self);
}
impl large_type_1_Impl for Ptr<large_type_1_> {
    fn destructor(&self) {
        field!((*self), value).write(0_());
        (*count_52.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait large_type_2_Impl {
    fn destructor(&self);
}
impl large_type_2_Impl for Ptr<large_type_2_> {
    fn destructor(&self) {
        field!((*self), value).write(0_());
        (*count_57.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait small_throws_on_copyImpl {
    fn destructor(&self);
}
impl small_throws_on_copyImpl for Ptr<small_throws_on_copy> {
    fn destructor(&self) {
        (*count_63.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait small_type_0_Impl {
    fn destructor(&self);
}
impl small_type_0_Impl for Ptr<small_type_0_> {
    fn destructor(&self) {
        field!((*self), value).write(-1_());
        (*count_32.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait small_type_1_Impl {
    fn destructor(&self);
}
impl small_type_1_Impl for Ptr<small_type_1_> {
    fn destructor(&self) {
        field!((*self), value).write(-1_());
        (*count_37.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait small_type_2_Impl {
    fn destructor(&self);
}
impl small_type_2_Impl for Ptr<small_type_2_> {
    fn destructor(&self) {
        field!((*self), value).write(-1_());
        (*count_42.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait throws_on_moveImpl {
    fn destructor(&self);
}
impl throws_on_moveImpl for Ptr<throws_on_move> {
    fn destructor(&self) {
        (*count_69.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_32.with(|_| ());
    let _ = copied_33.with(|_| ());
    let _ = moved_34.with(|_| ());
    let _ = const_copied_35.with(|_| ());
    let _ = count_37.with(|_| ());
    let _ = copied_38.with(|_| ());
    let _ = moved_39.with(|_| ());
    let _ = const_copied_40.with(|_| ());
    let _ = count_42.with(|_| ());
    let _ = copied_43.with(|_| ());
    let _ = moved_44.with(|_| ());
    let _ = const_copied_45.with(|_| ());
    let _ = count_47.with(|_| ());
    let _ = copied_48.with(|_| ());
    let _ = moved_49.with(|_| ());
    let _ = const_copied_50.with(|_| ());
    let _ = count_52.with(|_| ());
    let _ = copied_53.with(|_| ());
    let _ = const_copied_55.with(|_| ());
    let _ = count_57.with(|_| ());
    let _ = copied_58.with(|_| ());
    let _ = const_copied_60.with(|_| ());
    let _ = count_63.with(|_| ());
    let _ = copied_64.with(|_| ());
    let _ = moved_65.with(|_| ());
    let _ = count_66.with(|_| ());
    let _ = copied_67.with(|_| ());
    let _ = moved_68.with(|_| ());
    let _ = count_69.with(|_| ());
    let _ = copied_70.with(|_| ());
    let _ = moved_71.with(|_| ());
}
