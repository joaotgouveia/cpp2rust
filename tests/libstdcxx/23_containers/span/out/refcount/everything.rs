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
    _M_size: i32,
    #[offset(8)]
    #[byte_size(120)]
    _M_tests: Box<[FnPtr<fn()>]>,
}
impl __gnu_test_func_callback {
    pub fn new() -> Self {
        Self {
            _M_size: 0,
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
    pub fn count() -> u32 {
        return count__1.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__1.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__1.with(|rc| *rc.borrow()) == throw_on__2.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    pub fn reset() {
        count__1.with(|rc| *rc.borrow_mut() = 0_u32);
        throw_on__2.with(|rc| *rc.borrow_mut() = 0_u32);
    }
    pub fn throw_on(mut count: u32) {
        throw_on__2.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub fn count() -> u32 {
        return count__3.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__3.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__3.with(|rc| *rc.borrow()) == throw_on__4.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    pub fn reset() {
        count__3.with(|rc| *rc.borrow_mut() = 0_u32);
        throw_on__4.with(|rc| *rc.borrow_mut() = 0_u32);
    }
    pub fn throw_on(mut count: u32) {
        throw_on__4.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub fn count() -> u32 {
        return _M_count_5.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*_M_count_5.with(Value::clone).borrow_mut()).postfix_inc();
    }
    pub fn reset() {
        _M_count_5.with(|rc| *rc.borrow_mut() = 0_u32);
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct __gnu_test_copy_tracker {
    #[offset(0)]
    id_: i32,
    #[offset(4)]
    throw_on_copy_: bool,
}
impl __gnu_test_copy_tracker {
    pub fn new(id: Option<i32>, throw_on_copy: Option<bool>) -> Self {
        let mut id: i32 =
            id.unwrap_or_else(|| (*next_id__6.with(Value::clone).borrow_mut()).postfix_dec());
        let mut throw_on_copy: bool = throw_on_copy.unwrap_or_else(|| false);
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
                    ({ __gnu_test_copy_constructor::count() }).wrapping_add(1_u32),
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
pub fn operator_eq_8(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> bool {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } == {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
pub fn operator_lt_7(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> bool {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } < {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_semaphore {
    #[offset(0)]
    sem_set_: i32,
    #[offset(4)]
    pid_: (),
}
impl __gnu_test_semaphore {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
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
    let mut really_empty0: bool = unimplemented!();
    let mut really_empty1: bool = unimplemented!();
    let mut really_empty2: bool = (unimplemented!()).is_null();
    let mut really_empty3: bool = unimplemented!() == unimplemented!();
    let mut really_empty: bool =
        (((really_empty0) && (really_empty1)) && (really_empty2)) && (really_empty3);
    &(really_empty);
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(really_empty) {
            unimplemented!();
            std::process::abort();
        }
    }
    let value: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([0])));
    let muh_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 1_()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut original_bytes: Ptr<u8> = value.decay().reinterpret_cast::<u8>();
    elem!(original_bytes, 0).write(((1) as u8));
    elem!(original_bytes, 1).write(((2) as u8));
    elem!(original_bytes, 2).write(((3) as u8));
    elem!(original_bytes, 3).write(((4) as u8));
    let muh_byte_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let muh_mutable_byte_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let muh_original_byte_span: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut definitely_reinterpret_casted0: bool = unimplemented!();
    let mut definitely_reinterpret_casted1: bool = unimplemented!();
    let mut definitely_reinterpret_casted: bool =
        (definitely_reinterpret_casted0) && (definitely_reinterpret_casted1);
    &(definitely_reinterpret_casted);
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(definitely_reinterpret_casted) {
            unimplemented!();
            std::process::abort();
        }
    }
    let muh_original_byte_span_ptr_size: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut definitely_equivalent: bool = unimplemented!();
    &(definitely_equivalent);
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(definitely_equivalent) {
            unimplemented!();
            std::process::abort();
        }
    }
    return if ((definitely_equivalent) && (definitely_reinterpret_casted)) && (really_empty) {
        0
    } else {
        1
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(256)]
pub struct strawman_9 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
    #[offset(8)]
    pub z: bool,
    #[offset(12)]
    pub w: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct naked_span_10 {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<i8>,
    #[offset(8)]
    pub n: usize,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct strawman_span_11 {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<strawman_9>,
    #[offset(8)]
    pub n: usize,
}
thread_local!(
    static arr_data_12: Value<Vec<i32>> =
        Rc::new(RefCell::new(Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8])));
);
thread_local!(
    static data_13: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5, 6, 7, 8])));
);
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: 0_i32,
            pid_: unimplemented!(),
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write(unimplemented!());
        field!(this, sem_set_).write(unimplemented!());
        if (this.with(|__s| __s.sem_set_) == -1_i32) {
            unimplemented!();
        }
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0);
        if (unimplemented!() == -1_i32) {
            unimplemented!();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn set_memory_limits_14(size: Option<f32>) {
    let mut size: f32 = size.unwrap_or_else(|| (16_f64 as f32));
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size * 1048576_f32) as ());
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
pub fn set_file_limit_15(mut size: u64) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_16(mut mangled: Ptr<i8>, mut wanted: Ptr<i8>) {
    let status: Value<i32> = Rc::new(RefCell::new(0));
    let mut s: Ptr<i8> = Ptr::<i8>::null();
    let mut demangled: Ptr<i8> = unimplemented!();
    if !(demangled).is_null() {
        s = (demangled).clone();
    } else {
        switch!(match (*status.borrow()) {
            __v if __v == 0 => {
                s = Ptr::<i8>::from_string_literal(b"error code = 0: success");
                break;
            }
            __v if __v == -1_i32 => {
                s = Ptr::<i8>::from_string_literal(b"error code = -1: memory allocation failure");
                break;
            }
            __v if __v == -2_i32 => {
                s = Ptr::<i8>::from_string_literal(b"error code = -2: invalid mangled name");
                break;
            }
            __v if __v == -3_i32 => {
                s = Ptr::<i8>::from_string_literal(b"error code = -3: invalid arguments");
                break;
            }
            _ => {
                s = Ptr::<i8>::from_string_literal(b"error code unknown - who knows what happened");
            }
        });
    }
    let w: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = wanted.to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    if !(s.with_c_str(|__s| (*w.borrow())[..(*w.borrow()).len().saturating_sub(1)] == *__s)) {
        unimplemented!();
    }
    libcc2rs::free_refcount((demangled).to_any());
}
pub fn run_tests_wrapped_locale_17(mut name: Ptr<i8>, l: Ptr<__gnu_test_func_callback>) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut res: Ptr<i8> = Ptr::<i8>::from_string_literal(b"C");
    if !(res).is_null() {
        let mut preLC_ALL: Vec<i8> = {
            let mut __bytes = res.to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: i32 = 0;
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        let mut postLC_ALL: Vec<i8> = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"C").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                std::process::abort();
            }
        }
    } else {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"LC_ALL for ").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        unimplemented!();
        unimplemented!();
    }
}
pub fn run_tests_wrapped_env_18(
    mut name: Ptr<i8>,
    mut env: Ptr<i8>,
    l: Ptr<__gnu_test_func_callback>,
) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut oldENV: Ptr<i8> = match ::std::env::var(env.to_rust_string()) {
        Ok(__val) => Ptr::alloc_c_str(__val.as_bytes()),
        Err(_) => Ptr::null(),
    };
    if !(libc::setenv(env, name, 1) != 0) {
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: i32 = 0;
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        libc::setenv(
            env,
            if !(oldENV).is_null() {
                (oldENV).clone()
            } else {
                Ptr::<i8>::from_string_literal(b"")
            },
            1,
        );
    } else {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = env.to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static count__1: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static throw_on__2: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static count__3: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static throw_on__4: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static _M_count_5: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static next_id__6: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct __gnu_test_semun {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl __gnu_test_semun {
    pub fn val(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn buf(this: Ptr<Self>) -> Ptr<Ptr<()>> {
        this.reinterpret_cast()
    }
    pub fn array(this: Ptr<Self>) -> Ptr<Ptr<u16>> {
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
    mut sec: i32,
    mut min: i32,
    mut hour: i32,
    mut mday: i32,
    mut mon: i32,
    mut year: i32,
    mut wday: i32,
    mut yday: i32,
    mut isdst: i32,
) -> libcc2rs::Tm {
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
    static tmp_20: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
);
pub trait __gnu_test_copy_trackerImpl {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker>;
    fn destructor(&self);
    fn id(&self) -> i32;
}
impl __gnu_test_copy_trackerImpl for Ptr<__gnu_test_copy_tracker> {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker> {
        field!((*self), id_).write(({ __gnu_test_copy_trackerImpl::id(&rhs) }));
        if rhs.with(|__s| __s.throw_on_copy_) {
            ({
                __gnu_test_assignment_operator::throw_on(
                    ({ __gnu_test_assignment_operator::count() }).wrapping_add(1_u32),
                )
            });
        }
        ({ __gnu_test_assignment_operator::mark_call() });
        return (*self).clone();
    }
    fn destructor(&self) {
        ({ __gnu_test_destructor::mark_call() });
    }
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait __gnu_test_func_callbackImpl {
    fn copy_assign(&self, _a0: Ptr<__gnu_test_func_callback>) -> Ptr<__gnu_test_func_callback> {
        unimplemented!()
    }
    fn size(&self) -> i32;
    fn tests(&self) -> Ptr<FnPtr<fn()>>;
    fn push_back(&self, test: FnPtr<fn()>);
}
impl __gnu_test_func_callbackImpl for Ptr<__gnu_test_func_callback> {
    fn size(&self) -> i32 {
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
        __gnu_test_semun::val(val.as_pointer()).write(0);
        if ((*self).with(|__s| __s.pid_) == unimplemented!()) {
            unimplemented!();
        }
    }
    fn signal(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_i32) {
            unimplemented!();
        }
    }
    fn wait(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_i32) {
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
