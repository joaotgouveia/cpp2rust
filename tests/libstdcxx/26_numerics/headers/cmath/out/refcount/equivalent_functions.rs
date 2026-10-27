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
const _: () = assert!(
    true,
    "std::is_same<decltype(std::acosf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::asinf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::atanf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::atan2f(0.0, 0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::ceilf(0.0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::cosf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::coshf(0.0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::expf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::fabsf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::floorf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::fmodf(0.0, 0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::frexpf(0.0, nullptr)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::ldexpf(0.0, 0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::logf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::log10f(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::modff(0.0, nullptr)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::powf(0.0, 0.0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::sinf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sinhf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sqrtf(0.0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::tanf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tanhf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::acosl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::asinl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::atanl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::atan2l(0.0, 0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::ceill(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::cosl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::coshl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::expl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::fabsl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::floorl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::fmodl(0.0, 0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::frexpl(0.0, nullptr)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::ldexpl(0.0, 0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::logl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::log10l(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::modfl(0.0, nullptr)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::powl(0.0, 0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sinl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sinhl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sqrtl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tanl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tanhl(0.0)), long double>::value"
);
pub fn test_float_global_10() {
    let mut x: () = 1_f32;
    let e1: Value<()> = Rc::new(RefCell::new(0_()));
    let e2: Value<()> = Rc::new(RefCell::new(0_()));
    let i1: Value<()> = Rc::new(RefCell::new(0_f32));
    let i2: Value<()> = Rc::new(RefCell::new(0_f32));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ acosf_11(x) }) == ({ acosf_11(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (83_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::acosf(x) == std::acosf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ asinf_13(x) }) == ({ asinf_13(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (84_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::asinf(x) == std::asinf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ atanf_14(x) }) == ({ atanf_14(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (85_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::atanf(x) == std::atanf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___y: () = x;
            let ___x: () = x;
            atan2f_15(___y, ___x)
        }) == ({
            let ___y: () = x;
            let ___x: () = x;
            atan2f_15(___y, ___x)
        })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (86_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::atan2f(x, x) == std::atan2f(x, x)"))
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
        if !(({ ceilf_16(x) }) == ({ ceilf_16(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (87_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::ceilf(x) == std::ceilf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ cosf_17(x) }) == ({ cosf_17(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (88_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::cosf(x) == std::cosf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ coshf_18(x) }) == ({ coshf_18(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (89_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::coshf(x) == std::coshf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ expf_19(x) }) == ({ expf_19(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (90_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::expf(x) == std::expf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ fabsf_20(x) }) == ({ fabsf_20(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (91_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::fabsf(x) == std::fabsf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ floorf_21(x) }) == ({ floorf_21(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (92_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::floorf(x) == std::floorf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___x: () = x;
            let ___y: () = x;
            fmodf_22(___x, ___y)
        }) == ({
            let ___x: () = x;
            let ___y: () = x;
            fmodf_22(___x, ___y)
        })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (93_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::fmodf(x, x) == std::fmodf(x, x)"))
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
        if !(({ frexpf_23(x, (e1.as_pointer())) }) == ({ frexpf_23(x, (e2.as_pointer())) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (94_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"::frexpf(x, &e1) == std::frexpf(x, &e2)",
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
        if !((*e1.borrow()) == (*e2.borrow())) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (95_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"e1 == e2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ ldexpf_24(x, 1_()) }) == ({ ldexpf_24(x, 1_()) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (96_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::ldexpf(x, 1) == std::ldexpf(x, 1)"))
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
        if !(({ logf_25(x) }) == ({ logf_25(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (97_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::logf(x) == std::logf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ log10f_26(x) }) == ({ log10f_26(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (98_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::log10f(x) == std::log10f(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ modff_27(x, (i1.as_pointer())) }) == ({ modff_27(x, (i2.as_pointer())) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (99_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::modff(x, &i1) == std::modff(x, &i2)"))
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
        if !((*i1.borrow()) == (*i2.borrow())) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (100_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"i1 == i2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___x: () = x;
            let ___y: () = x;
            powf_28(___x, ___y)
        }) == ({
            let ___x: () = x;
            let ___y: () = x;
            powf_28(___x, ___y)
        })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (101_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::powf(x, x) == std::powf(x, x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ sinf_29(x) }) == ({ sinf_29(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (102_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::sinf(x) == std::sinf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ sinhf_30(x) }) == ({ sinhf_30(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (103_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::sinhf(x) == std::sinhf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ sqrtf_31(x) }) == ({ sqrtf_31(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (104_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::sqrtf(x) == std::sqrtf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ tanf_32(x) }) == ({ tanf_32(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (105_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::tanf(x) == std::tanf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ tanhf_33(x) }) == ({ tanhf_33(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (106_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::tanhf(x) == std::tanhf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn test_float_overload_34() {
    let mut x: () = 1_f32;
    let e1: Value<()> = Rc::new(RefCell::new(0_()));
    let e2: Value<()> = Rc::new(RefCell::new(0_()));
    let i1: Value<()> = Rc::new(RefCell::new(0_f32));
    let i2: Value<()> = Rc::new(RefCell::new(0_f32));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ acosf_11(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (119_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::acos(x) == std::acosf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ asinf_13(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (120_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::asin(x) == std::asinf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ atanf_14(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (121_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::atan(x) == std::atanf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()
            == ({
                let ___y: () = x;
                let ___x: () = x;
                atan2f_15(___y, ___x)
            }))
        {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (122_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::atan2(x, x) == std::atan2f(x, x)"))
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
        if !(unimplemented!() == ({ ceilf_16(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (123_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::ceil(x) == std::ceilf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ cosf_17(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (124_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::cos(x) == std::cosf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ coshf_18(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (125_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::cosh(x) == std::coshf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ expf_19(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (126_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::exp(x) == std::expf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ fabsf_20(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (127_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::fabs(x) == std::fabsf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ floorf_21(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (128_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::floor(x) == std::floorf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()
            == ({
                let ___x: () = x;
                let ___y: () = x;
                fmodf_22(___x, ___y)
            }))
        {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (129_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::fmod(x, x) == std::fmodf(x, x)"))
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
        if !(unimplemented!() == ({ frexpf_23(x, (e2.as_pointer())) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (130_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"std::frexp(x, &e1) == std::frexpf(x, &e2)",
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
        if !((*e1.borrow()) == (*e2.borrow())) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (131_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"e1 == e2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ ldexpf_24(x, 1_()) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (132_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::ldexp(x, 1) == std::ldexpf(x, 1)"))
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
        if !(unimplemented!() == ({ logf_25(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (133_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::log(x) == std::logf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ log10f_26(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (134_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::log10(x) == std::log10f(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ modff_27(x, (i2.as_pointer())) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (135_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"std::modf(x, &i1) == std::modff(x, &i2)",
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
        if !((*i1.borrow()) == (*i2.borrow())) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (136_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"i1 == i2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()
            == ({
                let ___x: () = x;
                let ___y: () = x;
                powf_28(___x, ___y)
            }))
        {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (137_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::pow(x, x) == std::powf(x, x)"))
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
        if !(unimplemented!() == ({ sinf_29(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (138_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::sin(x) == std::sinf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ sinhf_30(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (139_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::sinh(x) == std::sinhf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ sqrtf_31(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (140_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::sqrt(x) == std::sqrtf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ tanf_32(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (141_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::tan(x) == std::tanf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ tanhf_33(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (142_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_float_overload()")).into(),
                        (Ptr::<()>::from_string_literal(b"std::tanh(x) == std::tanhf(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn test_long_double_global_35() {
    let mut x: () = 1_f64;
    let e1: Value<()> = Rc::new(RefCell::new(0_()));
    let e2: Value<()> = Rc::new(RefCell::new(0_()));
    let i1: Value<()> = Rc::new(RefCell::new(0_f64));
    let i2: Value<()> = Rc::new(RefCell::new(0_f64));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ acosl_36(x) }) == ({ acosl_36(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (155_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::acosl(x) == std::acosl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ asinl_37(x) }) == ({ asinl_37(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (156_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::asinl(x) == std::asinl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ atanl_38(x) }) == ({ atanl_38(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (157_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::atanl(x) == std::atanl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___y: () = x;
            let ___x: () = x;
            atan2l_39(___y, ___x)
        }) == ({
            let ___y: () = x;
            let ___x: () = x;
            atan2l_39(___y, ___x)
        })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (158_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::atan2l(x, x) == std::atan2l(x, x)"))
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
        if !(({ ceill_40(x) }) == ({ ceill_40(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (159_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::ceill(x) == std::ceill(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ cosl_41(x) }) == ({ cosl_41(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (160_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::cosl(x) == std::cosl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ coshl_42(x) }) == ({ coshl_42(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (161_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::coshl(x) == std::coshl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ expl_43(x) }) == ({ expl_43(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (162_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::expl(x) == std::expl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ fabsl_44(x) }) == ({ fabsl_44(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (163_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::fabsl(x) == std::fabsl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ floorl_45(x) }) == ({ floorl_45(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (164_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::floorl(x) == std::floorl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___x: () = x;
            let ___y: () = x;
            fmodl_46(___x, ___y)
        }) == ({
            let ___x: () = x;
            let ___y: () = x;
            fmodl_46(___x, ___y)
        })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (165_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::fmodl(x, x) == std::fmodl(x, x)"))
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
        if !(({ frexpl_47(x, (e1.as_pointer())) }) == ({ frexpl_47(x, (e2.as_pointer())) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (166_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"::frexpl(x, &e1) == std::frexpl(x, &e2)",
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
        if !((*e1.borrow()) == (*e2.borrow())) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (167_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"e1 == e2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ ldexpl_48(x, 1_()) }) == ({ ldexpl_48(x, 1_()) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (168_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::ldexpl(x, 1) == std::ldexpl(x, 1)"))
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
        if !(({ logl_49(x) }) == ({ logl_49(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (169_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::logl(x) == std::logl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ log10l_50(x) }) == ({ log10l_50(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (170_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::log10l(x) == std::log10l(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ modfl_51(x, (i1.as_pointer())) }) == ({ modfl_51(x, (i2.as_pointer())) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (171_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::modfl(x, &i1) == std::modfl(x, &i2)"))
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
        if !((*i1.borrow()) == (*i2.borrow())) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (172_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"i1 == i2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___x: () = x;
            let ___y: () = x;
            powl_52(___x, ___y)
        }) == ({
            let ___x: () = x;
            let ___y: () = x;
            powl_52(___x, ___y)
        })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (173_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::powl(x, x) == std::powl(x, x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ sinl_53(x) }) == ({ sinl_53(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (174_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::sinl(x) == std::sinl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ sinhl_54(x) }) == ({ sinhl_54(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (175_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::sinhl(x) == std::sinhl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ sqrtl_55(x) }) == ({ sqrtl_55(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (176_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::sqrtl(x) == std::sqrtl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ tanl_56(x) }) == ({ tanl_56(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (177_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::tanl(x) == std::tanl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ tanhl_57(x) }) == ({ tanhl_57(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (178_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_global()")).into(),
                        (Ptr::<()>::from_string_literal(b"::tanhl(x) == std::tanhl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn test_long_double_overload_58() {
    let mut x: () = 1_f64;
    let e1: Value<()> = Rc::new(RefCell::new(0_()));
    let e2: Value<()> = Rc::new(RefCell::new(0_()));
    let i1: Value<()> = Rc::new(RefCell::new(0_f64));
    let i2: Value<()> = Rc::new(RefCell::new(0_f64));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ acosl_36(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (191_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::acos(x) == std::acosl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ asinl_37(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (192_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::asin(x) == std::asinl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ atanl_38(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (193_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::atan(x) == std::atanl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()
            == ({
                let ___y: () = x;
                let ___x: () = x;
                atan2l_39(___y, ___x)
            }))
        {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (194_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::atan2(x, x) == std::atan2l(x, x)"))
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
        if !(unimplemented!() == ({ ceill_40(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (195_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::ceil(x) == std::ceill(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ cosl_41(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (196_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::cos(x) == std::cosl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ coshl_42(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (197_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::cosh(x) == std::coshl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ expl_43(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (198_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::exp(x) == std::expl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ fabsl_44(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (199_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::fabs(x) == std::fabsl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ floorl_45(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (200_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::floor(x) == std::floorl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()
            == ({
                let ___x: () = x;
                let ___y: () = x;
                fmodl_46(___x, ___y)
            }))
        {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (201_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::fmod(x, x) == std::fmodl(x, x)"))
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
        if !(unimplemented!() == ({ frexpl_47(x, (e2.as_pointer())) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (202_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(
                            b"std::frexp(x, &e1) == std::frexpl(x, &e2)",
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
        if !((*e1.borrow()) == (*e2.borrow())) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (203_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"e1 == e2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ ldexpl_48(x, 1_()) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (204_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::ldexp(x, 1) == std::ldexpl(x, 1)"))
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
        if !(unimplemented!() == ({ logl_49(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (205_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::log(x) == std::logl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ log10l_50(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (206_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::log10(x) == std::log10l(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ modfl_51(x, (i2.as_pointer())) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (207_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(
                            b"std::modf(x, &i1) == std::modfl(x, &i2)",
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
        if !((*i1.borrow()) == (*i2.borrow())) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (208_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"i1 == i2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()
            == ({
                let ___x: () = x;
                let ___y: () = x;
                powl_52(___x, ___y)
            }))
        {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (209_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::pow(x, x) == std::powl(x, x)"))
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
        if !(unimplemented!() == ({ sinl_53(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (210_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::sin(x) == std::sinl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ sinhl_54(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (211_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::sinh(x) == std::sinhl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ sqrtl_55(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (212_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::sqrt(x) == std::sqrtl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ tanl_56(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (213_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::tan(x) == std::tanl(x)")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == ({ tanhl_57(x) })) {
            ({
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"equivalent_functions.cc")).into(),
                        (214_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test_long_double_overload()"))
                            .into(),
                        (Ptr::<()>::from_string_literal(b"std::tanh(x) == std::tanhl(x)")).into(),
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
    ({ test_float_global_10() });
    ({ test_float_overload_34() });
    ({ test_long_double_global_35() });
    ({ test_long_double_overload_58() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write((({ getpid_59() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_60(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_61(
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
pub fn set_memory_limits_62(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_63((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_64((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_63((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_64((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_63((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_64((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_65(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_63((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_64((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_66(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_67(
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
    let mut w: () = unimplemented!();
    if unimplemented!() {
        ({ __throw_runtime_error_3((s).clone()) });
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_68(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_69(
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
pub fn test_tm_71(
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
    (*tmp_72.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_72.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_72.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_72.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_72.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_72.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_72.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_72.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_72.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_72.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_72: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_59() })) {
            ({
                semctl_61(
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
            semop_70(
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
            semop_70(
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
