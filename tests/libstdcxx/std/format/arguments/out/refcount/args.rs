extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(128)]
pub struct __gnu_test_func_callback {
    #[offset(0)]
    _M_size: (),
    #[offset(8)]
    #[byte_size(120)]
    _M_tests: Box<[FnPtr<fn()>]>,
}
impl __gnu_test_func_callback {
    pub fn new() -> Self {
        Self {
            _M_size: 0_(),
            _M_tests: (0..15)
                .map(|_| FnPtr::<fn()>::null())
                .collect::<Box<[FnPtr<fn()>]>>(),
        }
    }
}
impl Default for __gnu_test_func_callback {
    fn default() -> Self {
        { __gnu_test_func_callback::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct __gnu_test_object_counter {}
impl __gnu_test_object_counter {
    pub fn new() -> Self {
        let __this: __gnu_test_object_counter = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<__gnu_test_object_counter>) -> Self {
        let __this: __gnu_test_object_counter = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for __gnu_test_object_counter {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_object_counter> =
            Rc::new(RefCell::new(__gnu_test_object_counter {}));
        __gnu_test_object_counter::copy_from(__src.as_pointer())
    }
}
impl Default for __gnu_test_object_counter {
    fn default() -> Self {
        { __gnu_test_object_counter::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_copy_constructor {}
impl __gnu_test_copy_constructor {
    pub fn count() -> () {
        return count__1.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__1.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__1.with(|rc| *rc.borrow()) == throw_on__2.with(|rc| *rc.borrow())) {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"copy_constructor::mark_call",
                ))
            });
        }
    }
    pub fn reset() {
        count__1.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__2.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__2.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub fn count() -> () {
        return count__4.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__4.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__4.with(|rc| *rc.borrow()) == throw_on__5.with(|rc| *rc.borrow())) {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"assignment_operator::mark_call",
                ))
            });
        }
    }
    pub fn reset() {
        count__4.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__5.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__5.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub fn count() -> () {
        return _M_count_6.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*_M_count_6.with(Value::clone).borrow_mut()).postfix_inc();
    }
    pub fn reset() {
        _M_count_6.with(|rc| *rc.borrow_mut() = 0_());
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct __gnu_test_copy_tracker {
    #[offset(0)]
    id_: (),
    #[offset(4)]
    throw_on_copy_: (),
}
impl __gnu_test_copy_tracker {
    pub fn new(id: Option<()>, throw_on_copy: Option<()>) -> Self {
        let mut id: () =
            id.unwrap_or_else(|| (*next_id__7.with(Value::clone).borrow_mut()).postfix_dec());
        let mut throw_on_copy: () = throw_on_copy.unwrap_or_else(|| false);
        Self {
            id_: id,
            throw_on_copy_: throw_on_copy,
        }
    }
    pub fn copy_from(rhs: Ptr<__gnu_test_copy_tracker>) -> Self {
        let __this: Value<__gnu_test_copy_tracker> = Rc::new(RefCell::new(Self {
            id_: ({ __gnu_test_copy_trackerImpl::id(&rhs) }),
            throw_on_copy_: rhs.with(|__s| __s.throw_on_copy_),
        }));
        let this: Ptr<__gnu_test_copy_tracker> = __this.as_pointer();
        if this.with(|__s| __s.throw_on_copy_) {
            ({
                __gnu_test_copy_constructor::throw_on(
                    ({ __gnu_test_copy_constructor::count() }).wrapping_add(1_()),
                )
            });
        }
        ({ __gnu_test_copy_constructor::mark_call() });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn reset() {
        ({ __gnu_test_copy_constructor::reset() });
        ({ __gnu_test_assignment_operator::reset() });
        ({ __gnu_test_destructor::reset() });
    }
}
impl std::cmp::Ord for __gnu_test_copy_tracker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_8(
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: self.id_.clone(),
                    throw_on_copy_: self.throw_on_copy_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: other.id_.clone(),
                    throw_on_copy_: other.throw_on_copy_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_8(
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: other.id_.clone(),
                    throw_on_copy_: other.throw_on_copy_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: self.id_.clone(),
                    throw_on_copy_: self.throw_on_copy_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for __gnu_test_copy_tracker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for __gnu_test_copy_tracker {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_9(
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: self.id_.clone(),
                    throw_on_copy_: self.throw_on_copy_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: other.id_.clone(),
                    throw_on_copy_: other.throw_on_copy_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_copy_tracker {}
impl Clone for __gnu_test_copy_tracker {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_copy_tracker> =
            Rc::new(RefCell::new(__gnu_test_copy_tracker {
                id_: self.id_.clone(),
                throw_on_copy_: self.throw_on_copy_.clone(),
            }));
        __gnu_test_copy_tracker::copy_from(__src.as_pointer())
    }
}
impl Default for __gnu_test_copy_tracker {
    fn default() -> Self {
        { __gnu_test_copy_tracker::new(None, None) }
    }
}
pub fn operator_eq_9(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } == {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
pub fn operator_lt_8(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } < {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_semaphore {
    #[offset(0)]
    sem_set_: (),
    #[offset(4)]
    pid_: ::libc::pid_t,
}
impl __gnu_test_semaphore {}
const _: () = assert!(true, "! std::is_default_constructible_v<std::format_args>");
const _: () = assert!(true, "! std::is_default_constructible_v<std::wformat_args>");
pub fn equals_10(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_11(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_12(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_13(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_14(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_15(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_16(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_17(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_18(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_19(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_20(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_21(fmt_arg: (), expected: Ptr<()>) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<Ptr<()>> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_22(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_23(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn equals_24(fmt_arg: (), expected: ()) -> () {
    let fmt_arg: Value<()> = Rc::new(RefCell::new(fmt_arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    return unimplemented!();
}
pub fn test_empty_25() {
    let args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (26_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_empty()")).into(),
                        (Ptr::<()>::from_string_literal(b"!args.get(0)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (27_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_empty()")).into(),
                        (Ptr::<()>::from_string_literal(b"!args.get(1)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (28_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_empty()")).into(),
                        (Ptr::<()>::from_string_literal(b"!args.get((std::size_t)-1)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_10(unimplemented!(), unimplemented!()) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (29_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_empty()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(args.get(0), std::monostate{})"))
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cargs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (32_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_empty()")).into(),
                        (Ptr::<()>::from_string_literal(b"!cargs.get(0)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_10(unimplemented!(), unimplemented!()) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (33_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_empty()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(cargs.get(0), std::monostate{})"))
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let wargs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (36_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_empty()")).into(),
                        (Ptr::<()>::from_string_literal(b"!wargs.get(0)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_11(unimplemented!(), unimplemented!()) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (37_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_empty()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(wargs.get(0), std::monostate{})"))
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub type E = ();
pub const E_ByGum: E = 0;
pub fn test_args_27() {
    let mut b: () = false;
    let i: Value<()> = Rc::new(RefCell::new(1_()));
    let c: Value<()> = Rc::new(RefCell::new(('2' as ())));
    let d: Value<()> = Rc::new(RefCell::new(3.4_f64));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_12(unimplemented!(), false) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (65_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(args.get(0), false)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_13(unimplemented!(), 1_()) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (66_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(args.get(1), 1)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_14(unimplemented!(), ('2' as ())) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (67_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(args.get(2), \'2\')")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_15(unimplemented!(), 3.4_f64) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (68_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(args.get(3), 3.4)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (69_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"!args.get(4)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut l: () = 5_();
    let ull: Value<()> = Rc::new(RefCell::new(6_()));
    let f: Value<()> = Rc::new(RefCell::new(7.8_f32));
    let cstore: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cargs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ equals_16(unimplemented!(), 5_()) }) {
                ({
                    __builtin_fprintf_26(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                            (80_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                            (Ptr::<()>::from_string_literal(b"equals(cargs.get(0), 5LL)")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_17(unimplemented!(), 6_()) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (81_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(cargs.get(1), 6ULL)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_18(unimplemented!(), 7.8_f32) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (82_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(cargs.get(2), 7.8f)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (83_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"!cargs.get(3)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut s: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_19(unimplemented!(), unimplemented!()) }) {
            ({
                __builtin_fprintf_26 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"args.cc" )  ) .into() , ( 86_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_args()" )  ) .into() , ( Ptr::<() >::from_string_literal(b"equals(std::format_args(std::make_format_args(s)).get(0), std::string_view(\"tenfour\"))" )  ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut nine: () = ('9' as ());
    let ten: Value<()> = Rc::new(RefCell::new((88 as ())));
    let eleven: Value<Box<[()]>> = Rc::new(RefCell::new(Box::from([
        101 as (), 108 as (), 101 as (), 118 as (), 101 as (), 110 as (), 0 as (),
    ])));
    let twelve13: Value<()> = Rc::new(RefCell::new(12.13_f64));
    let tenfour: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let wstore: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let wargs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_20(unimplemented!(), ('9' as ())) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (97_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"equals(wargs.get(0), static_cast<wchar_t>(\'9\'))",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_20(unimplemented!(), (88 as ())) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (98_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(wargs.get(1), L\'X\')")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            equals_21(
                unimplemented!(),
                (eleven.as_pointer() as Ptr<()>).reinterpret_cast::<()>(),
            )
        }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (99_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"equals(wargs.get(2), static_cast<const wchar_t*>(eleven))",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_22(unimplemented!(), 12.13_f64) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (100_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"equals(wargs.get(3), 12.13L)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_23(unimplemented!(), unimplemented!()) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (101_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"equals(wargs.get(4), std::wstring_view(tenfour))",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (102_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(b"!wargs.get(5)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut null: () = unimplemented!();
    let eebygum: Value<E> = Rc::new(RefCell::new(E_ByGum));
    let another_store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*args.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ equals_24(unimplemented!(), unimplemented!()) }) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (108_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"equals(args.get(0), static_cast<const void*>(nullptr))",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let is_handle: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return is_same_v_28.with(|rc| *rc.borrow());
        },
        |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return is_same_v_29.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_30.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_31.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_32.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_33.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_34.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_35.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_36.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_37.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_38.with(|rc| *rc.borrow());
        },
        |mut _a0: Ptr<()>| -> () {
            return is_same_v_39.with(|rc| *rc.borrow());
        },
        |_a0: ()| -> () {
            let _a0: Value<()> = Rc::new(RefCell::new(_a0));
            return is_same_v_40.with(|rc| *rc.borrow());
        },
        |mut _a0: ()| -> () {
            return is_same_v_41.with(|rc| *rc.borrow());
        }
    )));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (111_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_args()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"std::visit_format_arg(is_handle, args.get(1))",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn test_member_visit_42() {}
pub fn test_visited_as_handle_43() {
    let mut v: () = unimplemented!();
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let is_handle: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |arg: ()| -> () {
            let arg: Value<()> = Rc::new(RefCell::new(arg));
            return is_same_v_28.with(|rc| *rc.borrow());
        },
        |arg: ()| -> () {
            let arg: Value<()> = Rc::new(RefCell::new(arg));
            return is_same_v_29.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_30.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_31.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_32.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_33.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_34.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_35.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_36.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_37.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_38.with(|rc| *rc.borrow());
        },
        |mut arg: Ptr<()>| -> () {
            return is_same_v_39.with(|rc| *rc.borrow());
        },
        |arg: ()| -> () {
            let arg: Value<()> = Rc::new(RefCell::new(arg));
            return is_same_v_40.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_41.with(|rc| *rc.borrow());
        }
    )));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (161_()).into(),
                        (Ptr::<()>::from_string_literal(
                            b"void test_visited_as_handle() [T = __int128]",
                        ))
                        .into(),
                        (Ptr::<()>::from_string_literal(
                            b"std::visit_format_arg(is_handle, args.get(0))",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn test_visited_as_handle_44() {
    let mut v: () = unimplemented!();
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let is_handle: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |arg: ()| -> () {
            let arg: Value<()> = Rc::new(RefCell::new(arg));
            return is_same_v_28.with(|rc| *rc.borrow());
        },
        |arg: ()| -> () {
            let arg: Value<()> = Rc::new(RefCell::new(arg));
            return is_same_v_29.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_30.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_31.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_32.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_33.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_34.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_35.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_36.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_37.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_38.with(|rc| *rc.borrow());
        },
        |mut arg: Ptr<()>| -> () {
            return is_same_v_39.with(|rc| *rc.borrow());
        },
        |arg: ()| -> () {
            let arg: Value<()> = Rc::new(RefCell::new(arg));
            return is_same_v_40.with(|rc| *rc.borrow());
        },
        |mut arg: ()| -> () {
            return is_same_v_41.with(|rc| *rc.borrow());
        }
    )));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            ({
                __builtin_fprintf_26(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"args.cc")).into(),
                        (161_()).into(),
                        (Ptr::<()>::from_string_literal(
                            b"void test_visited_as_handle() [T = unsigned __int128]",
                        ))
                        .into(),
                        (Ptr::<()>::from_string_literal(
                            b"std::visit_format_arg(is_handle, args.get(0))",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_empty_25() });
    ({ test_args_27() });
    ({ test_member_visit_42() });
    ({ test_visited_as_handle_43() });
    ({ test_visited_as_handle_44() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write((({ getpid_45() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_46(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
        if (this.with(|__s| __s.sem_set_) == -1_()) {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not obtain semaphore set",
                ))
            });
        }
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if (({
            semctl_47(
                this.with(|__s| __s.sem_set_),
                0_(),
                16_(),
                Box::new([((*val.borrow()).clone()).into()]),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not initialize semaphore",
                ))
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn set_memory_limits_48(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_49((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_50((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_49((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_50((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_49((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_50((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_51(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_49((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_50((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_52(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_53(
            (mangled).clone(),
            Ptr::<()>::null(),
            Ptr::<()>::null(),
            (status.as_pointer()),
        )
    });
    if !(demangled).is_null() {
        s = (demangled).clone();
    } else {
        switch!(match (*status.borrow()) {
            __v if __v == 0_() => {
                s = Ptr::<()>::from_string_literal(b"error code = 0: success");
                break;
            }
            __v if __v == -1_() => {
                s = Ptr::<()>::from_string_literal(b"error code = -1: memory allocation failure");
                break;
            }
            __v if __v == -2_() => {
                s = Ptr::<()>::from_string_literal(b"error code = -2: invalid mangled name");
                break;
            }
            __v if __v == -3_() => {
                s = Ptr::<()>::from_string_literal(b"error code = -3: invalid arguments");
                break;
            }
            _ => {
                s = Ptr::<()>::from_string_literal(b"error code unknown - who knows what happened");
            }
        });
    }
    let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if !(unimplemented!()) {
        ({ __throw_runtime_error_3((s).clone()) });
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_54(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut res: Ptr<()> = unimplemented!();
    if !(res).is_null() {
        let mut preLC_ALL: () = unimplemented!();
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: () = 0_();
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                ({
                    __builtin_fprintf_26 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        ({ __throw_runtime_error_3(unimplemented!()) });
    }
}
pub fn run_tests_wrapped_env_55(
    mut name: Ptr<()>,
    mut env: Ptr<()>,
    l: Ptr<__gnu_test_func_callback>,
) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut oldENV: Ptr<()> = unimplemented!();
    if !(unimplemented!() != 0) {
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: () = 0_();
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        unimplemented!();
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ __throw_runtime_error_3(unimplemented!()) });
    }
}
thread_local!(
    pub static count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static count__1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static count__4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static _M_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static next_id__7: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct __gnu_test_semun {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl __gnu_test_semun {
    pub fn val(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn buf(this: Ptr<Self>) -> Ptr<Ptr<semid_ds>> {
        this.reinterpret_cast()
    }
    pub fn array(this: Ptr<Self>) -> Ptr<Ptr<()>> {
        this.reinterpret_cast()
    }
}
impl Default for __gnu_test_semun {
    fn default() -> Self {
        __gnu_test_semun {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
impl __gnu_test_semaphore {}
pub fn test_tm_57(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> ::libc::tm {
    (*tmp_58.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_58.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_58.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_58.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_58.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_58.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_58.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_58.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_58.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_58.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_58: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
);
pub trait __gnu_test_copy_trackerImpl {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker>;
    fn destructor(&self);
    fn id(&self) -> ();
}
impl __gnu_test_copy_trackerImpl for Ptr<__gnu_test_copy_tracker> {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker> {
        field!((*self), id_).write(({ __gnu_test_copy_trackerImpl::id(&rhs) }));
        if rhs.with(|__s| __s.throw_on_copy_) {
            ({
                __gnu_test_assignment_operator::throw_on(
                    ({ __gnu_test_assignment_operator::count() }).wrapping_add(1_()),
                )
            });
        }
        ({ __gnu_test_assignment_operator::mark_call() });
        return (*self).clone();
    }
    fn destructor(&self) {
        ({ __gnu_test_destructor::mark_call() });
    }
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait __gnu_test_func_callbackImpl {
    fn copy_assign(&self, _a0: Ptr<__gnu_test_func_callback>) -> Ptr<__gnu_test_func_callback> {
        unimplemented!()
    }
    fn size(&self) -> ();
    fn tests(&self) -> Ptr<FnPtr<fn()>>;
    fn push_back(&self, test: FnPtr<fn()>);
}
impl __gnu_test_func_callbackImpl for Ptr<__gnu_test_func_callback> {
    fn size(&self) -> () {
        return (*self).with(|__s| __s._M_size);
    }
    fn tests(&self) -> Ptr<FnPtr<fn()>> {
        return (array_field_ptr!((*self), _M_tests) as Ptr<FnPtr<fn()>>);
    }
    fn push_back(&self, mut test: FnPtr<fn()>) {
        array_elem!((*self), _M_tests, (*self).with(|__s| __s._M_size)).write((test).clone());
        field!((*self), _M_size).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait __gnu_test_object_counterImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter>;
}
impl __gnu_test_object_counterImpl for Ptr<__gnu_test_object_counter> {
    fn destructor(&self) {
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter> {
        return (*self).clone();
    }
}
pub trait __gnu_test_semaphoreImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn signal(&self) {
        unimplemented!()
    }
    fn wait(&self) {
        unimplemented!()
    }
}
impl __gnu_test_semaphoreImpl for Ptr<__gnu_test_semaphore> {
    fn destructor(&self) {
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if ((*self).with(|__s| __s.pid_) == ({ getpid_45() })) {
            ({
                semctl_47(
                    (*self).with(|__s| __s.sem_set_),
                    0_(),
                    0_(),
                    Box::new([((*val.borrow()).clone()).into()]),
                )
            });
        }
    }
    fn signal(&self) {
        let op: Value<Box<[sembuf]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (({
            semop_56(
                (*self).with(|__s| __s.sem_set_),
                (op.as_pointer() as Ptr<sembuf>),
                1_(),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not signal semaphore",
                ))
            });
        }
    }
    fn wait(&self) {
        let op: Value<Box<[sembuf]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (({
            semop_56(
                (*self).with(|__s| __s.sem_set_),
                (op.as_pointer() as Ptr<sembuf>),
                1_(),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not wait for semaphore",
                ))
            });
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__4.with(|_| ());
    let _ = throw_on__5.with(|_| ());
    let _ = _M_count_6.with(|_| ());
    let _ = next_id__7.with(|_| ());
}
