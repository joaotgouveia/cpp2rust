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
pub struct __gnu_test_BoundsContainer_char_ {
    pub first: *mut (),
    pub last: *mut (),
}
impl __gnu_test_BoundsContainer_char_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return ((self.last as usize - self.first as usize) / ::std::mem::size_of::<()>()) as ();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_BoundsContainer_signed_char_ {
    pub first: *mut (),
    pub last: *mut (),
}
impl __gnu_test_BoundsContainer_signed_char_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return ((self.last as usize - self.first as usize) / ::std::mem::size_of::<()>()) as ();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_BoundsContainer_unsigned_char_ {
    pub first: *mut (),
    pub last: *mut (),
}
impl __gnu_test_BoundsContainer_unsigned_char_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return ((self.last as usize - self.first as usize) / ::std::mem::size_of::<()>()) as ();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_BoundsContainer_int_ {
    pub first: *mut (),
    pub last: *mut (),
}
impl __gnu_test_BoundsContainer_int_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return ((self.last as usize - self.first as usize) / ::std::mem::size_of::<()>()) as ();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_BoundsContainer_volatile_int_ {
    pub first: *mut (),
    pub last: *mut (),
}
impl __gnu_test_BoundsContainer_volatile_int_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return ((self.last as usize - self.first as usize) / ::std::mem::size_of::<()>()) as ();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_BoundsContainer_volatile_char_ {
    pub first: *mut (),
    pub last: *mut (),
}
impl __gnu_test_BoundsContainer_volatile_char_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return ((self.last as usize - self.first as usize) / ::std::mem::size_of::<()>()) as ();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_OutputContainer_char_ {
    pub incrementedto: *mut (),
    pub writtento: *mut (),
}
impl __gnu_test_OutputContainer_char_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            incrementedto: _first,
            writtento: Box::leak(
                (0..(unsafe { __gnu_test_BoundsContainer_char_::size(&mut this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.writtento;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_OutputContainer_signed_char_ {
    pub incrementedto: *mut (),
    pub writtento: *mut (),
}
impl __gnu_test_OutputContainer_signed_char_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            incrementedto: _first,
            writtento: Box::leak(
                (0..(unsafe { __gnu_test_BoundsContainer_signed_char_::size(&mut this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.writtento;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_OutputContainer_unsigned_char_ {
    pub incrementedto: *mut (),
    pub writtento: *mut (),
}
impl __gnu_test_OutputContainer_unsigned_char_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            incrementedto: _first,
            writtento: Box::leak(
                (0..(unsafe { __gnu_test_BoundsContainer_unsigned_char_::size(&mut this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.writtento;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_OutputContainer_int_ {
    pub incrementedto: *mut (),
    pub writtento: *mut (),
}
impl __gnu_test_OutputContainer_int_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            incrementedto: _first,
            writtento: Box::leak(
                (0..(unsafe { __gnu_test_BoundsContainer_int_::size(&mut this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.writtento;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_OutputContainer_volatile_int_ {
    pub incrementedto: *mut (),
    pub writtento: *mut (),
}
impl __gnu_test_OutputContainer_volatile_int_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            incrementedto: _first,
            writtento: Box::leak(
                (0..(unsafe { __gnu_test_BoundsContainer_volatile_int_::size(&mut this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.writtento;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_OutputContainer_volatile_char_ {
    pub incrementedto: *mut (),
    pub writtento: *mut (),
}
impl __gnu_test_OutputContainer_volatile_char_ {
    pub unsafe fn new(mut _first: *mut (), mut _last: *mut ()) -> Self {
        let mut this = Self {
            incrementedto: _first,
            writtento: Box::leak(
                (0..(unsafe { __gnu_test_BoundsContainer_volatile_char_::size(&mut this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.writtento;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_output_iterator_wrapper_char_ {
    pub ptr: *mut (),
    pub SharedInfo: *mut __gnu_test_OutputContainer_char_,
}
impl __gnu_test_output_iterator_wrapper_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_output_iterator_wrapper_signed_char_ {
    pub ptr: *mut (),
    pub SharedInfo: *mut __gnu_test_OutputContainer_signed_char_,
}
impl __gnu_test_output_iterator_wrapper_signed_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_output_iterator_wrapper_unsigned_char_ {
    pub ptr: *mut (),
    pub SharedInfo: *mut __gnu_test_OutputContainer_unsigned_char_,
}
impl __gnu_test_output_iterator_wrapper_unsigned_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_output_iterator_wrapper_int_ {
    pub ptr: *mut (),
    pub SharedInfo: *mut __gnu_test_OutputContainer_int_,
}
impl __gnu_test_output_iterator_wrapper_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_output_iterator_wrapper_volatile_int_ {
    pub ptr: *mut (),
    pub SharedInfo: *mut __gnu_test_OutputContainer_volatile_int_,
}
impl __gnu_test_output_iterator_wrapper_volatile_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_output_iterator_wrapper_volatile_char_ {
    pub ptr: *mut (),
    pub SharedInfo: *mut __gnu_test_OutputContainer_volatile_char_,
}
impl __gnu_test_output_iterator_wrapper_volatile_char_ {}
impl __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_1(arr: *mut [(); 5]) -> Self {
        let mut this = __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_2(
            { (*arr).as_mut_ptr() },
            { (*arr).as_mut_ptr().offset((5_()) as isize) },
        );
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ {
    pub bounds: __gnu_test_OutputContainer_char_,
}
impl __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_2(mut first: *mut (), mut last: *mut ()) -> Self {
        let mut this = Self {
            bounds: __gnu_test_OutputContainer_char_::new({ first }, { last }),
        };
        this
    }
}
impl __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn destructor(&mut self) {
        __gnu_test_OutputContainer_char_::destructor(&mut self.bounds);
    }
}
impl __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_1(arr: *mut [(); 5]) -> Self {
        let mut this =
            __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_2(
                { (*arr).as_mut_ptr() },
                { (*arr).as_mut_ptr().offset((5_()) as isize) },
            );
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ {
    pub bounds: __gnu_test_OutputContainer_signed_char_,
}
impl __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_2(mut first: *mut (), mut last: *mut ()) -> Self {
        let mut this = Self {
            bounds: __gnu_test_OutputContainer_signed_char_::new({ first }, { last }),
        };
        this
    }
}
impl __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn destructor(&mut self) {
        __gnu_test_OutputContainer_signed_char_::destructor(&mut self.bounds);
    }
}
impl __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_1(arr: *mut [(); 5]) -> Self {
        let mut this =
            __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_2(
                { (*arr).as_mut_ptr() },
                { (*arr).as_mut_ptr().offset((5_()) as isize) },
            );
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ {
    pub bounds: __gnu_test_OutputContainer_unsigned_char_,
}
impl __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_2(mut first: *mut (), mut last: *mut ()) -> Self {
        let mut this = Self {
            bounds: __gnu_test_OutputContainer_unsigned_char_::new({ first }, { last }),
        };
        this
    }
}
impl __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn destructor(&mut self) {
        __gnu_test_OutputContainer_unsigned_char_::destructor(&mut self.bounds);
    }
}
impl __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_1(arr: *mut [(); 5]) -> Self {
        let mut this = __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_2(
            { (*arr).as_mut_ptr() },
            { (*arr).as_mut_ptr().offset((5_()) as isize) },
        );
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ {
    pub bounds: __gnu_test_OutputContainer_int_,
}
impl __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_2(mut first: *mut (), mut last: *mut ()) -> Self {
        let mut this = Self {
            bounds: __gnu_test_OutputContainer_int_::new({ first }, { last }),
        };
        this
    }
}
impl __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn destructor(&mut self) {
        __gnu_test_OutputContainer_int_::destructor(&mut self.bounds);
    }
}
impl __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_1(arr: *mut [(); 5]) -> Self {
        let mut this =
            __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_::new_2(
                { (*arr).as_mut_ptr() },
                { (*arr).as_mut_ptr().offset((5_()) as isize) },
            );
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_ {
    pub bounds: __gnu_test_OutputContainer_volatile_int_,
}
impl __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_2(mut first: *mut (), mut last: *mut ()) -> Self {
        let mut this = Self {
            bounds: __gnu_test_OutputContainer_volatile_int_::new({ first }, { last }),
        };
        this
    }
}
impl __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn destructor(&mut self) {
        __gnu_test_OutputContainer_volatile_int_::destructor(&mut self.bounds);
    }
}
impl __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_1(arr: *mut [(); 5]) -> Self {
        let mut this =
            __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_::new_2(
                { (*arr).as_mut_ptr() },
                { (*arr).as_mut_ptr().offset((5_()) as isize) },
            );
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_ {
    pub bounds: __gnu_test_OutputContainer_volatile_char_,
}
impl __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn new_2(mut first: *mut (), mut last: *mut ()) -> Self {
        let mut this = Self {
            bounds: __gnu_test_OutputContainer_volatile_char_::new({ first }, { last }),
        };
        this
    }
}
impl __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_ {
    pub unsafe fn destructor(&mut self) {
        __gnu_test_OutputContainer_volatile_char_::destructor(&mut self.bounds);
    }
}
pub unsafe fn test01_10() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = (('a' as ()) as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = char, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = (('a' as ()) as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = char, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_15() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 100_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 100_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = char, value = 100]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 100_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 100_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = char, value = 100]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_16() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 150_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 150_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = char, value = 150]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 150_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 150_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = char, value = 150]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_17() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 300_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 300_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = char, value = 300]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 300_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 300_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = char, value = 300]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_18() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 1000_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 1000_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = char, value = 1000]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 1000_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 1000_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = char, value = 1000]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_19() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = -10000_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = -10000_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = char, value = -10000L]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = -10000_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = -10000_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = char, value = -10000L]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_20() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = signed char, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = signed char, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_21() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 100_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 100_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = signed char, value = 100]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 100_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 100_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = signed char, value = 100]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_22() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 150_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 150_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = signed char, value = 150]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 150_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 150_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = signed char, value = 150]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_23() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 300_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 300_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = signed char, value = 300]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 300_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 300_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = signed char, value = 300]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_24() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = unsigned char, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = unsigned char, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_25() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 100_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 100_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = unsigned char, value = 100]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 100_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 100_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = unsigned char, value = 100]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_26() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 150_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 150_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = unsigned char, value = 150]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 150_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 150_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = unsigned char, value = 150]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_27() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 300_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 300_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = unsigned char, value = 300]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 300_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 300_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = unsigned char, value = 300]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_28() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = int, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = int, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_29() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = (97 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = (97 as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = int, value = u8\'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = (97 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = (97 as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = int, value = u8\'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_30() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = int, value = (signed char)\'a\']".as_ptr())
                                .into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = int, value = (signed char)\'a\']".as_ptr())
                                .into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_31() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = int, value = (unsigned char)\'a\']".as_ptr())
                                .into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_1({ &mut x });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = int, value = (unsigned char)\'a\']".as_ptr())
                                .into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_32() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = volatile int, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = ('a' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = ('a' as ());
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = volatile int, value = \'a\']".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_33() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 500_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 500_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = volatile int, value = 500]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 500_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 500_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = volatile int, value = 500]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test01_34() {
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        (unsafe {
            let mut ___x: () = 500_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 500_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (36_()).into(),
                            (c"void test01() [Out = volatile char, value = 500]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let mut x: [(); 5] = [unimplemented!(); 5];
        let mut rx: __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_ =
            __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_::new_1({
                &mut x
            });
        let _dtor_rx = ScopedDestructorUnsafe::new(
            &raw mut rx,
            __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_::destructor,
        );
        (unsafe {
            let mut ___x: () = 500_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut uninitialized_fill_n_11))(
                x.as_mut_ptr(),
                5_(),
                &mut ___x,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let mut ___value: () = 500_();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12))(
                    &mut x,
                    &mut ___value,
                    None,
                )
            }) == (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut size_13))(&mut x)
            })) {
                (unsafe {
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"94017.cc".as_ptr()).into(),
                            (43_()).into(),
                            (c"void test01() [Out = volatile char, value = 500]".as_ptr()).into(),
                            (c"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)"
                                .as_ptr())
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_10() });
    (unsafe { test01_15() });
    (unsafe { test01_16() });
    (unsafe { test01_17() });
    (unsafe { test01_18() });
    (unsafe { test01_19() });
    (unsafe { test01_20() });
    (unsafe { test01_21() });
    (unsafe { test01_22() });
    (unsafe { test01_23() });
    (unsafe { test01_24() });
    (unsafe { test01_25() });
    (unsafe { test01_26() });
    (unsafe { test01_27() });
    (unsafe { test01_28() });
    (unsafe { test01_29() });
    (unsafe { test01_30() });
    (unsafe { test01_31() });
    (unsafe { test01_32() });
    (unsafe { test01_32() });
    (unsafe { test01_33() });
    (unsafe { test01_34() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_35() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_36(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_37(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_38(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_39(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_40(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_39(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_40(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_39((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_40(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_41(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_39(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_40(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_42(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_43(
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
pub unsafe fn run_tests_wrapped_locale_44(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_14 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_45(
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
        if ((self.pid_) == (unsafe { getpid_35() })) {
            (unsafe { semctl_37(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_46(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_46(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_47(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48));
}
static mut tmp_48: std::cell::LazyCell<::libc::tm> =
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
