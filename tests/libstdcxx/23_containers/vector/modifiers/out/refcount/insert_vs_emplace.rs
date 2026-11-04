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
#[byte_size(24)]
pub struct X_special {
    #[offset(0)]
    pub c: u32,
    #[offset(4)]
    pub dt: u32,
    #[offset(8)]
    pub cc: u32,
    #[offset(12)]
    pub ca: u32,
    #[offset(16)]
    pub mc: u32,
    #[offset(20)]
    pub ma: u32,
}
impl CppMove for X_special {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(
    Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Destructor, Default,
)]
#[byte_size(16)]
pub struct X {
    #[offset(0)]
    i_: i32,
    #[offset(8)]
    #[byte_size(8)]
    p_: Ptr<i32>,
}
impl X {
    pub fn new(mut i: i32, mut p: Ptr<i32>) -> Self {
        let __this: X = Self {
            i_: i,
            p_: (p).clone(),
        };
        (*sp_9.with(Value::clone).borrow_mut()).c.postfix_inc();
        __this
    }
    pub fn copy_from(x: Ptr<X>) -> Self {
        let __this: X = Self {
            i_: x.with(|__s| __s.i_),
            p_: x.with(|__s| __s.p_.clone()),
        };
        (*sp_9.with(Value::clone).borrow_mut()).cc.postfix_inc();
        __this
    }
    pub fn move_from(x: Ptr<X>) -> Self {
        let __this: X = Self {
            i_: x.with(|__s| __s.i_),
            p_: x.with(|__s| __s.p_.clone()),
        };
        (*sp_9.with(Value::clone).borrow_mut()).mc.postfix_inc();
        __this
    }
}
impl Clone for X {
    fn clone(&self) -> Self {
        let __src: Value<X> = Rc::new(RefCell::new(X {
            i_: self.i_.clone(),
            p_: self.p_.clone(),
        }));
        X::copy_from(__src.as_pointer())
    }
}
impl CppMove for X {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_shl_10(os: Ptr<::std::fs::File>, sp: Ptr<X_special>) -> Ptr<::std::fs::File> {
    {
        let __s = {
            let __s = (os).clone();
            write_int_refcount(&__s, sp.with(|__s| __s.c));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = (os).clone();
            write_int_refcount(&__s, sp.with(|__s| __s.dt));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = (os).clone();
            write_int_refcount(&__s, sp.with(|__s| __s.cc));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = (os).clone();
            write_int_refcount(&__s, sp.with(|__s| __s.ca));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = (os).clone();
            write_int_refcount(&__s, sp.with(|__s| __s.mc));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = (os).clone();
            write_int_refcount(&__s, sp.with(|__s| __s.ma));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    return (os).clone();
}
thread_local!(
    pub static sp_9: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
);
pub fn operator_eq_11(lhs: Ptr<X_special>, rhs: Ptr<X_special>) -> bool {
    return ((((({ lhs.with(|__s| __s.c) } == { rhs.with(|__s| __s.c) })
        && ({ lhs.with(|__s| __s.dt) } == { rhs.with(|__s| __s.dt) }))
        && ({ lhs.with(|__s| __s.cc) } == { rhs.with(|__s| __s.cc) }))
        && ({ lhs.with(|__s| __s.ca) } == { rhs.with(|__s| __s.ca) }))
        && ({ lhs.with(|__s| __s.mc) } == { rhs.with(|__s| __s.mc) }))
        && ({ lhs.with(|__s| __s.ma) } == { rhs.with(|__s| __s.ma) });
}
pub fn test01_12() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 0_u32,
        dt: 1_u32,
        cc: 1_u32,
        ca: 0_u32,
        mc: 1_u32,
        ma: 3_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 4_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        match ({ <RandomAccessIterator<X>>::from(v.decay().begin()) },) {
            (mut __tmp_0,) => {
                let __off = __tmp_0.index();
                (*v.borrow_mut()).insert(__off, ({ (*x.borrow()).clone() }));
                (__tmp_0).clone()
            }
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 4_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        unimplemented!();
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test02_13() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 0_u32,
        dt: 0_u32,
        cc: 0_u32,
        ca: 0_u32,
        mc: 1_u32,
        ma: 3_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 4_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        match ({ <RandomAccessIterator<X>>::from(v.decay().begin()) },) {
            (mut __tmp_1,) => {
                let __off = __tmp_1.index();
                (*v.borrow_mut()).insert(__off, X::move_from({ x.as_pointer() }));
                (__tmp_1).clone()
            }
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 4_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        unimplemented!();
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test03_14() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 1_u32,
        dt: 1_u32,
        cc: 0_u32,
        ca: 0_u32,
        mc: 1_u32,
        ma: 3_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 4_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        match ({ <RandomAccessIterator<X>>::from(v.decay().begin()) },) {
            (mut __tmp_2,) => {
                let __off = __tmp_2.index();
                (*v.borrow_mut()).insert(__off, X::new({ 0 }, { Ptr::<i32>::null() }));
                (__tmp_2).clone()
            }
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 4_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        unimplemented!();
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test04_15() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 0_u32,
        dt: 3_u32,
        cc: 1_u32,
        ca: 0_u32,
        mc: 3_u32,
        ma: 0_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 3_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        match ({ <RandomAccessIterator<X>>::from(v.decay().begin()) },) {
            (mut __tmp_3,) => {
                let __off = __tmp_3.index();
                (*v.borrow_mut()).insert(__off, ({ (*x.borrow()).clone() }));
                (__tmp_3).clone()
            }
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 3_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        unimplemented!();
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test05_16() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 0_u32,
        dt: 3_u32,
        cc: 0_u32,
        ca: 0_u32,
        mc: 4_u32,
        ma: 0_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 3_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        match ({ <RandomAccessIterator<X>>::from(v.decay().begin()) },) {
            (mut __tmp_4,) => {
                let __off = __tmp_4.index();
                (*v.borrow_mut()).insert(__off, X::move_from({ x.as_pointer() }));
                (__tmp_4).clone()
            }
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 3_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        unimplemented!();
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test06_17() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 1_u32,
        dt: 4_u32,
        cc: 0_u32,
        ca: 0_u32,
        mc: 4_u32,
        ma: 0_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 3_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        match ({ <RandomAccessIterator<X>>::from(v.decay().begin()) },) {
            (mut __tmp_5,) => {
                let __off = __tmp_5.index();
                (*v.borrow_mut()).insert(__off, X::new({ 0 }, { Ptr::<i32>::null() }));
                (__tmp_5).clone()
            }
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let v: Value<Vec<X>> = Rc::new(RefCell::new(Vec::new()));
        if 3_usize as usize > (*v.borrow()).capacity() as usize {
            let len_0 = (*v.borrow()).len();
            (*v.borrow_mut()).reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            (*v.borrow_mut()).push(__a1)
        };
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        unimplemented!();
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test07_18() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 0_u32,
        dt: 0_u32,
        cc: 1_u32,
        ca: 0_u32,
        mc: 0_u32,
        ma: 0_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let mut v: Vec<X> = Vec::new();
        if 4_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let a0_clone = (*x.borrow()).clone();
            v.push(a0_clone)
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let mut v: Vec<X> = Vec::new();
        if 4_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __init = X::copy_from({ x.as_pointer() });
            v.push(__init)
        };
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test08_19() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 0_u32,
        dt: 0_u32,
        cc: 0_u32,
        ca: 0_u32,
        mc: 1_u32,
        ma: 0_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let mut v: Vec<X> = Vec::new();
        if 4_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __a1 = X::move_from({ x.as_pointer() });
            v.push(__a1)
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let mut v: Vec<X> = Vec::new();
        if 4_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __init = X::move_from({ x.as_pointer() });
            v.push(__init)
        };
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test09_20() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 1_u32,
        dt: 1_u32,
        cc: 0_u32,
        ca: 0_u32,
        mc: 1_u32,
        ma: 0_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let mut v: Vec<X> = Vec::new();
        if 4_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let mut v: Vec<X> = Vec::new();
        if 4_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(4_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __init = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__init)
        };
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test10_21() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 0_u32,
        dt: 3_u32,
        cc: 1_u32,
        ca: 0_u32,
        mc: 3_u32,
        ma: 0_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let mut v: Vec<X> = Vec::new();
        if 3_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let a0_clone = (*x.borrow()).clone();
            v.push(a0_clone)
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let mut v: Vec<X> = Vec::new();
        if 3_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __init = X::copy_from({ x.as_pointer() });
            v.push(__init)
        };
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test11_22() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 0_u32,
        dt: 3_u32,
        cc: 0_u32,
        ca: 0_u32,
        mc: 4_u32,
        ma: 0_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let mut v: Vec<X> = Vec::new();
        if 3_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __a1 = X::move_from({ x.as_pointer() });
            v.push(__a1)
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let mut v: Vec<X> = Vec::new();
        if 3_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        let x: Value<X> = Rc::new(RefCell::new(X::new({ 0 }, { Ptr::<i32>::null() })));
        let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __init = X::move_from({ x.as_pointer() });
            v.push(__init)
        };
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn test12_23() {
    let expected: Value<X_special> = Rc::new(RefCell::new(X_special {
        c: 1_u32,
        dt: 4_u32,
        cc: 0_u32,
        ca: 0_u32,
        mc: 4_u32,
        ma: 0_u32,
    }));
    let ins: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    let emp: Value<X_special> = Rc::new(RefCell::new(<X_special>::default()));
    {
        let mut v: Vec<X> = Vec::new();
        if 3_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        (*ins.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    {
        let mut v: Vec<X> = Vec::new();
        if 3_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(3_usize as usize - len_0 as usize);
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        {
            let __a1 = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__a1)
        };
        sp_9.with(|rc| *rc.borrow_mut() = <X_special>::default());
        {
            let __init = X::new({ 0 }, { Ptr::<i32>::null() });
            v.push(__init)
        };
        (*emp.borrow_mut()) = (*sp_9.with(Value::clone).borrow()).clone();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, emp.as_pointer())
        }) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let _lhs: Ptr<X_special> = ins.as_pointer();
            operator_eq_11(_lhs, expected.as_pointer())
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
    ({ test01_12() });
    ({ test02_13() });
    ({ test03_14() });
    ({ test04_15() });
    ({ test05_16() });
    ({ test06_17() });
    ({ test07_18() });
    ({ test08_19() });
    ({ test09_20() });
    ({ test10_21() });
    ({ test11_22() });
    ({ test12_23() });
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
pub fn set_memory_limits_24(size: Option<f32>) {
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
pub fn set_file_limit_25(mut size: u64) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_26(mut mangled: Ptr<i8>, mut wanted: Ptr<i8>) {
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
pub fn run_tests_wrapped_locale_27(mut name: Ptr<i8>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_28(
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
pub fn test_tm_29(
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
    (*tmp_30.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_30.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_30: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
);
pub trait XImpl {
    fn destructor(&self);
    fn copy_assign(&self, x: Ptr<X>) -> Ptr<X>;
    fn move_assign(&self, x: Ptr<X>) -> Ptr<X>;
}
impl XImpl for Ptr<X> {
    fn destructor(&self) {
        (*sp_9.with(Value::clone).borrow_mut()).dt.postfix_inc();
    }
    fn copy_assign(&self, x: Ptr<X>) -> Ptr<X> {
        field!((*self), i_).write({ x.with(|__s| __s.i_) });
        field!((*self), p_).write({ x.with(|__s| __s.p_.clone()) });
        (*sp_9.with(Value::clone).borrow_mut()).ca.postfix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, x: Ptr<X>) -> Ptr<X> {
        field!((*self), i_).write({ x.with(|__s| __s.i_) });
        field!((*self), p_).write({ x.with(|__s| __s.p_.clone()) });
        (*sp_9.with(Value::clone).borrow_mut()).ma.postfix_inc();
        return (*self).clone();
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
    let _ = sp_9.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
