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
const _: () = assert!(
    true,
    "__gnu_test::is_customization_point_object(std::ranges::crend)"
);
pub fn rbegin_9(r: Ptr<R1>) -> Ptr<()> {
    return (field_ptr!(r, j));
}
pub fn rend_10(r: Ptr<R1>) -> Ptr<()> {
    return (field_ptr!(r, j)).offset((1_()) as isize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct R1 {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub j: (),
}
impl Default for R1 {
    fn default() -> Self {
        { R1 { i: 0_(), j: 0_() } }
    }
}
thread_local!(
    pub static enable_borrowed_range_11: Value<()> = Rc::new(RefCell::new(true));
);
pub fn test01_12() {
    let r: Value<R1> = Rc::new(RefCell::new(<R1>::default()));
    let c: Ptr<R1> = r.as_pointer();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ ({ (*crend_13.with(Value::clone).borrow_mut())(r.as_pointer()) }) } == {
            ({
                let ___t: Ptr<R1> = (c).clone();
                (*rend_14.with(Value::clone).borrow_mut())(___t)
            })
            .clone()
        }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        {}
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ ({ (*crend_13.with(Value::clone).borrow_mut())(r.as_pointer()) }) } == {
            ({
                let ___t: Ptr<R1> = (c).clone();
                (*rend_14.with(Value::clone).borrow_mut())(___t)
            })
            .clone()
        }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        {}
    }
}
pub fn begin_15(r: Ptr<R2>) -> Ptr<()> {
    return (array_field_ptr!(r, l) as Ptr<()>);
}
pub fn end_16(r: Ptr<R2>) -> Ptr<()> {
    return (array_field_ptr!(r, l) as Ptr<()>).offset((2_()) as isize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct R2 {
    #[offset(0)]
    #[byte_size(8)]
    pub a: Box<[()]>,
    #[offset(8)]
    #[byte_size(16)]
    pub l: Box<[()]>,
}
impl Default for R2 {
    fn default() -> Self {
        {
            R2 {
                a: Box::new([unimplemented!(), unimplemented!()]),
                l: Box::new([unimplemented!(), unimplemented!()]),
            }
        }
    }
}
thread_local!(
    pub static enable_borrowed_range_17: Value<()> = Rc::new(RefCell::new(true));
);
pub fn test02_18() {
    let r: Value<R2> = Rc::new(RefCell::new(<R2>::default()));
    let c: Ptr<R2> = r.as_pointer();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn rbegin_19(_a0: Ptr<R3>) -> Ptr<()> {
    return Ptr::<()>::null();
}
pub fn rend_20(r: Ptr<R3>) -> Ptr<()> {
    return (field_ptr!(r, i));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct R3 {
    #[offset(0)]
    pub i: (),
}
impl Default for R3 {
    fn default() -> Self {
        { R3 { i: 0_() } }
    }
}
pub fn rbegin_21(_a0: Ptr<R4>) -> Ptr<()> {
    return Ptr::<()>::null();
}
pub fn rend_22(r: Ptr<R4>) -> Ptr<()> {
    return (field_ptr!(r, i));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct R4 {
    #[offset(0)]
    pub i: (),
}
impl Default for R4 {
    fn default() -> Self {
        { R4 { i: 0_() } }
    }
}
pub fn test03_23() {
    let r: Value<R3> = Rc::new(RefCell::new(<R3>::default()));
    let c: Ptr<R3> = r.as_pointer();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ ({ (*crend_13.with(Value::clone).borrow_mut())(r.as_pointer()) }) } == {
            ({
                let ___t: Ptr<R3> = (c).clone();
                (*rend_14.with(Value::clone).borrow_mut())(___t)
            })
            .clone()
        }) {
            unimplemented!();
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(std::ranges::crend(r))");;
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            ({
                let ___t: Ptr<R3> = (c).clone();
                (*crend_13.with(Value::clone).borrow_mut())(___t)
            })
            .clone()
        } == {
            ({
                let ___t: Ptr<R3> = (c).clone();
                (*rend_14.with(Value::clone).borrow_mut())(___t)
            })
            .clone()
        }) {
            unimplemented!();
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(std::ranges::crend(c))");;
    let r4: Value<R4> = Rc::new(RefCell::new(<R4>::default()));
    let c4: Ptr<R4> = r4.as_pointer();
    let mut b: Ptr<()> = ({ (*rbegin_24.with(Value::clone).borrow_mut())(r4.as_pointer()) });
    let mut s0: Ptr<()> = ({ (*rend_14.with(Value::clone).borrow_mut())(r4.as_pointer()) });
    const _: () = assert!(true, "std::ranges::__access::__adl_rend<R4&>");;
    let mut s: Ptr<()> = ({ (*crend_13.with(Value::clone).borrow_mut())(r4.as_pointer()) });
    let mut s2: Ptr<()> = ({
        let ___t: Ptr<R4> = (c4).clone();
        (*rend_14.with(Value::clone).borrow_mut())(___t)
    })
    .clone();
}
pub fn test04_25() {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!(), unimplemented!()])));
    let c: Ptr<()> = a.as_pointer();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        {}
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_12() });
    ({ test02_18() });
    ({ test03_23() });
    ({ test04_25() });
    0
}
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
pub fn set_memory_limits_26(size: Option<()>) {
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
pub fn set_file_limit_27(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_28(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_29(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_30(
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
pub fn test_tm_31(
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
    (*tmp_32.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_32.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_32: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub trait R1Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn rbegin(&self) -> Ptr<()>;
    fn rend(&self) -> Ptr<()>;
}
impl R1Impl for Ptr<R1> {
    fn rbegin(&self) -> Ptr<()> {
        return (field_ptr!((*self), i));
    }
    fn rend(&self) -> Ptr<()> {
        return (field_ptr!((*self), i)).offset((1_()) as isize);
    }
}
pub trait R2Impl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
}
impl R2Impl for Ptr<R2> {
    fn begin(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), a) as Ptr<()>);
    }
    fn end(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), a) as Ptr<()>).offset((2_()) as isize);
    }
}
pub trait R3Impl {
    fn begin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn rbegin(&self) -> Ptr<()>;
    fn rend(&self) -> Ptr<()>;
}
impl R3Impl for Ptr<R3> {
    fn rbegin(&self) -> Ptr<()> {
        return (field_ptr!((*self), i)).offset((1_()) as isize);
    }
    fn rend(&self) -> Ptr<()> {
        return Ptr::<()>::null();
    }
}
pub trait R4Impl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
    fn rbegin(&self) -> Ptr<()>;
    fn rend(&self) -> Ptr<()>;
}
impl R4Impl for Ptr<R4> {
    fn begin(&self) -> Ptr<()> {
        return Ptr::<()>::null();
    }
    fn end(&self) -> Ptr<()> {
        return Ptr::<()>::null();
    }
    fn rbegin(&self) -> Ptr<()> {
        return (field_ptr!((*self), i)).offset((1_()) as isize);
    }
    fn rend(&self) -> Ptr<()> {
        return Ptr::<()>::null();
    }
}
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
    let _ = enable_borrowed_range_11.with(|_| ());
    let _ = enable_borrowed_range_17.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
