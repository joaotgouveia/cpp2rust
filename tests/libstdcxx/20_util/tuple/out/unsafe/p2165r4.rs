extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg)]
pub struct __gnu_test_func_callback {
    _M_size: (),
    _M_tests: [Option<unsafe fn()>; 15],
}
impl __gnu_test_func_callback {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            _M_size: 0_(),
            _M_tests: [None; 15],
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return self._M_size;
    }
    pub unsafe fn tests(&self) -> *const Option<unsafe fn()> {
        return self._M_tests.as_ptr();
    }
    pub unsafe fn push_back(&mut self, mut test: Option<unsafe fn()>) {
        self._M_tests[(self._M_size) as usize] = test;
        self._M_size.prefix_inc();
    }
}
impl Default for __gnu_test_func_callback {
    fn default() -> Self {
        unsafe { __gnu_test_func_callback::new() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_object_counter {}
impl __gnu_test_object_counter {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const __gnu_test_object_counter) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_dec();
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_test_object_counter,
    ) -> *mut __gnu_test_object_counter {
        return &mut (*(self as *mut __gnu_test_object_counter));
    }
}
impl Clone for __gnu_test_object_counter {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_object_counter::copy_from(self as *const __gnu_test_object_counter) }
    }
}
impl Default for __gnu_test_object_counter {
    fn default() -> Self {
        unsafe { __gnu_test_object_counter::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_copy_constructor {}
impl __gnu_test_copy_constructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__1))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)))
        {
            (unsafe { __throw_runtime_error_3(c"copy_constructor::mark_call".as_ptr()) });
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__4));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__4)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__4))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__5)))
        {
            (unsafe { __throw_runtime_error_3(c"assignment_operator::mark_call".as_ptr()) });
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__4)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__5)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__5)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_6));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_6)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_6)) = 0_();
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_copy_tracker {
    id_: (),
    throw_on_copy_: (),
}
impl __gnu_test_copy_tracker {
    pub unsafe fn new(mut id: Option<()>, mut throw_on_copy: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__7)).postfix_dec()
        });
        let mut throw_on_copy: () = throw_on_copy.unwrap_or_else(|| unsafe { false });
        let mut this = Self {
            id_: id,
            throw_on_copy_: throw_on_copy,
        };
        this
    }
    pub unsafe fn copy_from(rhs: *const __gnu_test_copy_tracker) -> Self {
        let mut this = Self {
            id_: (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }),
            throw_on_copy_: (*rhs).throw_on_copy_,
        };
        if this.throw_on_copy_ {
            (unsafe {
                __gnu_test_copy_constructor::throw_on(
                    (unsafe { __gnu_test_copy_constructor::count() }).wrapping_add(1_()),
                )
            });
        }
        (unsafe { __gnu_test_copy_constructor::mark_call() });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        rhs: *const __gnu_test_copy_tracker,
    ) -> *mut __gnu_test_copy_tracker {
        self.id_ = (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) });
        if (*rhs).throw_on_copy_ {
            (unsafe {
                __gnu_test_assignment_operator::throw_on(
                    (unsafe { __gnu_test_assignment_operator::count() }).wrapping_add(1_()),
                )
            });
        }
        (unsafe { __gnu_test_assignment_operator::mark_call() });
        return &mut (*(self as *mut __gnu_test_copy_tracker));
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { __gnu_test_destructor::mark_call() });
    }
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn reset() {
        (unsafe { __gnu_test_copy_constructor::reset() });
        (unsafe { __gnu_test_assignment_operator::reset() });
        (unsafe { __gnu_test_destructor::reset() });
    }
}
impl std::cmp::Ord for __gnu_test_copy_tracker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_8(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_8(
                other as *const __gnu_test_copy_tracker,
                self as *const __gnu_test_copy_tracker,
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
        unsafe {
            operator_eq_9(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_copy_tracker {}
impl Clone for __gnu_test_copy_tracker {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_copy_tracker::copy_from(self as *const __gnu_test_copy_tracker) }
    }
}
impl Default for __gnu_test_copy_tracker {
    fn default() -> Self {
        unsafe { __gnu_test_copy_tracker::new(None, None) }
    }
}
pub unsafe fn operator_eq_9(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_8(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        < (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_semaphore {
    sem_set_: (),
    pid_: ::libc::pid_t,
}
impl __gnu_test_semaphore {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
pub unsafe fn test01_10() -> () {
    let mut tuple_like: std_array_A__3_ = unsafe { std::mem::zeroed::<()>() };
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_12 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) && ( ( ( ( * ( unsafe { get_13 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) ) && ( ( ( ( * ( unsafe { get_14 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 46_() ) .into() , ( c"auto test01()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 0 && std::get<1>(t3b).m == 0 && std::get<2>(t3b).m == 0" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_12 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) && ( ( ( ( * ( unsafe { get_13 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) ) && ( ( ( ( * ( unsafe { get_14 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 50_() ) .into() , ( c"auto test01()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 1 && std::get<1>(t3b).m == 1 && std::get<2>(t3b).m == 1" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_12 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) && ( ( ( ( * ( unsafe { get_13 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) ) && ( ( ( ( * ( unsafe { get_14 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 54_() ) .into() , ( c"auto test01()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 2 && std::get<1>(t3b).m == 2 && std::get<2>(t3b).m == 2" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_12 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) && ( ( ( ( * ( unsafe { get_13 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) ) && ( ( ( ( * ( unsafe { get_14 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 58_() ) .into() , ( c"auto test01()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 3 && std::get<1>(t3b).m == 3 && std::get<2>(t3b).m == 3" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<tuple<const int&, int, int>, tuple_like_t<long>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<tuple<int, const int&, int>, tuple_like_t<long>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<tuple<int, int, const int&>, tuple_like_t<long>>"
    );;
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_11 {
    pub m: (),
}
impl B_11 {
    pub unsafe fn new_1(_a0: *mut A) -> Self {
        let mut this = Self { m: 0_() };
        this
    }
    pub unsafe fn new_2(_a0: *mut A) -> Self {
        let mut this = Self { m: 1_() };
        this
    }
    pub unsafe fn new_3(_a0: *const A) -> Self {
        let mut this = Self { m: 2_() };
        this
    }
    pub unsafe fn new_4(_a0: *const A) -> Self {
        let mut this = Self { m: 3_() };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct alloc_B01 {
    pub m: (),
}
impl alloc_B01 {
    pub unsafe fn new_1(mut _a0: (), mut _a1: (), _a2: *mut A) -> Self {
        let mut this = Self { m: 0_() };
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut _a1: (), _a2: *mut A) -> Self {
        let mut this = Self { m: 1_() };
        this
    }
    pub unsafe fn new_3(mut _a0: (), mut _a1: (), _a2: *const A) -> Self {
        let mut this = Self { m: 2_() };
        this
    }
    pub unsafe fn new_4(mut _a0: (), mut _a1: (), _a2: *const A) -> Self {
        let mut this = Self { m: 3_() };
        this
    }
}
pub unsafe fn test01_16() -> () {
    let mut tuple_like: std_array_A__3_ = unsafe { std::mem::zeroed::<()>() };
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_17 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) && ( ( ( ( * ( unsafe { get_18 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) ) && ( ( ( ( * ( unsafe { get_19 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 96_() ) .into() , ( c"auto alloc::test01()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 0 && std::get<1>(t3b).m == 0 && std::get<2>(t3b).m == 0" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_17 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) && ( ( ( ( * ( unsafe { get_18 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) ) && ( ( ( ( * ( unsafe { get_19 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 100_() ) .into() , ( c"auto alloc::test01()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 1 && std::get<1>(t3b).m == 1 && std::get<2>(t3b).m == 1" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_17 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) && ( ( ( ( * ( unsafe { get_18 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) ) && ( ( ( ( * ( unsafe { get_19 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 104_() ) .into() , ( c"auto alloc::test01()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 2 && std::get<1>(t3b).m == 2 && std::get<2>(t3b).m == 2" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_17 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) && ( ( ( ( * ( unsafe { get_18 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) ) && ( ( ( ( * ( unsafe { get_19 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 108_() ) .into() , ( c"auto alloc::test01()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 3 && std::get<1>(t3b).m == 3 && std::get<2>(t3b).m == 3" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<tuple<const int&, int, int>,\n\t\t\t\t\t    allocator_arg_t, allocator<int>,\n\t\t\t\t\t    tuple_like_t<long>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<tuple<int, const int&, int>,\n\t\t\t\t\t    allocator_arg_t, allocator<int>,\n\t\t\t\t\t    tuple_like_t<long>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<tuple<int, int, const int&>,\n\t\t\t\t\t    allocator_arg_t, allocator<int>,\n\t\t\t\t\t    tuple_like_t<long>>"
    );;
    return true;
}
pub unsafe fn test02_20() -> () {
    const _: () = assert!(
        true,
        "!std::is_convertible_v<tuple_like_t<A>, tuple<B, B, B>>"
    );;
    let mut tuple_like: std_array_A__3_ = unsafe { std::mem::zeroed::<()>() };
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_22 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) && ( ( ( ( * ( unsafe { get_23 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) ) && ( ( ( ( * ( unsafe { get_24 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 147_() ) .into() , ( c"auto test02()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 0 && std::get<1>(t3b).m == 0 && std::get<2>(t3b).m == 0" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_22 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) && ( ( ( ( * ( unsafe { get_23 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) ) && ( ( ( ( * ( unsafe { get_24 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 151_() ) .into() , ( c"auto test02()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 1 && std::get<1>(t3b).m == 1 && std::get<2>(t3b).m == 1" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_22 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) && ( ( ( ( * ( unsafe { get_23 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) ) && ( ( ( ( * ( unsafe { get_24 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 155_() ) .into() , ( c"auto test02()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 2 && std::get<1>(t3b).m == 2 && std::get<2>(t3b).m == 2" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_22 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) && ( ( ( ( * ( unsafe { get_23 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) ) && ( ( ( ( * ( unsafe { get_24 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 159_() ) .into() , ( c"auto test02()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 3 && std::get<1>(t3b).m == 3 && std::get<2>(t3b).m == 3" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_21 {
    pub m: (),
}
impl B_21 {
    pub unsafe fn new_1(_a0: *mut A) -> Self {
        let mut this = Self { m: 0_() };
        this
    }
    pub unsafe fn new_2(_a0: *mut A) -> Self {
        let mut this = Self { m: 1_() };
        this
    }
    pub unsafe fn new_3(_a0: *const A) -> Self {
        let mut this = Self { m: 2_() };
        this
    }
    pub unsafe fn new_4(_a0: *const A) -> Self {
        let mut this = Self { m: 3_() };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct alloc_B02 {
    pub m: (),
}
impl alloc_B02 {
    pub unsafe fn new_1(mut _a0: (), mut _a1: (), _a2: *mut A) -> Self {
        let mut this = Self { m: 0_() };
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut _a1: (), _a2: *mut A) -> Self {
        let mut this = Self { m: 1_() };
        this
    }
    pub unsafe fn new_3(mut _a0: (), mut _a1: (), _a2: *const A) -> Self {
        let mut this = Self { m: 2_() };
        this
    }
    pub unsafe fn new_4(mut _a0: (), mut _a1: (), _a2: *const A) -> Self {
        let mut this = Self { m: 3_() };
        this
    }
}
pub unsafe fn test02_25() -> () {
    const _: () = assert!(
        true,
        "!std::is_convertible_v<tuple_like_t<A>, tuple<B, B, B>>"
    );;
    let mut tuple_like: std_array_A__3_ = unsafe { std::mem::zeroed::<()>() };
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_26 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) && ( ( ( ( * ( unsafe { get_27 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) ) && ( ( ( ( * ( unsafe { get_28 ( & mut t3b , ) } )  )  . m  ) == ( 0_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 194_() ) .into() , ( c"auto alloc::test02()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 0 && std::get<1>(t3b).m == 0 && std::get<2>(t3b).m == 0" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_26 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) && ( ( ( ( * ( unsafe { get_27 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) ) && ( ( ( ( * ( unsafe { get_28 ( & mut t3b , ) } )  )  . m  ) == ( 1_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 198_() ) .into() , ( c"auto alloc::test02()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 1 && std::get<1>(t3b).m == 1 && std::get<2>(t3b).m == 1" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_26 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) && ( ( ( ( * ( unsafe { get_27 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) ) && ( ( ( ( * ( unsafe { get_28 ( & mut t3b , ) } )  )  . m  ) == ( 2_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 202_() ) .into() , ( c"auto alloc::test02()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 2 && std::get<1>(t3b).m == 2 && std::get<2>(t3b).m == 2" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    (unsafe {
        lambda_unsafe! ( { let tuple_like : *mut std_array_A__3_ = & mut tuple_like ;
 } , | | { let mut t3b : () = unimplemented!() ;
  ;
 ;
 let mut __do_while = true ;
 'loop_: while __do_while || ( false ) { __do_while = false ;
 if ! ( ( ( ( ( ( ( * ( unsafe { get_26 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) && ( ( ( ( * ( unsafe { get_27 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) ) && ( ( ( ( * ( unsafe { get_28 ( & mut t3b , ) } )  )  . m  ) == ( 3_() ) ) ) ) ) { ( unsafe { __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 206_() ) .into() , ( c"auto alloc::test02()::(lambda)::operator()() const" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 3 && std::get<1>(t3b).m == 3 && std::get<2>(t3b).m == 3" .as_ptr() ) .into() , ] ) ) } )  ;
 unimplemented!()  ;
 } } } )  .call ( )
    });
    return true;
}
pub unsafe fn test03_29() -> () {
    let mut tuple_like: std_array_A__3_ = unsafe { std::mem::zeroed::<()>() };
    let mut t3b: () = unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((((*(unsafe { get_31(&mut t3b) })).m) == (0_()))
            && (((*(unsafe { get_32(&mut t3b) })).m) == (0_())))
            && (((*(unsafe { get_33(&mut t3b) })).m) == (0_())))
        {
            (unsafe {
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 233_() ) .into() , ( c"bool test03() [tuple_like_t = tuple_like_array]" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 0 && std::get<1>(t3b).m == 0 && std::get<2>(t3b).m == 0" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((((*(unsafe { get_31(&mut t3b) })).m) == (1_()))
            && (((*(unsafe { get_32(&mut t3b) })).m) == (1_())))
            && (((*(unsafe { get_33(&mut t3b) })).m) == (1_())))
        {
            (unsafe {
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 235_() ) .into() , ( c"bool test03() [tuple_like_t = tuple_like_array]" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 1 && std::get<1>(t3b).m == 1 && std::get<2>(t3b).m == 1" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((((*(unsafe { get_31(&mut t3b) })).m) == (2_()))
            && (((*(unsafe { get_32(&mut t3b) })).m) == (2_())))
            && (((*(unsafe { get_33(&mut t3b) })).m) == (2_())))
        {
            (unsafe {
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 237_() ) .into() , ( c"bool test03() [tuple_like_t = tuple_like_array]" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 2 && std::get<1>(t3b).m == 2 && std::get<2>(t3b).m == 2" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((((*(unsafe { get_31(&mut t3b) })).m) == (3_()))
            && (((*(unsafe { get_32(&mut t3b) })).m) == (3_())))
            && (((*(unsafe { get_33(&mut t3b) })).m) == (3_())))
        {
            (unsafe {
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 239_() ) .into() , ( c"bool test03() [tuple_like_t = tuple_like_array]" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 3 && std::get<1>(t3b).m == 3 && std::get<2>(t3b).m == 3" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_30 {
    pub m: (),
}
impl B_30 {
    pub unsafe fn operator_assign_1(&mut self, _a0: *mut A) -> *mut B_30 {
        self.m = 0_();
        return &mut (*(self as *mut B_30));
    }
    pub unsafe fn operator_assign_2(&mut self, _a0: *mut A) -> *mut B_30 {
        self.m = 1_();
        return &mut (*(self as *mut B_30));
    }
    pub unsafe fn operator_assign_3(&mut self, _a0: *const A) -> *mut B_30 {
        self.m = 2_();
        return &mut (*(self as *mut B_30));
    }
    pub unsafe fn operator_assign_4(&mut self, _a0: *const A) -> *mut B_30 {
        self.m = 3_();
        return &mut (*(self as *mut B_30));
    }
}
pub unsafe fn test04_34() -> () {
    let mut tuple_like: std_array_A__3_ = unsafe { std::mem::zeroed::<()>() };
    let t3b: () = unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((((*(unsafe { get_36(&t3b) })).m) == (0_()))
            && (((*(unsafe { get_37(&t3b) })).m) == (0_())))
            && (((*(unsafe { get_38(&t3b) })).m) == (0_())))
        {
            (unsafe {
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 263_() ) .into() , ( c"bool test04() [tuple_like_t = tuple_like_array]" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 0 && std::get<1>(t3b).m == 0 && std::get<2>(t3b).m == 0" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((((*(unsafe { get_36(&t3b) })).m) == (1_()))
            && (((*(unsafe { get_37(&t3b) })).m) == (1_())))
            && (((*(unsafe { get_38(&t3b) })).m) == (1_())))
        {
            (unsafe {
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 265_() ) .into() , ( c"bool test04() [tuple_like_t = tuple_like_array]" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 1 && std::get<1>(t3b).m == 1 && std::get<2>(t3b).m == 1" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((((*(unsafe { get_36(&t3b) })).m) == (2_()))
            && (((*(unsafe { get_37(&t3b) })).m) == (2_())))
            && (((*(unsafe { get_38(&t3b) })).m) == (2_())))
        {
            (unsafe {
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 267_() ) .into() , ( c"bool test04() [tuple_like_t = tuple_like_array]" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 2 && std::get<1>(t3b).m == 2 && std::get<2>(t3b).m == 2" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((((*(unsafe { get_36(&t3b) })).m) == (3_()))
            && (((*(unsafe { get_37(&t3b) })).m) == (3_())))
            && (((*(unsafe { get_38(&t3b) })).m) == (3_())))
        {
            (unsafe {
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"p2165r4.cc" .as_ptr() ) .into() , ( 269_() ) .into() , ( c"bool test04() [tuple_like_t = tuple_like_array]" .as_ptr() ) .into() , ( c"std::get<0>(t3b).m == 3 && std::get<1>(t3b).m == 3 && std::get<2>(t3b).m == 3" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_35 {
    pub m: (),
}
impl B_35 {
    pub unsafe fn operator_assign_1(&self, _a0: *mut A) -> *const B_35 {
        (*(&raw const self.m).cast_mut()) = 0_();
        return &(*(self as *const B_35));
    }
    pub unsafe fn operator_assign_2(&self, _a0: *mut A) -> *const B_35 {
        (*(&raw const self.m).cast_mut()) = 1_();
        return &(*(self as *const B_35));
    }
    pub unsafe fn operator_assign_3(&self, _a0: *const A) -> *const B_35 {
        (*(&raw const self.m).cast_mut()) = 2_();
        return &(*(self as *const B_35));
    }
    pub unsafe fn operator_assign_4(&self, _a0: *const A) -> *const B_35 {
        (*(&raw const self.m).cast_mut()) = 3_();
        return &(*(self as *const B_35));
    }
}
pub unsafe fn test05_39() -> () {
    const _: () = assert!(true, "tuple{{1, 2, 3}} == tuple_like_t{{1, 2, 3}}");;
    const _: () = assert!(true, "tuple{{1, 2, 4}} != tuple_like_t{{1, 2, 3}}");;
    const _: () = assert!(true, "tuple_like_t{{1, 2, 3}} == tuple{{1, 2, 3}}");;
    const _: () = assert!(true, "tuple_like_t{{1, 2, 3}} != tuple{{1, 2, 4}}");;
    const _: () = assert!(
        true,
        "(tuple{{1, 2, 3}} <=> tuple_like_t{{1, 2, 3}}) == std::strong_ordering::equal"
    );;
    const _: () = assert!(
        true,
        "(tuple{{1, 2, 4}} <=> tuple_like_t{{1, 2, 3}}) == std::strong_ordering::greater"
    );;
    const _: () = assert!(
        true,
        "(tuple_like_t{{1, 2, 3}} <=> tuple{{1, 2, 3}}) == std::strong_ordering::equal"
    );;
    const _: () = assert!(
        true,
        "(tuple_like_t{{1, 2, 3}} <=> tuple{{1, 2, 4}}) == std::strong_ordering::less"
    );;
    const _: () = assert!(true, "tuple{{1, 2, 4}} > tuple_like_t{{1, 2, 3}}");;
    const _: () = assert!(true, "tuple_like_t{{1, 2, 3}} < tuple{{1, 2, 4}}");;
    const _: () = assert!(
        true,
        "std::same_as<std::common_reference_t<tuple_like_t<int>,\n\t\t\t\t\t\t      tuple<int, long, int>>,\n\t\t\t      tuple<int, long, int>>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<std::common_reference_t<tuple<int, long, int>,\n\t\t\t\t\t\t      tuple_like_t<int>>,\n\t\t\t      tuple<int, long, int>>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<std::common_type_t<tuple_like_t<const int&>,\n\t\t\t\t\t\t tuple<int, long, int>>,\n\t\t\t      tuple<int, long, int>>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<std::common_type_t<tuple<int, long, int>,\n\t\t\t\t\t\t tuple_like_t<const int&>>,\n\t\t\t      tuple<int, long, int>>"
    );;
    return true;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    const _: () = assert!(true, "test01<tuple_like_array>()");;
    const _: () = assert!(true, "alloc::test01<tuple_like_array>()");;
    const _: () = assert!(true, "test02<tuple_like_array>()");;
    const _: () = assert!(true, "alloc::test02<tuple_like_array>()");;
    const _: () = assert!(true, "test03<tuple_like_array>()");;
    const _: () = assert!(true, "test04<tuple_like_array>()");;
    const _: () = assert!(true, "test05<tuple_like_array>()");;
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_40() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_41(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_42(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_43(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_44(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_45(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_44(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_45(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_44((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_45(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_46(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_44(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_45(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_47(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_48(
            mangled,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            (&mut status as *mut ()),
        )
    });
    if !(demangled).is_null() {
        s = (demangled).cast_const();
    } else {
        switch!(match status {
            __v if __v == 0_() => {
                s = c"error code = 0: success".as_ptr();
                break;
            }
            __v if __v == -1_() => {
                s = c"error code = -1: memory allocation failure".as_ptr();
                break;
            }
            __v if __v == -2_() => {
                s = c"error code = -2: invalid mangled name".as_ptr();
                break;
            }
            __v if __v == -3_() => {
                s = c"error code = -3: invalid arguments".as_ptr();
                break;
            }
            _ => {
                s = c"error code unknown - who knows what happened".as_ptr();
            }
        });
    }
    let mut w: () = unimplemented!();
    if !(unimplemented!()) {
        (unsafe { __throw_runtime_error_3(s) });
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_49(mut name: *const (), l: *const __gnu_test_func_callback) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut res: *const () = (unimplemented!()).cast_const();
    if !(res).is_null() {
        let mut preLC_ALL: () = unimplemented!();
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        (unsafe { __throw_runtime_error_3(unimplemented!()) });
    }
}
pub unsafe fn run_tests_wrapped_env_50(
    mut name: *const (),
    mut env: *const (),
    l: *const __gnu_test_func_callback,
) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut oldENV: *const () = (unimplemented!()).cast_const();
    if !(unimplemented!() != 0) {
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        unimplemented!();
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { __throw_runtime_error_3(unimplemented!()) });
    }
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut _M_count_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut next_id__7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union __gnu_test_semun {
    pub val: (),
    pub buf: *mut semid_ds,
    pub array: *mut (),
}
impl Default for __gnu_test_semun {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl __gnu_test_semaphore {}
impl __gnu_test_semaphore {
    pub unsafe fn destructor(&mut self) {
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((self.pid_) == (unsafe { getpid_40() })) {
            (unsafe { semctl_42(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_51(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_51(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_52(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_53));
}
static mut tmp_53: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
