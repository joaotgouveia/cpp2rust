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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct test_offsets_ok {
    #[offset(0)]
    pub in_size: usize,
    #[offset(8)]
    pub out_size: usize,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct test_offsets_partial {
    #[offset(0)]
    pub in_size: usize,
    #[offset(8)]
    pub out_size: usize,
    #[offset(16)]
    pub expected_in_next: usize,
    #[offset(24)]
    pub expected_out_next: usize,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct test_offsets_error_unsigned_char_ {
    #[offset(0)]
    pub in_size: usize,
    #[offset(8)]
    pub out_size: usize,
    #[offset(16)]
    pub expected_in_next: usize,
    #[offset(24)]
    pub expected_out_next: usize,
    #[offset(32)]
    pub replace_char: u8,
    #[offset(40)]
    pub replace_pos: usize,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct test_offsets_error_char16_t_ {
    #[offset(0)]
    pub in_size: usize,
    #[offset(8)]
    pub out_size: usize,
    #[offset(16)]
    pub expected_in_next: usize,
    #[offset(24)]
    pub expected_out_next: usize,
    #[offset(32)]
    pub replace_char: u16,
    #[offset(40)]
    pub replace_pos: usize,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct test_offsets_error_wchar_t_ {
    #[offset(0)]
    pub in_size: usize,
    #[offset(8)]
    pub out_size: usize,
    #[offset(16)]
    pub expected_in_next: usize,
    #[offset(24)]
    pub expected_out_next: usize,
    #[offset(32)]
    pub replace_char: i32,
    #[offset(40)]
    pub replace_pos: usize,
}
pub fn array_size_9(_a0: Ptr<u8>) -> usize {
    return (11_u64 as usize);
}
pub fn array_size_10(_a0: Ptr<u32>) -> usize {
    return (5_u64 as usize);
}
pub fn array_size_11(_a0: Ptr<u16>) -> usize {
    return (6_u64 as usize);
}
pub fn array_size_12(_a0: Ptr<u8>) -> usize {
    return (7_u64 as usize);
}
pub fn array_size_13(_a0: Ptr<u16>) -> usize {
    return (4_u64 as usize);
}
pub fn array_size_14(_a0: Ptr<i8>) -> usize {
    return (12_u64 as usize);
}
pub fn array_size_15(_a0: Ptr<i8>) -> usize {
    return (10_u64 as usize);
}
pub fn array_size_16(_a0: Ptr<i8>) -> usize {
    return (8_u64 as usize);
}
pub fn array_size_17(_a0: Ptr<i8>) -> usize {
    return (6_u64 as usize);
}
pub fn array_size_18(_a0: Ptr<i32>) -> usize {
    return (5_u64 as usize);
}
pub fn array_size_19(_a0: Ptr<i8>) -> usize {
    return (11_u64 as usize);
}
pub fn array_size_20(_a0: Ptr<i32>) -> usize {
    return (4_u64 as usize);
}
pub fn array_size_21(_a0: Ptr<i32>) -> usize {
    return (6_u64 as usize);
}
pub fn utf8_to_utf32_in_ok_22(cvt: Ptr<()>) {
    let input: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    let expected: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let in_: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let exp: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 4_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_ok]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 6_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 10_usize,
            out_size: 4_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_ok>),
        (offsets.as_pointer() as Ptr<test_offsets_ok>).offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_19(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_20(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.in_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.out_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.out_size < ({ array_size_20(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.out_size) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_ok>),
        (offsets.as_pointer() as Ptr<test_offsets_ok>).offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_19(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_18(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.in_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.out_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.out_size < ({ array_size_18(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.out_size) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub fn utf8_to_utf32_in_partial_23(cvt: Ptr<()>) {
    let input: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    let expected: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let in_: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let exp: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 4_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_partial]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 6_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_partial>),
        (offsets.as_pointer() as Ptr<test_offsets_partial>).offset((16_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_19(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_20(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_20(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.expected_out_next) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.expected_in_next) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub fn utf8_to_utf32_in_error_24(cvt: Ptr<()>) {
    let input: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0",
    )));
    let expected: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        55040 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let in_: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let exp: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 4_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_error_unsigned_char_]>> =
        Rc::new(RefCell::new(Box::new([
            test_offsets_error_unsigned_char_ {
                in_size: 1_usize,
                out_size: 4_usize,
                expected_in_next: 0_usize,
                expected_out_next: 0_usize,
                replace_char: 255_u8,
                replace_pos: 0_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 4_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 255_u8,
                replace_pos: 1_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 255_u8,
                replace_pos: 3_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 6_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 1_usize,
                out_size: 4_usize,
                expected_in_next: 0_usize,
                expected_out_next: 0_usize,
                replace_char: 170_u8,
                replace_pos: 0_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 4_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 170_u8,
                replace_pos: 1_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 170_u8,
                replace_pos: 3_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 170_u8,
                replace_pos: 6_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 4_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 2_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 4_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 255_u8,
                replace_pos: 2_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 255_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 5_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 8_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 255_u8,
                replace_pos: 5_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 8_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 9_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 9_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 5_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 8_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 9_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 5_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 255_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 8_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 9_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 9_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 8_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 9_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 8_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 160_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 172_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 176_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 188_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 4_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 192_u8,
                replace_pos: 1_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 4_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 193_u8,
                replace_pos: 1_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 4_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 224_u8,
                replace_pos: 3_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 240_u8,
                replace_pos: 6_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 245_u8,
                replace_pos: 6_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 4_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 154_u8,
                replace_pos: 7_usize,
            },
        ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_error_unsigned_char_>),
        (offsets.as_pointer() as Ptr<test_offsets_error_unsigned_char_>).offset((38_i64) as isize),
    ) {
        let mut t: test_offsets_error_unsigned_char_ = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_19(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_20(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: i8 = (*in_.borrow())[(t.replace_pos) as usize];
        (*in_.borrow_mut())[(t.replace_pos) as usize] = { (t.replace_char as i8) };
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_20(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.expected_out_next) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.expected_in_next) {
                unimplemented!();
                std::process::abort();
            }
        }
        (*in_.borrow_mut())[(t.replace_pos) as usize] = old_char;
    }
}
pub fn utf8_to_utf32_in_25(cvt: Ptr<()>) {
    ({ utf8_to_utf32_in_ok_22((cvt).clone()) });
    ({ utf8_to_utf32_in_partial_23((cvt).clone()) });
    ({ utf8_to_utf32_in_error_24((cvt).clone()) });
}
pub fn utf32_to_utf8_out_ok_26(cvt: Ptr<()>) {
    let input: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    let expected: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let in_: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let exp: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 4_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_ok]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 6_usize,
        },
        test_offsets_ok {
            in_size: 4_usize,
            out_size: 10_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_ok>),
        (offsets.as_pointer() as Ptr<test_offsets_ok>).offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_18(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_15(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i32> = Ptr::<i32>::null();
        let mut out_next: Ptr<i8> = Ptr::<i8>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i32>).offset((t.in_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i8>).offset((t.out_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.out_size < ({ array_size_15(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(((*out.borrow())[(t.out_size) as usize] as i32) == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub fn utf32_to_utf8_out_partial_27(cvt: Ptr<()>) {
    let input: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    let expected: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let in_: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let exp: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 4_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_partial]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_partial>),
        (offsets.as_pointer() as Ptr<test_offsets_partial>).offset((10_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_18(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_15(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i32> = Ptr::<i32>::null();
        let mut out_next: Ptr<i8> = Ptr::<i8>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i32>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i8>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_15(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(((*out.borrow())[(t.expected_out_next) as usize] as i32) == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub fn utf32_to_utf8_out_error_28(cvt: Ptr<()>) {
    let input: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    let expected: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let in_: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let exp: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 4_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_error_wchar_t_]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_i32,
            replace_pos: 0_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56319_i32,
            replace_pos: 1_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56320_i32,
            replace_pos: 2_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 57343_i32,
            replace_pos: 3_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 1114112_i32,
            replace_pos: 0_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 1114112_i32,
            replace_pos: 1_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 1114112_i32,
            replace_pos: 2_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 1114112_i32,
            replace_pos: 3_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_error_wchar_t_>),
        (offsets.as_pointer() as Ptr<test_offsets_error_wchar_t_>).offset((8_i64) as isize),
    ) {
        let mut t: test_offsets_error_wchar_t_ = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_18(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_15(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: i32 = (*in_.borrow())[(t.replace_pos) as usize];
        (*in_.borrow_mut())[(t.replace_pos) as usize] = { t.replace_char };
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i32> = Ptr::<i32>::null();
        let mut out_next: Ptr<i8> = Ptr::<i8>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i32>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i8>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_15(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(((*out.borrow())[(t.expected_out_next) as usize] as i32) == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*in_.borrow_mut())[(t.replace_pos) as usize] = old_char;
    }
}
pub fn utf32_to_utf8_out_29(cvt: Ptr<()>) {
    ({ utf32_to_utf8_out_ok_26((cvt).clone()) });
    ({ utf32_to_utf8_out_partial_27((cvt).clone()) });
    ({ utf32_to_utf8_out_error_28((cvt).clone()) });
}
pub fn test_utf8_utf32_cvt_30(cvt: Ptr<()>) {
    ({ utf8_to_utf32_in_25((cvt).clone()) });
    ({ utf32_to_utf8_out_29((cvt).clone()) });
}
pub fn utf8_to_utf16_in_ok_31(cvt: Ptr<()>) {
    let input: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    let expected: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let in_: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let exp: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..6).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u16>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 5_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_ok]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 6_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 10_usize,
            out_size: 5_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_ok>),
        (offsets.as_pointer() as Ptr<test_offsets_ok>).offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_19(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_18(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.in_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.out_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.out_size < ({ array_size_18(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.out_size) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_ok>),
        (offsets.as_pointer() as Ptr<test_offsets_ok>).offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_19(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_21(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.in_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.out_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.out_size < ({ array_size_21(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.out_size) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub fn utf8_to_utf16_in_partial_32(cvt: Ptr<()>) {
    let input: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    let expected: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let in_: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let exp: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..6).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u16>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 5_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_partial]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 6_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_partial>),
        (offsets.as_pointer() as Ptr<test_offsets_partial>).offset((20_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_19(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_18(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_18(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.expected_out_next) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.expected_in_next) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub fn utf8_to_utf16_in_error_33(cvt: Ptr<()>) {
    let input: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0",
    )));
    let expected: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        55040 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let in_: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let exp: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..6).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u16>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 5_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_error_unsigned_char_]>> =
        Rc::new(RefCell::new(Box::new([
            test_offsets_error_unsigned_char_ {
                in_size: 1_usize,
                out_size: 5_usize,
                expected_in_next: 0_usize,
                expected_out_next: 0_usize,
                replace_char: 255_u8,
                replace_pos: 0_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 5_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 255_u8,
                replace_pos: 1_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 255_u8,
                replace_pos: 3_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 6_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 1_usize,
                out_size: 5_usize,
                expected_in_next: 0_usize,
                expected_out_next: 0_usize,
                replace_char: 170_u8,
                replace_pos: 0_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 5_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 170_u8,
                replace_pos: 1_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 170_u8,
                replace_pos: 3_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 170_u8,
                replace_pos: 6_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 5_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 2_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 5_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 255_u8,
                replace_pos: 2_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 255_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 5_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 8_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 255_u8,
                replace_pos: 5_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 8_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 9_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 9_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 5_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 8_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 9_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 5_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 255_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 8_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 9_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 7_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 9_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: (('z' as i8) as u8),
                replace_pos: 8_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 9_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 255_u8,
                replace_pos: 8_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 160_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 172_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 176_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 188_u8,
                replace_pos: 4_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 5_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 192_u8,
                replace_pos: 1_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 3_usize,
                out_size: 5_usize,
                expected_in_next: 1_usize,
                expected_out_next: 1_usize,
                replace_char: 193_u8,
                replace_pos: 1_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 6_usize,
                out_size: 5_usize,
                expected_in_next: 3_usize,
                expected_out_next: 2_usize,
                replace_char: 224_u8,
                replace_pos: 3_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 240_u8,
                replace_pos: 6_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 245_u8,
                replace_pos: 6_usize,
            },
            test_offsets_error_unsigned_char_ {
                in_size: 10_usize,
                out_size: 5_usize,
                expected_in_next: 6_usize,
                expected_out_next: 3_usize,
                replace_char: 154_u8,
                replace_pos: 7_usize,
            },
        ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_error_unsigned_char_>),
        (offsets.as_pointer() as Ptr<test_offsets_error_unsigned_char_>).offset((38_i64) as isize),
    ) {
        let mut t: test_offsets_error_unsigned_char_ = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_19(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_18(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: i8 = (*in_.borrow())[(t.replace_pos) as usize];
        (*in_.borrow_mut())[(t.replace_pos) as usize] = { (t.replace_char as i8) };
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_18(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.expected_out_next) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.expected_in_next) {
                unimplemented!();
                std::process::abort();
            }
        }
        (*in_.borrow_mut())[(t.replace_pos) as usize] = old_char;
    }
}
pub fn utf8_to_utf16_in_34(cvt: Ptr<()>) {
    ({ utf8_to_utf16_in_ok_31((cvt).clone()) });
    ({ utf8_to_utf16_in_partial_32((cvt).clone()) });
    ({ utf8_to_utf16_in_error_33((cvt).clone()) });
}
pub fn utf16_to_utf8_out_ok_35(cvt: Ptr<()>) {
    let input: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    let expected: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let in_: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..6).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let exp: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u16>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 5_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_ok]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 6_usize,
        },
        test_offsets_ok {
            in_size: 5_usize,
            out_size: 10_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_ok>),
        (offsets.as_pointer() as Ptr<test_offsets_ok>).offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_21(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_15(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i32> = Ptr::<i32>::null();
        let mut out_next: Ptr<i8> = Ptr::<i8>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i32>).offset((t.in_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i8>).offset((t.out_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.out_size < ({ array_size_15(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(((*out.borrow())[(t.out_size) as usize] as i32) == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub fn utf16_to_utf8_out_partial_36(cvt: Ptr<()>) {
    let input: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    let expected: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let in_: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..6).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let exp: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u16>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 5_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_partial]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_partial>),
        (offsets.as_pointer() as Ptr<test_offsets_partial>).offset((15_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_21(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_15(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i32> = Ptr::<i32>::null();
        let mut out_next: Ptr<i8> = Ptr::<i8>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i32>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i8>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_15(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(((*out.borrow())[(t.expected_out_next) as usize] as i32) == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub fn utf16_to_utf8_out_error_37(cvt: Ptr<()>) {
    let input: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    let expected: Value<Box<[u8]>> = Rc::new(RefCell::new(u8::array_from_literal(
        b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0",
    )));
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let in_: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..6).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let exp: Value<Box<[i8]>> = Rc::new(RefCell::new((0..11).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u16>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u8>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i8));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 5_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!() == 10_usize) {
            unimplemented!();
            std::process::abort();
        }
    }
    let offsets: Value<Box<[test_offsets_error_wchar_t_]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_i32,
            replace_pos: 0_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56319_i32,
            replace_pos: 0_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56320_i32,
            replace_pos: 0_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 57343_i32,
            replace_pos: 0_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 55296_i32,
            replace_pos: 1_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56319_i32,
            replace_pos: 1_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56320_i32,
            replace_pos: 1_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 57343_i32,
            replace_pos: 1_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 55296_i32,
            replace_pos: 2_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56319_i32,
            replace_pos: 2_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56320_i32,
            replace_pos: 2_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 57343_i32,
            replace_pos: 2_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56320_i32,
            replace_pos: 3_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 57343_i32,
            replace_pos: 3_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 55296_i32,
            replace_pos: 4_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56319_i32,
            replace_pos: 4_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: ((122 as u16) as i32),
            replace_pos: 4_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_error_wchar_t_>),
        (offsets.as_pointer() as Ptr<test_offsets_error_wchar_t_>).offset((17_i64) as isize),
    ) {
        let mut t: test_offsets_error_wchar_t_ = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_21(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_15(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: i32 = (*in_.borrow())[(t.replace_pos) as usize];
        (*in_.borrow_mut())[(t.replace_pos) as usize] = { t.replace_char };
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i32> = Ptr::<i32>::null();
        let mut out_next: Ptr<i8> = Ptr::<i8>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i32>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i8>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_15(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(((*out.borrow())[(t.expected_out_next) as usize] as i32) == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*in_.borrow_mut())[(t.replace_pos) as usize] = old_char;
    }
}
pub fn utf16_to_utf8_out_38(cvt: Ptr<()>) {
    ({ utf16_to_utf8_out_ok_35((cvt).clone()) });
    ({ utf16_to_utf8_out_partial_36((cvt).clone()) });
    ({ utf16_to_utf8_out_error_37((cvt).clone()) });
}
pub fn test_utf8_utf16_cvt_39(cvt: Ptr<()>) {
    ({ utf8_to_utf16_in_34((cvt).clone()) });
    ({ utf16_to_utf8_out_38((cvt).clone()) });
}
pub type utf16_endianess = u32;
pub const utf16_endianess_utf16_big_endian: utf16_endianess = 0;
pub const utf16_endianess_utf16_little_endian: utf16_endianess = 1;
pub fn utf16_to_bytes_40(
    mut f: Ptr<u16>,
    mut l: Ptr<u16>,
    o: Ptr<i8>,
    mut e: utf16_endianess,
) -> Ptr<i8> {
    let o: Value<Ptr<i8>> = Rc::new(RefCell::new(o));
    if ((e as i32) == (utf16_endianess_utf16_big_endian as i32)) {
        'loop_: while ({ (f).clone() } != { (l).clone() }) {
            let __rhs = (((((f.read()) as i32) >> 8) & 255) as i8);
            (*o.borrow_mut()).postfix_inc().write(__rhs);
            let __rhs = ((((f.read()) as i32) & 255) as i8);
            (*o.borrow_mut()).postfix_inc().write(__rhs);
            f.prefix_inc();
        }
    } else {
        'loop_: while ({ (f).clone() } != { (l).clone() }) {
            let __rhs = ((((f.read()) as i32) & 255) as i8);
            (*o.borrow_mut()).postfix_inc().write(__rhs);
            let __rhs = (((((f.read()) as i32) >> 8) & 255) as i8);
            (*o.borrow_mut()).postfix_inc().write(__rhs);
            f.prefix_inc();
        }
    }
    return (*o.borrow()).clone();
}
pub fn utf16_to_bytes_41(
    mut f: Ptr<u16>,
    mut l: Ptr<u16>,
    o: Ptr<i8>,
    mut e: utf16_endianess,
) -> Ptr<i8> {
    let o: Value<Ptr<i8>> = Rc::new(RefCell::new(o));
    if ((e as i32) == (utf16_endianess_utf16_big_endian as i32)) {
        'loop_: while ({ (f).clone() } != { (l).clone() }) {
            let __rhs = (((((f.read()) as i32) >> 8) & 255) as i8);
            (*o.borrow_mut()).postfix_inc().write(__rhs);
            let __rhs = ((((f.read()) as i32) & 255) as i8);
            (*o.borrow_mut()).postfix_inc().write(__rhs);
            f.prefix_inc();
        }
    } else {
        'loop_: while ({ (f).clone() } != { (l).clone() }) {
            let __rhs = ((((f.read()) as i32) & 255) as i8);
            (*o.borrow_mut()).postfix_inc().write(__rhs);
            let __rhs = (((((f.read()) as i32) >> 8) & 255) as i8);
            (*o.borrow_mut()).postfix_inc().write(__rhs);
            f.prefix_inc();
        }
    }
    return (*o.borrow()).clone();
}
pub fn utf16_to_utf32_in_ok_42(cvt: Ptr<()>, mut endianess: utf16_endianess) {
    let input: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    let expected: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let in_: Value<Box<[i8]>> = Rc::new(RefCell::new((0..12).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let exp: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    ({
        let _f: Ptr<u16> = unimplemented!();
        let _l: Ptr<u16> = (input.as_pointer() as Ptr<u16>).to_end();
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
    });
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let offsets: Value<Box<[test_offsets_ok]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 4_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 6_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 10_usize,
            out_size: 4_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_ok>),
        (offsets.as_pointer() as Ptr<test_offsets_ok>).offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_14(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_20(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.in_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.out_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.out_size < ({ array_size_20(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.out_size) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_ok>),
        (offsets.as_pointer() as Ptr<test_offsets_ok>).offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_14(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_18(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.in_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.out_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.out_size < ({ array_size_18(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.out_size) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub fn utf16_to_utf32_in_partial_43(cvt: Ptr<()>, mut endianess: utf16_endianess) {
    let input: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    let expected: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let in_: Value<Box<[i8]>> = Rc::new(RefCell::new((0..12).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let exp: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    ({
        let _f: Ptr<u16> = unimplemented!();
        let _l: Ptr<u16> = (input.as_pointer() as Ptr<u16>).to_end();
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
    });
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let offsets: Value<Box<[test_offsets_partial]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 1_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 1_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 2_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 1_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 6_usize,
            out_size: 2_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 3_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 2_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_partial>),
        (offsets.as_pointer() as Ptr<test_offsets_partial>).offset((16_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_14(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_20(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_20(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.expected_out_next) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.expected_in_next) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub fn utf16_to_utf32_in_error_44(cvt: Ptr<()>, mut endianess: utf16_endianess) {
    let input: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    let expected: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let exp: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (expected.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let offsets: Value<Box<[test_offsets_error_char16_t_]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56319_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56320_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 57343_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 55296_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 56319_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 56320_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 57343_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 55296_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 56319_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 56320_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 57343_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 56320_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 57343_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 55296_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 56319_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (122 as u16),
            replace_pos: 4_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_error_char16_t_>),
        (offsets.as_pointer() as Ptr<test_offsets_error_char16_t_>).offset((17_i64) as isize),
    ) {
        let mut t: test_offsets_error_char16_t_ = (*t.upgrade().deref()).clone();
        let in_: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..12).map(|_| 0_i8).collect::<Box<[i8]>>()));
        let out: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32, 0_i32])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_14(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_20(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: u16 = (*input.borrow())[(t.replace_pos) as usize];
        (*input.borrow_mut())[(t.replace_pos) as usize] = { t.replace_char };
        ({
            let _f: Ptr<u16> = unimplemented!();
            let _l: Ptr<u16> = (input.as_pointer() as Ptr<u16>).to_end();
            utf16_to_bytes_41(_f, _l, unimplemented!(), endianess)
        });
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i8> = Ptr::<i8>::null();
        let mut out_next: Ptr<i32> = Ptr::<i32>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i8>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i32>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_20(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((*out.borrow())[(t.expected_out_next) as usize] == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*state.borrow_mut()) = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(len >= 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == t.expected_in_next) {
                unimplemented!();
                std::process::abort();
            }
        }
        (*input.borrow_mut())[(t.replace_pos) as usize] = old_char;
    }
}
pub fn utf32_to_utf16_out_ok_45(cvt: Ptr<()>, mut endianess: utf16_endianess) {
    let input: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    let expected: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let in_: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let exp: Value<Box<[i8]>> = Rc::new(RefCell::new((0..12).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    ({
        let _f: Ptr<u16> = unimplemented!();
        let _l: Ptr<u16> = (expected.as_pointer() as Ptr<u16>).to_end();
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
    });
    let offsets: Value<Box<[test_offsets_ok]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 4_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 6_usize,
        },
        test_offsets_ok {
            in_size: 4_usize,
            out_size: 10_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_ok>),
        (offsets.as_pointer() as Ptr<test_offsets_ok>).offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_18(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_15(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i32> = Ptr::<i32>::null();
        let mut out_next: Ptr<i8> = Ptr::<i8>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i32>).offset((t.in_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i8>).offset((t.out_size) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.out_size < ({ array_size_15(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(((*out.borrow())[(t.out_size) as usize] as i32) == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub fn utf32_to_utf16_out_partial_46(cvt: Ptr<()>, mut endianess: utf16_endianess) {
    let input: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    let expected: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let in_: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let exp: Value<Box<[i8]>> = Rc::new(RefCell::new((0..12).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    ({
        let _f: Ptr<u16> = unimplemented!();
        let _l: Ptr<u16> = (expected.as_pointer() as Ptr<u16>).to_end();
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
    });
    let offsets: Value<Box<[test_offsets_partial]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 1_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 3_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_partial>),
        (offsets.as_pointer() as Ptr<test_offsets_partial>).offset((10_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_18(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_15(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i32> = Ptr::<i32>::null();
        let mut out_next: Ptr<i8> = Ptr::<i8>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i32>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i8>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_15(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(((*out.borrow())[(t.expected_out_next) as usize] as i32) == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub fn utf32_to_utf16_out_error_47(cvt: Ptr<()>, mut endianess: utf16_endianess) {
    let input: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::from([
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ])));
    let expected: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ])));
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let in_: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let exp: Value<Box<[i8]>> = Rc::new(RefCell::new((0..12).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        let mut __first = unimplemented!();
        let __last = (input.as_pointer() as Ptr<u32>).to_end();
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(((__first.iter_deref().read()) as i32));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    ({
        let _f: Ptr<u16> = unimplemented!();
        let _l: Ptr<u16> = (expected.as_pointer() as Ptr<u16>).to_end();
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
    });
    let offsets: Value<Box<[test_offsets_error_wchar_t_]>> = Rc::new(RefCell::new(Box::new([
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_i32,
            replace_pos: 0_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
            replace_char: 56319_i32,
            replace_pos: 1_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
            replace_char: 56320_i32,
            replace_pos: 2_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 57343_i32,
            replace_pos: 3_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 1114112_i32,
            replace_pos: 0_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
            replace_char: 1114112_i32,
            replace_pos: 1_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
            replace_char: 1114112_i32,
            replace_pos: 2_usize,
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 1114112_i32,
            replace_pos: 3_usize,
        },
    ])));
    'loop_: for t in IterRange::new(
        (offsets.as_pointer() as Ptr<test_offsets_error_wchar_t_>),
        (offsets.as_pointer() as Ptr<test_offsets_error_wchar_t_>).offset((8_i64) as isize),
    ) {
        let mut t: test_offsets_error_wchar_t_ = (*t.upgrade().deref()).clone();
        let out: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.in_size <= ({ array_size_18(in_.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.out_size <= ({ array_size_15(out.as_pointer()) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_in_next <= t.in_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(t.expected_out_next <= t.out_size) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: i32 = (*in_.borrow())[(t.replace_pos) as usize];
        (*in_.borrow_mut())[(t.replace_pos) as usize] = { t.replace_char };
        let state: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let mut in_next: Ptr<i32> = Ptr::<i32>::null();
        let mut out_next: Ptr<i8> = Ptr::<i8>::null();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (res as i32) } == { (unimplemented!() as i32) }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (in_next).clone() } == {
                (in_.as_pointer() as Ptr<i32>).offset((t.expected_in_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (out_next).clone() } == {
                (out.as_pointer() as Ptr<i8>).offset((t.expected_out_next) as isize)
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!() == 0) {
                unimplemented!();
                std::process::abort();
            }
        }
        if (t.expected_out_next < ({ array_size_15(out.as_pointer()) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !(((*out.borrow())[(t.expected_out_next) as usize] as i32) == 0) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        (*in_.borrow_mut())[(t.replace_pos) as usize] = old_char;
    }
}
pub fn test_utf16_utf32_cvt_48(cvt: Ptr<()>, mut endianess: utf16_endianess) {
    ({
        let _cvt: Ptr<()> = (cvt).clone();
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_ok_42(_cvt, _endianess)
    });
    ({
        let _cvt: Ptr<()> = (cvt).clone();
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_partial_43(_cvt, _endianess)
    });
    ({
        let _cvt: Ptr<()> = (cvt).clone();
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_error_44(_cvt, _endianess)
    });
    ({
        let _cvt: Ptr<()> = (cvt).clone();
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_ok_45(_cvt, _endianess)
    });
    ({
        let _cvt: Ptr<()> = (cvt).clone();
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_partial_46(_cvt, _endianess)
    });
    ({
        let _cvt: Ptr<()> = (cvt).clone();
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_error_47(_cvt, _endianess)
    });
}
pub fn test_utf8_utf32_codecvts_49() {
    let cvt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_utf8_utf32_cvt_30(cvt.as_pointer()) });
}
pub fn test_utf8_utf16_codecvts_50() {
    let cvt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_utf8_utf16_cvt_39(cvt.as_pointer()) });
}
pub fn test_utf8_ucs2_codecvts_51() {}
pub fn test_utf16_utf32_codecvts_52() {
    let cvt3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_utf16_utf32_cvt_48(cvt3.as_pointer(), utf16_endianess_utf16_big_endian) });
    let cvt4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_utf16_utf32_cvt_48(cvt4.as_pointer(), utf16_endianess_utf16_little_endian) });
}
pub fn test_utf16_ucs2_codecvts_53() {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_utf8_utf32_codecvts_49() });
    ({ test_utf8_utf16_codecvts_50() });
    ({ test_utf8_ucs2_codecvts_51() });
    ({ test_utf16_utf32_codecvts_52() });
    ({ test_utf16_ucs2_codecvts_53() });
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
pub fn set_memory_limits_54(size: Option<f32>) {
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
pub fn set_file_limit_55(mut size: u64) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_56(mut mangled: Ptr<i8>, mut wanted: Ptr<i8>) {
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
pub fn run_tests_wrapped_locale_57(mut name: Ptr<i8>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_58(
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
pub fn test_tm_59(
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
    (*tmp_60.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_60.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_60.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_60.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_60.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_60.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_60.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_60.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_60.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_60.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_60: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
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
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
