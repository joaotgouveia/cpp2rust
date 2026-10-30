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
thread_local!(
    pub static buf_9: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..100).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
);
pub fn to_string_10(mut val: ()) -> () {
    let mut str: () = unimplemented!();
    let mut len: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < ::std::mem::size_of::<[(); 100]>()) {
            unimplemented!();
            unimplemented!();
        }
    }
    return unimplemented!();
}
pub fn to_string_11(mut val: ()) -> () {
    let mut str: () = unimplemented!();
    let mut len: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < ::std::mem::size_of::<[(); 100]>()) {
            unimplemented!();
            unimplemented!();
        }
    }
    return unimplemented!();
}
pub fn to_string_12(mut val: ()) -> () {
    let mut str: () = unimplemented!();
    let mut len: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < ::std::mem::size_of::<[(); 100]>()) {
            unimplemented!();
            unimplemented!();
        }
    }
    return unimplemented!();
}
pub fn to_string_13(mut val: ()) -> () {
    let mut str: () = unimplemented!();
    let mut len: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < ::std::mem::size_of::<[(); 100]>()) {
            unimplemented!();
            unimplemented!();
        }
    }
    return unimplemented!();
}
pub fn to_string_14(mut val: ()) -> () {
    let mut str: () = unimplemented!();
    let mut len: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < ::std::mem::size_of::<[(); 100]>()) {
            unimplemented!();
            unimplemented!();
        }
    }
    return unimplemented!();
}
pub fn to_string_15(mut val: ()) -> () {
    let mut str: () = unimplemented!();
    let mut len: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < ::std::mem::size_of::<[(); 100]>()) {
            unimplemented!();
            unimplemented!();
        }
    }
    return unimplemented!();
}
thread_local!(
    pub static values_16: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        16_(),
        48_(),
        80_(),
        128_(),
        192_(),
        256_(),
        384_(),
        448_(),
        512_(),
        1024_(),
        2048_(),
        3072_(),
        4096_(),
        6144_(),
        8192_(),
        16384_(),
        32768_(),
        49152_(),
        65536_(),
        65793_(),
        524288_(),
        526086_(),
        786432_(),
        838860_(),
        1048576_(),
        1052688_(),
        8388608_(),
        8417376_(),
        12648190_(),
        203238975_(),
        16777216_(),
        16781312_(),
        16809984_(),
        16842752_(),
        17301504_(),
        17825792_(),
        19088743_(),
        268435456_(),
        269488144_(),
        305419896_(),
        2147483664_(),
        2271560481_(),
        2863311530_(),
        4026531840_(),
        4027584528_(),
        4042260480_(),
        4042322160_(),
        4043247600_(),
        4279234575_(),
        4294901760_(),
        4294902000_(),
        4294905840_(),
        4294967040_(),
    ])));
);
thread_local!(
    pub static empty_string_capacity_17: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn check_value_18(mut val: ()) {
    let mut s: () = unimplemented!();
    let mut expected: () = ({ to_string_10(val) });
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
        if !((unimplemented!().read()) == ('\0' as ())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn check_value_19(mut val: ()) {
    let mut s: () = unimplemented!();
    let mut expected: () = ({ to_string_11(val) });
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
        if !((unimplemented!().read()) == ('\0' as ())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn check_value_20(mut val: ()) {
    let mut s: () = unimplemented!();
    let mut expected: () = ({ to_string_12(val) });
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
        if !((unimplemented!().read()) == ('\0' as ())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn check_value_21(mut val: ()) {
    let mut s: () = unimplemented!();
    let mut expected: () = ({ to_string_13(val) });
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
        if !((unimplemented!().read()) == ('\0' as ())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn check_value_22(mut val: ()) {
    let mut s: () = unimplemented!();
    let mut expected: () = ({ to_string_14(val) });
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
        if !((unimplemented!().read()) == ('\0' as ())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn check_value_23(mut val: ()) {
    let mut s: () = unimplemented!();
    let mut expected: () = ({ to_string_15(val) });
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
        if !((unimplemented!().read()) == ('\0' as ())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
thread_local!(
    pub static width_24: Value<()> = Rc::new(RefCell::new(16_()));
);
pub fn check_values_25() {
    'loop_: for v in IterRange::new(
        (values_16.with(|v| v.as_pointer()) as Ptr<()>),
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: () = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: () = (v + i);
            ({ check_value_18(val) });
            i.prefix_inc();
        }
        {}
    }
}
pub fn check_values_26() {
    'loop_: for v in IterRange::new(
        (values_16.with(|v| v.as_pointer()) as Ptr<()>),
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: () = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: () = (v as ()).wrapping_add(i);
            ({ check_value_19(val) });
            i.prefix_inc();
        }
        {}
    }
}
pub fn check_values_27() {
    'loop_: for v in IterRange::new(
        (values_16.with(|v| v.as_pointer()) as Ptr<()>),
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: () = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: () = (v + i);
            ({ check_value_20(val) });
            i.prefix_inc();
        }
        {
            'loop_: for v2 in IterRange::new(
                (values_16.with(|v| v.as_pointer()) as Ptr<()>),
                (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
            ) {
                let mut v2: () = (v2.read());
                let mut i: () = -width_24.with(|rc| *rc.borrow());
                'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
                    let mut hi: () = v2;
                    hi = { (hi).wrapping_add(i) };
                    hi <<= 32_();
                    let mut val: () = (hi | v);
                    ({ check_value_20(val) });
                    i.prefix_inc();
                }
            }
        }
    }
}
pub fn check_values_28() {
    'loop_: for v in IterRange::new(
        (values_16.with(|v| v.as_pointer()) as Ptr<()>),
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: () = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: () = (v).wrapping_add(i);
            ({ check_value_21(val) });
            i.prefix_inc();
        }
        {
            'loop_: for v2 in IterRange::new(
                (values_16.with(|v| v.as_pointer()) as Ptr<()>),
                (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
            ) {
                let mut v2: () = (v2.read());
                let mut i: () = -width_24.with(|rc| *rc.borrow());
                'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
                    let mut hi: () = v2;
                    hi = { (hi).wrapping_add(i) };
                    hi <<= 32_();
                    let mut val: () = (hi | v);
                    ({ check_value_21(val) });
                    i.prefix_inc();
                }
            }
        }
    }
}
pub fn check_values_29() {
    'loop_: for v in IterRange::new(
        (values_16.with(|v| v.as_pointer()) as Ptr<()>),
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: () = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: () = (v + i);
            ({ check_value_22(val) });
            i.prefix_inc();
        }
        {
            'loop_: for v2 in IterRange::new(
                (values_16.with(|v| v.as_pointer()) as Ptr<()>),
                (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
            ) {
                let mut v2: () = (v2.read());
                let mut i: () = -width_24.with(|rc| *rc.borrow());
                'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
                    let mut hi: () = v2;
                    hi = { (hi).wrapping_add(i) };
                    hi <<= 32_();
                    let mut val: () = (hi | v);
                    ({ check_value_22(val) });
                    i.prefix_inc();
                }
            }
        }
    }
}
pub fn check_values_30() {
    'loop_: for v in IterRange::new(
        (values_16.with(|v| v.as_pointer()) as Ptr<()>),
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: () = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: () = (v).wrapping_add(i);
            ({ check_value_23(val) });
            i.prefix_inc();
        }
        {
            'loop_: for v2 in IterRange::new(
                (values_16.with(|v| v.as_pointer()) as Ptr<()>),
                (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_()) as isize),
            ) {
                let mut v2: () = (v2.read());
                let mut i: () = -width_24.with(|rc| *rc.borrow());
                'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
                    let mut hi: () = v2;
                    hi = { (hi).wrapping_add(i) };
                    hi <<= 32_();
                    let mut val: () = (hi | v);
                    ({ check_value_23(val) });
                    i.prefix_inc();
                }
            }
        }
    }
}
pub fn test02_31() {
    ({ check_values_25() });
    ({ check_values_26() });
    ({ check_values_27() });
    ({ check_values_28() });
    ({ check_values_29() });
    ({ check_values_30() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test02_31() });
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
pub fn set_memory_limits_32(size: Option<()>) {
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
pub fn set_file_limit_33(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_34(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_35(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_36(
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
pub fn test_tm_37(
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
    (*tmp_38.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_38.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_38.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_38.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_38.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_38.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_38.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_38.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_38.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_38.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_38: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    let _ = buf_9.with(|_| ());
    let _ = values_16.with(|_| ());
    let _ = empty_string_capacity_17.with(|_| ());
    let _ = width_24.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
