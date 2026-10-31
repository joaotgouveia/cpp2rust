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
thread_local!(
    pub static valid_size_9: Value<i32> = Rc::new(RefCell::new(3));
);
thread_local!(
    pub static out_of_bound_10: Value<i32> =
        Rc::new(RefCell::new((valid_size_9.with(|rc| *rc.borrow()) + 1)));
);
thread_local!(
    pub static data_11: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, -999_i32])));
);
thread_local!(
    pub static advance_error_12: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static address_error_13: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_add_14(i: Iter, mut n: i32) -> Iter {
    let i: Value<Iter> = Rc::new(RefCell::new(i));
    return (*({ IterImpl::operator_add_assign(&i.as_pointer(), n) })
        .upgrade()
        .deref())
    .clone();
}
pub fn operator_add_15(mut n: i32, mut i: Iter) -> Iter {
    return ({
        let _i: Iter = i;
        operator_add_14(_i, n)
    });
}
pub fn operator_sub_16(mut i: Iter, mut n: i32) -> Iter {
    return ({
        let _i: Iter = i;
        operator_add_14(_i, -n)
    });
}
pub fn operator_sub_17(mut i: Iter, mut j: Iter) -> i32 {
    return (i.index - j.index);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Iter {
    #[offset(0)]
    pub index: i32,
}
impl std::cmp::Ord for Iter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_iterator_ptrs(self, other, |__lhs, __rhs| {
                IterImpl::operator_cmp(__lhs, __rhs.clone())
            })
        }
    }
}
impl std::cmp::PartialOrd for Iter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Iter {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_iterator_ptrs(self, other, |__lhs, __rhs| {
                IterImpl::operator_eq(__lhs, __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for Iter {}
impl CppAdvance for Iter {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_4();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_5();
    }
}
impl CppIterator<i32> for Iter {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_iterator_ptr(&mut self.clone(), |__it| IterImpl::operator_deref(__it))
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_iterator_ptr(self, |__it| {
            IterImpl::operator_inc_4(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for Iter {}
impl CppBidirectionalIterator<i32> for Iter {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_iterator_ptr(self, |__it| {
            IterImpl::operator_dec_5(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<i32> for Iter {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_iterator_ptr(self, |__it| {
            IterImpl::operator_add_assign(__it, n as _);
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_iterator_ptrs(self, from, |__lhs, __rhs| {
            (operator_sub_17(__lhs.read(), __rhs.read())) as isize
        })
    }
}
impl Default for Iter {
    fn default() -> Self {
        { Iter { index: 0_i32 } }
    }
}
const _: () = assert!(true, "std::contiguous_iterator<Iter>");
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i: Value<Box<[i32]>> = Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        let mut __first = Iter { index: 0 };
        let __last = Iter {
            index: out_of_bound_10.with(|rc| *rc.borrow()),
        };
        let mut __out = (i.as_pointer() as Ptr<i32>);
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(advance_error_12.with(|rc| *rc.borrow())) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(address_error_13.with(|rc| *rc.borrow())) {
            unimplemented!();
            std::process::abort();
        }
    }
    advance_error_12.with(|rc| {
        *rc.borrow_mut() = {
            address_error_13.with(|rc| *rc.borrow_mut() = false);
            address_error_13.with(|rc| *rc.borrow())
        }
    });
    {
        let mut __first = unimplemented!();
        let __last = (i.as_pointer() as Ptr<i32>).to_end();
        let mut __out = Iter { index: 0 };
        while __first != __last {
            __out.iter_deref().write((__first.iter_deref().read()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(advance_error_12.with(|rc| *rc.borrow())) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(address_error_13.with(|rc| *rc.borrow())) {
            unimplemented!();
            std::process::abort();
        }
    }
    0
}
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
pub fn set_memory_limits_18(size: Option<f32>) {
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
pub fn set_file_limit_19(mut size: u64) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_20(mut mangled: Ptr<i8>, mut wanted: Ptr<i8>) {
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
pub fn run_tests_wrapped_locale_21(mut name: Ptr<i8>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_22(
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
    if !({
        let __name = env.to_rust_string();
        if 1 != 0 || ::std::env::var_os(&__name).is_none() {
            unsafe { ::std::env::set_var(__name, name.to_rust_string()) };
        }
        0
    } != 0)
    {
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: i32 = 0;
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        {
            let __name = env.to_rust_string();
            if 1 != 0 || ::std::env::var_os(&__name).is_none() {
                unsafe {
                    ::std::env::set_var(
                        __name,
                        if !(oldENV).is_null() {
                            (oldENV).clone()
                        } else {
                            Ptr::<i8>::from_string_literal(b"")
                        }
                        .to_rust_string(),
                    )
                };
            }
            0
        };
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
pub fn test_tm_23(
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
    (*tmp_24.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_24.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_24.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_24.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_24.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_24.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_24.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_24.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_24.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_24.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_24: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
);
pub trait IterImpl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32>;
    fn operator_index(&self, n: i32) -> Ptr<i32>;
    fn operator_inc_4(&self) -> Ptr<Iter>;
    fn operator_dec_5(&self) -> Ptr<Iter>;
    fn operator_post_inc_6(&self, _a0: i32) -> Iter;
    fn operator_post_dec_7(&self, _a0: i32) -> Iter;
    fn operator_eq(&self, _a0: Ptr<Iter>) -> bool;
    fn operator_cmp(&self, _a0: Ptr<Iter>) -> std::cmp::Ordering;
    fn operator_add_assign(&self, n: i32) -> Ptr<Iter>;
    fn operator_sub_assign(&self, n: i32) -> Ptr<Iter>;
}
impl IterImpl for Ptr<Iter> {
    fn operator_deref(&self) -> Ptr<i32> {
        std::process::abort();
        panic!("ub: non-void function does not return a value")
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        if ((*self).with(|__s| __s.index) < 0)
            || ((*self).with(|__s| __s.index) > valid_size_9.with(|rc| *rc.borrow()))
        {
            address_error_13.with(|rc| *rc.borrow_mut() = true);
            return (data_11.with(|v| v.as_pointer()) as Ptr<i32>)
                .offset((valid_size_9.with(|rc| *rc.borrow())) as isize);
        }
        return (data_11.with(|v| v.as_pointer()) as Ptr<i32>)
            .offset(((*self).with(|__s| __s.index)) as isize);
    }
    fn operator_index(&self, mut n: i32) -> Ptr<i32> {
        return ({
            IterImpl::operator_deref(
                &Rc::new(RefCell::new(
                    ({
                        let _i: Iter = (*(*self).upgrade().deref()).clone();
                        operator_add_14(_i, n)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn operator_inc_4(&self) -> Ptr<Iter> {
        return ({ IterImpl::operator_add_assign(&(*self), 1) });
    }
    fn operator_dec_5(&self) -> Ptr<Iter> {
        return ({ IterImpl::operator_sub_assign(&(*self), 1) });
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> Iter {
        return Iter {
            index: field!((*self), index).with_mut(|__v| __v.postfix_inc()),
        };
    }
    fn operator_post_dec_7(&self, mut _a0: i32) -> Iter {
        return Iter {
            index: field!((*self), index).with_mut(|__v| __v.postfix_dec()),
        };
    }
    fn operator_eq(&self, _a0: Ptr<Iter>) -> bool {
        return ({ (*self).with(|__s| __s.index) } == { _a0.with(|__s| __s.index) });
    }
    fn operator_cmp(&self, _a0: Ptr<Iter>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.index)),
                &(_a0.with(|__s| __s.index)),
            )));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow());
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_add_assign(&self, mut n: i32) -> Ptr<Iter> {
        {
            let __rhs = n;
            field!((*self), index).with_mut(|__v| *__v = *__v + __rhs)
        };
        if ((*self).with(|__s| __s.index) < 0)
            || ((*self).with(|__s| __s.index) > valid_size_9.with(|rc| *rc.borrow()))
        {
            advance_error_12.with(|rc| *rc.borrow_mut() = true);
        }
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: i32) -> Ptr<Iter> {
        return ({ IterImpl::operator_add_assign(&(*self), -n) });
    }
}
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
    let _ = valid_size_9.with(|_| ());
    let _ = out_of_bound_10.with(|_| ());
    let _ = data_11.with(|_| ());
    let _ = advance_error_12.with(|_| ());
    let _ = address_error_13.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
