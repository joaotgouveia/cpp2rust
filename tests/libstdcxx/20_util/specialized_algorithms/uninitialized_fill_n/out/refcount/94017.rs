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
pub struct __gnu_test_BoundsContainer_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<()>,
}
impl __gnu_test_BoundsContainer_char_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_signed_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<()>,
}
impl __gnu_test_BoundsContainer_signed_char_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_unsigned_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<()>,
}
impl __gnu_test_BoundsContainer_unsigned_char_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_volatile_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<()>,
}
impl __gnu_test_BoundsContainer_volatile_int_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_volatile_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<()>,
}
impl __gnu_test_BoundsContainer_volatile_char_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_OutputContainer_char_ {
    #[offset(16)]
    #[byte_size(8)]
    pub incrementedto: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub writtento: Ptr<()>,
}
impl __gnu_test_OutputContainer_char_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        let __this: Value<__gnu_test_OutputContainer_char_> = Rc::new(RefCell::new(Self {
            incrementedto: (_first).clone(),
            writtento: Ptr::alloc_array(
                (0..({ __gnu_test_BoundsContainer_char_Impl::size(&this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
        }));
        let this: Ptr<__gnu_test_OutputContainer_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_OutputContainer_signed_char_ {
    #[offset(16)]
    #[byte_size(8)]
    pub incrementedto: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub writtento: Ptr<()>,
}
impl __gnu_test_OutputContainer_signed_char_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        let __this: Value<__gnu_test_OutputContainer_signed_char_> = Rc::new(RefCell::new(Self {
            incrementedto: (_first).clone(),
            writtento: Ptr::alloc_array(
                (0..({ __gnu_test_BoundsContainer_signed_char_Impl::size(&this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
        }));
        let this: Ptr<__gnu_test_OutputContainer_signed_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_OutputContainer_unsigned_char_ {
    #[offset(16)]
    #[byte_size(8)]
    pub incrementedto: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub writtento: Ptr<()>,
}
impl __gnu_test_OutputContainer_unsigned_char_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        let __this: Value<__gnu_test_OutputContainer_unsigned_char_> =
            Rc::new(RefCell::new(Self {
                incrementedto: (_first).clone(),
                writtento: Ptr::alloc_array(
                    (0..({ __gnu_test_BoundsContainer_unsigned_char_Impl::size(&this) }))
                        .map(|_| unimplemented!())
                        .collect::<Box<[()]>>(),
                ),
            }));
        let this: Ptr<__gnu_test_OutputContainer_unsigned_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_OutputContainer_volatile_int_ {
    #[offset(16)]
    #[byte_size(8)]
    pub incrementedto: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub writtento: Ptr<()>,
}
impl __gnu_test_OutputContainer_volatile_int_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        let __this: Value<__gnu_test_OutputContainer_volatile_int_> = Rc::new(RefCell::new(Self {
            incrementedto: (_first).clone(),
            writtento: Ptr::alloc_array(
                (0..({ __gnu_test_BoundsContainer_volatile_int_Impl::size(&this) }))
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            ),
        }));
        let this: Ptr<__gnu_test_OutputContainer_volatile_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_OutputContainer_volatile_char_ {
    #[offset(16)]
    #[byte_size(8)]
    pub incrementedto: Ptr<()>,
    #[offset(24)]
    #[byte_size(8)]
    pub writtento: Ptr<()>,
}
impl __gnu_test_OutputContainer_volatile_char_ {
    pub fn new(mut _first: Ptr<()>, mut _last: Ptr<()>) -> Self {
        let __this: Value<__gnu_test_OutputContainer_volatile_char_> =
            Rc::new(RefCell::new(Self {
                incrementedto: (_first).clone(),
                writtento: Ptr::alloc_array(
                    (0..({ __gnu_test_BoundsContainer_volatile_char_Impl::size(&this) }))
                        .map(|_| unimplemented!())
                        .collect::<Box<[()]>>(),
                ),
            }));
        let this: Ptr<__gnu_test_OutputContainer_volatile_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_output_iterator_wrapper_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_OutputContainer_char_>,
}
impl __gnu_test_output_iterator_wrapper_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_output_iterator_wrapper_signed_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_OutputContainer_signed_char_>,
}
impl __gnu_test_output_iterator_wrapper_signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_output_iterator_wrapper_unsigned_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_OutputContainer_unsigned_char_>,
}
impl __gnu_test_output_iterator_wrapper_unsigned_char_ {}
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
impl __gnu_test_output_iterator_wrapper_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_output_iterator_wrapper_volatile_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_OutputContainer_volatile_int_>,
}
impl __gnu_test_output_iterator_wrapper_volatile_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_output_iterator_wrapper_volatile_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_OutputContainer_volatile_char_>,
}
impl __gnu_test_output_iterator_wrapper_volatile_char_ {}
impl __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ {
    pub fn new_1(arr: Ptr<()>) -> Self {
        __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_2({ (arr).clone() }, {
            (arr).offset((5_()) as isize)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(32)]
    pub bounds: __gnu_test_OutputContainer_char_,
}
impl __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_ {
    pub fn new_2(mut first: Ptr<()>, mut last: Ptr<()>) -> Self {
        Self {
            bounds: __gnu_test_OutputContainer_char_::new({ (first).clone() }, { (last).clone() }),
        }
    }
}
impl __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ {
    pub fn new_1(arr: Ptr<()>) -> Self {
        __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_2(
            { (arr).clone() },
            { (arr).offset((5_()) as isize) },
        )
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(32)]
    pub bounds: __gnu_test_OutputContainer_signed_char_,
}
impl __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_ {
    pub fn new_2(mut first: Ptr<()>, mut last: Ptr<()>) -> Self {
        Self {
            bounds: __gnu_test_OutputContainer_signed_char_::new({ (first).clone() }, {
                (last).clone()
            }),
        }
    }
}
impl __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ {
    pub fn new_1(arr: Ptr<()>) -> Self {
        __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_2(
            { (arr).clone() },
            { (arr).offset((5_()) as isize) },
        )
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(32)]
    pub bounds: __gnu_test_OutputContainer_unsigned_char_,
}
impl __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_ {
    pub fn new_2(mut first: Ptr<()>, mut last: Ptr<()>) -> Self {
        Self {
            bounds: __gnu_test_OutputContainer_unsigned_char_::new({ (first).clone() }, {
                (last).clone()
            }),
        }
    }
}
impl __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_ {
    pub fn new_1(arr: Ptr<()>) -> Self {
        __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_2({ (arr).clone() }, {
            (arr).offset((5_()) as isize)
        })
    }
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
impl __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_ {
    pub fn new_1(arr: Ptr<()>) -> Self {
        __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_::new_2(
            { (arr).clone() },
            { (arr).offset((5_()) as isize) },
        )
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(32)]
    pub bounds: __gnu_test_OutputContainer_volatile_int_,
}
impl __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_ {
    pub fn new_2(mut first: Ptr<()>, mut last: Ptr<()>) -> Self {
        Self {
            bounds: __gnu_test_OutputContainer_volatile_int_::new({ (first).clone() }, {
                (last).clone()
            }),
        }
    }
}
impl __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_ {
    pub fn new_1(arr: Ptr<()>) -> Self {
        __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_::new_2(
            { (arr).clone() },
            { (arr).offset((5_()) as isize) },
        )
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(32)]
pub struct __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(32)]
    pub bounds: __gnu_test_OutputContainer_volatile_char_,
}
impl __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_ {
    pub fn new_2(mut first: Ptr<()>, mut last: Ptr<()>) -> Self {
        Self {
            bounds: __gnu_test_OutputContainer_volatile_char_::new({ (first).clone() }, {
                (last).clone()
            }),
        }
    }
}
pub fn test01_10() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new((('a' as ()) as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new((('a' as ()) as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_15() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(100_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(100_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = 100]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(100_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(100_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = 100]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_16() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(150_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(150_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = 150]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(150_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(150_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = 150]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_17() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(300_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(300_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = 300]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(300_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(300_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = 300]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_18() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(1000_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(1000_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = 1000]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(1000_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(1000_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = 1000]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_19() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(-10000_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(-10000_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = -10000L]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(-10000_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(-10000_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = char, value = -10000L]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_20() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = signed char, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = signed char, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_21() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(100_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(100_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = signed char, value = 100]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(100_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(100_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = signed char, value = 100]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_22() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(150_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(150_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = signed char, value = 150]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(150_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(150_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = signed char, value = 150]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_23() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(300_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(300_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = signed char, value = 300]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(300_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(300_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = signed char, value = 300]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_24() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = unsigned char, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = unsigned char, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_25() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(100_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(100_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = unsigned char, value = 100]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(100_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(100_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = unsigned char, value = 100]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_26() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(150_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(150_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = unsigned char, value = 150]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(150_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(150_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = unsigned char, value = 150]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_27() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(300_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(300_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = unsigned char, value = 300]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(300_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(300_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = unsigned char, value = 300]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_28() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = int, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = int, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_29() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new((97 as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new((97 as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = int, value = u8\'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new((97 as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new((97 as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = int, value = u8\'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_30() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = int, value = (signed char)\'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = int, value = (signed char)\'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_31() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = int, value = (unsigned char)\'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = int, value = (unsigned char)\'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_32() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = volatile int, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(('a' as ())));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(('a' as ())));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = volatile int, value = \'a\']",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_33() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(500_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(500_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = volatile int, value = 500]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(500_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(500_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = volatile int, value = 500]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub fn test01_34() {
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(500_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(500_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (36_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = volatile char, value = 500]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
                            .into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    {
        let x: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let rx: Value<__gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_::new_1({
                    x.as_pointer()
                }),
            ));
        let _dtor_rx = ScopedDestructor::new(&rx, |__p| __p.destructor());
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(500_()));
            (*uninitialized_fill_n_11.with(Value::clone).borrow_mut())(
                (x.as_pointer() as Ptr<()>),
                5_(),
                ___x.as_pointer(),
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let ___value: Value<()> = Rc::new(RefCell::new(500_()));
                (*count_12.with(Value::clone).borrow_mut())(
                    x.as_pointer(),
                    ___value.as_pointer(),
                    None,
                )
            }) == ({ (*size_13.with(Value::clone).borrow_mut())(x.as_pointer()) }))
            {
                ({
                    __builtin_fprintf_14(
                        libcc2rs::stderr_unsafe(),
                        Ptr::<()>::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n"),
                        Box::new([
                            (Ptr::<()>::from_string_literal(b"94017.cc")).into(),
                            (43_()).into(),
                            (Ptr::<()>::from_string_literal(
                                b"void test01() [Out = volatile char, value = 500]",
                            ))
                            .into(),
                            (Ptr::<()>::from_string_literal(
                                b"ranges::count(x, static_cast<Out>(value)) == ranges::size(x)",
                            ))
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
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_10() });
    ({ test01_15() });
    ({ test01_16() });
    ({ test01_17() });
    ({ test01_18() });
    ({ test01_19() });
    ({ test01_20() });
    ({ test01_21() });
    ({ test01_22() });
    ({ test01_23() });
    ({ test01_24() });
    ({ test01_25() });
    ({ test01_26() });
    ({ test01_27() });
    ({ test01_28() });
    ({ test01_29() });
    ({ test01_30() });
    ({ test01_31() });
    ({ test01_32() });
    ({ test01_32() });
    ({ test01_33() });
    ({ test01_34() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write((({ getpid_35() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_36(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_37(
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
pub fn set_memory_limits_38(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_39((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_40((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_39((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_40((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_39((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_40((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_41(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_39((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_40((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_42(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_43(
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
    let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if !(unimplemented!()) {
        ({ __throw_runtime_error_3((s).clone()) });
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_44(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                    __builtin_fprintf_14 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        ({ __throw_runtime_error_3(unimplemented!()) });
    }
}
pub fn run_tests_wrapped_env_45(
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
pub fn test_tm_47(
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
    (*tmp_48.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_48.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_48: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
);
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __gnu_test_WritableObject_volatile_char_;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __gnu_test_WritableObject_volatile_int_;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __gnu_test_WritableObject_char_;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __gnu_test_WritableObject_unsigned_char_;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __gnu_test_WritableObject_signed_char_;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __gnu_test_WritableObject_int_;
pub trait __gnu_test_BoundsContainer_char_Impl {
    fn size(&self) -> ();
}
impl __gnu_test_BoundsContainer_char_Impl for Ptr<__gnu_test_BoundsContainer_char_> {
    fn size(&self) -> () {
        return ((*self).with(|__s| __s.last.clone()) - (*self).with(|__s| __s.first.clone()))
            as ();
    }
}
pub trait __gnu_test_BoundsContainer_int_Impl {
    fn size(&self) -> ();
}
impl __gnu_test_BoundsContainer_int_Impl for Ptr<__gnu_test_BoundsContainer_int_> {
    fn size(&self) -> () {
        return ((*self).with(|__s| __s.last.clone()) - (*self).with(|__s| __s.first.clone()))
            as ();
    }
}
pub trait __gnu_test_BoundsContainer_signed_char_Impl {
    fn size(&self) -> ();
}
impl __gnu_test_BoundsContainer_signed_char_Impl for Ptr<__gnu_test_BoundsContainer_signed_char_> {
    fn size(&self) -> () {
        return ((*self).with(|__s| __s.last.clone()) - (*self).with(|__s| __s.first.clone()))
            as ();
    }
}
pub trait __gnu_test_BoundsContainer_unsigned_char_Impl {
    fn size(&self) -> ();
}
impl __gnu_test_BoundsContainer_unsigned_char_Impl
    for Ptr<__gnu_test_BoundsContainer_unsigned_char_>
{
    fn size(&self) -> () {
        return ((*self).with(|__s| __s.last.clone()) - (*self).with(|__s| __s.first.clone()))
            as ();
    }
}
pub trait __gnu_test_BoundsContainer_volatile_char_Impl {
    fn size(&self) -> ();
}
impl __gnu_test_BoundsContainer_volatile_char_Impl
    for Ptr<__gnu_test_BoundsContainer_volatile_char_>
{
    fn size(&self) -> () {
        return ((*self).with(|__s| __s.last.clone()) - (*self).with(|__s| __s.first.clone()))
            as ();
    }
}
pub trait __gnu_test_BoundsContainer_volatile_int_Impl {
    fn size(&self) -> ();
}
impl __gnu_test_BoundsContainer_volatile_int_Impl
    for Ptr<__gnu_test_BoundsContainer_volatile_int_>
{
    fn size(&self) -> () {
        return ((*self).with(|__s| __s.last.clone()) - (*self).with(|__s| __s.first.clone()))
            as ();
    }
}
pub trait __gnu_test_OutputContainer_char_Impl {
    fn destructor(&self);
}
impl __gnu_test_OutputContainer_char_Impl for Ptr<__gnu_test_OutputContainer_char_> {
    fn destructor(&self) {
        (*self).with(|__s| __s.writtento.clone()).delete();
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
pub trait __gnu_test_OutputContainer_signed_char_Impl {
    fn destructor(&self);
}
impl __gnu_test_OutputContainer_signed_char_Impl for Ptr<__gnu_test_OutputContainer_signed_char_> {
    fn destructor(&self) {
        (*self).with(|__s| __s.writtento.clone()).delete();
    }
}
pub trait __gnu_test_OutputContainer_unsigned_char_Impl {
    fn destructor(&self);
}
impl __gnu_test_OutputContainer_unsigned_char_Impl
    for Ptr<__gnu_test_OutputContainer_unsigned_char_>
{
    fn destructor(&self) {
        (*self).with(|__s| __s.writtento.clone()).delete();
    }
}
pub trait __gnu_test_OutputContainer_volatile_char_Impl {
    fn destructor(&self);
}
impl __gnu_test_OutputContainer_volatile_char_Impl
    for Ptr<__gnu_test_OutputContainer_volatile_char_>
{
    fn destructor(&self) {
        (*self).with(|__s| __s.writtento.clone()).delete();
    }
}
pub trait __gnu_test_OutputContainer_volatile_int_Impl {
    fn destructor(&self);
}
impl __gnu_test_OutputContainer_volatile_int_Impl
    for Ptr<__gnu_test_OutputContainer_volatile_int_>
{
    fn destructor(&self) {
        (*self).with(|__s| __s.writtento.clone()).delete();
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
pub trait __gnu_test_output_iterator_wrapper_char_Impl {
    fn operator_deref(&self) -> __gnu_test_WritableObject_char_ {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__gnu_test_output_iterator_wrapper_char_> {
        unimplemented!()
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> __gnu_test_output_iterator_wrapper_char_ {
        unimplemented!()
    }
}
impl __gnu_test_output_iterator_wrapper_char_Impl
    for Ptr<__gnu_test_output_iterator_wrapper_char_>
{
}
pub trait __gnu_test_output_iterator_wrapper_int_Impl {
    fn operator_deref(&self) -> __gnu_test_WritableObject_int_ {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__gnu_test_output_iterator_wrapper_int_> {
        unimplemented!()
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> __gnu_test_output_iterator_wrapper_int_ {
        unimplemented!()
    }
}
impl __gnu_test_output_iterator_wrapper_int_Impl for Ptr<__gnu_test_output_iterator_wrapper_int_> {}
pub trait __gnu_test_output_iterator_wrapper_signed_char_Impl {
    fn operator_deref(&self) -> __gnu_test_WritableObject_signed_char_ {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__gnu_test_output_iterator_wrapper_signed_char_> {
        unimplemented!()
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> __gnu_test_output_iterator_wrapper_signed_char_ {
        unimplemented!()
    }
}
impl __gnu_test_output_iterator_wrapper_signed_char_Impl
    for Ptr<__gnu_test_output_iterator_wrapper_signed_char_>
{
}
pub trait __gnu_test_output_iterator_wrapper_unsigned_char_Impl {
    fn operator_deref(&self) -> __gnu_test_WritableObject_unsigned_char_ {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__gnu_test_output_iterator_wrapper_unsigned_char_> {
        unimplemented!()
    }
    fn operator_post_inc_7(
        &self,
        mut _a0: (),
    ) -> __gnu_test_output_iterator_wrapper_unsigned_char_ {
        unimplemented!()
    }
}
impl __gnu_test_output_iterator_wrapper_unsigned_char_Impl
    for Ptr<__gnu_test_output_iterator_wrapper_unsigned_char_>
{
}
pub trait __gnu_test_output_iterator_wrapper_volatile_char_Impl {
    fn operator_deref(&self) -> __gnu_test_WritableObject_volatile_char_ {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__gnu_test_output_iterator_wrapper_volatile_char_> {
        unimplemented!()
    }
    fn operator_post_inc_7(
        &self,
        mut _a0: (),
    ) -> __gnu_test_output_iterator_wrapper_volatile_char_ {
        unimplemented!()
    }
}
impl __gnu_test_output_iterator_wrapper_volatile_char_Impl
    for Ptr<__gnu_test_output_iterator_wrapper_volatile_char_>
{
}
pub trait __gnu_test_output_iterator_wrapper_volatile_int_Impl {
    fn operator_deref(&self) -> __gnu_test_WritableObject_volatile_int_ {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__gnu_test_output_iterator_wrapper_volatile_int_> {
        unimplemented!()
    }
    fn operator_post_inc_7(&self, mut _a0: ()) -> __gnu_test_output_iterator_wrapper_volatile_int_ {
        unimplemented!()
    }
}
impl __gnu_test_output_iterator_wrapper_volatile_int_Impl
    for Ptr<__gnu_test_output_iterator_wrapper_volatile_int_>
{
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_35() })) {
            ({
                semctl_37(
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
            semop_46(
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
            semop_46(
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
pub trait __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_Impl {
    fn destructor(&self);
}
impl __gnu_test_test_range_char____gnu_test_output_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_range_char____gnu_test_output_iterator_wrapper_>
{
    fn destructor(&self) {
        field_ptr!(self, bounds).destructor();
    }
}
pub trait __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_Impl {
    fn destructor(&self);
}
impl __gnu_test_test_range_int____gnu_test_output_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_range_int____gnu_test_output_iterator_wrapper_>
{
    fn destructor(&self) {
        field_ptr!(self, bounds).destructor();
    }
}
pub trait __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_Impl {
    fn destructor(&self);
}
impl __gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_range_signed_char____gnu_test_output_iterator_wrapper_>
{
    fn destructor(&self) {
        field_ptr!(self, bounds).destructor();
    }
}
pub trait __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_Impl {
    fn destructor(&self);
}
impl __gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_range_unsigned_char____gnu_test_output_iterator_wrapper_>
{
    fn destructor(&self) {
        field_ptr!(self, bounds).destructor();
    }
}
pub trait __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_Impl {
    fn destructor(&self);
}
impl __gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_range_volatile_char____gnu_test_output_iterator_wrapper_>
{
    fn destructor(&self) {
        field_ptr!(self, bounds).destructor();
    }
}
pub trait __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_Impl {
    fn destructor(&self);
}
impl __gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_range_volatile_int____gnu_test_output_iterator_wrapper_>
{
    fn destructor(&self) {
        field_ptr!(self, bounds).destructor();
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
