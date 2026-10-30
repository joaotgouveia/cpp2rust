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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_tracker_allocator_counter {}
impl __gnu_test_tracker_allocator_counter {
    pub fn allocate(mut blocksize: ()) {
        allocationCount__9.with(|rc| {
            *rc.borrow_mut() =
                { (allocationCount__9.with(|rc| *rc.borrow())).wrapping_add(blocksize) }
        });
    }
    pub fn construct() {
        (*constructCount__11.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn destroy() {
        (*destructCount__12.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn deallocate(mut blocksize: ()) {
        deallocationCount__10.with(|rc| {
            *rc.borrow_mut() =
                { (deallocationCount__10.with(|rc| *rc.borrow())).wrapping_add(blocksize) }
        });
    }
    pub fn get_allocation_count() -> () {
        return allocationCount__9.with(|rc| *rc.borrow());
    }
    pub fn get_deallocation_count() -> () {
        return deallocationCount__10.with(|rc| *rc.borrow());
    }
    pub fn get_construct_count() -> () {
        return constructCount__11.with(|rc| *rc.borrow());
    }
    pub fn get_destruct_count() -> () {
        return destructCount__12.with(|rc| *rc.borrow());
    }
    pub fn reset() {
        allocationCount__9.with(|rc| *rc.borrow_mut() = 0_());
        deallocationCount__10.with(|rc| *rc.borrow_mut() = 0_());
        constructCount__11.with(|rc| *rc.borrow_mut() = 0_());
        destructCount__12.with(|rc| *rc.borrow_mut() = 0_());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_uneq_allocator_base {}
impl __gnu_test_uneq_allocator_base {
    pub fn get_map() -> Ptr<()> {
        return alloc_map_13.with(|v| v.as_pointer());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_SimpleAllocator_char_ {}
impl __gnu_test_SimpleAllocator_char_ {}
pub fn operator_eq_14(
    mut _a0: __gnu_test_NullablePointer_void_,
    mut _a1: __gnu_test_NullablePointer_void_,
) -> () {
    return true;
}
pub fn operator_ne_15(
    mut _a0: __gnu_test_NullablePointer_void_,
    mut _a1: __gnu_test_NullablePointer_void_,
) -> () {
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_NullablePointer_void_ {}
impl __gnu_test_NullablePointer_void_ {
    pub fn new_1(mut _a0: ()) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {}
    }
}
impl CppEq for __gnu_test_NullablePointer_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_14(__lhs.read(), __rhs.read())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_NullablePointer_void_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    value: (),
}
impl __gnu_test_NullablePointer_void_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_NullablePointer_const_void_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    value: (),
}
impl __gnu_test_NullablePointer_const_void_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_PointerBase_void_void_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_PointerBase_void_const_void_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct __gnu_test_default_resource_mgr {
    #[offset(0)]
    #[byte_size(8)]
    pub prev: Ptr<()>,
}
impl __gnu_test_default_resource_mgr {
    pub fn new(mut r: Ptr<()>) -> Self {
        Self {
            prev: unimplemented!(),
        }
    }
}
thread_local!(
    static alloc_map_13: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn test_recommended_practice_16() {
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "stream_extractable<parse_manip, char>");;
    const _: () = assert!(!(false), "not stream_extractable<parse_manip, wchar_t>");;
    const _: () = assert!(true, "stream_extractable<wparse_manip, wchar_t>");;
    const _: () = assert!(!(false), "not stream_extractable<wparse_manip, char>");;
    const _: () = assert!(true, "not std::is_move_constructible_v<parse_manip>");;
    const _: () = assert!(true, "not std::is_move_assignable_v<parse_manip>");;
    const _: () = assert!(!(false), "not stream_extractable<parse_manip&, char>");;
    const _: () = assert!(!(false), "not stream_extractable<const parse_manip&, char>");;
    const _: () = assert!(!(false), "not stream_extractable<wparse_manip&, wchar_t>");;
    const _: () = assert!(
        !(false),
        "not stream_extractable<const wparse_manip&, wchar_t>"
    );;
}
thread_local!(
    pub static f_17: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N_A {}
pub fn from_stream_18(_a0: Ptr<()>, mut fmt: Ptr<()>, _a2: Ptr<N_A>) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (fmt).clone() } == { unimplemented!() }) {
            unimplemented!();
            unimplemented!();
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N_B {}
pub fn from_stream_19(_a0: Ptr<()>, mut fmt: Ptr<()>, _a2: Ptr<N_B>, mut abbrev: Ptr<()>) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (fmt).clone() } == { unimplemented!() }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!((abbrev).is_null())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N_C {}
pub fn from_stream_20(
    _a0: Ptr<()>,
    mut fmt: Ptr<()>,
    _a2: Ptr<N_C>,
    mut abbrev: Ptr<()>,
    mut offset: Ptr<()>,
) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (fmt).clone() } == { unimplemented!() }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((abbrev).is_null()) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!((offset).is_null())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N_D {}
pub fn from_stream_21(
    _a0: Ptr<()>,
    mut fmt: Ptr<()>,
    _a2: Ptr<N_D>,
    mut abbrev: Ptr<()>,
    mut offset: Ptr<()>,
) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (fmt).clone() } == { unimplemented!() }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!((abbrev).is_null())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!((offset).is_null())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N_E {}
pub fn from_stream_22(
    _a0: Ptr<()>,
    mut _a1: Ptr<()>,
    _a2: Ptr<N_E>,
    _a3: Option<Ptr<()>>,
    _a4: Option<Ptr<()>>,
) {
    let mut _a3: Ptr<()> = _a3.unwrap_or_else(|| Ptr::<()>::null());
    let mut _a4: Ptr<()> = _a4.unwrap_or_else(|| Ptr::<()>::null());
}
pub fn test_adl_23() {
    let mut abbrev: () = unimplemented!();
    let offset: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut a: N_A = <N_A>::default();
    &(unimplemented!());
    let mut b: N_B = <N_B>::default();
    &(unimplemented!());
    let mut c: N_C = <N_C>::default();
    &(unimplemented!());
    const _: () = assert!(true, "parsable<const char*, N::C, string, minutes>");;
    let mut d: N_D = <N_D>::default();
    &(unimplemented!());
    const _: () = assert!(true, "parsable<const char*, N::D, minutes>");;
    const _: () = assert!(true, "parsable<const wchar_t*, N::E, wstring>");;
    const _: () = assert!(true, "parsable<const wchar_t*, N::E, wstring>");;
    const _: () = assert!(true, "parsable<const wchar_t*, N::E, minutes>");;
    const _: () = assert!(true, "parsable<const wchar_t*, N::E, wstring, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::A, std::string>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::A, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::A, string, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::B>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::B, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::B, string, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::C>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::C, string>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::D>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::D, string>");;
    const _: () = assert!(!(false), "not parsable<string, std::chrono::year, wstring>");;
    const _: () = assert!(
        !(false),
        "not parsable<string, std::chrono::year, wstring, minutes>"
    );;
    const _: () = assert!(true, "parsable<String, std::chrono::year>");;
    const _: () = assert!(true, "parsable<String, std::chrono::year, String>");;
    const _: () = assert!(true, "parsable<String, std::chrono::year, minutes>");;
    const _: () = assert!(true, "parsable<String, std::chrono::year, String, minutes>");;
    const _: () = assert!(true, "parsable<const char*, std::chrono::year, String>");;
    const _: () = assert!(
        true,
        "parsable<const char*, std::chrono::year, String, minutes>"
    );;
    const _: () = assert!(!(false), "not parsable<string, std::chrono::year, String>");;
    const _: () = assert!(
        !(false),
        "not parsable<string, std::chrono::year, String, minutes>"
    );;
    const _: () = assert!(!(false), "not parsable<String, std::chrono::year, string>");;
    const _: () = assert!(
        !(false),
        "not parsable<String, std::chrono::year, string, minutes>"
    );;
}
pub fn test_whitespace_24() {
    let min: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let is: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (unimplemented!())) {
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
pub fn test_errors_25() {
    let min: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let is: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
pub fn test_modifiers_26() {
    let min: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let is: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
pub fn test_wchar_27() {
    let is: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ms: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        if !(unimplemented!() == 125_()) {
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
    ({ test_recommended_practice_16() });
    ({ test_adl_23() });
    ({ test_whitespace_24() });
    ({ test_errors_25() });
    ({ test_modifiers_26() });
    ({ test_wchar_27() });
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
pub fn set_memory_limits_28(size: Option<()>) {
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
pub fn set_file_limit_29(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_30(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn test_tm_33(
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
    (*tmp_34.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_34.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_34.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_34.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_34.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_34.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_34.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_34.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_34.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_34.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_34: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    static allocationCount__9: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static deallocationCount__10: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static constructCount__11: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static destructCount__12: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn check_construct_destroy_35(mut tag: Ptr<()>, mut expected_c: (), mut expected_d: ()) -> () {
    let ret: Value<()> = Rc::new(RefCell::new(true));
    if (({ __gnu_test_tracker_allocator_counter::get_construct_count() }) != expected_c)
        || (({ __gnu_test_tracker_allocator_counter::get_destruct_count() }) != expected_d)
    {
        unimplemented!();
        (*ret.borrow_mut()) = false;
    }
    return (*ret.borrow());
}
pub trait __gnu_test_NullablePointer_const_void_ptr_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl __gnu_test_NullablePointer_const_void_ptr_Impl
    for Ptr<__gnu_test_NullablePointer_const_void_ptr_>
{
}
pub trait __gnu_test_NullablePointer_void_Impl {
    fn to_(&self) -> ();
}
impl __gnu_test_NullablePointer_void_Impl for Ptr<__gnu_test_NullablePointer_void_> {
    fn to_(&self) -> () {
        return false;
    }
}
pub trait __gnu_test_NullablePointer_void_ptr_Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl __gnu_test_NullablePointer_void_ptr_Impl for Ptr<__gnu_test_NullablePointer_void_ptr_> {}
pub trait __gnu_test_PointerBase_void_const_void_Impl {
    fn operator_arrow(&self) -> () {
        unimplemented!()
    }
}
impl __gnu_test_PointerBase_void_const_void_Impl for Ptr<__gnu_test_PointerBase_void_const_void_> {}
pub trait __gnu_test_PointerBase_void_void_Impl {
    fn operator_arrow(&self) -> () {
        unimplemented!()
    }
}
impl __gnu_test_PointerBase_void_void_Impl for Ptr<__gnu_test_PointerBase_void_void_> {}
pub trait __gnu_test_SimpleAllocator_char_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl __gnu_test_SimpleAllocator_char_Impl for Ptr<__gnu_test_SimpleAllocator_char_> {}
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
pub trait __gnu_test_default_resource_mgrImpl {
    fn destructor(&self);
}
impl __gnu_test_default_resource_mgrImpl for Ptr<__gnu_test_default_resource_mgr> {
    fn destructor(&self) {
        unimplemented!();
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
    let _ = f_17.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
    let _ = allocationCount__9.with(|_| ());
    let _ = deallocationCount__10.with(|_| ());
    let _ = constructCount__11.with(|_| ());
    let _ = destructCount__12.with(|_| ());
}
