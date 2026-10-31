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
    pub static buf_9: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..100).map(|_| 0_i8).collect::<Box<[i8]>>()));
);
pub fn to_string_10(mut val: i32) -> Vec<i8> {
    let mut str: Vec<i8> = vec![i8::default()];
    let mut len: i32 = {
        let __s = libcc2rs::format_c(
            &Ptr::<i8>::from_string_literal(b"%d").to_rust_string(),
            &Box::<[VaArg]>::from([(val).into()]),
        );
        let __b = __s.as_bytes();
        if ::std::mem::size_of::<[i8; 100]>() > 0 {
            let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 100]>() - 1);
            (buf_9.with(|v| v.as_pointer()) as Ptr<i8>).write_c_bytes(&__b[..__n]);
            elem!((buf_9.with(|v| v.as_pointer()) as Ptr::<i8>), __n).write(0);
        }
        __b.len() as i32
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < (::std::mem::size_of::<[i8; 100]>() as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
    return {
        let mut __v = Vec::with_capacity((len as usize) as usize + 1);
        (buf_9.with(|v| v.as_pointer()) as Ptr<i8>)
            .with_slice((len as usize) as usize, |__s| __v.extend_from_slice(__s));
        __v.push(0_i8);
        __v
    };
}
pub fn to_string_11(mut val: u32) -> Vec<i8> {
    let mut str: Vec<i8> = vec![i8::default()];
    let mut len: i32 = {
        let __s = libcc2rs::format_c(
            &Ptr::<i8>::from_string_literal(b"%u").to_rust_string(),
            &Box::<[VaArg]>::from([(val).into()]),
        );
        let __b = __s.as_bytes();
        if ::std::mem::size_of::<[i8; 100]>() > 0 {
            let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 100]>() - 1);
            (buf_9.with(|v| v.as_pointer()) as Ptr<i8>).write_c_bytes(&__b[..__n]);
            elem!((buf_9.with(|v| v.as_pointer()) as Ptr::<i8>), __n).write(0);
        }
        __b.len() as i32
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < (::std::mem::size_of::<[i8; 100]>() as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
    return {
        let mut __v = Vec::with_capacity((len as usize) as usize + 1);
        (buf_9.with(|v| v.as_pointer()) as Ptr<i8>)
            .with_slice((len as usize) as usize, |__s| __v.extend_from_slice(__s));
        __v.push(0_i8);
        __v
    };
}
pub fn to_string_12(mut val: i64) -> Vec<i8> {
    let mut str: Vec<i8> = vec![i8::default()];
    let mut len: i32 = {
        let __s = libcc2rs::format_c(
            &Ptr::<i8>::from_string_literal(b"%ld").to_rust_string(),
            &Box::<[VaArg]>::from([(val).into()]),
        );
        let __b = __s.as_bytes();
        if ::std::mem::size_of::<[i8; 100]>() > 0 {
            let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 100]>() - 1);
            (buf_9.with(|v| v.as_pointer()) as Ptr<i8>).write_c_bytes(&__b[..__n]);
            elem!((buf_9.with(|v| v.as_pointer()) as Ptr::<i8>), __n).write(0);
        }
        __b.len() as i32
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < (::std::mem::size_of::<[i8; 100]>() as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
    return {
        let mut __v = Vec::with_capacity((len as usize) as usize + 1);
        (buf_9.with(|v| v.as_pointer()) as Ptr<i8>)
            .with_slice((len as usize) as usize, |__s| __v.extend_from_slice(__s));
        __v.push(0_i8);
        __v
    };
}
pub fn to_string_13(mut val: u64) -> Vec<i8> {
    let mut str: Vec<i8> = vec![i8::default()];
    let mut len: i32 = {
        let __s = libcc2rs::format_c(
            &Ptr::<i8>::from_string_literal(b"%lu").to_rust_string(),
            &Box::<[VaArg]>::from([(val).into()]),
        );
        let __b = __s.as_bytes();
        if ::std::mem::size_of::<[i8; 100]>() > 0 {
            let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 100]>() - 1);
            (buf_9.with(|v| v.as_pointer()) as Ptr<i8>).write_c_bytes(&__b[..__n]);
            elem!((buf_9.with(|v| v.as_pointer()) as Ptr::<i8>), __n).write(0);
        }
        __b.len() as i32
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < (::std::mem::size_of::<[i8; 100]>() as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
    return {
        let mut __v = Vec::with_capacity((len as usize) as usize + 1);
        (buf_9.with(|v| v.as_pointer()) as Ptr<i8>)
            .with_slice((len as usize) as usize, |__s| __v.extend_from_slice(__s));
        __v.push(0_i8);
        __v
    };
}
pub fn to_string_14(mut val: i64) -> Vec<i8> {
    let mut str: Vec<i8> = vec![i8::default()];
    let mut len: i32 = {
        let __s = libcc2rs::format_c(
            &Ptr::<i8>::from_string_literal(b"%lld").to_rust_string(),
            &Box::<[VaArg]>::from([(val).into()]),
        );
        let __b = __s.as_bytes();
        if ::std::mem::size_of::<[i8; 100]>() > 0 {
            let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 100]>() - 1);
            (buf_9.with(|v| v.as_pointer()) as Ptr<i8>).write_c_bytes(&__b[..__n]);
            elem!((buf_9.with(|v| v.as_pointer()) as Ptr::<i8>), __n).write(0);
        }
        __b.len() as i32
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < (::std::mem::size_of::<[i8; 100]>() as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
    return {
        let mut __v = Vec::with_capacity((len as usize) as usize + 1);
        (buf_9.with(|v| v.as_pointer()) as Ptr<i8>)
            .with_slice((len as usize) as usize, |__s| __v.extend_from_slice(__s));
        __v.push(0_i8);
        __v
    };
}
pub fn to_string_15(mut val: u64) -> Vec<i8> {
    let mut str: Vec<i8> = vec![i8::default()];
    let mut len: i32 = {
        let __s = libcc2rs::format_c(
            &Ptr::<i8>::from_string_literal(b"%llu").to_rust_string(),
            &Box::<[VaArg]>::from([(val).into()]),
        );
        let __b = __s.as_bytes();
        if ::std::mem::size_of::<[i8; 100]>() > 0 {
            let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 100]>() - 1);
            (buf_9.with(|v| v.as_pointer()) as Ptr<i8>).write_c_bytes(&__b[..__n]);
            elem!((buf_9.with(|v| v.as_pointer()) as Ptr::<i8>), __n).write(0);
        }
        __b.len() as i32
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(len < (::std::mem::size_of::<[i8; 100]>() as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
    return {
        let mut __v = Vec::with_capacity((len as usize) as usize + 1);
        (buf_9.with(|v| v.as_pointer()) as Ptr<i8>)
            .with_slice((len as usize) as usize, |__s| __v.extend_from_slice(__s));
        __v.push(0_i8);
        __v
    };
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
        (2147483664_u32 as ()),
        (2271560481_u32 as ()),
        (2863311530_u32 as ()),
        (4026531840_u32 as ()),
        (4027584528_u32 as ()),
        (4042260480_u32 as ()),
        (4042322160_u32 as ()),
        (4043247600_u32 as ()),
        (4279234575_u32 as ()),
        (4294901760_u32 as ()),
        (4294902000_u32 as ()),
        (4294905840_u32 as ()),
        (4294967040_u32 as ()),
    ])));
);
thread_local!(
    pub static empty_string_capacity_17: Value<usize> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn check_value_18(mut val: i32) {
    let mut s: Vec<i8> = unimplemented!();
    let mut expected: Vec<i8> = ({ to_string_10(val) });
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
        if !(((unimplemented!().read()) as i32) == (('\0' as i8) as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn check_value_19(mut val: u32) {
    let mut s: Vec<i8> = unimplemented!();
    let mut expected: Vec<i8> = ({ to_string_11(val) });
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
        if !(((unimplemented!().read()) as i32) == (('\0' as i8) as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn check_value_20(mut val: i64) {
    let mut s: Vec<i8> = unimplemented!();
    let mut expected: Vec<i8> = ({ to_string_12(val) });
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
        if !(((unimplemented!().read()) as i32) == (('\0' as i8) as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn check_value_21(mut val: u64) {
    let mut s: Vec<i8> = unimplemented!();
    let mut expected: Vec<i8> = ({ to_string_13(val) });
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
        if !(((unimplemented!().read()) as i32) == (('\0' as i8) as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn check_value_22(mut val: i64) {
    let mut s: Vec<i8> = unimplemented!();
    let mut expected: Vec<i8> = ({ to_string_14(val) });
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
        if !(((unimplemented!().read()) as i32) == (('\0' as i8) as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn check_value_23(mut val: u64) {
    let mut s: Vec<i8> = unimplemented!();
    let mut expected: Vec<i8> = ({ to_string_15(val) });
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
        if !(((unimplemented!().read()) as i32) == (('\0' as i8) as i32)) {
            unimplemented!();
            std::process::abort();
        }
    }
}
thread_local!(
    pub static width_24: Value<i32> = Rc::new(RefCell::new(16));
);
pub fn check_values_25() {
    'loop_: for v in IterRange::new(
        (values_16.with(|v| v.as_pointer()) as Ptr<()>),
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: i32 = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: i32 = ((v as i32) + i);
            ({ check_value_18(val) });
            i.prefix_inc();
        }
        {}
    }
}
pub fn check_values_26() {
    'loop_: for v in IterRange::new(
        (values_16.with(|v| v.as_pointer()) as Ptr<()>),
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: i32 = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: u32 = (v as u32).wrapping_add((i as u32));
            ({ check_value_19(val) });
            i.prefix_inc();
        }
        {}
    }
}
pub fn check_values_27() {
    'loop_: for v in IterRange::new(
        (values_16.with(|v| v.as_pointer()) as Ptr<()>),
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: i32 = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: i64 = ((v as i64) + (i as i64));
            ({ check_value_20(val) });
            i.prefix_inc();
        }
        {
            'loop_: for v2 in IterRange::new(
                (values_16.with(|v| v.as_pointer()) as Ptr<()>),
                (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
            ) {
                let mut v2: () = (v2.read());
                let mut i: i32 = -width_24.with(|rc| *rc.borrow());
                'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
                    let mut hi: () = v2;
                    hi = { (hi).wrapping_add((i as ())) };
                    hi <<= 32;
                    let mut val: i64 = ((hi as i64) | (v as i64));
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
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: i32 = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: u64 = (v as u64).wrapping_add((i as u64));
            ({ check_value_21(val) });
            i.prefix_inc();
        }
        {
            'loop_: for v2 in IterRange::new(
                (values_16.with(|v| v.as_pointer()) as Ptr<()>),
                (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
            ) {
                let mut v2: () = (v2.read());
                let mut i: i32 = -width_24.with(|rc| *rc.borrow());
                'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
                    let mut hi: () = v2;
                    hi = { (hi).wrapping_add((i as ())) };
                    hi <<= 32;
                    let mut val: u64 = ((hi as u64) | (v as u64));
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
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: i32 = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: i64 = ((v as i64) + (i as i64));
            ({ check_value_22(val) });
            i.prefix_inc();
        }
        {
            'loop_: for v2 in IterRange::new(
                (values_16.with(|v| v.as_pointer()) as Ptr<()>),
                (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
            ) {
                let mut v2: () = (v2.read());
                let mut i: i32 = -width_24.with(|rc| *rc.borrow());
                'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
                    let mut hi: () = v2;
                    hi = { (hi).wrapping_add((i as ())) };
                    hi <<= 32;
                    let mut val: i64 = ((hi as i64) | (v as i64));
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
        (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
    ) {
        let mut v: () = (v.read());
        let mut i: i32 = -width_24.with(|rc| *rc.borrow());
        'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
            let mut val: u64 = (v as u64).wrapping_add((i as u64));
            ({ check_value_23(val) });
            i.prefix_inc();
        }
        {
            'loop_: for v2 in IterRange::new(
                (values_16.with(|v| v.as_pointer()) as Ptr<()>),
                (values_16.with(|v| v.as_pointer()) as Ptr<()>).offset((53_i64) as isize),
            ) {
                let mut v2: () = (v2.read());
                let mut i: i32 = -width_24.with(|rc| *rc.borrow());
                'loop_: while (i < width_24.with(|rc| *rc.borrow())) {
                    let mut hi: () = v2;
                    hi = { (hi).wrapping_add((i as ())) };
                    hi <<= 32;
                    let mut val: u64 = ((hi as u64) | (v as u64));
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
fn main_0() -> i32 {
    ({ test02_31() });
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
pub fn set_memory_limits_32(size: Option<f32>) {
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
pub fn set_file_limit_33(mut size: u64) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_34(mut mangled: Ptr<i8>, mut wanted: Ptr<i8>) {
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
pub fn run_tests_wrapped_locale_35(mut name: Ptr<i8>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_36(
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
pub fn test_tm_37(
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
    static tmp_38: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
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
