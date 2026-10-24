extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn compare_paths_0(p1: Ptr<()>, p2: Ptr<()>) {
    if unimplemented!() {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if unimplemented!() {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    let mut d1: () = unimplemented!();
    let mut d2: () = unimplemented!();
    if (d1 != d2) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
}
thread_local!(
    pub static test_paths_1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
);
pub fn root_path_2() -> () {
    return unimplemented!();
}
pub fn nonexistent_path_3(file: Option<()>) -> () {
    let file: Value<()> = Rc::new(RefCell::new(file.unwrap_or_else(|| unimplemented!())));
    if (unimplemented!() > 3_()) && (unimplemented!() == 0_()) {
        unimplemented!();
    }
    let mut pos: () = unimplemented!();
    if (pos != npos_4.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut fd: () = ({ mkstemp_6((unimplemented!())) });
    if (fd == -1_()) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub type __gnu_test_scoped_file_adopt_file_t = ();
pub const __gnu_test_scoped_file_adopt_file_t_adopt_file: __gnu_test_scoped_file_adopt_file_t = 0;
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Destructor)]
#[byte_size(40)]
pub struct __gnu_test_scoped_file {
    #[offset(0)]
    #[byte_size(40)]
    pub path: (),
}
impl __gnu_test_scoped_file {
    pub fn new_1(p: Option<Ptr<()>>) -> Self {
        let mut __p_default: Option<Value<()>> = None;
        let p: Ptr<()> = p.unwrap_or_else(|| {
            __p_default
                .insert(Rc::new(RefCell::new(({ nonexistent_path_3(None) }))))
                .as_pointer()
        });
        let __this: __gnu_test_scoped_file = Self {
            path: unimplemented!(),
        };
        unimplemented!();
        __this
    }
    pub fn new_2(p: (), mut _a1: __gnu_test_scoped_file_adopt_file_t) -> Self {
        let p: Value<()> = Rc::new(RefCell::new(p));
        Self {
            path: unimplemented!(),
        }
    }
    pub fn move_from(_a0: Ptr<__gnu_test_scoped_file>) -> Self {
        Self {
            path: unimplemented!(),
        }
    }
}
impl Default for __gnu_test_scoped_file {
    fn default() -> Self {
        { __gnu_test_scoped_file::new_1(None) }
    }
}
pub fn permissions_are_testable_7(print_msg: Option<()>) -> () {
    let mut print_msg: () = print_msg.unwrap_or_else(|| true);
    let mut testable: () = false;
    if (unimplemented!() != 0_()) {
        testable = true;
    }
    if (print_msg) && (!(testable)) {
        libc::puts(Ptr::<()>::from_string_literal(
            b"Skipping tests that depend on filesystem permissions",
        ));
    }
    return testable;
}
thread_local!(
    static counter_5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
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
        (*count_8.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<__gnu_test_object_counter>) -> Self {
        let __this: __gnu_test_object_counter = Self {};
        (*count_8.with(Value::clone).borrow_mut()).prefix_inc();
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
        return count__9.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__9.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__9.with(|rc| *rc.borrow()) == throw_on__10.with(|rc| *rc.borrow())) {
            ({
                __throw_runtime_error_11(Ptr::<()>::from_string_literal(
                    b"copy_constructor::mark_call",
                ))
            });
        }
    }
    pub fn reset() {
        count__9.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__10.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__10.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub fn count() -> () {
        return count__12.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__12.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__12.with(|rc| *rc.borrow()) == throw_on__13.with(|rc| *rc.borrow())) {
            ({
                __throw_runtime_error_11(Ptr::<()>::from_string_literal(
                    b"assignment_operator::mark_call",
                ))
            });
        }
    }
    pub fn reset() {
        count__12.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__13.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__13.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub fn count() -> () {
        return _M_count_14.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*_M_count_14.with(Value::clone).borrow_mut()).postfix_inc();
    }
    pub fn reset() {
        _M_count_14.with(|rc| *rc.borrow_mut() = 0_());
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
            id.unwrap_or_else(|| (*next_id__15.with(Value::clone).borrow_mut()).postfix_dec());
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
            if operator_lt_16(
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
            } else if operator_lt_16(
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
            operator_eq_17(
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
pub fn operator_eq_17(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } == {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
pub fn operator_lt_16(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
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
pub struct __gnu_test_BoundsContainer_const_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<()>,
}
impl __gnu_test_BoundsContainer_const_char_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_const_wchar_t_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<()>,
}
impl __gnu_test_BoundsContainer_const_wchar_t_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_input_iterator_wrapper_const_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_BoundsContainer_const_char_>,
}
impl __gnu_test_input_iterator_wrapper_const_char_ {
    pub fn new_1(
        mut _ptr: Ptr<()>,
        mut SharedInfo_in: Ptr<__gnu_test_BoundsContainer_const_char_>,
    ) -> Self {
        let __this: Value<__gnu_test_input_iterator_wrapper_const_char_> =
            Rc::new(RefCell::new(Self {
                ptr: (_ptr).clone(),
                SharedInfo: (SharedInfo_in).clone(),
            }));
        let this: Ptr<__gnu_test_input_iterator_wrapper_const_char_> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ this.with(|__s| __s.ptr.clone()) } >= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ this.with(|__s| __s.ptr.clone()) } <= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.last.clone())
            })) {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 250_() ) .into() , ( Ptr::<() >::from_string_literal(b"__gnu_test::input_iterator_wrapper<const char>::input_iterator_wrapper(T *, ContainerType *) [T = const char]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr >= SharedInfo->first && ptr <= SharedInfo->last" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for __gnu_test_input_iterator_wrapper_const_char_ {
    fn eq(&self, other: &Self) -> bool {
        {
            __gnu_test_input_iterator_wrapper_const_char_Impl::operator_eq(
                &Rc::new(RefCell::new(
                    __gnu_test_input_iterator_wrapper_const_char_ {
                        ptr: self.ptr.clone(),
                        SharedInfo: self.SharedInfo.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    __gnu_test_input_iterator_wrapper_const_char_ {
                        ptr: other.ptr.clone(),
                        SharedInfo: other.SharedInfo.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_input_iterator_wrapper_const_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_BoundsContainer_const_wchar_t_>,
}
impl __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    pub fn new_1(
        mut _ptr: Ptr<()>,
        mut SharedInfo_in: Ptr<__gnu_test_BoundsContainer_const_wchar_t_>,
    ) -> Self {
        let __this: Value<__gnu_test_input_iterator_wrapper_const_wchar_t_> =
            Rc::new(RefCell::new(Self {
                ptr: (_ptr).clone(),
                SharedInfo: (SharedInfo_in).clone(),
            }));
        let this: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ this.with(|__s| __s.ptr.clone()) } >= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ this.with(|__s| __s.ptr.clone()) } <= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.last.clone())
            })) {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 250_() ) .into() , ( Ptr::<() >::from_string_literal(b"__gnu_test::input_iterator_wrapper<const wchar_t>::input_iterator_wrapper(T *, ContainerType *) [T = const wchar_t]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr >= SharedInfo->first && ptr <= SharedInfo->last" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    fn eq(&self, other: &Self) -> bool {
        {
            __gnu_test_input_iterator_wrapper_const_wchar_t_Impl::operator_eq(
                &Rc::new(RefCell::new(
                    __gnu_test_input_iterator_wrapper_const_wchar_t_ {
                        ptr: self.ptr.clone(),
                        SharedInfo: self.SharedInfo.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    __gnu_test_input_iterator_wrapper_const_wchar_t_ {
                        ptr: other.ptr.clone(),
                        SharedInfo: other.SharedInfo.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_input_iterator_wrapper_const_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(16)]
    pub bounds: __gnu_test_BoundsContainer_const_char_,
}
impl __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            bounds: __gnu_test_BoundsContainer_const_char_::new({ (_first).clone() }, {
                (_last).clone()
            }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(16)]
    pub bounds: __gnu_test_BoundsContainer_const_wchar_t_,
}
impl __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            bounds: __gnu_test_BoundsContainer_const_wchar_t_::new({ (_first).clone() }, {
                (_last).clone()
            }),
        }
    }
}
pub fn test_19(p: Ptr<()>, mut s: Ptr<()>) {
    let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let oper: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let func: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let input_range: Value<
        __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_,
    > = Rc::new(RefCell::new(
        __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::new(
            { (s).clone() },
            { s.offset((unimplemented!()) as isize) },
        ),
    ));
    let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ compare_paths_0(oper.as_pointer(), expected.as_pointer()) });
    ({ compare_paths_0(func.as_pointer(), expected.as_pointer()) });
    ({ compare_paths_0(range.as_pointer(), expected.as_pointer()) });
}
pub fn test_20(p: Ptr<()>, mut s: Ptr<()>) {
    let expected: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let oper: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let func: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let input_range: Value<
        __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_,
    > = Rc::new(RefCell::new(
        __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_::new(
            { (s).clone() },
            { s.offset((unimplemented!()) as isize) },
        ),
    ));
    let range: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ compare_paths_0(oper.as_pointer(), expected.as_pointer()) });
    ({ compare_paths_0(func.as_pointer(), expected.as_pointer()) });
    ({ compare_paths_0(range.as_pointer(), expected.as_pointer()) });
}
pub fn test01_21() {
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"/foo/"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"baz"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"baz"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"/foo/bar"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"/foo/bar"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b""))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"rel"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"/file"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"file"))
    });
}
pub fn test02_22() {
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"foo"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"foo"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b""))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"/bar"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"c:/bar"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"c:"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b""))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"/bar"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"c:\\bar"))
    });
}
pub fn test03_23() {
    'loop_: for p in IterRange::new(
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<()>),
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<()>).offset((18_()) as isize),
    ) {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: for q in IterRange::new(
            (test_paths_1.with(|v| v.as_pointer()) as Ptr<()>),
            (test_paths_1.with(|v| v.as_pointer()) as Ptr<()>).offset((18_()) as isize),
        ) {
            let q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_19(p.as_pointer(), unimplemented!()) });
            {}
        }
    }
}
pub fn test04_24() {
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_20(
            _p.as_pointer(),
            Ptr::<()>::from_string_literal(&[47 as (), 98 as (), 97 as (), 114 as (), 0 as ()]),
        )
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(_p.as_pointer(), Ptr::<()>::from_string_literal(b"/bar"))
    });
    ({
        let _p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_20(
            _p.as_pointer(),
            Ptr::<()>::from_string_literal(&[47 as (), 98 as (), 97 as (), 114 as (), 0 as ()]),
        )
    });
}
pub fn test05_25() {
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({
        let _p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        compare_paths_0(p.as_pointer(), _p2.as_pointer())
    });
    let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({
        let _p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        compare_paths_0(p2.as_pointer(), _p2.as_pointer())
    });
    let p3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let second: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({
        let _p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        compare_paths_0(p3.as_pointer(), _p2.as_pointer())
    });
}
pub fn test06_26() {
    let mut s0: () = unimplemented!();
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut s: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        unimplemented!();
        i.prefix_inc();
    }
    ({ test_19(p.as_pointer(), unimplemented!()) });
    let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_19(p2.as_pointer(), unimplemented!()) });
}
pub fn test07_27() {
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let p0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ compare_paths_0(p.as_pointer(), p0.as_pointer()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_21() });
    ({ test02_22() });
    ({ test03_23() });
    ({ test04_24() });
    ({ test05_25() });
    ({ test06_26() });
    ({ test07_27() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write((({ getpid_28() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_29(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
        if (this.with(|__s| __s.sem_set_) == -1_()) {
            ({
                __throw_runtime_error_11(Ptr::<()>::from_string_literal(
                    b"could not obtain semaphore set",
                ))
            });
        }
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if (({
            semctl_30(
                this.with(|__s| __s.sem_set_),
                0_(),
                16_(),
                Box::new([((*val.borrow()).clone()).into()]),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_11(Ptr::<()>::from_string_literal(
                    b"could not initialize semaphore",
                ))
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn set_memory_limits_31(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_32((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_33((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_32((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_33((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_32((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_33((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_34(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_32((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_33((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_35(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_36(
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
    let mut w: () = unimplemented!();
    if unimplemented!() {
        ({ __throw_runtime_error_11((s).clone()) });
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_37(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        ({ __throw_runtime_error_11(unimplemented!()) });
    }
}
pub fn run_tests_wrapped_env_38(
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
        ({ __throw_runtime_error_11(unimplemented!()) });
    }
}
thread_local!(
    pub static count_8: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static count__9: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__10: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static count__12: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__13: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static _M_count_14: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static next_id__15: Value<()> = Rc::new(RefCell::new(0_()));
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
pub fn test_tm_40(
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
    (*tmp_41.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_41.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_41: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
);
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy;
pub trait __gnu_test_BoundsContainer_const_char_Impl {
    fn size(&self) -> () {
        unimplemented!()
    }
}
impl __gnu_test_BoundsContainer_const_char_Impl for Ptr<__gnu_test_BoundsContainer_const_char_> {}
pub trait __gnu_test_BoundsContainer_const_wchar_t_Impl {
    fn size(&self) -> () {
        unimplemented!()
    }
}
impl __gnu_test_BoundsContainer_const_wchar_t_Impl
    for Ptr<__gnu_test_BoundsContainer_const_wchar_t_>
{
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
pub trait __gnu_test_input_iterator_wrapper_const_char_Impl {
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_>) -> ();
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_>) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_const_char_>;
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_const_char_Impl
    for Ptr<__gnu_test_input_iterator_wrapper_const_char_>
{
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_>) -> () {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.SharedInfo.clone()) } == {
                    in_.with(|__s| __s.SharedInfo.clone())
                }))
            {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 262_() ) .into() , ( Ptr::<() >::from_string_literal(b"bool __gnu_test::input_iterator_wrapper<const char>::operator==(const input_iterator_wrapper<T> &) const [T = const char]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"SharedInfo && SharedInfo == in.SharedInfo" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ in_.with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            })) {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 263_() ) .into() , ( Ptr::<() >::from_string_literal(b"bool __gnu_test::input_iterator_wrapper<const char>::operator==(const input_iterator_wrapper<T> &) const [T = const char]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr>=SharedInfo->first && in.ptr>=SharedInfo->first" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return ({ (*self).with(|__s| __s.ptr.clone()) } == { in_.with(|__s| __s.ptr.clone()) });
    }
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_>) -> () {
        return !({
            let _in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_> = (in_).clone();
            __gnu_test_input_iterator_wrapper_const_char_Impl::operator_eq(&(*self), _in_)
        });
    }
    fn operator_deref(&self) -> Ptr<()> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 276_() ) .into() , ( Ptr::<() >::from_string_literal(b"T &__gnu_test::input_iterator_wrapper<const char>::operator*() const [T = const char]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"SharedInfo && ptr < SharedInfo->last" )  ) .into() , ] ) )
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
                    .with(|__s| __s.first.clone())
            }) {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 277_() ) .into() , ( Ptr::<() >::from_string_literal(b"T &__gnu_test::input_iterator_wrapper<const char>::operator*() const [T = const char]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr >= SharedInfo->first" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return (*self).with(|__s| __s.ptr.clone());
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_const_char_> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 290_() ) .into() , ( Ptr::<() >::from_string_literal(b"input_iterator_wrapper<T> &__gnu_test::input_iterator_wrapper<const char>::operator++() [T = const char]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"SharedInfo && ptr < SharedInfo->last" )  ) .into() , ] ) )
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
                    .with(|__s| __s.first.clone())
            }) {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 291_() ) .into() , ( Ptr::<() >::from_string_literal(b"input_iterator_wrapper<T> &__gnu_test::input_iterator_wrapper<const char>::operator++() [T = const char]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr>=SharedInfo->first" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        field!((*self), ptr).with_mut(|__v| __v.postfix_inc());
        field!((*self).with(|__s| __s.SharedInfo.clone()), first)
            .write((*self).with(|__s| __s.ptr.clone()));
        return (*self).clone();
    }
}
pub trait __gnu_test_input_iterator_wrapper_const_wchar_t_Impl {
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>) -> ();
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>;
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_const_wchar_t_Impl
    for Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>
{
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>) -> () {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.SharedInfo.clone()) } == {
                    in_.with(|__s| __s.SharedInfo.clone())
                }))
            {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 262_() ) .into() , ( Ptr::<() >::from_string_literal(b"bool __gnu_test::input_iterator_wrapper<const wchar_t>::operator==(const input_iterator_wrapper<T> &) const [T = const wchar_t]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"SharedInfo && SharedInfo == in.SharedInfo" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ in_.with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            })) {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 263_() ) .into() , ( Ptr::<() >::from_string_literal(b"bool __gnu_test::input_iterator_wrapper<const wchar_t>::operator==(const input_iterator_wrapper<T> &) const [T = const wchar_t]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr>=SharedInfo->first && in.ptr>=SharedInfo->first" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return ({ (*self).with(|__s| __s.ptr.clone()) } == { in_.with(|__s| __s.ptr.clone()) });
    }
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>) -> () {
        return !({
            let _in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_> = (in_).clone();
            __gnu_test_input_iterator_wrapper_const_wchar_t_Impl::operator_eq(&(*self), _in_)
        });
    }
    fn operator_deref(&self) -> Ptr<()> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 276_() ) .into() , ( Ptr::<() >::from_string_literal(b"T &__gnu_test::input_iterator_wrapper<const wchar_t>::operator*() const [T = const wchar_t]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"SharedInfo && ptr < SharedInfo->last" )  ) .into() , ] ) )
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
                    .with(|__s| __s.first.clone())
            }) {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 277_() ) .into() , ( Ptr::<() >::from_string_literal(b"T &__gnu_test::input_iterator_wrapper<const wchar_t>::operator*() const [T = const wchar_t]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr >= SharedInfo->first" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return (*self).with(|__s| __s.ptr.clone());
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 290_() ) .into() , ( Ptr::<() >::from_string_literal(b"input_iterator_wrapper<T> &__gnu_test::input_iterator_wrapper<const wchar_t>::operator++() [T = const wchar_t]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"SharedInfo && ptr < SharedInfo->last" )  ) .into() , ] ) )
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
                    .with(|__s| __s.first.clone())
            }) {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 291_() ) .into() , ( Ptr::<() >::from_string_literal(b"input_iterator_wrapper<T> &__gnu_test::input_iterator_wrapper<const wchar_t>::operator++() [T = const wchar_t]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"ptr>=SharedInfo->first" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        field!((*self), ptr).with_mut(|__v| __v.postfix_inc());
        field!((*self).with(|__s| __s.SharedInfo.clone()), first)
            .write((*self).with(|__s| __s.ptr.clone()));
        return (*self).clone();
    }
}
pub trait __gnu_test_object_counterImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter>;
}
impl __gnu_test_object_counterImpl for Ptr<__gnu_test_object_counter> {
    fn destructor(&self) {
        (*count_8.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter> {
        return (*self).clone();
    }
}
pub trait __gnu_test_scoped_fileImpl {
    fn destructor(&self);
    fn move_assign(&self, _a0: Ptr<__gnu_test_scoped_file>) -> Ptr<__gnu_test_scoped_file>;
}
impl __gnu_test_scoped_fileImpl for Ptr<__gnu_test_scoped_file> {
    fn destructor(&self) {
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    fn move_assign(&self, _a0: Ptr<__gnu_test_scoped_file>) -> Ptr<__gnu_test_scoped_file> {
        unimplemented!();
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_28() })) {
            ({
                semctl_30(
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
            semop_39(
                (*self).with(|__s| __s.sem_set_),
                (op.as_pointer() as Ptr<sembuf>),
                1_(),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_11(Ptr::<()>::from_string_literal(
                    b"could not signal semaphore",
                ))
            });
        }
    }
    fn wait(&self) {
        let op: Value<Box<[sembuf]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (({
            semop_39(
                (*self).with(|__s| __s.sem_set_),
                (op.as_pointer() as Ptr<sembuf>),
                1_(),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_11(Ptr::<()>::from_string_literal(
                    b"could not wait for semaphore",
                ))
            });
        }
    }
}
pub trait __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_Impl {
    fn it_3(&self, mut pos: ()) -> __gnu_test_input_iterator_wrapper_const_char_ {
        unimplemented!()
    }
    fn it_4(&self, pos: Ptr<()>) -> __gnu_test_input_iterator_wrapper_const_char_;
    fn val(&self, mut pos: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_const_char_;
    fn end(&self) -> __gnu_test_input_iterator_wrapper_const_char_;
    fn size(&self) -> () {
        unimplemented!()
    }
}
impl __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_>
{
    fn it_4(&self, mut pos: Ptr<()>) -> __gnu_test_input_iterator_wrapper_const_char_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (pos).clone() } >= { (*self).with(|__s| __s.bounds.first.clone()) })
                && ({ (pos).clone() } <= { (*self).with(|__s| __s.bounds.last.clone()) }))
            {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 628_() ) .into() , ( Ptr::<() >::from_string_literal(b"ItType<T> __gnu_test::test_container<const char, __gnu_test::input_iterator_wrapper>::it(T *) [T = const char, ItType = __gnu_test::input_iterator_wrapper]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"pos >= bounds.first && pos <= bounds.last" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return __gnu_test_input_iterator_wrapper_const_char_::new_1({ (pos).clone() }, {
            (field_ptr!((*self), bounds))
        });
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_const_char_ {
        return ({
            let _pos: Ptr<()> = (*self).with(|__s| __s.bounds.first.clone());
            __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
    fn end(&self) -> __gnu_test_input_iterator_wrapper_const_char_ {
        return ({
            let _pos: Ptr<()> = (*self).with(|__s| __s.bounds.last.clone());
            __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
}
pub trait __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_Impl {
    fn it_3(&self, mut pos: ()) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        unimplemented!()
    }
    fn it_4(&self, pos: Ptr<()>) -> __gnu_test_input_iterator_wrapper_const_wchar_t_;
    fn val(&self, mut pos: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_;
    fn end(&self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_;
    fn size(&self) -> () {
        unimplemented!()
    }
}
impl __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_>
{
    fn it_4(&self, mut pos: Ptr<()>) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (pos).clone() } >= { (*self).with(|__s| __s.bounds.first.clone()) })
                && ({ (pos).clone() } <= { (*self).with(|__s| __s.bounds.last.clone()) }))
            {
                ({
                    __builtin_fprintf_18 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"source.cc" )  ) .into() , ( 628_() ) .into() , ( Ptr::<() >::from_string_literal(b"ItType<T> __gnu_test::test_container<const wchar_t, __gnu_test::input_iterator_wrapper>::it(T *) [T = const wchar_t, ItType = __gnu_test::input_iterator_wrapper]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"pos >= bounds.first && pos <= bounds.last" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return __gnu_test_input_iterator_wrapper_const_wchar_t_::new_1({ (pos).clone() }, {
            (field_ptr!((*self), bounds))
        });
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        return ({
            let _pos: Ptr<()> = (*self).with(|__s| __s.bounds.first.clone());
            __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
    fn end(&self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        return ({
            let _pos: Ptr<()> = (*self).with(|__s| __s.bounds.last.clone());
            __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = test_paths_1.with(|_| ());
    let _ = count_8.with(|_| ());
    let _ = count__9.with(|_| ());
    let _ = throw_on__10.with(|_| ());
    let _ = count__12.with(|_| ());
    let _ = throw_on__13.with(|_| ());
    let _ = _M_count_14.with(|_| ());
    let _ = next_id__15.with(|_| ());
}
