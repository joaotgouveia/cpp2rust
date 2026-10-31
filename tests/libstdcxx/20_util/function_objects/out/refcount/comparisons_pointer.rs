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
    pub static b_9: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..8).map(|_| 0_i32).collect::<Box<[i32]>>()));
);
thread_local!(
    pub static a_10: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..8).map(|_| 0_i32).collect::<Box<[i32]>>()));
);
pub fn test01_11() {
    let mut p: Ptr<i32> = (a_10.with(|v| v.as_pointer()) as Ptr<i32>).offset((8) as isize);
    let gt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, unimplemented!());
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, unimplemented!());
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((!(unimplemented!())) && (!(unimplemented!()))));
        __s
    };
    let mut sum: i32 = 0;
    let mut n: i32 = 0;
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "gt(a+1, a)");;
    const _: () = assert!(true, "!gt(a, a+1)");;
    unimplemented!();
    unimplemented!();
    sum = 0;
    let mut p2: Ptr<i32> = (a_10.with(|v| v.as_pointer()) as Ptr<i32>).offset((8) as isize);
    let gt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, unimplemented!());
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, unimplemented!());
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((!(unimplemented!())) && (!(unimplemented!()))));
        __s
    };
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "gt2(a+1, a)");;
    const _: () = assert!(true, "!gt2(a, a+1)");;
}
pub fn test02_12() {
    let mut p: Ptr<i32> = (a_10.with(|v| v.as_pointer()) as Ptr<i32>).offset((8) as isize);
    let lt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, unimplemented!());
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, unimplemented!());
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((!(unimplemented!())) && (!(unimplemented!()))));
        __s
    };
    let mut sum: i32 = 0;
    let mut n: i32 = 0;
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "lt(a, a+1)");;
    const _: () = assert!(true, "!lt(a+1, a)");;
    unimplemented!();
    unimplemented!();
    sum = 0;
    let mut p2: Ptr<i32> = (a_10.with(|v| v.as_pointer()) as Ptr<i32>).offset((8) as isize);
    let lt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, unimplemented!());
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, unimplemented!());
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((!(unimplemented!())) && (!(unimplemented!()))));
        __s
    };
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "lt2(a, a+1)");;
    const _: () = assert!(true, "!lt2(a+1, a)");;
}
pub fn test03_13() {
    let mut p: Ptr<i32> = (a_10.with(|v| v.as_pointer()) as Ptr<i32>).offset((8) as isize);
    let ge: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, !(unimplemented!()));
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, !(unimplemented!()));
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((unimplemented!()) && (unimplemented!())));
        __s
    };
    let mut sum: i32 = 0;
    let mut n: i32 = 0;
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "!ge(a, a+1)");;
    const _: () = assert!(true, "ge(a, a)");;
    const _: () = assert!(true, "ge(a+1, a)");;
    unimplemented!();
    unimplemented!();
    sum = 0;
    let mut p2: Ptr<i32> = (a_10.with(|v| v.as_pointer()) as Ptr<i32>).offset((8) as isize);
    let ge2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, !(unimplemented!()));
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, !(unimplemented!()));
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((unimplemented!()) && (unimplemented!())));
        __s
    };
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "!ge2(a, a+1)");;
    const _: () = assert!(true, "ge2(a, a)");;
    const _: () = assert!(true, "ge2(a+1, a)");;
}
pub fn test04_14() {
    let mut p: Ptr<i32> = (a_10.with(|v| v.as_pointer()) as Ptr<i32>).offset((8) as isize);
    let le: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, !(unimplemented!()));
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, !(unimplemented!()));
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((unimplemented!()) && (unimplemented!())));
        __s
    };
    let mut sum: i32 = 0;
    let mut n: i32 = 0;
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "!le(a+1, a)");;
    const _: () = assert!(true, "le(a, a)");;
    const _: () = assert!(true, "le(a, a+1)");;
    unimplemented!();
    unimplemented!();
    sum = 0;
    let mut p2: Ptr<i32> = (a_10.with(|v| v.as_pointer()) as Ptr<i32>).offset((8) as isize);
    let le2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, !(unimplemented!()));
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, !(unimplemented!()));
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((unimplemented!()) && (unimplemented!())));
        __s
    };
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "!le2(a+1, a)");;
    const _: () = assert!(true, "le2(a, a)");;
    const _: () = assert!(true, "le2(a, a+1)");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(Ptr< X  >  , to_pconstX)]
#[byte_size(1)]
pub struct X {}
thread_local!(
    pub static x_15: Value<X> = Rc::new(RefCell::new(<X>::default()));
);
thread_local!(
    pub static y_16: Value<Box<[X]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <X>::default()).collect::<Box<[X]>>(),
    ));
);
pub fn test05_17() {
    let lt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut p: Ptr<X> = (y_16.with(|v| v.as_pointer()) as Ptr<X>).offset((4) as isize);
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, unimplemented!());
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, unimplemented!());
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((!(unimplemented!())) && (!(unimplemented!()))));
        __s
    };
    let mut sum: i32 = 0;
    let mut n: i32 = 0;
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "lt(y, y+1)");;
    const _: () = assert!(true, "!lt(y+1, y)");;
    unimplemented!();
    unimplemented!();
    sum = 0;
    let mut p2: Ptr<X> = (y_16.with(|v| v.as_pointer()) as Ptr<X>).offset((4) as isize);
    let lt2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = {
                        let __s = (ss.as_pointer() as ::std::fs::File);
                        write_bool_refcount(&__s, unimplemented!());
                        __s
                    };
                    let _ = __s.write_all(&[(' ' as i8) as u8]);
                    __s
                };
                write_bool_refcount(&__s, unimplemented!());
                __s
            };
            let _ = __s.write_all(&[(' ' as i8) as u8]);
            __s
        };
        write_bool_refcount(&__s, ((!(unimplemented!())) && (!(unimplemented!()))));
        __s
    };
    'loop_: while unimplemented!() {
        sum += n;
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(sum == 1) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "lt2(y, y+1)");;
    const _: () = assert!(true, "!lt2(y+1, y)");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Overloaded {}
impl CppLess for Overloaded {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_18(__lhs.read(), __rhs.read())
    }
}
impl CppEq for Overloaded {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_18(__lhs.read(), __rhs.read()))
            && !(operator_lt_18(__rhs.read(), __lhs.read()))
    }
}
pub fn operator_gt_19(mut _a0: Overloaded, mut _a1: Overloaded) -> bool {
    return false;
}
pub fn operator_lt_18(mut _a0: Overloaded, mut _a1: Overloaded) -> bool {
    return false;
}
pub fn operator_ge_20(mut _a0: Overloaded, mut _a1: Overloaded) -> bool {
    return true;
}
pub fn operator_le_21(mut _a0: Overloaded, mut _a1: Overloaded) -> bool {
    return true;
}
pub fn test06_22() {
    let gt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let lt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ge: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let le: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut o: Overloaded = <Overloaded>::default();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
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
    ({ test01_11() });
    ({ test02_12() });
    ({ test03_13() });
    ({ test04_14() });
    ({ test05_17() });
    ({ test06_22() });
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
pub fn set_memory_limits_23(size: Option<f32>) {
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
pub fn set_file_limit_24(mut size: u64) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_25(mut mangled: Ptr<i8>, mut wanted: Ptr<i8>) {
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
pub fn run_tests_wrapped_locale_26(mut name: Ptr<i8>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_27(
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
    if !(libc::setenv(env, name, 1) != 0) {
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: i32 = 0;
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        libc::setenv(
            env,
            if !(oldENV).is_null() {
                (oldENV).clone()
            } else {
                Ptr::<i8>::from_string_literal(b"")
            },
            1,
        );
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
pub fn test_tm_28(
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
    (*tmp_29.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_29.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_29: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
);
pub trait OverloadedImpl {
    fn operator_gt(&self, _a0: i32) -> bool;
    fn operator_lt(&self, _a0: i32) -> bool;
    fn operator_ge(&self, _a0: i32) -> bool;
    fn operator_le(&self, _a0: i32) -> bool;
}
impl OverloadedImpl for Ptr<Overloaded> {
    fn operator_gt(&self, mut _a0: i32) -> bool {
        return true;
    }
    fn operator_lt(&self, mut _a0: i32) -> bool {
        return false;
    }
    fn operator_ge(&self, mut _a0: i32) -> bool {
        return true;
    }
    fn operator_le(&self, mut _a0: i32) -> bool {
        return false;
    }
}
pub trait XImpl {
    fn to_pconstX(&self) -> Ptr<X>;
}
impl XImpl for Ptr<X> {
    fn to_pconstX(&self) -> Ptr<X> {
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
    let _ = b_9.with(|_| ());
    let _ = a_10.with(|_| ());
    let _ = x_15.with(|_| ());
    let _ = y_16.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
