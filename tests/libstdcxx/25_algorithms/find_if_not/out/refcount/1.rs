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
impl CppMove for __gnu_test_object_counter {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for __gnu_test_copy_constructor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for __gnu_test_assignment_operator {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for __gnu_test_destructor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for __gnu_test_copy_tracker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for __gnu_test_semaphore {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<i32>,
}
impl __gnu_test_BoundsContainer_int_ {
    pub fn new(mut _first: Ptr<i32>, mut _last: Ptr<i32>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
impl CppMove for __gnu_test_BoundsContainer_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_input_iterator_wrapper_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_BoundsContainer_int_>,
}
impl __gnu_test_input_iterator_wrapper_int_ {
    pub fn new_1(
        mut _ptr: Ptr<i32>,
        mut SharedInfo_in: Ptr<__gnu_test_BoundsContainer_int_>,
    ) -> Self {
        let __this: Value<__gnu_test_input_iterator_wrapper_int_> = Rc::new(RefCell::new(Self {
            ptr: (_ptr).clone(),
            SharedInfo: (SharedInfo_in).clone(),
        }));
        let this: Ptr<__gnu_test_input_iterator_wrapper_int_> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ this.with(|__s| __s.ptr.clone()) } >= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ this.with(|__s| __s.ptr.clone()) } <= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.last.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for __gnu_test_input_iterator_wrapper_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                __gnu_test_input_iterator_wrapper_int_Impl::operator_eq(__lhs, __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for __gnu_test_input_iterator_wrapper_int_ {}
impl CppAdvance for __gnu_test_input_iterator_wrapper_int_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
impl CppIterator<i32> for __gnu_test_input_iterator_wrapper_int_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            __gnu_test_input_iterator_wrapper_int_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            __gnu_test_input_iterator_wrapper_int_Impl::operator_inc_9(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for __gnu_test_input_iterator_wrapper_int_ {}
impl CppMove for __gnu_test_input_iterator_wrapper_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(16)]
    pub bounds: __gnu_test_BoundsContainer_int_,
}
impl __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_ {
    pub fn new(mut _first: Ptr<i32>, mut _last: Ptr<i32>) -> Self {
        Self {
            bounds: __gnu_test_BoundsContainer_int_::new({ (_first).clone() }, { (_last).clone() }),
        }
    }
}
impl CppMove for __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static array_9: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 1, 0, 1])));
);
pub fn predicate_10(i: Ptr<i32>) -> bool {
    return ((i.read()) == 0);
}
pub fn test1_11() {
    let con: Value<__gnu_test_test_container_int____gnu_test_input_iterator_wrapper_> =
        Rc::new(RefCell::new(
            __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_::new(
                { (array_9.with(|v| v.as_pointer()) as Ptr<i32>) },
                { (array_9.with(|v| v.as_pointer()) as Ptr<i32>) },
            ),
        ));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { unimplemented!().ptr.clone() } } == {
            (array_9.with(|v| v.as_pointer()) as Ptr<i32>)
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test2_12() {
    let con: Value<__gnu_test_test_container_int____gnu_test_input_iterator_wrapper_> =
        Rc::new(RefCell::new(
            __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_::new(
                { (array_9.with(|v| v.as_pointer()) as Ptr<i32>) },
                { (array_9.with(|v| v.as_pointer()) as Ptr<i32>).offset((1) as isize) },
            ),
        ));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { unimplemented!().ptr.clone() } } == {
            (array_9.with(|v| v.as_pointer()) as Ptr<i32>).offset((1) as isize)
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test3_13() {
    let con: Value<__gnu_test_test_container_int____gnu_test_input_iterator_wrapper_> =
        Rc::new(RefCell::new(
            __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_::new(
                { (array_9.with(|v| v.as_pointer()) as Ptr<i32>) },
                { (array_9.with(|v| v.as_pointer()) as Ptr<i32>).offset((6) as isize) },
            ),
        ));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { unimplemented!().ptr.clone() } } == {
            (array_9.with(|v| v.as_pointer()) as Ptr<i32>).offset((3) as isize)
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test1_11() });
    ({ test2_12() });
    ({ test3_13() });
    return 0;
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
    if !(match 1 != 0 || ::std::env::var_os(env.to_rust_string()).is_none() {
        true => {
            unsafe { ::std::env::set_var(env.to_rust_string(), name.to_rust_string()) };
            0
        }
        false => 0,
    } != 0)
    {
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: i32 = 0;
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        match ({
            if !(oldENV).is_null() {
                (oldENV).clone()
            } else {
                Ptr::<i8>::from_string_literal(b"")
            }
        },)
        {
            (mut __tmp_0,) => match 1 != 0 || ::std::env::var_os(env.to_rust_string()).is_none() {
                true => {
                    unsafe { ::std::env::set_var(env.to_rust_string(), __tmp_0.to_rust_string()) };
                    0
                }
                false => 0,
            },
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
impl CppMove for __gnu_test_semun {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __gnu_test_input_iterator_wrapper_int__post_inc_proxy;
pub trait __gnu_test_BoundsContainer_int_Impl {
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_BoundsContainer_int_Impl for Ptr<__gnu_test_BoundsContainer_int_> {}
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
        field!((*self), _M_size).with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub trait __gnu_test_input_iterator_wrapper_int_Impl {
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_int_>) -> bool;
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_int_>) -> bool;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_int_>;
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
    ) -> __gnu_test_input_iterator_wrapper_int__post_inc_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_int_Impl for Ptr<__gnu_test_input_iterator_wrapper_int_> {
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_int_>) -> bool {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.SharedInfo.clone()) } == {
                    in_.with(|__s| __s.SharedInfo.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ in_.with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        return ({ (*self).with(|__s| __s.ptr.clone()) } == { in_.with(|__s| __s.ptr.clone()) });
    }
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_int_>) -> bool {
        return !({
            let _in_: Ptr<__gnu_test_input_iterator_wrapper_int_> = (in_).clone();
            __gnu_test_input_iterator_wrapper_int_Impl::operator_eq(&(*self), _in_)
        });
    }
    fn operator_deref(&self) -> Ptr<i32> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        return (*self).with(|__s| __s.ptr.clone());
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_int_> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!((*self), ptr).with_mut(|__v| __v.postfix_inc());
        field!((*self).with(|__s| __s.SharedInfo.clone()), first)
            .write((*self).with(|__s| __s.ptr.clone()));
        return (*self).clone();
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
pub trait __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_Impl {
    fn it_3(&self, mut pos: i32) -> __gnu_test_input_iterator_wrapper_int_ {
        unimplemented!()
    }
    fn it_4(&self, pos: Ptr<i32>) -> __gnu_test_input_iterator_wrapper_int_;
    fn val(&self, mut pos: i32) -> Ptr<i32> {
        unimplemented!()
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_int_;
    fn end(&self) -> __gnu_test_input_iterator_wrapper_int_;
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_container_int____gnu_test_input_iterator_wrapper_>
{
    fn it_4(&self, mut pos: Ptr<i32>) -> __gnu_test_input_iterator_wrapper_int_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (pos).clone() } >= { (*self).with(|__s| __s.bounds.first.clone()) })
                && ({ (pos).clone() } <= { (*self).with(|__s| __s.bounds.last.clone()) }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        return __gnu_test_input_iterator_wrapper_int_::new_1({ (pos).clone() }, {
            (field_ptr!((*self), bounds))
        });
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_int_ {
        return ({
            let _pos: Ptr<i32> = (*self).with(|__s| __s.bounds.first.clone());
            __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_Impl::it_4(self, _pos)
        });
    }
    fn end(&self) -> __gnu_test_input_iterator_wrapper_int_ {
        return ({
            let _pos: Ptr<i32> = (*self).with(|__s| __s.bounds.last.clone());
            __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_Impl::it_4(self, _pos)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = array_9.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
