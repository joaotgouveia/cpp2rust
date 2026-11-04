extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Celsius {
    #[offset(0)]
    pub degrees: i32,
}
impl Celsius {
    pub fn new(mut d: i32) -> Self {
        Self { degrees: d }
    }
}
impl CppMove for Celsius {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(4)]
pub struct Meters {
    #[offset(0)]
    pub value: i32,
}
impl CppMove for Meters {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new, ptr)]
#[byte_size(4)]
pub struct Label {
    #[offset(0)]
    pub id: i32,
}
impl Label {
    pub fn new(c: Ptr<Celsius>) -> Self {
        Self {
            id: c.with(|__s| __s.degrees),
        }
    }
}
impl CppMove for Label {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new_1, ptr)]
#[cpp_from_move(Celsius, new_2, ptr)]
#[byte_size(8)]
pub struct Owner {
    #[offset(0)]
    pub taken: i32,
    #[offset(4)]
    pub copied: i32,
}
impl Owner {
    pub fn new_1(c: Ptr<Celsius>) -> Self {
        Self {
            taken: 0,
            copied: c.with(|__s| __s.degrees),
        }
    }
    pub fn new_2(c: Ptr<Celsius>) -> Self {
        Self {
            taken: c.with(|__s| __s.degrees),
            copied: 0,
        }
    }
}
impl CppMove for Owner {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new, value)]
#[byte_size(4)]
pub struct Holder {
    #[offset(0)]
    pub value: i32,
}
impl Holder {
    pub fn new(mut c: Celsius) -> Self {
        Self { value: c.degrees }
    }
}
impl CppMove for Holder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct Fallback {
    #[offset(0)]
    pub value: i32,
}
impl Fallback {
    pub fn new(v: Option<i32>) -> Self {
        let mut v: i32 = v.unwrap_or_else(|| -1_i32);
        Self { value: v }
    }
}
impl CppMove for Fallback {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Fallback {
    fn default() -> Self {
        { Fallback::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Counter {
    #[offset(0)]
    pub hits: i32,
}
impl CppMove for Counter {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Counter, new, mut_ptr)]
#[byte_size(4)]
pub struct Observer {
    #[offset(0)]
    pub seen: i32,
}
impl Observer {
    pub fn new(c: Ptr<Counter>) -> Self {
        let __this: Observer = Self {
            seen: c.with(|__s| __s.hits),
        };
        field!(c, hits).with_mut(|__v| __v.postfix_inc());
        __this
    }
}
impl CppMove for Observer {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Tag {
    pub fn new(u: Ptr<Celsius>) -> Self {
        Self {
            code: (u.with(|__s| __s.degrees) + 100),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new, ptr)]
#[byte_size(4)]
pub struct Tag {
    #[offset(0)]
    pub code: i32,
}
impl CppMove for Tag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(4)]
pub struct Name {
    #[offset(0)]
    pub len: i32,
}
impl Name {
    pub fn new(mut s: Ptr<i8>) -> Self {
        let __this: Value<Name> = Rc::new(RefCell::new(Self { len: 0 }));
        let this: Ptr<Name> = __this.as_pointer();
        'loop_: while ((elem!(s, this.with(|__s| __s.len)).read()) != 0) {
            field!(this, len).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppMove for Name {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let widened: Value<std_pair_long__unsigned_int_> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(-1_i32));
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(2_u32));
        std_pair_long__unsigned_int_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    ({ cpp2rust_assert_fail_0(({ (*widened.borrow()).first } == -1_i64)) });
    ({ cpp2rust_assert_fail_0(({ (*widened.borrow()).second } == 2_u32)) });
    let from_ints: Value<std_pair_Celsius__Celsius_> = Rc::new(RefCell::new({
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
        let __tmp_3: Value<i32> = Rc::new(RefCell::new(2));
        std_pair_Celsius__Celsius_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
    }));
    ({ cpp2rust_assert_fail_0(({ (*from_ints.borrow()).first.degrees } == 1)) });
    ({ cpp2rust_assert_fail_0(({ (*from_ints.borrow()).second.degrees } == 2)) });
    let m: Value<Meters> = Rc::new(RefCell::new(Meters { value: 7 }));
    let from_operators: Value<std_pair_int__long_> = Rc::new(RefCell::new(
        std_pair_int__long_::new_1({ m.as_pointer() }, { m.as_pointer() }),
    ));
    ({ cpp2rust_assert_fail_0(({ (*from_operators.borrow()).first } == 7)) });
    ({ cpp2rust_assert_fail_0(({ (*from_operators.borrow()).second } == 7_i64)) });
    let from_moved_pair: Value<std_pair_long__double_> = Rc::new(RefCell::new(
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(3));
            let ___y: Value<i32> = Rc::new(RefCell::new(4));
            make_pair_1(___x.as_pointer(), ___y.as_pointer())
        }),
    ));
    ({ cpp2rust_assert_fail_0(({ (*from_moved_pair.borrow()).first } == 3_i64)) });
    ({ cpp2rust_assert_fail_0(({ (*from_moved_pair.borrow()).second } == 4_f64)) });
    let ints: Value<std_pair_int__int_> = Rc::new(RefCell::new({
        let __tmp_4: Value<i32> = Rc::new(RefCell::new(5));
        let __tmp_5: Value<i32> = Rc::new(RefCell::new(6));
        std_pair_int__int_::new_1({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
    }));
    let from_copied_pair: Value<std_pair_Celsius__long_> =
        Rc::new(RefCell::new(std_pair_Celsius__long_::new_1({
            ints.as_pointer()
        })));
    ({ cpp2rust_assert_fail_0(({ (*from_copied_pair.borrow()).first.degrees } == 5)) });
    ({ cpp2rust_assert_fail_0(({ (*from_copied_pair.borrow()).second } == 6_i64)) });
    let meters: Value<std_pair_Meters__Meters_> = Rc::new(RefCell::new({
        let __tmp_6: Value<Meters> = Rc::new(RefCell::new(Meters { value: 8 }));
        let __tmp_7: Value<Meters> = Rc::new(RefCell::new(Meters { value: 9 }));
        std_pair_Meters__Meters_::new_1({ __tmp_6.as_pointer() }, { __tmp_7.as_pointer() })
    }));
    let from_operator_pair: Value<std_pair_int__int_> =
        Rc::new(RefCell::new(std_pair_int__int_::new_2({
            meters.as_pointer()
        })));
    ({ cpp2rust_assert_fail_0(({ (*from_operator_pair.borrow()).first } == 8)) });
    ({ cpp2rust_assert_fail_0(({ (*from_operator_pair.borrow()).second } == 9)) });
    let celsius: Value<std_pair_Celsius__Celsius_> = Rc::new(RefCell::new({
        let __tmp_8: Value<i32> = Rc::new(RefCell::new(10));
        let __tmp_9: Value<i32> = Rc::new(RefCell::new(11));
        std_pair_Celsius__Celsius_::new_1({ __tmp_8.as_pointer() }, { __tmp_9.as_pointer() })
    }));
    let from_record_pair: Value<std_pair_Label__Owner_> =
        Rc::new(RefCell::new(std_pair_Label__Owner_::new_1({
            celsius.as_pointer()
        })));
    ({ cpp2rust_assert_fail_0(({ (*from_record_pair.borrow()).first.id } == 10)) });
    ({ cpp2rust_assert_fail_0(({ (*from_record_pair.borrow()).second.copied } == 11)) });
    ({ cpp2rust_assert_fail_0(({ (*from_record_pair.borrow()).second.taken } == 0)) });
    let from_moved_record_pair: Value<std_pair_Label__Owner_> = Rc::new(RefCell::new(
        ({
            let ___x: Value<Celsius> = Rc::new(RefCell::new(Celsius::new({ 12 })));
            let ___y: Value<Celsius> = Rc::new(RefCell::new(Celsius::new({ 13 })));
            make_pair_2(___x.as_pointer(), ___y.as_pointer())
        }),
    ));
    ({ cpp2rust_assert_fail_0(({ (*from_moved_record_pair.borrow()).first.id } == 12)) });
    ({ cpp2rust_assert_fail_0(({ (*from_moved_record_pair.borrow()).second.taken } == 13)) });
    ({ cpp2rust_assert_fail_0(({ (*from_moved_record_pair.borrow()).second.copied } == 0)) });
    let from_by_value_pair: Value<std_pair_Holder__Holder_> =
        Rc::new(RefCell::new(std_pair_Holder__Holder_::new_1({
            celsius.as_pointer()
        })));
    ({ cpp2rust_assert_fail_0(({ (*from_by_value_pair.borrow()).first.value } == 10)) });
    ({ cpp2rust_assert_fail_0(({ (*from_by_value_pair.borrow()).second.value } == 11)) });
    let from_moved_by_value_pair: Value<std_pair_Holder__Holder_> = Rc::new(RefCell::new(
        ({
            let ___x: Value<Celsius> = Rc::new(RefCell::new(Celsius::new({ 14 })));
            let ___y: Value<Celsius> = Rc::new(RefCell::new(Celsius::new({ 15 })));
            make_pair_2(___x.as_pointer(), ___y.as_pointer())
        }),
    ));
    ({ cpp2rust_assert_fail_0(({ (*from_moved_by_value_pair.borrow()).first.value } == 14)) });
    ({ cpp2rust_assert_fail_0(({ (*from_moved_by_value_pair.borrow()).second.value } == 15)) });
    let mut defaulted: Fallback = Fallback::new(None);
    ({ cpp2rust_assert_fail_0((defaulted.value == -1_i32)) });
    let from_defaulted_ctor: Value<std_pair_Fallback__long_> =
        Rc::new(RefCell::new(std_pair_Fallback__long_::new_1({
            ints.as_pointer()
        })));
    ({ cpp2rust_assert_fail_0(({ (*from_defaulted_ctor.borrow()).first.value } == 5)) });
    ({ cpp2rust_assert_fail_0(({ (*from_defaulted_ctor.borrow()).second } == 6_i64)) });
    let counter: Value<Counter> = Rc::new(RefCell::new(Counter { hits: 3 }));
    let forwarded_mutable_ref: Value<std_pair_Observer__int_> = Rc::new(RefCell::new({
        let __tmp_10: Value<i32> = Rc::new(RefCell::new(0));
        std_pair_Observer__int_::new_1({ counter.as_pointer() }, { __tmp_10.as_pointer() })
    }));
    ({ cpp2rust_assert_fail_0(({ (*forwarded_mutable_ref.borrow()).first.seen } == 3)) });
    ({ cpp2rust_assert_fail_0(({ (*counter.borrow()).hits } == 4)) });
    let from_template_ctor: Value<std_pair_Tag__Tag_> =
        Rc::new(RefCell::new(std_pair_Tag__Tag_::new_1({
            celsius.as_pointer()
        })));
    ({ cpp2rust_assert_fail_0(({ (*from_template_ctor.borrow()).first.code } == 110)) });
    ({ cpp2rust_assert_fail_0(({ (*from_template_ctor.borrow()).second.code } == 111)) });
    let lvalue_source: Value<Celsius> = Rc::new(RefCell::new(Celsius::new({ 16 })));
    let moved_source: Value<Celsius> = Rc::new(RefCell::new(Celsius::new({ 17 })));
    let forwarded_records: Value<std_pair_Label__Owner_> = Rc::new(RefCell::new(
        std_pair_Label__Owner_::new_2({ lvalue_source.as_pointer() }, {
            moved_source.as_pointer()
        }),
    ));
    ({ cpp2rust_assert_fail_0(({ (*forwarded_records.borrow()).first.id } == 16)) });
    ({ cpp2rust_assert_fail_0(({ (*forwarded_records.borrow()).second.taken } == 17)) });
    ({ cpp2rust_assert_fail_0(({ (*forwarded_records.borrow()).second.copied } == 0)) });
    let from_string_literals: Value<std_pair_Name__Name_> = Rc::new(RefCell::new(
        std_pair_Name__Name_::new_1({ Ptr::<i8>::from_string_literal(b"ab") }, {
            Ptr::<i8>::from_string_literal(b"cde")
        }),
    ));
    ({ cpp2rust_assert_fail_0(({ (*from_string_literals.borrow()).first.len } == 2)) });
    ({ cpp2rust_assert_fail_0(({ (*from_string_literals.borrow()).second.len } == 3)) });
    return 0;
}
pub trait MetersImpl {
    fn to_i32(&self) -> i32;
}
impl MetersImpl for Ptr<Meters> {
    fn to_i32(&self) -> i32 {
        return (*self).with(|__s| __s.value);
    }
}
pub fn __cpp2rust_init_globals() {}
