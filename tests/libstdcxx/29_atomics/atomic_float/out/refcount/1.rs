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
pub fn test01_10() {
    let mut mo: () = memory_order_relaxed_11.with(|rc| *rc.borrow());
    let mut ok: () = unimplemented!();
    let mut expected: () = unimplemented!();
    {
        let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ok = unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (37_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (38_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load(mo) == a0.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (41_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut f0: () = unimplemented!();
        let mut f1: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 12.5_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (44_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 12.5f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (45_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 0.5_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (46_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 0.5f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == f1) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (47_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == f1")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 12.5_f32;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.6_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (52_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == 1.6f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(expected == 12.5_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (53_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 12.5f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.5_f32)) && (expected == 12.5_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (56_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 12.5f && expected == 12.5f",
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
            if !(expected == 12.5_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (57_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 12.5f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.6_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 3.2_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (60_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a0.load() == 3.2f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(expected == 1.6_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (61_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 1.6f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.5_f32)) && (expected == 12.5_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (64_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 12.5f && expected == 12.5f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 3.2_f32;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0.64_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (69_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == .64f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 12.5_f32;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.6_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (73_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1.load() == 1.6f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 0.64_f32)) && (expected == 0.64_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (76_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a0.load() == .64f && expected == .64f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 1.6_f32)) && (expected == 1.6_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (79_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 1.6f && expected == 1.6f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.64_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 12.8_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (83_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a0.load() == 12.8f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.6_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 2.56_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (86_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a1.load() == 2.56f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.8_f32)) && (expected == 12.8_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (89_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a0.load() == 12.8f && expected == 12.8f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 2.56_f32)) && (expected == 2.56_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (92_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 2.56f && expected == 2.56f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 12.8_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (95_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 12.8f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 14_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (96_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 14.0f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f1 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f1 == 2.56_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (98_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f1 == 2.56f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 4.96_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (99_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1 == 4.96f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 14_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (102_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 14.0f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 12.8_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (103_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 12.8f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f1 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f1 == 4.96_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (105_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f1 == 4.96f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.46_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (106_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1 == 1.46f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 14_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (109_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 14.0f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 14_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (110_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 14.0f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 13.2_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (113_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 13.2f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 13.2_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (114_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 13.2f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ok = unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (124_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (125_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load(mo) == a0.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (128_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut f0: () = unimplemented!();
        let mut f1: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 12.5_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (131_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 12.5f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (132_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 0.5_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (133_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 0.5f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == f1) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (134_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == f1")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 12.5_f32;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.6_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (139_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == 1.6f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(expected == 12.5_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (140_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 12.5f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.5_f32)) && (expected == 12.5_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (143_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 12.5f && expected == 12.5f",
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
            if !(expected == 12.5_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (144_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 12.5f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.6_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 3.2_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (147_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a0.load() == 3.2f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(expected == 1.6_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (148_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 1.6f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.5_f32)) && (expected == 12.5_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (151_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 12.5f && expected == 12.5f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 3.2_f32;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0.64_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (156_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == .64f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 12.5_f32;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.6_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (160_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1.load() == 1.6f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 0.64_f32)) && (expected == 0.64_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (163_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a0.load() == .64f && expected == .64f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 1.6_f32)) && (expected == 1.6_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (166_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 1.6f && expected == 1.6f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.64_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 12.8_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (170_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a0.load() == 12.8f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.6_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 2.56_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (173_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a1.load() == 2.56f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.8_f32)) && (expected == 12.8_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (176_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a0.load() == 12.8f && expected == 12.8f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f32;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 2.56_f32)) && (expected == 2.56_f32)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (179_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 2.56f && expected == 2.56f",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 12.8_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (182_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 12.8f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 14_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (183_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 14.0f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f1 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f1 == 2.56_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (185_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f1 == 2.56f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 4.96_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (186_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1 == 4.96f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 14_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (189_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 14.0f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 12.8_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (190_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 12.8f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f1 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f1 == 4.96_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (192_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f1 == 4.96f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.46_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (193_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1 == 1.46f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 14_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (196_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 14.0f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 14_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (197_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 14.0f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 13.2_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (200_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 13.2f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 13.2_f32) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (201_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 13.2f")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test02_13() {
    let mut mo: () = memory_order_relaxed_11.with(|rc| *rc.borrow());
    let mut ok: () = unimplemented!();
    let mut expected: () = unimplemented!();
    {
        let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ok = unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (218_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (219_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load(mo) == a0.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (222_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut f0: () = unimplemented!();
        let mut f1: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 12.5_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (225_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 12.5")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (226_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 0.5_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (227_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 0.5")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == f1) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (228_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == f1")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 12.5_f64;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.6_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (233_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == 1.6")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(expected == 12.5_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (234_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 12.5")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.5_f64)) && (expected == 12.5_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (237_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 12.5 && expected == 12.5",
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
            if !(expected == 12.5_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (238_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 12.5")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.6_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 3.2_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (241_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a0.load() == 3.2")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(expected == 1.6_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (242_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 1.6")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.5_f64)) && (expected == 12.5_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (245_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 12.5 && expected == 12.5",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 3.2_f64;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0.64_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (250_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == .64")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 12.5_f64;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.6_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (254_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1.load() == 1.6")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 0.64_f64)) && (expected == 0.64_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (257_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a0.load() == .64 && expected == .64",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 1.6_f64)) && (expected == 1.6_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (260_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 1.6 && expected == 1.6",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.64_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 12.8_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (264_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a0.load() == 12.8")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.6_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 2.56_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (267_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a1.load() == 2.56")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.8_f64)) && (expected == 12.8_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (270_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a0.load() == 12.8 && expected == 12.8",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 2.56_f64)) && (expected == 2.56_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (273_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 2.56 && expected == 2.56",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 12.8_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (276_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 12.8")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 14_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (277_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 14.0")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f1 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f1 == 2.56_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (279_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f1 == 2.56")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 4.96_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (280_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1 == 4.96")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 14_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (283_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 14.0")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 12.8_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (284_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 12.8")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f1 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f1 == 4.96_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (286_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f1 == 4.96")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.46_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (287_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1 == 1.46")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 14_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (290_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 14.0")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 14_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (291_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 14.0")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 13.2_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (294_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 13.2")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 13.2_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (295_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 13.2")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let a0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ok = unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (305_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (306_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load(mo) == a0.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (309_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut f0: () = unimplemented!();
        let mut f1: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 12.5_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (312_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 12.5")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == unimplemented!()) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (313_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == a1.load()")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 0.5_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (314_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 0.5")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == f1) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (315_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == f1")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 12.5_f64;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.6_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (320_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == 1.6")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(expected == 12.5_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (321_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 12.5")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.5_f64)) && (expected == 12.5_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (324_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 12.5 && expected == 12.5",
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
            if !(expected == 12.5_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (325_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 12.5")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.6_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 3.2_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (328_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a0.load() == 3.2")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(expected == 1.6_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (329_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"expected == 1.6")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.5_f64)) && (expected == 12.5_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (332_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 12.5 && expected == 12.5",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 3.2_f64;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0.64_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (337_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0.load() == .64")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 12.5_f64;
        'loop_: while !(unimplemented!()) {}
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.6_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (341_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1.load() == 1.6")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 0.64_f64)) && (expected == 0.64_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (344_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a0.load() == .64 && expected == .64",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 1.6_f64)) && (expected == 1.6_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (347_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 1.6 && expected == 1.6",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.64_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 12.8_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (351_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a0.load() == 12.8")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 1.6_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((ok) && (unimplemented!() == 2.56_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (354_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"ok && a1.load() == 2.56")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 12.8_f64)) && (expected == 12.8_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (357_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a0.load() == 12.8 && expected == 12.8",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        expected = 0.5_f64;
        ok = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((!(ok)) && (unimplemented!() == 2.56_f64)) && (expected == 2.56_f64)) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (360_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(
                                b"!ok && a1.load() == 2.56 && expected == 2.56",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 12.8_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (363_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 12.8")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 14_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (364_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 14.0")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f1 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f1 == 2.56_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (366_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f1 == 2.56")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 4.96_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (367_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1 == 4.96")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 14_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (370_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 14.0")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 12.8_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (371_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 12.8")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f1 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f1 == 4.96_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (373_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f1 == 4.96")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 1.46_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (374_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a1 == 1.46")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 14_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (377_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 14.0")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 14_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (378_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 14.0")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        f0 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(f0 == 13.2_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (381_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"f0 == 13.2")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 13.2_f64) {
                ({
                    __builtin_fprintf_12(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"1.cc")).into(),
                            (382_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                            (Ptr::<()>::from_string_literal(b"a0 == 13.2")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test03_14() {
    let mut mo: () = memory_order_relaxed_11.with(|rc| *rc.borrow());
    let mut ok: () = unimplemented!();
    let mut expected: () = unimplemented!();
    {}
    {}
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_10() });
    ({ test02_13() });
    ({ test03_14() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write((({ getpid_15() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_16(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_17(
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
pub fn set_memory_limits_18(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_19((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_20((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_19((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_20((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_19((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_20((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_21(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_19((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_20((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_22(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_23(
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
pub fn run_tests_wrapped_locale_24(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                    __builtin_fprintf_12 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
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
pub fn run_tests_wrapped_env_25(
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
pub fn test_tm_27(
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
    (*tmp_28.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_28.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_28: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_15() })) {
            ({
                semctl_17(
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
            semop_26(
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
            semop_26(
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
