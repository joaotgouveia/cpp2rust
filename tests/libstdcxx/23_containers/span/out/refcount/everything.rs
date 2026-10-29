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
            unimplemented!();
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
        return count__3.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__3.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__3.with(|rc| *rc.borrow()) == throw_on__4.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    pub fn reset() {
        count__3.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__4.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__4.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub fn count() -> () {
        return _M_count_5.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*_M_count_5.with(Value::clone).borrow_mut()).postfix_inc();
    }
    pub fn reset() {
        _M_count_5.with(|rc| *rc.borrow_mut() = 0_());
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
            id.unwrap_or_else(|| (*next_id__6.with(Value::clone).borrow_mut()).postfix_dec());
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
impl CppLess for __gnu_test_copy_tracker {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for __gnu_test_copy_tracker {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
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
pub fn operator_eq_8(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } == {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
pub fn operator_lt_7(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
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
    pid_: (),
}
impl __gnu_test_semaphore {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    const _: () = assert!(
        (8usize <= 8usize),
        "sizeof(std::span<char, 0>) <= sizeof(char*)"
    );;
    const _: () = assert!(
        (8usize <= 8usize),
        "sizeof(std::span<const char, 0>) <= sizeof(const char*)"
    );;
    const _: () = assert!(
        (8usize <= 8usize),
        "sizeof(std::span<strawman, 0>) <= sizeof(strawman*)"
    );;
    const _: () = assert!(
        (8usize <= 8usize),
        "sizeof(std::span<strawman, 1>) <= sizeof(strawman*)"
    );;
    const _: () = assert!(
        (16usize <= 16usize),
        "sizeof(std::span<char>) <= sizeof(naked_span)"
    );;
    const _: () = assert!(
        (16usize <= 16usize),
        "sizeof(std::span<strawman>) <= sizeof(strawman_span)"
    );;
    let arr_data_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "arr_data_span.size() == 9");;
    const _: () = assert!(true, "arr_data_span.size_bytes() == 9 * sizeof(int)");;
    const _: () = assert!(true, "*arr_data_span.begin() == 0");;
    const _: () = assert!(true, "*arr_data_span.data() == 0");;
    const _: () = assert!(true, "arr_data_span.front() == 0");;
    const _: () = assert!(true, "arr_data_span.back() == 8");;
    const _: () = assert!(true, "arr_data_span[0] == 0");;
    const _: () = assert!(true, "arr_data_span[1] == 1");;
    const _: () = assert!(true, "arr_data_span[2] == 2");;
    const _: () = assert!(true, "arr_data_span[3] == 3");;
    const _: () = assert!(true, "arr_data_span[4] == 4");;
    const _: () = assert!(true, "arr_data_span[5] == 5");;
    const _: () = assert!(true, "arr_data_span[6] == 6");;
    const _: () = assert!(true, "arr_data_span[7] == 7");;
    const _: () = assert!(true, "arr_data_span[8] == 8");;
    const _: () = assert!(true, "!arr_data_span.empty()");;
    const _: () = assert!(true, "decltype(arr_data_span)::extent == 9");;
    let data_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "data_span.size() == 9");;
    const _: () = assert!(true, "data_span.size_bytes() == 9 * sizeof(int)");;
    const _: () = assert!(true, "*data_span.begin() == 0");;
    const _: () = assert!(true, "*data_span.data() == 0");;
    const _: () = assert!(true, "data_span.front() == 0");;
    const _: () = assert!(true, "data_span.back() == 8");;
    const _: () = assert!(true, "data_span[0] == 0");;
    const _: () = assert!(true, "data_span[1] == 1");;
    const _: () = assert!(true, "data_span[2] == 2");;
    const _: () = assert!(true, "data_span[3] == 3");;
    const _: () = assert!(true, "data_span[4] == 4");;
    const _: () = assert!(true, "data_span[5] == 5");;
    const _: () = assert!(true, "data_span[6] == 6");;
    const _: () = assert!(true, "data_span[7] == 7");;
    const _: () = assert!(true, "data_span[8] == 8");;
    const _: () = assert!(true, "!data_span.empty()");;
    const _: () = assert!(true, "decltype(data_span)::extent == 9");;
    let data_span_first: Value<()> = Rc::new(RefCell::new(({ (*data_span.borrow()).first() })));
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_first)>, std::span<const int, 3>>"
    );;
    const _: () = assert!(true, "decltype(data_span_first)::extent == 3");;
    const _: () = assert!(true, "data_span_first.size() == 3");;
    const _: () = assert!(true, "data_span_first.front() == 0");;
    const _: () = assert!(true, "data_span_first.back() == 2");;
    let data_span_first_dyn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_first_dyn)>, std::span<const int>>"
    );;
    const _: () = assert!(
        true,
        "decltype(data_span_first_dyn)::extent == std::dynamic_extent"
    );;
    const _: () = assert!(true, "data_span_first_dyn.size() == 4");;
    const _: () = assert!(true, "data_span_first_dyn.front() == 0");;
    const _: () = assert!(true, "data_span_first_dyn.back() == 3");;
    let data_span_last: Value<()> = Rc::new(RefCell::new(({ (*data_span.borrow()).last() })));
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_last)>, std::span<const int, 5>>"
    );;
    const _: () = assert!(true, "decltype(data_span_last)::extent == 5");;
    const _: () = assert!(true, "data_span_last.size() == 5");;
    const _: () = assert!(true, "data_span_last.front() == 4");;
    const _: () = assert!(true, "data_span_last.back() == 8");;
    let data_span_last_dyn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_last_dyn)>, std::span<const int>>"
    );;
    const _: () = assert!(
        true,
        "decltype(data_span_last_dyn)::extent == std::dynamic_extent"
    );;
    const _: () = assert!(true, "data_span_last_dyn.size() == 6");;
    const _: () = assert!(true, "data_span_last_dyn.front() == 3");;
    const _: () = assert!(true, "data_span_last_dyn.back() == 8");;
    let data_span_subspan: Value<()> = Rc::new(RefCell::new(({ (*data_span.borrow()).subspan() })));
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_subspan)>, std::span<const int, 3>>"
    );;
    const _: () = assert!(true, "decltype(data_span_subspan)::extent == 3");;
    const _: () = assert!(true, "data_span_subspan.size() == 3");;
    const _: () = assert!(true, "data_span_subspan.front() == 1");;
    const _: () = assert!(true, "data_span_subspan.back() == 3");;
    let data_span_subspan_offset: Value<()> =
        Rc::new(RefCell::new(({ (*data_span.borrow()).subspan() })));
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_subspan_offset)>, std::span<const int, 1>>"
    );;
    const _: () = assert!(true, "decltype(data_span_subspan_offset)::extent == 1");;
    const _: () = assert!(true, "data_span_subspan_offset.size() == 1");;
    const _: () = assert!(true, "data_span_subspan_offset.front() == 8");;
    const _: () = assert!(true, "data_span_subspan_offset.back() == 8");;
    let data_span_subspan_empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_subspan_empty)>, std::span<const int>>"
    );;
    const _: () = assert!(
        true,
        "decltype(data_span_subspan_empty)::extent == std::dynamic_extent"
    );;
    const _: () = assert!(true, "data_span_subspan_empty.size() == 0");;
    let data_span_subspan_empty_static: Value<()> =
        Rc::new(RefCell::new(({ (*data_span.borrow()).subspan() })));
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_subspan_empty_static)>,\n    std::span<const int, 0>>"
    );;
    const _: () = assert!(
        true,
        "decltype(data_span_subspan_empty_static)::extent == 0"
    );;
    const _: () = assert!(true, "data_span_subspan_empty.size() == 0");;
    let shorts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut really_empty0: () = unimplemented!();
    let mut really_empty1: () = unimplemented!();
    let mut really_empty2: () = (unimplemented!()).is_null();
    let mut really_empty3: () = unimplemented!();
    let mut really_empty: () =
        (((really_empty0) && (really_empty1)) && (really_empty2)) && (really_empty3);
    &(really_empty);
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(really_empty) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut value: () = unimplemented!();
    let muh_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 1_()) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut original_bytes: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    elem!(original_bytes, 0_()).write(((1_()) as ()));
    elem!(original_bytes, 1_()).write(((2_()) as ()));
    elem!(original_bytes, 2_()).write(((3_()) as ()));
    elem!(original_bytes, 3_()).write(((4_()) as ()));
    let muh_byte_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let muh_mutable_byte_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let muh_original_byte_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut definitely_reinterpret_casted0: () = unimplemented!();
    let mut definitely_reinterpret_casted1: () = unimplemented!();
    let mut definitely_reinterpret_casted: () =
        (definitely_reinterpret_casted0) && (definitely_reinterpret_casted1);
    &(definitely_reinterpret_casted);
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(definitely_reinterpret_casted) {
            unimplemented!();
            unimplemented!();
        }
    }
    let muh_original_byte_span_ptr_size: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut definitely_equivalent: () = unimplemented!();
    &(definitely_equivalent);
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(definitely_equivalent) {
            unimplemented!();
            unimplemented!();
        }
    }
    return if ((definitely_equivalent) && (definitely_reinterpret_casted)) && (really_empty) {
        0_()
    } else {
        1_()
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(256)]
pub struct strawman_9 {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
    #[offset(8)]
    pub z: (),
    #[offset(12)]
    pub w: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct naked_span_10 {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<()>,
    #[offset(8)]
    pub n: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct strawman_span_11 {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<strawman_9>,
    #[offset(8)]
    pub n: (),
}
thread_local!(
    static arr_data_12: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    static data_13: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
    ])));
);
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: unimplemented!(),
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write(unimplemented!());
        field!(this, sem_set_).write(unimplemented!());
        if (this.with(|__s| __s.sem_set_) == -1_()) {
            unimplemented!();
        }
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if (unimplemented!() == -1_()) {
            unimplemented!();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn set_memory_limits_14(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = (size * 1048576_());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn set_file_limit_15(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_16(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = unimplemented!();
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
        unimplemented!();
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_17(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn run_tests_wrapped_env_18(
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
        unimplemented!();
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
    static count__3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static _M_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static next_id__6: Value<()> = Rc::new(RefCell::new(0_()));
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
    pub fn buf(this: Ptr<Self>) -> Ptr<Ptr<()>> {
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
pub fn test_tm_19(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> () {
    (*tmp_20.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_20.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_20.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_20.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_20.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_20.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_20.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_20.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_20.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_20.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_20: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        if ((*self).with(|__s| __s.pid_) == unimplemented!()) {
            unimplemented!();
        }
    }
    fn signal(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_()) {
            unimplemented!();
        }
    }
    fn wait(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_()) {
            unimplemented!();
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
