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
pub fn eq_10(x: Ptr<()>, y: Ptr<()>) -> () {
    let mut nan_reals: () = (unimplemented!()) && (unimplemented!());
    let mut nan_imags: () = (unimplemented!()) && (unimplemented!());
    let mut sign_reals: () = ({ ({ copysign_11(1_(), unimplemented!()) }) } == {
        ({ copysign_11(1_(), unimplemented!()) })
    });
    let mut sign_imags: () = ({ ({ copysign_11(1_(), unimplemented!()) }) } == {
        ({ copysign_11(1_(), unimplemented!()) })
    });
    return ((({ unimplemented!() } == { unimplemented!() }) && (sign_reals)) || (nan_reals))
        && ((({ unimplemented!() } == { unimplemented!() }) && (sign_imags)) || (nan_imags));
}
pub fn eq_12(x: Ptr<()>, y: Ptr<()>) -> () {
    let mut nan_reals: () = (unimplemented!()) && (unimplemented!());
    let mut nan_imags: () = (unimplemented!()) && (unimplemented!());
    let mut sign_reals: () = ({ unimplemented!() } == { unimplemented!() });
    let mut sign_imags: () = ({ unimplemented!() } == { unimplemented!() });
    return ((({ unimplemented!() } == { unimplemented!() }) && (sign_reals)) || (nan_reals))
        && ((({ unimplemented!() } == { unimplemented!() }) && (sign_imags)) || (nan_imags));
}
pub fn eq_13(x: Ptr<()>, y: Ptr<()>) -> () {
    let mut nan_reals: () = (unimplemented!()) && (unimplemented!());
    let mut nan_imags: () = (unimplemented!()) && (unimplemented!());
    let mut sign_reals: () = ({ unimplemented!() } == { unimplemented!() });
    let mut sign_imags: () = ({ unimplemented!() } == { unimplemented!() });
    return ((({ unimplemented!() } == { unimplemented!() }) && (sign_reals)) || (nan_reals))
        && ((({ unimplemented!() } == { unimplemented!() }) && (sign_imags)) || (nan_imags));
}
pub fn test01_14() {
    let mut qnan: () = unimplemented!();
    let mut pinf: () = unimplemented!();
    let mut ninf: () = -pinf;
    let c00: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c00.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (63_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c00) , c00 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (64_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c00) , -c00 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c00.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (66_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c00) , c00 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (67_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c00) , -c00 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c01: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c01.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (70_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c01) , c01 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (71_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c01) , -c01 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c01.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (73_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c01) , c01 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (74_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c01) , -c01 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c10.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (77_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c10) , c10 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (78_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c10) , -c10 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c12: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c12.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (81_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c12) , c12 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (82_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c12) , -c12 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c0q.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (85_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c0q) , c0q )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (86_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c0q) , -c0q )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c1q.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (89_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c1q) , c1q )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (90_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c1q) , -c1q )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = cq0.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (93_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(cq0) , cq0 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (94_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-cq0) , -cq0 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = cq1.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (97_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(cq1) , cq1 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (98_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-cq1) , -cq1 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = cqq.as_pointer();
            eq_10(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (101_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(cqq) , cqq )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (102_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-cqq) , -cqq )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (105_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c0p) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (106_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c0p) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (109_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c1p) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (110_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c1p) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (113_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cqp) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (114_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cqp) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (117_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cpp) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (118_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cpp) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (121_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c0n) , std::complex<double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (122_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c0n) , std::complex<double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (125_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c1n) , std::complex<double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (126_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c1n) , std::complex<double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (129_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cqn) , std::complex<double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (130_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cqn) , std::complex<double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (133_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cpn) , std::complex<double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (134_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cpn) , std::complex<double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (137_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cnn) , std::complex<double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (138_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cnn) , std::complex<double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (141_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cp0) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (142_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cp0) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (145_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cp1) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (146_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cp1) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (149_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cpq) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (150_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cpq) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (153_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cn0) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (154_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cn0) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (157_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cn1) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (158_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cn1) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (161_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cnq) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (162_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cnq) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (165_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cnp) , std::complex<double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_10(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (166_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cnp) , std::complex<double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn test02_16() {
    let mut qnan: () = unimplemented!();
    let mut pinf: () = unimplemented!();
    let mut ninf: () = -pinf;
    let c00: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c00.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (177_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c00) , c00 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (178_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c00) , -c00 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c00.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (180_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c00) , c00 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (181_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c00) , -c00 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c01: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c01.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (184_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c01) , c01 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (185_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c01) , -c01 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c01.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (187_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c01) , c01 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (188_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c01) , -c01 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c10.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (191_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c10) , c10 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (192_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c10) , -c10 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c12: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c12.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (195_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c12) , c12 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (196_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c12) , -c12 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c0q.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (199_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c0q) , c0q )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (200_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c0q) , -c0q )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c1q.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (203_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c1q) , c1q )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (204_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c1q) , -c1q )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = cq0.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (207_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(cq0) , cq0 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (208_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-cq0) , -cq0 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = cq1.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (211_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(cq1) , cq1 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (212_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-cq1) , -cq1 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = cqq.as_pointer();
            eq_12(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (215_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(cqq) , cqq )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (216_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-cqq) , -cqq )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (219_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c0p) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (220_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c0p) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (223_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c1p) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (224_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c1p) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (227_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cqp) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (228_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cqp) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (231_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cpp) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (232_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cpp) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (235_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c0n) , std::complex<float>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (236_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c0n) , std::complex<float>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (239_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c1n) , std::complex<float>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (240_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c1n) , std::complex<float>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (243_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cqn) , std::complex<float>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (244_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cqn) , std::complex<float>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (247_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cpn) , std::complex<float>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (248_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cpn) , std::complex<float>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (251_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cnn) , std::complex<float>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (252_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cnn) , std::complex<float>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (255_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cp0) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (256_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cp0) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (259_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cp1) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (260_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cp1) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (263_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cpq) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (264_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cpq) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (267_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cn0) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (268_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cn0) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (271_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cn1) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (272_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cn1) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (275_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cnq) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (276_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cnq) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (279_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cnp) , std::complex<float>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_12(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (280_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test02()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cnp) , std::complex<float>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn test03_17() {
    let mut qnan: () = unimplemented!();
    let mut pinf: () = unimplemented!();
    let mut ninf: () = -pinf;
    let c00: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c00.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (291_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c00) , c00 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (292_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c00) , -c00 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c00.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (294_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c00) , c00 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (295_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c00) , -c00 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c01: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c01.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (298_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c01) , c01 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (299_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c01) , -c01 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c01.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (301_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c01) , c01 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (302_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c01) , -c01 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c10.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (305_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c10) , c10 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (306_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c10) , -c10 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c12: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c12.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (309_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c12) , c12 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (310_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c12) , -c12 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c0q.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (313_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c0q) , c0q )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (314_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c0q) , -c0q )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = c1q.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (317_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(c1q) , c1q )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (318_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-c1q) , -c1q )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = cq0.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (321_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(cq0) , cq0 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (322_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-cq0) , -cq0 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = cq1.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (325_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(cq1) , cq1 )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (326_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-cq1) , -cq1 )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Ptr<()> = cqq.as_pointer();
            eq_13(_x.as_pointer(), _y)
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (329_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(cqq) , cqq )")).into(),
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
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (330_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(b"eq( std::proj(-cqq) , -cqq )")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (333_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c0p) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (334_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c0p) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (337_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c1p) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (338_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c1p) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (341_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cqp) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (342_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cqp) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (345_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cpp) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (346_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cpp) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (349_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c0n) , std::complex<long double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (350_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c0n) , std::complex<long double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (353_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(c1n) , std::complex<long double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (354_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-c1n) , std::complex<long double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (357_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cqn) , std::complex<long double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (358_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cqn) , std::complex<long double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (361_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cpn) , std::complex<long double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (362_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cpn) , std::complex<long double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (365_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cnn) , std::complex<long double>(pinf, -0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (366_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cnn) , std::complex<long double>(pinf, +0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (369_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cp0) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (370_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cp0) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (373_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cp1) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (374_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cp1) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (377_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cpq) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (378_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cpq) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (381_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cn0) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (382_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cn0) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (385_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cn1) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (386_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cn1) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (389_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cnq) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (390_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cnq) , std::complex<long double>(pinf, -0.0) )",
                        ))
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (393_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(cnp) , std::complex<long double>(pinf, +0.0) )",
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
        if !({
            let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            eq_13(_x.as_pointer(), _y.as_pointer())
        }) {
            ({
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"proj.cc")).into(),
                        (394_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test03()")).into(),
                        (Ptr::<()>::from_string_literal(
                            b"eq( std::proj(-cnp) , std::complex<long double>(pinf, -0.0) )",
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
    ({ test01_14() });
    ({ test02_16() });
    ({ test03_17() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write((({ getpid_18() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_19(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_20(
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
pub fn set_memory_limits_21(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_22((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_23((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_22((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_23((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_22((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_23((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_24(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_22((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_23((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_25(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_26(
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
    let mut w: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { (wanted).clone() },
        None,
    );
    if unimplemented!() {
        ({ __throw_runtime_error_3((s).clone()) });
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_27(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut res: Ptr<()> = unimplemented!();
    if !(res).is_null() {
        let mut preLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (res).clone() },
                None,
            );
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: () = 0_();
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { unimplemented!() },
                None,
            );
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                ({
                    __builtin_fprintf_15 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { Ptr::<()>::from_string_literal(b"LC_ALL for ") },
            None,
        );
        unimplemented!();
        ({ __throw_runtime_error_3(unimplemented!()) });
    }
}
pub fn run_tests_wrapped_env_28(
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
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { (env).clone() },
            None,
        );
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
pub fn test_tm_30(
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
    (*tmp_31.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_31.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_31: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_18() })) {
            ({
                semctl_20(
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
            semop_29(
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
            semop_29(
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
