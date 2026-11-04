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
pub fn string_stuff_9() -> i32 {
    let failures: Value<i32> = Rc::new(RefCell::new(0));
    let s: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"abcde").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let i1: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::Vec(s.decay())));
    if ((((*i1.borrow()).iter_deref().read()) as i32) != (('a' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow_mut()).iter_inc();
    if ((((*i1.borrow()).iter_deref().read()) as i32) != (('b' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow_mut()).iter_post_inc().iter_deref().read()) as i32) != (('b' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()).iter_deref().read()) as i32) != (('c' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow_mut()).iter_inc().iter_inc();
    if ((((*i1.borrow()).iter_deref().read()) as i32) != (('e' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow_mut()).iter_dec();
    if ((((*i1.borrow()).iter_deref().read()) as i32) != (('d' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow_mut()).iter_post_dec().iter_deref().read()) as i32) != (('d' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()).iter_deref().read()) as i32) != (('c' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow_mut()).iter_dec().iter_dec();
    if ((((*i1.borrow()).iter_deref().read()) as i32) != (('a' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let i2: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    (*i2.borrow_mut()) = RandomAccessIterator::Vec(s.decay().to_last());
    let mut d1: isize = 0_isize;
    d1 = ((*i2.borrow()).iter_distance(&({ (*i1.borrow()).clone() }).into()) as i64 as isize);
    if (d1 != 5_isize) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let mut v1: i8 = 0_i8;
    v1 = ((*i1.borrow()).iter_at(0_isize).read());
    if ((v1 as i32) != (('a' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let r1: Ptr<i8> = (*i1.borrow()).iter_at(0_isize);
    if (((r1.read()) as i32) != (('a' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    r1.write(('x' as i8));
    if (((r1.read()) as i32) != (('x' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    r1.write(('a' as i8));
    if ((((*i1.borrow()) != (*i2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) == (*i2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) < (*i2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) > (*i2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) <= (*i2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) >= (*i2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let i3: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    (*i3.borrow_mut()) = (*i1.borrow()).clone();
    if ((((*i3.borrow()) == (*i1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i3.borrow_mut()).iter_add_assign(5_isize);
    if ((((*i3.borrow()) == (*i2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i3.borrow_mut()).iter_add_assign(-5_isize);
    if ((((*i3.borrow()) == (*i1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*i3.borrow()).iter_add(5_isize) != (*i2.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*i3.borrow()).iter_add(5_isize) != (*i2.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*i2.borrow()).iter_add(-5_isize) != (*i3.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()).iter_at(0_isize).read()) as i32) != (('a' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow()).iter_at(4_isize).write(('x' as i8));
    if ((((*i2.borrow()).iter_at((-1_i32 as isize)).read()) as i32) != (('x' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow()).iter_at(4_isize).write(('e' as i8));
    (*i1.borrow()).iter_at(2_isize).write(('x' as i8));
    if ((((*i2.borrow()).iter_at((-3_i32 as isize)).read()) as i32) != (('x' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow()).iter_at(2_isize).write(('c' as i8));
    let ci1: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new(
        <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(s.decay())),
    ));
    if ((((*ci1.borrow()).iter_deref().read()) as i32) != (('a' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci1.borrow_mut()).iter_inc();
    if ((((*ci1.borrow()).iter_deref().read()) as i32) != (('b' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow_mut()).iter_post_inc().iter_deref().read()) as i32) != (('b' as i8) as i32))
    {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()).iter_deref().read()) as i32) != (('c' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci1.borrow_mut()).iter_inc().iter_inc();
    if ((((*ci1.borrow()).iter_deref().read()) as i32) != (('e' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci1.borrow_mut()).iter_dec();
    if ((((*ci1.borrow()).iter_deref().read()) as i32) != (('d' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow_mut()).iter_post_dec().iter_deref().read()) as i32) != (('d' as i8) as i32))
    {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()).iter_deref().read()) as i32) != (('c' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci1.borrow_mut()).iter_dec().iter_dec();
    if ((((*ci1.borrow()).iter_deref().read()) as i32) != (('a' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let ci2: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    (*ci2.borrow_mut()) =
        <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(s.decay().to_last()));
    let mut d2: isize = 0_isize;
    d2 = ((*ci2.borrow()).iter_distance(&({ (*ci1.borrow()).clone() }).into()) as i64 as isize);
    if (d2 != 5_isize) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let mut v2: i8 = 0_i8;
    v2 = ((*ci1.borrow()).iter_at(0_isize).read());
    if ((v2 as i32) != (('a' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let r2: Ptr<i8> = (*ci1.borrow()).iter_at(0_isize);
    if (((r2.read()) as i32) != (('a' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) != (*ci2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) == (*ci2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) < (*ci2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) > (*ci2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) <= (*ci2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) >= (*ci2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let ci3: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    (*ci3.borrow_mut()) = (*ci1.borrow()).clone();
    if ((((*ci3.borrow()) == (*ci1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci3.borrow_mut()).iter_add_assign(5_isize);
    if ((((*ci3.borrow()) == (*ci2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci3.borrow_mut()).iter_add_assign(-5_isize);
    if ((((*ci3.borrow()) == (*ci1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*ci3.borrow()).iter_add(5_isize) != (*ci2.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*ci3.borrow()).iter_add(5_isize) != (*ci2.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*ci2.borrow()).iter_add(-5_isize) != (*ci3.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()).iter_at(2_isize).read()) as i32) != (('c' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci2.borrow()).iter_at((-1_i32 as isize)).read()) as i32) != (('e' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let ci4: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new(
        <RandomAccessIterator<i8>>::from(({ (*i1.borrow()).clone() })),
    ));
    if ((((*ci4.borrow()) == (*i1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) != (*i1.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) < ({ (*i1.borrow()).clone() }).into()) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) > ({ (*i1.borrow()).clone() }).into()) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) <= ({ (*i1.borrow()).clone() }).into()) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) >= ({ (*i1.borrow()).clone() }).into()) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci4.borrow_mut()) = <RandomAccessIterator<i8>>::from(({ (*i2.borrow()).clone() }));
    if ((((*i2.borrow()) == (*ci4.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i2.borrow()) < ({ (*ci4.borrow()).clone() }).into()) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i2.borrow()) > ({ (*ci4.borrow()).clone() }).into()) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i2.borrow()) <= ({ (*ci4.borrow()).clone() }).into()) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i2.borrow()) >= ({ (*ci4.borrow()).clone() }).into()) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let mut cs: Vec<i8> = {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"ABCDE").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    };
    let ci5: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new(unimplemented!()));
    if ((((*ci5.borrow()).iter_at(0_isize).read()) as i32) != (('A' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    return (*failures.borrow());
}
pub fn vector_stuff_10() -> i32 {
    let failures: Value<i32> = Rc::new(RefCell::new(0));
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 4;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 5;
        (*v.borrow_mut()).push(__a1)
    };
    let i1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().begin()));
    if (((*i1.borrow()).iter_deref().read()) != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow_mut()).iter_inc();
    if (((*i1.borrow()).iter_deref().read()) != 2) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*i1.borrow_mut()).iter_post_inc().iter_deref().read()) != 2) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*i1.borrow()).iter_deref().read()) != 3) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow_mut()).iter_inc().iter_inc();
    if (((*i1.borrow()).iter_deref().read()) != 5) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow_mut()).iter_dec();
    if (((*i1.borrow()).iter_deref().read()) != 4) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*i1.borrow_mut()).iter_post_dec().iter_deref().read()) != 4) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*i1.borrow()).iter_deref().read()) != 3) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow_mut()).iter_dec().iter_dec();
    if (((*i1.borrow()).iter_deref().read()) != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let i2: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    (*i2.borrow_mut()) = v.decay().end();
    let mut d1: isize = 0_isize;
    d1 = ((*i2.borrow()).iter_distance(&({ (*i1.borrow()).clone() }).into()) as i64 as isize);
    if (d1 != 5_isize) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let mut v1: i32 = 0_i32;
    v1 = ((*i1.borrow()).iter_at(0_isize).read());
    if (v1 != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let r1: Ptr<i32> = (*i1.borrow()).iter_at(0_isize);
    if ((r1.read()) != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    r1.write(9);
    if ((r1.read()) != 9) {
        (*failures.borrow_mut()).prefix_inc();
    }
    r1.write(1);
    if ((((*i1.borrow()) != (*i2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) == (*i2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) < (*i2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) > (*i2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) <= (*i2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i1.borrow()) >= (*i2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let i3: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    (*i3.borrow_mut()) = (*i1.borrow()).clone();
    if ((((*i3.borrow()) == (*i1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i3.borrow_mut()).iter_add_assign(5_isize);
    if ((((*i3.borrow()) == (*i2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i3.borrow_mut()).iter_add_assign(-5_isize);
    if ((((*i3.borrow()) == (*i1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*i3.borrow()).iter_add(5_isize) != (*i2.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*i3.borrow()).iter_add(5_isize) != (*i2.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*i2.borrow()).iter_add(-5_isize) != (*i3.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*i1.borrow()).iter_at(0_isize).read()) != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow()).iter_at(4_isize).write(9);
    if (((*i2.borrow()).iter_at((-1_i32 as isize)).read()) != 9) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow()).iter_at(4_isize).write(5);
    (*i1.borrow()).iter_at(2_isize).write(9);
    if (((*i2.borrow()).iter_at((-3_i32 as isize)).read()) != 9) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*i1.borrow()).iter_at(2_isize).write(3);
    let ci1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        <RandomAccessIterator<i32>>::from(v.decay().begin()),
    ));
    if (((*ci1.borrow()).iter_deref().read()) != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci1.borrow_mut()).iter_inc();
    if (((*ci1.borrow()).iter_deref().read()) != 2) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*ci1.borrow_mut()).iter_post_inc().iter_deref().read()) != 2) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*ci1.borrow()).iter_deref().read()) != 3) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci1.borrow_mut()).iter_inc().iter_inc();
    if (((*ci1.borrow()).iter_deref().read()) != 5) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci1.borrow_mut()).iter_dec();
    if (((*ci1.borrow()).iter_deref().read()) != 4) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*ci1.borrow_mut()).iter_post_dec().iter_deref().read()) != 4) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*ci1.borrow()).iter_deref().read()) != 3) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci1.borrow_mut()).iter_dec().iter_dec();
    if (((*ci1.borrow()).iter_deref().read()) != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let ci2: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    (*ci2.borrow_mut()) = <RandomAccessIterator<i32>>::from(v.decay().end());
    let mut d2: isize = 0_isize;
    d2 = ((*ci2.borrow()).iter_distance(&({ (*ci1.borrow()).clone() }).into()) as i64 as isize);
    if (d2 != 5_isize) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let mut v2: i32 = 0_i32;
    v2 = ((*ci1.borrow()).iter_at(0_isize).read());
    if (v2 != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let r2: Ptr<i32> = (*ci1.borrow()).iter_at(0_isize);
    if ((r2.read()) != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) != (*ci2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) == (*ci2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) < (*ci2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) > (*ci2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) <= (*ci2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci1.borrow()) >= (*ci2.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let ci3: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    (*ci3.borrow_mut()) = (*ci1.borrow()).clone();
    if ((((*ci3.borrow()) == (*ci1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci3.borrow_mut()).iter_add_assign(5_isize);
    if ((((*ci3.borrow()) == (*ci2.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci3.borrow_mut()).iter_add_assign(-5_isize);
    if ((((*ci3.borrow()) == (*ci1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*ci3.borrow()).iter_add(5_isize) != (*ci2.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*ci3.borrow()).iter_add(5_isize) != (*ci2.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (*ci2.borrow()).iter_add(-5_isize) != (*ci3.borrow()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*ci1.borrow()).iter_at(2_isize).read()) != 3) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if (((*ci2.borrow()).iter_at((-1_i32 as isize)).read()) != 5) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let ci4: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        <RandomAccessIterator<i32>>::from(({ (*i1.borrow()).clone() })),
    ));
    if ((((*ci4.borrow()) == (*i1.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) != (*i1.borrow())) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) < ({ (*i1.borrow()).clone() }).into()) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) > ({ (*i1.borrow()).clone() }).into()) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) <= ({ (*i1.borrow()).clone() }).into()) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*ci4.borrow()) >= ({ (*i1.borrow()).clone() }).into()) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    (*ci4.borrow_mut()) = <RandomAccessIterator<i32>>::from(({ (*i2.borrow()).clone() }));
    if ((((*i2.borrow()) == (*ci4.borrow())) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i2.borrow()) < ({ (*ci4.borrow()).clone() }).into()) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i2.borrow()) > ({ (*ci4.borrow()).clone() }).into()) as i32) != (false as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i2.borrow()) <= ({ (*ci4.borrow()).clone() }).into()) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    if ((((*i2.borrow()) >= ({ (*ci4.borrow()).clone() }).into()) as i32) != (true as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let cv: Value<Vec<i32>> = Rc::new(RefCell::new((*v.borrow()).clone()));
    let ci5: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(cv.decay().begin()));
    if (((*ci5.borrow()).iter_at(0_isize).read()) != 1) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let vs: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"abc").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        (*vs.borrow_mut()).push(__a1)
    };
    let ivs: Value<RandomAccessIterator<Vec<i8>>> = Rc::new(RefCell::new(vs.decay().begin()));
    if (((elem!(({ (*ivs.borrow()).iter_deref().decay() }), 1).read()) as i32)
        != (('b' as i8) as i32))
    {
        (*failures.borrow_mut()).prefix_inc();
    }
    return (*failures.borrow());
}
pub fn reverse_stuff_11() -> i32 {
    let failures: Value<i32> = Rc::new(RefCell::new(0));
    let mut s: Vec<i8> = {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"abcde").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    };
    let ri: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (((unimplemented!().read()) as i32) != (('e' as i8) as i32)) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let mut d: () = unimplemented!();
    d = (unimplemented!() as ());
    if (d != 5_()) {
        (*failures.borrow_mut()).prefix_inc();
    }
    let mut cs: Vec<i8> = {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"abcde").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    };
    let cri: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if unimplemented!() != unimplemented!() {
        (*failures.borrow_mut()).prefix_inc();
    }
    return (*failures.borrow());
}
pub fn wrong_stuff_12() -> i32 {
    let failures: Value<i32> = Rc::new(RefCell::new(0));
    return (*failures.borrow());
}
pub fn test6642_13() -> i32 {
    let s: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    let it: Value<RandomAccessIterator<i8>> =
        Rc::new(RefCell::new(RandomAccessIterator::Vec(s.decay())));
    let cit: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new(
        <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(s.decay())),
    ));
    return ((*it.borrow()).iter_distance(&({ (*cit.borrow()).clone() }).into()) as i64 as i32);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut failures: i32 = 0;
    failures += ({ string_stuff_9() });
    failures += ({ vector_stuff_10() });
    failures += ({ reverse_stuff_11() });
    failures += ({ wrong_stuff_12() });
    failures += ({ test6642_13() });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(failures == 0) {
            unimplemented!();
            std::process::abort();
        }
    }
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
