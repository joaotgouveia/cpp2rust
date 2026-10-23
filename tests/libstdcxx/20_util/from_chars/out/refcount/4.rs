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
    let mut s: () = unimplemented!();
    let mut d: () = unimplemented!();
    let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (fmt.read());
        unimplemented!();
        (*res.borrow_mut()) = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (45_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"std::isinf(d)")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ { (*res.borrow()).ptr.clone() } } == {
                unimplemented!().offset((3_()) as isize)
            }) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (46_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 3")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*res.borrow()).ec } == unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (47_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        (*res.borrow_mut()) = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (51_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"std::isinf(d)")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(d < 0_()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (52_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"d < 0")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ { (*res.borrow()).ptr.clone() } } == {
                unimplemented!().offset((4_()) as isize)
            }) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (53_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 4")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*res.borrow()).ec } == unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (54_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        (*res.borrow_mut()) = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (58_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"std::isinf(d)")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ { (*res.borrow()).ptr.clone() } } == {
                unimplemented!().offset((8_()) as isize)
            }) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (59_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 8")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*res.borrow()).ec } == unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (60_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        (*res.borrow_mut()) = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (64_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"std::isnan(d)")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ { (*res.borrow()).ptr.clone() } } == {
                unimplemented!().offset((3_()) as isize)
            }) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (65_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 3")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*res.borrow()).ec } == unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (66_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        (*res.borrow_mut()) = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (70_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"std::isnan(d)")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ { (*res.borrow()).ptr.clone() } } == {
                unimplemented!().offset((unimplemented!()) as isize)
            }) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (71_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + s.length()"))
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
            if !({ (*res.borrow()).ec } == unimplemented!()) {
                ({
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                            (72_()).into(),
                            (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                            (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test02_12() {
    let mut s: () = unimplemented!();
    let mut d: () = 1_f64;
    let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 0_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (85_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.0")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == { unimplemented!().offset((1_()) as isize) }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (86_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 1")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (87_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 0_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (92_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.0")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == { unimplemented!().offset((1_()) as isize) }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (93_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 1")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (94_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 1_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (99_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 1.0")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == { unimplemented!() }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (100_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data()")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (101_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc::invalid_argument"))
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 0_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (106_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.0")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == { unimplemented!().offset((1_()) as isize) }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (107_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 1")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (108_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 0_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (113_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.0")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == { unimplemented!().offset((1_()) as isize) }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (114_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 1")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (115_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn test03_13() {
    let mut s: () = unimplemented!();
    let mut d: () = 1_f64;
    let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 50_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (127_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.5e+2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == {
            unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset(-((1_()) as isize))
                .offset(-((3_()) as isize))
        }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (128_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"res.ptr == s.data() + s.length() - 1 - 3",
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
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (129_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 0.5_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (133_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.5")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == { unimplemented!().offset((3_()) as isize) }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (134_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 3")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (135_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 50_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (140_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.5e+2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == {
            unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset(-((1_()) as isize))
                .offset(-((3_()) as isize))
        }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (141_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"res.ptr == s.data() + s.length() - 1 - 3",
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
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (142_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 50_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (147_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.5e+2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == {
            unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset(-((1_()) as isize))
                .offset(-((3_()) as isize))
        }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (148_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"res.ptr == s.data() + s.length() - 1 - 3",
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
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (149_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 0.3671875_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (154_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0x0.5Ep0")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == { unimplemented!().offset((4_()) as isize) }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (155_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + 4")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (156_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 0.40625_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (161_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.40625")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == {
            unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset(-((3_()) as isize))
        }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (162_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + s.length() - 3"))
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
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (163_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn test04_14() {
    let mut s: () = unimplemented!();
    let mut d: () = 1_f64;
    let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == {
            unimplemented!().offset((unimplemented!()) as isize)
        }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (174_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test04()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + s.length()"))
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
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (175_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test04()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 0_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (176_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test04()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.0")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == {
            unimplemented!().offset((unimplemented!()) as isize)
        }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (180_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test04()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + s.length()"))
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
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (181_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test04()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 0.5_f64) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (182_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test04()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 0.5")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut len: () = unimplemented!();
    unimplemented!();
    (*res.borrow_mut()) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*res.borrow()).ptr.clone() } } == { unimplemented!().offset((len) as isize) }) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (188_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test04()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ptr == s.data() + len")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*res.borrow()).ec } == unimplemented!()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (189_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test04()")).into(),
                        (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(d == 50_()) {
            ({
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                        (190_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test04()")).into(),
                        (Ptr::<()>::from_string_literal(b"d == 50")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn to_string_15(mut val: ()) -> () {
    let mut s: () = unimplemented!();
    let iter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: while (val != 0_()) {
        unimplemented!().write(('0' as ()).wrapping_add(((val).wrapping_rem(10_()))));
        val = { (val).wrapping_div(10_()) };
    }
    return unimplemented!();
}
pub fn test05_16() {
    let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut flt: () = unimplemented!();
    let mut dbl: () = unimplemented!();
    let mut ldbl: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 100_()) {
        let mut s: () = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        let mut s1: Ptr<()> = unimplemented!();
        let mut s1_end: Ptr<()> = s1.offset((len) as isize);
        'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut fmt: () = (fmt.read());
            if (fmt == unimplemented!()) && (i > 9_()) {
                continue 'loop_;
            }
            (*res.borrow_mut()) = unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ (*res.borrow()).ec } == unimplemented!()) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (233_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ { (*res.borrow()).ptr.clone() } } == { (s1_end).clone() }) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (234_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ptr == s1_end")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(flt == i) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (235_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"flt == i")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            (*res.borrow_mut()) = unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ (*res.borrow()).ec } == unimplemented!()) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (238_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ { (*res.borrow()).ptr.clone() } } == { (s1_end).clone() }) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (239_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ptr == s1_end")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(dbl == i) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (240_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"dbl == i")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            (*res.borrow_mut()) = unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ (*res.borrow()).ec } == unimplemented!()) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (243_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ { (*res.borrow()).ptr.clone() } } == { (s1_end).clone() }) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (244_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ptr == s1_end")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(ldbl == i) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (245_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"ldbl == i")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
        }
        if (i > 9_()) {
            i.prefix_inc();
            continue 'loop_;
        }
        let s2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            ('.' as ()),
            (s1.read()),
            ('e' as ()),
            ('0' as ()),
            ('0' as ()),
            ('0' as ()),
            ('1' as ()),
        ])));
        let mut s2_end: Ptr<()> =
            (s2.as_pointer() as Ptr<()>).offset((::std::mem::size_of::<[(); 7]>()) as isize);
        let s3: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            (s1.read()),
            ('0' as ()),
            ('e' as ()),
            ('-' as ()),
            ('0' as ()),
            ('0' as ()),
            ('1' as ()),
        ])));
        let mut s3_end: Ptr<()> =
            (s3.as_pointer() as Ptr<()>).offset((::std::mem::size_of::<[(); 7]>()) as isize);
        'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut fmt: () = (fmt.read());
            (*res.borrow_mut()) = unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ (*res.borrow()).ec } == unimplemented!()) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (263_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ { (*res.borrow()).ptr.clone() } } == { (s2_end).clone() }) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (264_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ptr == s2_end")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(flt == i) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (265_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"flt == i")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            (*res.borrow_mut()) = unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ (*res.borrow()).ec } == unimplemented!()) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (268_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ { (*res.borrow()).ptr.clone() } } == { (s3_end).clone() }) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (269_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ptr == s3_end")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(flt == i) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (270_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"flt == i")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            (*res.borrow_mut()) = unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ (*res.borrow()).ec } == unimplemented!()) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (273_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ { (*res.borrow()).ptr.clone() } } == { (s2_end).clone() }) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (274_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ptr == s2_end")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(dbl == i) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (275_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"dbl == i")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            (*res.borrow_mut()) = unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ (*res.borrow()).ec } == unimplemented!()) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (278_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ { (*res.borrow()).ptr.clone() } } == { (s3_end).clone() }) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (279_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ptr == s3_end")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(dbl == i) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (280_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"dbl == i")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            (*res.borrow_mut()) = unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ (*res.borrow()).ec } == unimplemented!()) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (283_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ { (*res.borrow()).ptr.clone() } } == { (s2_end).clone() }) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (284_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ptr == s2_end")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(ldbl == i) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (285_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"ldbl == i")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            (*res.borrow_mut()) = unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ (*res.borrow()).ec } == unimplemented!()) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (288_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ec == std::errc{}")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !({ { (*res.borrow()).ptr.clone() } } == { (s3_end).clone() }) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (289_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"res.ptr == s3_end")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(ldbl == i) {
                    ({
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            Ptr::<()>::from_string_literal(
                                b"%s:%d: %s: Assertion \'%s\' failed.\n",
                            ),
                            Box::new([
                                (Ptr::<()>::from_string_literal(b"4.cc")).into(),
                                (290_()).into(),
                                (Ptr::<()>::from_string_literal(b"void test05()")).into(),
                                (Ptr::<()>::from_string_literal(b"ldbl == i")).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
        }
        i.prefix_inc();
    }
}
pub fn test_max_mantissa_17() {
    {
        println!(
            "Testing {}-bit float, using {}-bit integer",
            ((unimplemented!() + ({ log2_18(unimplemented!(),) })) + 1_()),
            (::std::mem::size_of::<()>() as ()).wrapping_mul(8_())
        );
        let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut flt: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 10_()) {
            let mut val: () =
                (((1_() << unimplemented!()) as ()).wrapping_sub(1_()) as ()).wrapping_sub(i);
            let mut s: () = unimplemented!();
            let mut len: () = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (fmt.read());
                (*res.borrow_mut()) = unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ (*res.borrow()).ec } == unimplemented!()) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 324_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ec == std::errc{}" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ { (*res.borrow()).ptr.clone() } } == {
                        unimplemented!().offset((len) as isize)
                    }) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 325_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ptr == s.data() + len" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !(flt == val) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 326_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"flt == val" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            unimplemented!();
            let mut orig_len: () = len;
            unimplemented!();
            len = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (fmt.read());
                (*res.borrow_mut()) = unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ (*res.borrow()).ec } == unimplemented!()) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 337_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ec == std::errc{}" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ { (*res.borrow()).ptr.clone() } } == {
                        unimplemented!().offset((len) as isize)
                    }) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 338_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ptr == s.data() + len" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !(flt == val) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 339_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"flt == val" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut s2: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                (*res.borrow_mut()) = unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ (*res.borrow()).ec } == unimplemented!()) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 345_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ec == std::errc{}" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ { (*res.borrow()).ptr.clone() } } == {
                        unimplemented!().offset((len) as isize)
                    }) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 346_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ptr == s.data() + len" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !(flt == val) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 347_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"flt == val" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            i.prefix_inc();
        }
    }
}
pub fn test_max_mantissa_19() {
    {
        println!(
            "Testing {}-bit float, using {}-bit integer",
            ((unimplemented!() + ({ log2_18(unimplemented!(),) })) + 1_()),
            (::std::mem::size_of::<()>() as ()).wrapping_mul(8_())
        );
        let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut flt: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 10_()) {
            let mut val: () =
                (((1_() << unimplemented!()) as ()).wrapping_sub(1_()) as ()).wrapping_sub(i);
            let mut s: () = unimplemented!();
            let mut len: () = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (fmt.read());
                (*res.borrow_mut()) = unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ (*res.borrow()).ec } == unimplemented!()) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 324_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ec == std::errc{}" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ { (*res.borrow()).ptr.clone() } } == {
                        unimplemented!().offset((len) as isize)
                    }) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 325_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ptr == s.data() + len" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !(flt == val) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 326_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"flt == val" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            unimplemented!();
            let mut orig_len: () = len;
            unimplemented!();
            len = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (fmt.read());
                (*res.borrow_mut()) = unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ (*res.borrow()).ec } == unimplemented!()) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 337_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ec == std::errc{}" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ { (*res.borrow()).ptr.clone() } } == {
                        unimplemented!().offset((len) as isize)
                    }) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 338_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ptr == s.data() + len" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !(flt == val) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 339_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"flt == val" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut s2: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                (*res.borrow_mut()) = unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ (*res.borrow()).ec } == unimplemented!()) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 345_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ec == std::errc{}" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ { (*res.borrow()).ptr.clone() } } == {
                        unimplemented!().offset((len) as isize)
                    }) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 346_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ptr == s.data() + len" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !(flt == val) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 347_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"flt == val" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            i.prefix_inc();
        }
    }
}
pub fn test_max_mantissa_20() {
    {
        println!(
            "Testing {}-bit float, using {}-bit integer",
            ((unimplemented!() + ({ log2_18(unimplemented!(),) })) + 1_()),
            (::std::mem::size_of::<()>() as ()).wrapping_mul(8_())
        );
        let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut flt: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 10_()) {
            let mut val: () =
                (((1_() << unimplemented!()) as ()).wrapping_sub(1_()) as ()).wrapping_sub(i);
            let mut s: () = ({ to_string_15(val) });
            let mut len: () = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (fmt.read());
                (*res.borrow_mut()) = unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ (*res.borrow()).ec } == unimplemented!()) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 324_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ec == std::errc{}" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ { (*res.borrow()).ptr.clone() } } == {
                        unimplemented!().offset((len) as isize)
                    }) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 325_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ptr == s.data() + len" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !(flt == val) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 326_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"flt == val" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            unimplemented!();
            let mut orig_len: () = len;
            unimplemented!();
            len = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (fmt.read());
                (*res.borrow_mut()) = unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ (*res.borrow()).ec } == unimplemented!()) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 337_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ec == std::errc{}" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ { (*res.borrow()).ptr.clone() } } == {
                        unimplemented!().offset((len) as isize)
                    }) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 338_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ptr == s.data() + len" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !(flt == val) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 339_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"flt == val" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut s2: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                (*res.borrow_mut()) = unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ (*res.borrow()).ec } == unimplemented!()) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 345_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ec == std::errc{}" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !({ { (*res.borrow()).ptr.clone() } } == {
                        unimplemented!().offset((len) as isize)
                    }) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 346_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"res.ptr == s.data() + len" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !(flt == val) {
                        ({
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"4.cc" )  ) .into() , ( 347_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"flt == val" )  ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            i.prefix_inc();
        }
    }
}
pub fn test06_21() {
    ({ test_max_mantissa_17() });
    ({ test_max_mantissa_19() });
    ({ test_max_mantissa_20() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_10() });
    ({ test02_12() });
    ({ test03_13() });
    ({ test04_14() });
    ({ test05_16() });
    ({ test06_21() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write((({ getpid_22() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_23(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_24(
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
pub fn set_memory_limits_25(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_26((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_27((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_26((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_27((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_26((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_27((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_28(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_26((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_27((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_29(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_30(
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
pub fn run_tests_wrapped_locale_31(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                    __builtin_fprintf_11 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
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
pub fn run_tests_wrapped_env_32(
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
pub fn test_tm_34(
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
    (*tmp_35.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_35.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_35.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_35.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_35.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_35.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_35.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_35.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_35.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_35.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_35: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_22() })) {
            ({
                semctl_24(
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
            semop_33(
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
            semop_33(
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
