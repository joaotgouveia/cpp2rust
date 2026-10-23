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
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"copy_constructor::mark_call",
                ))
            });
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
        return count__4.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__4.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__4.with(|rc| *rc.borrow()) == throw_on__5.with(|rc| *rc.borrow())) {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"assignment_operator::mark_call",
                ))
            });
        }
    }
    pub fn reset() {
        count__4.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__5.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__5.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub fn count() -> () {
        return _M_count_6.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*_M_count_6.with(Value::clone).borrow_mut()).postfix_inc();
    }
    pub fn reset() {
        _M_count_6.with(|rc| *rc.borrow_mut() = 0_());
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
            id.unwrap_or_else(|| (*next_id__7.with(Value::clone).borrow_mut()).postfix_dec());
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
impl std::cmp::Ord for __gnu_test_copy_tracker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_8(
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: self.id_.clone(),
                    throw_on_copy_: self.throw_on_copy_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: other.id_.clone(),
                    throw_on_copy_: other.throw_on_copy_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_8(
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: other.id_.clone(),
                    throw_on_copy_: other.throw_on_copy_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: self.id_.clone(),
                    throw_on_copy_: self.throw_on_copy_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for __gnu_test_copy_tracker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for __gnu_test_copy_tracker {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_9(
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: self.id_.clone(),
                    throw_on_copy_: self.throw_on_copy_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: other.id_.clone(),
                    throw_on_copy_: other.throw_on_copy_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_copy_tracker {}
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
pub fn operator_eq_9(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } == {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
pub fn operator_lt_8(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
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
    pid_: ::libc::pid_t,
}
impl __gnu_test_semaphore {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<()>,
}
impl __gnu_test_BoundsContainer_int_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_OutputContainer_int_ {
    #[offset(16)]
    #[byte_size(8)]
    pub incrementedto: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub writtento: Ptr<()>,
}
impl __gnu_test_OutputContainer_int_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        let __this: Value<__gnu_test_OutputContainer_int_> = Rc::new(RefCell::new(Self {
            incrementedto: (_first).clone(),
            writtento: Ptr::alloc_array(
                (0..({ __gnu_test_BoundsContainer_int_Impl::size(&this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
        }));
        let this: Ptr<__gnu_test_OutputContainer_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_WritableObject_int_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_OutputContainer_int_>,
}
impl __gnu_test_WritableObject_int_ {
    pub fn new(
        mut ptr_in: Ptr<()>,
        mut SharedInfo_in: Ptr<__gnu_test_OutputContainer_int_>,
    ) -> Self {
        Self {
            ptr: (ptr_in).clone(),
            SharedInfo: (SharedInfo_in).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_output_iterator_wrapper_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_OutputContainer_int_>,
}
impl __gnu_test_output_iterator_wrapper_int_ {
    pub fn new_1(
        mut _ptr: Ptr<()>,
        mut SharedInfo_in: Ptr<__gnu_test_OutputContainer_int_>,
    ) -> Self {
        let __this: Value<__gnu_test_output_iterator_wrapper_int_> = Rc::new(RefCell::new(Self {
            ptr: (_ptr).clone(),
            SharedInfo: (SharedInfo_in).clone(),
        }));
        let this: Ptr<__gnu_test_output_iterator_wrapper_int_> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ this.with(|__s| __s.ptr.clone()) } >= {
                (this.with(|__s| __s.SharedInfo.clone()) as *mut __gnu_test_BoundsContainer_int_)
                    .with(|__s| __s.first.clone())
            }) && ({ this.with(|__s| __s.ptr.clone()) } <= {
                (this.with(|__s| __s.SharedInfo.clone()) as *mut __gnu_test_BoundsContainer_int_)
                    .with(|__s| __s.last.clone())
            })) {
                ({
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"2.cc" )  ) .into() , ( 148_() ) .into() , ( Ptr::<() >::from_string_literal(b"__gnu_test::output_iterator_wrapper<int>::output_iterator_wrapper(T *, ContainerType *) [T = int]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr >= SharedInfo->first && ptr <= SharedInfo->last" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ {
    pub fn new_1(arr: Ptr<()>) -> Self {
        __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_2({ (arr).clone() }, {
            (arr).offset((3_()) as isize)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
struct __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator {}
pub fn operator_eq_11(
    s : Ptr< __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__sentinel___gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator_  >,
    i: Ptr<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator>,
) -> () {
    return ({ s.with(|__s| __s.end.clone()) } == {
        i.with(|__s| (__s as __gnu_test_output_iterator_wrapper_int_).ptr.clone())
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__sentinel___gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator_
{
    #[offset(0)]
    #[byte_size(8)]
    pub end: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(32)]
    pub bounds: __gnu_test_OutputContainer_int_,
}
impl __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ {
    pub fn new_2(mut first: Ptr<()>, mut last: Ptr<()>) -> Self {
        Self {
            bounds: __gnu_test_OutputContainer_int_::new({ (first).clone() }, { (last).clone() }),
        }
    }
}
pub fn test01_12() {
    let x: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let rx: Value<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper_> =
        Rc::new(RefCell::new(
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_1({
                x.as_pointer()
            }),
        ));
    let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
    let r0: Value<()> = Rc::new(RefCell::new(
        ({ (*iota_13.with(Value::clone).borrow_mut())(rx.as_pointer(), 0_()) }),
    ));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            {
                ((*r0.borrow()).out as __gnu_test_output_iterator_wrapper_int_)
                    .ptr
                    .clone()
            }
        } == { (x.as_pointer() as Ptr<()>).offset((3_()) as isize) })
        {
            ({
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"2.cc")).into(),
                        (19_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"r0.out.ptr == x+3")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*r0.borrow()).value } == 3_()) {
            ({
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"2.cc")).into(),
                        (20_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"r0.value == 3")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let ___r2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 2_()])));
            (*equal_14.with(Value::clone).borrow_mut())(
                x.as_pointer(),
                ___r2.as_pointer(),
                None,
                None,
                None,
            )
        }) {
            ({
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"2.cc")).into(),
                        (21_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"ranges::equal(x, (int[]){0,1,2})"))
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let r1: Value<()> = Rc::new(RefCell::new(
        ({
            let ___first: Ptr<()> = (x.as_pointer() as Ptr<()>);
            let ___last: Ptr<()> = (x.as_pointer() as Ptr<()>).offset((2_()) as isize);
            (*iota_13.with(Value::clone).borrow_mut())(___first, ___last, 5_())
        }),
    ));
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ { (*r1.borrow()).out.clone() } } == {
            (x.as_pointer() as Ptr<()>).offset((2_()) as isize)
        }) {
            ({
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"2.cc")).into(),
                        (23_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"r1.out == x+2")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({ (*r1.borrow()).value } == 7_()) {
            ({
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"2.cc")).into(),
                        (24_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"r1.value == 7")).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !({
            let ___r2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([5_(), 6_(), 2_()])));
            (*equal_14.with(Value::clone).borrow_mut())(
                x.as_pointer(),
                ___r2.as_pointer(),
                None,
                None,
                None,
            )
        }) {
            ({
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                    Box::new([
                        (Ptr::<()>::from_string_literal(b"2.cc")).into(),
                        (25_()).into(),
                        (Ptr::<()>::from_string_literal(b"void test01()")).into(),
                        (Ptr::<()>::from_string_literal(b"ranges::equal(x, (int[]){5,6,2})"))
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_12() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write((({ getpid_15() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_16(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
        if (this.with(|__s| __s.sem_set_) == -1_()) {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not obtain semaphore set",
                ))
            });
        }
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if (({
            semctl_17(
                this.with(|__s| __s.sem_set_),
                0_(),
                16_(),
                Box::new([((*val.borrow()).clone()).into()]),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not initialize semaphore",
                ))
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn set_memory_limits_18(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_19((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_20((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_19((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_20((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_19((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_20((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_21(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_19((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_20((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_22(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_23(
            (mangled).clone(),
            Ptr::<()>::null(),
            Ptr::<()>::null(),
            (status.as_pointer()),
        )
    });
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
    let w: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { (wanted).clone() },
            None,
        ),
    ));
    if !(unimplemented!()) {
        ({ __throw_runtime_error_3((s).clone()) });
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_24(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut res: Ptr<()> = unimplemented!();
    if !(res).is_null() {
        let mut preLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (res).clone() },
                None,
            );
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: () = 0_();
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { unimplemented!() },
                None,
            );
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                ({
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { Ptr::<()>::from_string_literal(b"LC_ALL for ") },
            None,
        );
        unimplemented!();
        ({ __throw_runtime_error_3(unimplemented!()) });
    }
}
pub fn run_tests_wrapped_env_25(
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
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { (env).clone() },
            None,
        );
        unimplemented!();
        unimplemented!();
        ({ __throw_runtime_error_3(unimplemented!()) });
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
    static count__4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static _M_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static next_id__7: Value<()> = Rc::new(RefCell::new(0_()));
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
    pub fn buf(this: Ptr<Self>) -> Ptr<Ptr<semid_ds>> {
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
pub fn test_tm_27(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> ::libc::tm {
    (*tmp_28.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_28.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_28.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_28: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
);
pub trait __gnu_test_BoundsContainer_int_Impl {
    fn size(&self) -> ();
}
impl __gnu_test_BoundsContainer_int_Impl for Ptr<__gnu_test_BoundsContainer_int_> {
    fn size(&self) -> () {
        return ((*self).with(|__s| __s.last.clone()) - (*self).with(|__s| __s.first.clone()))
            as ();
    }
}
pub trait __gnu_test_OutputContainer_int_Impl {
    fn destructor(&self);
}
impl __gnu_test_OutputContainer_int_Impl for Ptr<__gnu_test_OutputContainer_int_> {
    fn destructor(&self) {
        (*self).with(|__s| __s.writtento.clone()).delete();
    }
}
pub trait __gnu_test_WritableObject_int_Impl {
    fn operator_assign_2_const_int_ref(&self, new_val: Ptr<()>);
}
impl __gnu_test_WritableObject_int_Impl for Ptr<__gnu_test_WritableObject_int_> {
    fn operator_assign_2_const_int_ref(&self, new_val: Ptr<()>) {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((elem!(
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.writtento.clone()),
                ((*self).with(|__s| __s.ptr.clone())
                    - ((*self).with(|__s| __s.SharedInfo.clone())
                        as *mut __gnu_test_BoundsContainer_int_)
                        .with(|__s| __s.first.clone())) as ()
            )
            .read())
                == 0_())
            {
                ({
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"2.cc" )  ) .into() , ( 104_() ) .into() , ( Ptr::<() >::from_string_literal(b"typename std::enable_if<std::is_assignable<T &, U>::value>::type __gnu_test::WritableObject<int>::operator=(U &&) const [T = int, U = const int &]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"SharedInfo->writtento[ptr - SharedInfo->first] == 0" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        elem!(
            (*self)
                .with(|__s| __s.SharedInfo.clone())
                .with(|__s| __s.writtento.clone()),
            ((*self).with(|__s| __s.ptr.clone())
                - ((*self).with(|__s| __s.SharedInfo.clone())
                    as *mut __gnu_test_BoundsContainer_int_)
                    .with(|__s| __s.first.clone())) as ()
        )
        .write((1_() != 0));
        (*self)
            .with(|__s| __s.ptr.clone())
            .write({ (new_val.read()) });
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
pub trait __gnu_test_output_iterator_wrapper_int_Impl {
    fn operator_deref(&self) -> __gnu_test_WritableObject_int_;
    fn operator_inc_6(&self) -> Ptr<__gnu_test_output_iterator_wrapper_int_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> __gnu_test_output_iterator_wrapper_int_ {
        unimplemented!()
    }
}
impl __gnu_test_output_iterator_wrapper_int_Impl for Ptr<__gnu_test_output_iterator_wrapper_int_> {
    fn operator_deref(&self) -> __gnu_test_WritableObject_int_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } < {
                ((*self).with(|__s| __s.SharedInfo.clone()) as *mut __gnu_test_BoundsContainer_int_)
                    .with(|__s| __s.last.clone())
            }) {
                ({
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"2.cc" )  ) .into() , ( 161_() ) .into() , ( Ptr::<() >::from_string_literal(b"WritableObject<T> __gnu_test::output_iterator_wrapper<int>::operator*() const [T = int]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr < SharedInfo->last" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((elem!(
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.writtento.clone()),
                ((*self).with(|__s| __s.ptr.clone())
                    - ((*self).with(|__s| __s.SharedInfo.clone())
                        as *mut __gnu_test_BoundsContainer_int_)
                        .with(|__s| __s.first.clone())) as ()
            )
            .read())
                == false)
            {
                ({
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"2.cc" )  ) .into() , ( 162_() ) .into() , ( Ptr::<() >::from_string_literal(b"WritableObject<T> __gnu_test::output_iterator_wrapper<int>::operator*() const [T = int]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"SharedInfo->writtento[ptr - SharedInfo->first] == false" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return __gnu_test_WritableObject_int_::new({ (*self).with(|__s| __s.ptr.clone()) }, {
            (*self).with(|__s| __s.SharedInfo.clone())
        });
    }
    fn operator_inc_6(&self) -> Ptr<__gnu_test_output_iterator_wrapper_int_> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    ((*self).with(|__s| __s.SharedInfo.clone())
                        as *mut __gnu_test_BoundsContainer_int_)
                        .with(|__s| __s.last.clone())
                }))
            {
                ({
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"2.cc" )  ) .into() , ( 169_() ) .into() , ( Ptr::<() >::from_string_literal(b"output_iterator_wrapper<T> &__gnu_test::output_iterator_wrapper<int>::operator++() [T = int]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"SharedInfo && ptr < SharedInfo->last" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.incrementedto.clone())
            }) {
                ({
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"2.cc" )  ) .into() , ( 170_() ) .into() , ( Ptr::<() >::from_string_literal(b"output_iterator_wrapper<T> &__gnu_test::output_iterator_wrapper<int>::operator++() [T = int]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr>=SharedInfo->incrementedto" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        field!((*self), ptr).with_mut(|__v| __v.postfix_inc());
        field!((*self).with(|__s| __s.SharedInfo.clone()), incrementedto)
            .write((*self).with(|__s| __s.ptr.clone()));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_15() })) {
            ({
                semctl_17(
                    (*self).with(|__s| __s.sem_set_),
                    0_(),
                    0_(),
                    Box::new([((*val.borrow()).clone()).into()]),
                )
            });
        }
    }
    fn signal(&self) {
        let op: Value<Box<[sembuf]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (({
            semop_26(
                (*self).with(|__s| __s.sem_set_),
                (op.as_pointer() as Ptr<sembuf>),
                1_(),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not signal semaphore",
                ))
            });
        }
    }
    fn wait(&self) {
        let op: Value<Box<[sembuf]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (({
            semop_26(
                (*self).with(|__s| __s.sem_set_),
                (op.as_pointer() as Ptr<sembuf>),
                1_(),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not wait for semaphore",
                ))
            });
        }
    }
}
pub trait __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_Impl {
    fn get_iterator(
        &self,
        p: Ptr<()>,
    ) -> __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator;
    fn begin(&self) -> __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator;
    fn end ( &self , ) -> __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__sentinel___gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator_  ;
    fn destructor(&self);
}
impl __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper_>
{
    fn get_iterator(
        &self,
        mut p: Ptr<()>,
    ) -> __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator {
        {
            return __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator::new_1(
                { (p).clone() },
                { (field_ptr!((*self), bounds)) },
            );
        }
        panic!("ub: non-void function does not return a value")
    }
    fn begin(&self) -> __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator {
        return ({
            let _p: Ptr<()> = (*self).with(|__s| {
                (__s.bounds as __gnu_test_BoundsContainer_int_)
                    .first
                    .clone()
            });
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_Impl::get_iterator(
                self, _p,
            )
        });
    }    fn end ( &self , ) -> __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__sentinel___gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator_{
        return __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__sentinel___gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator_ { end : (* self ) .with(|__s| ( __s . bounds  as __gnu_test_BoundsContainer_int_ ) . last .clone())  , }   ;
    }
    fn destructor(&self) {
        field_ptr!(self, bounds).destructor();
    }
}
pub trait __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iteratorImpl {
    fn operator_inc(
        &self,
    ) -> Ptr<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator>;
}
impl __gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iteratorImpl
    for Ptr<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator>
{
    fn operator_inc(
        &self,
    ) -> Ptr<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper__iterator> {
        ({ __gnu_test_output_iterator_wrapper_int_Impl::operator_inc_6(self) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__4.with(|_| ());
    let _ = throw_on__5.with(|_| ());
    let _ = _M_count_6.with(|_| ());
    let _ = next_id__7.with(|_| ());
}
