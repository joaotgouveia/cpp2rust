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
    "__gnu_test::is_customization_point_object(std::ranges::iter_move)"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(4)]
pub struct X {
    #[offset(0)]
    pub value: (),
}
impl X {
    pub fn new(mut i: ()) -> Self {
        Self { value: i }
    }
    pub fn move_from(x: Ptr<X>) -> Self {
        let __this: X = Self {
            value: x.with(|__s| __s.value),
        };
        field!(x, value).write(-2_());
        __this
    }
}
pub fn test_X_9(mut i: (), mut j: ()) -> () {
    let x1: Value<X> = Rc::new(RefCell::new(X::new({ i })));
    let x2: Value<X> = Rc::new(RefCell::new(X::new({ j })));
    &({
        let ___e: Value<Ptr<X>> = Rc::new(RefCell::new((x1.as_pointer())));
        (*iter_move_10.with(Value::clone).borrow_mut())(___e.as_pointer())
    });
    ({
        XImpl::move_assign(
            &x1.as_pointer(),
            ({
                let ___e: Value<Ptr<X>> = Rc::new(RefCell::new((x2.as_pointer())));
                (*iter_move_10.with(Value::clone).borrow_mut())(___e.as_pointer())
            })
            .clone(),
        )
    });
    return ({ (*x1.borrow()).value } == j) && ({ (*x2.borrow()).value } == -1_());
}
const _: () = assert!(true, "test_X(1, 2)");
pub fn test01_11() {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        {}
    }
}
const _: () = assert!(true, "std::same_as<rval_ref<int*>, int&&>");
const _: () = assert!(true, "std::same_as<rval_ref<const int*>, const int&&>");
const _: () = assert!(
    true,
    "std::same_as<rval_ref<std::move_iterator<int*>>, int&&>"
);
pub fn iter_move_12(_a0: Ptr<Iter>) -> Ptr<()> {
    return i_13.with(|v| v.as_pointer());
}
pub fn iter_move_14(_a0: Ptr<Iter>) -> () {
    return 2_();
}
pub fn operator_deref_15(_a0: Ptr<Iter>) -> () {
    return 4_f32;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Iter {}
thread_local!(
    static i_13: Value<()> = Rc::new(RefCell::new(1_()));
);
thread_local!(
    static s_16: Value<()> = Rc::new(RefCell::new(3_()));
);
pub fn test_adl_17() {
    let it: Value<Iter> = Rc::new(RefCell::new(<Iter>::default()));
    let cit: Ptr<Iter> = it.as_pointer();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ (*iter_move_10.with(Value::clone).borrow_mut())(it.as_pointer()) }) == 1_()) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({ (*iter_move_10.with(Value::clone).borrow_mut())(it.as_pointer()) }) == 2_()) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___e: Ptr<Iter> = (cit).clone();
            (*iter_move_10.with(Value::clone).borrow_mut())(___e)
        }) == 3_())
        {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___e: Ptr<Iter> = (cit).clone();
            (*iter_move_10.with(Value::clone).borrow_mut())(___e)
        }) == 4_f32)
        {
            unimplemented!();
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::ranges::iter_move(it)), int&>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::ranges::iter_move(std::move(it))),\n\t\t\t     long>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::ranges::iter_move(cit)),\n\t\t\t     const short&&>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::ranges::iter_move(std::move(cit))),\n\t\t\t     float>"
    );;
    const _: () = assert!(true, "std::same_as<rval_ref<Iter>, int&>");;
    const _: () = assert!(true, "std::same_as<rval_ref<Iter&>, int&>");;
    const _: () = assert!(true, "std::same_as<rval_ref<const Iter>, const short&&>");;
    const _: () = assert!(true, "std::same_as<rval_ref<const Iter&>, const short&&>");;
}
pub fn test_pr106612_18() {
    const _: () = assert!(true, "iter_movable<I&>");;
    const _: () = assert!(true, "iter_movable<I const&>");;
    const _: () = assert!(!(false), "! iter_movable<I>");;
    const _: () = assert!(true, "std::same_as<std::iter_rvalue_reference_t<I>, int&&>");;
    const _: () = assert!(
        true,
        "std::same_as<std::iter_rvalue_reference_t<const I>, int>"
    );;
    const _: () = assert!(true, "iter_movable<I2&>");;
    const _: () = assert!(true, "iter_movable<I2 const&>");;
    const _: () = assert!(true, "iter_movable<I2>");;
    const _: () = assert!(
        true,
        "std::is_void_v<decltype(std::ranges::iter_move(I2{{}}))>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<std::iter_rvalue_reference_t<I2>, int&&>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<std::iter_rvalue_reference_t<I2 const>, int>"
    );;
    const _: () = assert!(true, "iter_movable<I3&>");;
    const _: () = assert!(true, "iter_movable<I3>");;
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::ranges::iter_move(I3{{}})), F>"
    );;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct I_19 {
    #[offset(0)]
    pub i: (),
}
impl Default for I_19 {
    fn default() -> Self {
        {
            I_19 {
                i: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct I2_20 {
    #[offset(0)]
    pub i: (),
}
impl Default for I2_20 {
    fn default() -> Self {
        {
            I2_20 {
                i: unimplemented!(),
            }
        }
    }
}
pub type E_21 = ();
pub const E_21_e: E_21 = 0;
pub type F_22 = ();
pub const F_22_f: F_22 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct I3_23 {}
pub fn test_pr119469_24() {
    const _: () = assert!(true, "std::same_as<std::iter_rvalue_reference_t<F>, F&>");;
    const _: () = assert!(true, "std::same_as<std::iter_rvalue_reference_t<F&>, F&>");;
    const _: () = assert!(true, "std::same_as<std::iter_rvalue_reference_t<F&&>, F&>");;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_11() });
    ({ test_adl_17() });
    ({ test_pr119469_24() });
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
pub fn set_memory_limits_25(size: Option<()>) {
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
pub fn set_file_limit_26(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_27(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_28(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_29(
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
) -> () {
    (*tmp_31.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_31.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_31.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_31: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub trait I2_20Impl {
    fn operator_deref_1(&self) -> Ptr<()>;
    fn operator_deref_2(&self) -> ();
    fn operator_deref_3(&self) {
        unimplemented!()
    }
}
impl I2_20Impl for Ptr<I2_20> {
    fn operator_deref_1(&self) -> Ptr<()> {
        return field_ptr!((*self), i);
    }
    fn operator_deref_2(&self) -> () {
        return (*self).with(|__s| __s.i);
    }
}
pub trait I3_23Impl {
    fn operator_deref_1(&self) -> E_21;
    fn operator_deref_2(&self) -> F_22;
}
impl I3_23Impl for Ptr<I3_23> {
    fn operator_deref_1(&self) -> E_21 {
        return E_21_e;
    }
    fn operator_deref_2(&self) -> F_22 {
        return F_22_f;
    }
}
pub trait I_19Impl {
    fn operator_deref_1(&self) -> Ptr<()>;
    fn operator_deref_2(&self) -> ();
}
impl I_19Impl for Ptr<I_19> {
    fn operator_deref_1(&self) -> Ptr<()> {
        return field_ptr!((*self), i);
    }
    fn operator_deref_2(&self) -> () {
        return (*self).with(|__s| __s.i);
    }
}
pub trait IterImpl {
    fn operator_deref(&self) -> Ptr<()>;
}
impl IterImpl for Ptr<Iter> {
    fn operator_deref(&self) -> Ptr<()> {
        return s_16.with(|v| v.as_pointer());
    }
}
pub trait XImpl {
    fn move_assign(&self, x: Ptr<X>) -> Ptr<X>;
}
impl XImpl for Ptr<X> {
    fn move_assign(&self, x: Ptr<X>) -> Ptr<X> {
        field!((*self), value).write({ x.with(|__s| __s.value) });
        field!(x, value).write(-1_());
        return (*self).clone();
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
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
