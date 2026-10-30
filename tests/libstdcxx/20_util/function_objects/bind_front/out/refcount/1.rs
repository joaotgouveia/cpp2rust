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
pub fn test01_9() {
    const _: () = assert!(
        true,
        "std::is_same_v<\n      decltype(bind_front(std::declval<F>(), std::declval<int>())),\n      decltype(bind_front(std::declval<F&>(), std::declval<int&>()))\n      >"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<\n      decltype(bind_front(std::declval<F>(), std::declval<int>())),\n      decltype(bind_front(std::declval<const F&>(), std::declval<const int&>()))\n      >"
    );;
    const _: () = assert!(
        true,
        "!std::is_same_v<\n      decltype(bind_front(std::declval<F>(), std::declval<int&>())),\n      decltype(bind_front(std::declval<F>(), std::ref(std::declval<int&>())))\n      >"
    );;
    const _: () = assert!(
        true,
        "!std::is_same_v<\n      decltype(bind_front(std::declval<F>(), std::declval<const int&>())),\n      decltype(bind_front(std::declval<F>(), std::cref(std::declval<int&>())))\n      >"
    );;
    const _: () = assert!(
        true,
        "!std::is_same_v<\n      decltype(bind_front(std::declval<F>(), std::ref(std::declval<int&>()))),\n      decltype(bind_front(std::declval<F>(), std::cref(std::declval<int&>())))\n      >"
    );;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct F_10 {}
pub fn test02_11() {
    let mut f: F_13 = <F_13>::default();
    let g: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cg: Ptr<()> = g.as_pointer();
    let mut q: quals_12 = <quals_12>::default();
    q = ({
        let ___self: Ptr<()> = g.as_pointer();
        (*g.borrow_mut())(___self)
    })
    .clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(q.as_const)) && (q.as_lvalue)) {
            unimplemented!();
            unimplemented!();
        }
    }
    q = ({
        let ___self: Ptr<()> = g.as_pointer();
        (*g.borrow_mut())(___self)
    })
    .clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(q.as_const)) && (!(q.as_lvalue))) {
            unimplemented!();
            unimplemented!();
        }
    }
    q = ({
        let ___self: Ptr<()> = (cg).clone();
        (*cg.upgrade().deref())(___self)
    })
    .clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((q.as_const) && (q.as_lvalue)) {
            unimplemented!();
            unimplemented!();
        }
    }
    q = ({
        let ___self: Ptr<()> = (cg).clone();
        (*cg.upgrade().deref())(___self)
    })
    .clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((q.as_const) && (!(q.as_lvalue))) {
            unimplemented!();
            unimplemented!();
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
pub struct quals_12 {
    #[offset(0)]
    pub as_const: (),
    #[offset(1)]
    pub as_lvalue: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct F_13 {}
pub fn test03_14() {
    let i: Value<()> = Rc::new(RefCell::new(5_()));
    let vp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    *vp.borrow_mut() = ((vp.as_pointer()) as Ptr<()>).to_any();
    let g1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "is_invocable_r_v<int&, G1&, void*>");;
    const _: () = assert!(true, "is_invocable_r_v<void*, const G1&, void*>");;
    const _: () = assert!(true, "is_invocable_r_v<void*, G1&&, void*>");;
    let mut p1: () = ({
        let ___self: Ptr<()> = g1.as_pointer();
        (*g1.borrow_mut())(___self, vp.as_pointer())
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (p1).clone() } == { (*vp.borrow()).clone() }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let g2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "is_invocable_r_v<int&, G2&, void*>");;
    const _: () = assert!(true, "is_invocable_r_v<int&, G2&&, void*>");;
    const _: () = assert!(true, "is_invocable_r_v<void*, const G2&, void*>");;
    const _: () = assert!(true, "is_invocable_r_v<void*, const G2&&, void*>");;
    let i2: Ptr<()> = ({
        let ___self: Ptr<()> = g2.as_pointer();
        (*g2.borrow_mut())(___self, vp.as_pointer())
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (i2).clone() } == { (i.as_pointer()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let i2r: Ptr<()> = ({
        let ___self: Ptr<()> = g2.as_pointer();
        (*g2.borrow_mut())(___self, vp.as_pointer())
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (i2r).clone() } == { (i.as_pointer()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut p2: () = ({
        let ___self: Ptr<()> = g2.as_pointer();
        (*g2.borrow_mut())(___self, vp.as_pointer())
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (p2).clone() } == { (*vp.borrow()).clone() }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let g3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "is_invocable_r_v<void*, G3&, void*>");;
    const _: () = assert!(true, "is_invocable_r_v<void*, G3&&, void*>");;
    const _: () = assert!(true, "is_invocable_r_v<void*, const G3&, void*>");;
    const _: () = assert!(true, "is_invocable_r_v<void*, const G3&&, void*>");;
    let g4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "is_invocable_r_v<int&, G4&>");;
    const _: () = assert!(true, "is_invocable_r_v<int&, G4&&>");;
    const _: () = assert!(true, "is_invocable_r_v<void*, const G4&>");;
    const _: () = assert!(true, "is_invocable_r_v<void*, const G4&&>");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct F_15 {}
pub fn f_16(mut i: (), mut j: (), mut k: ()) -> () {
    return ((i + j) + k);
}
pub fn test04_17() {
    let g: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___self: Ptr<()> = g.as_pointer();
            let ___call_args_1: Value<()> = Rc::new(RefCell::new(1_()));
            let ___call_args_2: Value<()> = Rc::new(RefCell::new(2_()));
            let ___call_args_3: Value<()> = Rc::new(RefCell::new(3_()));
            (*g.borrow_mut())(
                ___self,
                ___call_args_1.as_pointer(),
                ___call_args_2.as_pointer(),
                ___call_args_3.as_pointer(),
            )
        }) == 6_())
        {
            unimplemented!();
            unimplemented!();
        }
    }
    let g1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___self: Ptr<()> = g1.as_pointer();
            let ___call_args_1: Value<()> = Rc::new(RefCell::new(2_()));
            let ___call_args_2: Value<()> = Rc::new(RefCell::new(3_()));
            (*g1.borrow_mut())(
                ___self,
                ___call_args_1.as_pointer(),
                ___call_args_2.as_pointer(),
            )
        }) == 6_())
        {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___self: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let ___call_args_1: Value<()> = Rc::new(RefCell::new(2_()));
            let ___call_args_2: Value<()> = Rc::new(RefCell::new(3_()));
            unimplemented!()(
                ___self.as_pointer(),
                ___call_args_1.as_pointer(),
                ___call_args_2.as_pointer(),
            )
        }) == 6_())
        {
            unimplemented!();
            unimplemented!();
        }
    }
    let g2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___self: Ptr<()> = g2.as_pointer();
            let ___call_args: Value<()> = Rc::new(RefCell::new(3_()));
            (*g2.borrow_mut())(___self, ___call_args.as_pointer())
        }) == 6_())
        {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___self: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let ___call_args: Value<()> = Rc::new(RefCell::new(3_()));
            unimplemented!()(___self.as_pointer(), ___call_args.as_pointer())
        }) == 6_())
        {
            unimplemented!();
            unimplemented!();
        }
    }
    let g3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___self: Ptr<()> = g3.as_pointer();
            (*g3.borrow_mut())(___self)
        }) == 6_())
        {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(({
            let ___self: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!()(___self.as_pointer())
        }) == 6_())
        {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_9() });
    ({ test02_11() });
    ({ test03_14() });
    ({ test04_17() });
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
pub fn set_memory_limits_18(size: Option<()>) {
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
pub fn set_file_limit_19(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_20(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_21(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_22(
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
pub fn test_tm_23(
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
    static tmp_24: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub trait F_10Impl {
    fn operator_call(&self);
}
impl F_10Impl for Ptr<F_10> {
    fn operator_call(&self) {}
}
pub trait F_13Impl {
    fn operator_call_1(&self) -> quals_12;
    fn operator_call_2(&self) -> quals_12;
    fn operator_call_3(&self) -> quals_12;
    fn operator_call_4(&self) -> quals_12;
}
impl F_13Impl for Ptr<F_13> {
    fn operator_call_1(&self) -> quals_12 {
        return quals_12 {
            as_const: false,
            as_lvalue: true,
        };
    }
    fn operator_call_2(&self) -> quals_12 {
        return quals_12 {
            as_const: true,
            as_lvalue: true,
        };
    }
    fn operator_call_3(&self) -> quals_12 {
        return quals_12 {
            as_const: false,
            as_lvalue: false,
        };
    }
    fn operator_call_4(&self) -> quals_12 {
        return quals_12 {
            as_const: true,
            as_lvalue: false,
        };
    }
}
pub trait F_15Impl {
    fn operator_call_1(&self, i: Ptr<()>, _a1: ()) -> Ptr<()>;
    fn operator_call_2(&self, _a0: (), p: ()) -> ();
}
impl F_15Impl for Ptr<F_15> {
    fn operator_call_1(&self, i: Ptr<()>, mut _a1: ()) -> Ptr<()> {
        return (i).clone();
    }
    fn operator_call_2(&self, mut _a0: (), p: ()) -> () {
        let p: Value<()> = Rc::new(RefCell::new(p));
        return (*p.borrow()).clone();
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
