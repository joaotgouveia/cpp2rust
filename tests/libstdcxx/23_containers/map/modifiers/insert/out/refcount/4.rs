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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct __gnu_test_rvalstruct {
    #[offset(0)]
    pub val: i32,
    #[offset(4)]
    pub valid: bool,
}
impl __gnu_test_rvalstruct {
    pub fn new_2() -> Self {
        Self {
            val: 0,
            valid: true,
        }
    }
    pub fn new_1(mut inval: i32) -> Self {
        Self {
            val: inval,
            valid: true,
        }
    }
    pub fn move_from(in_: Ptr<__gnu_test_rvalstruct>) -> Self {
        let __this: Value<__gnu_test_rvalstruct> = Rc::new(RefCell::new(Self {
            val: 0_i32,
            valid: false,
        }));
        let this: Ptr<__gnu_test_rvalstruct> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_.with(|__s| __s.valid) as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!(this, val).write({ in_.with(|__s| __s.val) });
        field!(in_, valid).write(false);
        field!(this, valid).write(true);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppLess for __gnu_test_rvalstruct {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_9(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for __gnu_test_rvalstruct {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for __gnu_test_rvalstruct {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for __gnu_test_rvalstruct {
    fn default() -> Self {
        { __gnu_test_rvalstruct::new_2() }
    }
}
pub fn operator_eq_10(lhs: Ptr<__gnu_test_rvalstruct>, rhs: Ptr<__gnu_test_rvalstruct>) -> bool {
    return ({ lhs.with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
}
pub fn operator_lt_9(lhs: Ptr<__gnu_test_rvalstruct>, rhs: Ptr<__gnu_test_rvalstruct>) -> bool {
    return ({ lhs.with(|__s| __s.val) } < { rhs.with(|__s| __s.val) });
}
pub fn swap_11(lhs: Ptr<__gnu_test_rvalstruct>, rhs: Ptr<__gnu_test_rvalstruct>) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((lhs.with(|__s| __s.valid)) && (rhs.with(|__s| __s.valid))) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut temp: i32 = lhs.with(|__s| __s.val);
    field!(lhs, val).write({ rhs.with(|__s| __s.val) });
    field!(rhs, val).write(temp);
}
#[derive(
    Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, CopyAssign, MoveAssign, Destructor,
)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct __gnu_test_copycounter {
    #[offset(0)]
    pub val: i32,
    #[offset(4)]
    pub valid: bool,
}
impl __gnu_test_copycounter {
    pub fn new_2() -> Self {
        Self {
            val: 0,
            valid: true,
        }
    }
    pub fn new_1(mut inval: i32) -> Self {
        Self {
            val: inval,
            valid: true,
        }
    }
    pub fn copy_from(in_: Ptr<__gnu_test_copycounter>) -> Self {
        let __this: __gnu_test_copycounter = Self {
            val: in_.with(|__s| __s.val),
            valid: true,
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_.with(|__s| __s.valid) as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        (*copycount_12.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(in_: Ptr<__gnu_test_copycounter>) -> Self {
        let __this: Value<__gnu_test_copycounter> = Rc::new(RefCell::new(Self {
            val: 0_i32,
            valid: false,
        }));
        let this: Ptr<__gnu_test_copycounter> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_.with(|__s| __s.valid) as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!(this, val).write({ in_.with(|__s| __s.val) });
        field!(in_, valid).write(false);
        field!(this, valid).write(true);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppLess for __gnu_test_copycounter {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_13(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for __gnu_test_copycounter {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_14(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for __gnu_test_copycounter {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_copycounter> = Rc::new(RefCell::new(__gnu_test_copycounter {
            val: self.val.clone(),
            valid: self.valid.clone(),
        }));
        __gnu_test_copycounter::copy_from(__src.as_pointer())
    }
}
impl CppMove for __gnu_test_copycounter {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for __gnu_test_copycounter {
    fn default() -> Self {
        { __gnu_test_copycounter::new_2() }
    }
}
thread_local!(
    pub static copycount_12: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn operator_eq_14(lhs: Ptr<__gnu_test_copycounter>, rhs: Ptr<__gnu_test_copycounter>) -> bool {
    return ({ lhs.with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
}
pub fn operator_lt_13(lhs: Ptr<__gnu_test_copycounter>, rhs: Ptr<__gnu_test_copycounter>) -> bool {
    return ({ lhs.with(|__s| __s.val) } < { rhs.with(|__s| __s.val) });
}
pub fn swap_15(lhs: Ptr<__gnu_test_copycounter>, rhs: Ptr<__gnu_test_copycounter>) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((lhs.with(|__s| __s.valid)) && (rhs.with(|__s| __s.valid))) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut temp: i32 = lhs.with(|__s| __s.val);
    field!(lhs, val).write({ rhs.with(|__s| __s.val) });
    field!(rhs, val).write(temp);
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct __gnu_test_rvalstruct_compare_by_value {
    #[offset(0)]
    pub val: i32,
    #[offset(4)]
    pub ok: bool,
}
impl __gnu_test_rvalstruct_compare_by_value {
    pub fn new(mut v: i32) -> Self {
        Self { val: v, ok: true }
    }
    pub fn copy_from(rh: Ptr<__gnu_test_rvalstruct_compare_by_value>) -> Self {
        let __this: __gnu_test_rvalstruct_compare_by_value = Self {
            val: rh.with(|__s| __s.val),
            ok: rh.with(|__s| __s.ok),
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(rh.with(|__s| __s.ok)) {
                unimplemented!();
                std::process::abort();
            }
        }
        __this
    }
    pub fn move_from(rh: Ptr<__gnu_test_rvalstruct_compare_by_value>) -> Self {
        let __this: __gnu_test_rvalstruct_compare_by_value = Self {
            val: rh.with(|__s| __s.val),
            ok: rh.with(|__s| __s.ok),
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(rh.with(|__s| __s.ok)) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!(rh, ok).write(false);
        __this
    }
}
impl CppLess for __gnu_test_rvalstruct_compare_by_value {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_16(__lhs.read(), __rhs.read())
    }
}
impl CppEq for __gnu_test_rvalstruct_compare_by_value {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_16(__lhs.read(), __rhs.read()))
            && !(operator_lt_16(__rhs.read(), __lhs.read()))
    }
}
impl Clone for __gnu_test_rvalstruct_compare_by_value {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_rvalstruct_compare_by_value> =
            Rc::new(RefCell::new(__gnu_test_rvalstruct_compare_by_value {
                val: self.val.clone(),
                ok: self.ok.clone(),
            }));
        __gnu_test_rvalstruct_compare_by_value::copy_from(__src.as_pointer())
    }
}
impl CppMove for __gnu_test_rvalstruct_compare_by_value {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_lt_16(
    mut lh: __gnu_test_rvalstruct_compare_by_value,
    mut rh: __gnu_test_rvalstruct_compare_by_value,
) -> bool {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(rh.ok) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(lh.ok) {
            unimplemented!();
            std::process::abort();
        }
    }
    return (lh.val < rh.val);
}
pub fn order_17(
    mut lh: __gnu_test_rvalstruct_compare_by_value,
    mut rh: __gnu_test_rvalstruct_compare_by_value,
) -> bool {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(rh.ok) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(lh.ok) {
            unimplemented!();
            std::process::abort();
        }
    }
    return (lh.val < rh.val);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct __gnu_test_throwing_move_constructor {}
impl __gnu_test_throwing_move_constructor {
    pub fn move_from(_a0: Ptr<__gnu_test_throwing_move_constructor>) -> Self {
        let __this: __gnu_test_throwing_move_constructor = Self {};
        {
            std::process::abort();
            &(1)
        };
        __this
    }
}
impl CppMove for __gnu_test_throwing_move_constructor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn test01_18() {
    let m: Value<Map<i32, (Value<i32>, Value<__gnu_test_rvalstruct>)>> =
        Rc::new(RefCell::new(Map::new()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let p: Value<BidirectionalIterator<(Value<i32>, Value<__gnu_test_rvalstruct>)>> =
        Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*m.borrow()).len() == 1_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 1_isize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*p.borrow())
            == m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<__gnu_test_rvalstruct>)>| __m.begin()))
        {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*(*(*p.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*p.borrow())
            .iter_deref()
            .with(|__s| (*({ __s }).1.borrow()).val)
            == 3)
        {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test02_19() {
    let m: Value<Map<i32, (Value<i32>, Value<__gnu_test_rvalstruct>)>> =
        Rc::new(RefCell::new(Map::new()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let p1: Value<BidirectionalIterator<(Value<i32>, Value<__gnu_test_rvalstruct>)>> =
        Rc::new(RefCell::new(unimplemented!()));
    let p2: Value<BidirectionalIterator<(Value<i32>, Value<__gnu_test_rvalstruct>)>> =
        Rc::new(RefCell::new(unimplemented!()));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*m.borrow()).len() == 1_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*p1.borrow()) == (*p2.borrow())) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*(*(*p1.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*p2.borrow())
            .iter_deref()
            .with(|__s| (*({ __s }).1.borrow()).val)
            == 3)
        {
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
    ({ test01_18() });
    ({ test02_19() });
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
pub fn set_memory_limits_20(size: Option<f32>) {
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
pub fn set_file_limit_21(mut size: u64) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_22(mut mangled: Ptr<i8>, mut wanted: Ptr<i8>) {
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
pub fn run_tests_wrapped_locale_23(mut name: Ptr<i8>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_24(
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
pub fn test_tm_25(
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
    (*tmp_26.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_26.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_26.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_26.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_26.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_26.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_26.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_26.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_26.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_26.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_26: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
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
pub trait __gnu_test_copycounterImpl {
    fn operator_assign_5(&self, newval: i32) -> Ptr<__gnu_test_copycounter>;
    fn copy_assign(&self, in_: Ptr<__gnu_test_copycounter>) -> bool;
    fn move_assign(&self, in_: Ptr<__gnu_test_copycounter>) -> Ptr<__gnu_test_copycounter>;
    fn destructor(&self);
}
impl __gnu_test_copycounterImpl for Ptr<__gnu_test_copycounter> {
    fn operator_assign_5(&self, mut newval: i32) -> Ptr<__gnu_test_copycounter> {
        field!((*self), val).write(newval);
        field!((*self), valid).write(true);
        return (*self).clone();
    }
    fn copy_assign(&self, in_: Ptr<__gnu_test_copycounter>) -> bool {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_.with(|__s| __s.valid) as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        (*copycount_12.with(Value::clone).borrow_mut()).prefix_inc();
        field!((*self), val).write({ in_.with(|__s| __s.val) });
        field!((*self), valid).write(true);
        return true;
    }
    fn move_assign(&self, in_: Ptr<__gnu_test_copycounter>) -> Ptr<__gnu_test_copycounter> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_.with(|__s| __s.valid) as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!((*self), val).write({ in_.with(|__s| __s.val) });
        field!(in_, valid).write(false);
        field!((*self), valid).write(true);
        return (*self).clone();
    }
    fn destructor(&self) {
        field!((*self), valid).write(false);
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
pub trait __gnu_test_rvalstructImpl {
    fn operator_assign_3(&self, newval: i32) -> Ptr<__gnu_test_rvalstruct>;
    fn move_assign(&self, in_: Ptr<__gnu_test_rvalstruct>) -> Ptr<__gnu_test_rvalstruct>;
}
impl __gnu_test_rvalstructImpl for Ptr<__gnu_test_rvalstruct> {
    fn operator_assign_3(&self, mut newval: i32) -> Ptr<__gnu_test_rvalstruct> {
        field!((*self), val).write(newval);
        field!((*self), valid).write(true);
        return (*self).clone();
    }
    fn move_assign(&self, in_: Ptr<__gnu_test_rvalstruct>) -> Ptr<__gnu_test_rvalstruct> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((*self) != (in_)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_.with(|__s| __s.valid) as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!((*self), val).write({ in_.with(|__s| __s.val) });
        field!(in_, valid).write(false);
        field!((*self), valid).write(true);
        return (*self).clone();
    }
}
pub trait __gnu_test_rvalstruct_compare_by_valueImpl {
    fn copy_assign(
        &self,
        rh: Ptr<__gnu_test_rvalstruct_compare_by_value>,
    ) -> Ptr<__gnu_test_rvalstruct_compare_by_value>;
    fn move_assign(
        &self,
        rh: Ptr<__gnu_test_rvalstruct_compare_by_value>,
    ) -> Ptr<__gnu_test_rvalstruct_compare_by_value>;
}
impl __gnu_test_rvalstruct_compare_by_valueImpl for Ptr<__gnu_test_rvalstruct_compare_by_value> {
    fn copy_assign(
        &self,
        rh: Ptr<__gnu_test_rvalstruct_compare_by_value>,
    ) -> Ptr<__gnu_test_rvalstruct_compare_by_value> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(rh.with(|__s| __s.ok)) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!((*self), val).write({ rh.with(|__s| __s.val) });
        field!((*self), ok).write({ rh.with(|__s| __s.ok) });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        rh: Ptr<__gnu_test_rvalstruct_compare_by_value>,
    ) -> Ptr<__gnu_test_rvalstruct_compare_by_value> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(rh.with(|__s| __s.ok)) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!((*self), val).write({ rh.with(|__s| __s.val) });
        field!((*self), ok).write({ rh.with(|__s| __s.ok) });
        field!(rh, ok).write(false);
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
    let _ = copycount_12.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
