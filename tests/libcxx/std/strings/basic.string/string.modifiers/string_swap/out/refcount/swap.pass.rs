extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<()>) -> Ptr<()> {
    b"";
    return (value).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_4: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_char_ {}
impl safe_allocator_char_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for safe_allocator_char_ {
    fn default() -> Self {
        { safe_allocator_char_::new() }
    }
}
pub fn is_string_asan_correct_6(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_7(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_8(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_9(_a0: Ptr<()>) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct operator_hijacker {}
impl std::cmp::Ord for operator_hijacker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_hijackerImpl::operator_lt(
                &Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
                Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_hijackerImpl::operator_lt(
                &Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
                Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for operator_hijacker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for operator_hijacker {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_hijackerImpl::operator_eq(
                &Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
                Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for operator_hijacker {}
const _: () = assert!(
    true,
    "std::is_trivially_copyable<operator_hijacker>::value &&     //\n                  std::is_copy_constructible<operator_hijacker>::value && //\n                  std::is_move_constructible<operator_hijacker>::value && //\n                  std::is_copy_assignable<operator_hijacker>::value &&    //\n                  std::is_move_assignable<operator_hijacker>::value"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct operator_hijacker_allocator_char_ {}
pub fn test_10(s1: (), s2: ()) {
    let s1: Value<()> = Rc::new(RefCell::new(s1));
    let s2: Value<()> = Rc::new(RefCell::new(s2));
    let mut s1_: () = unimplemented!();
    let mut s2_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_11(s1: (), s2: ()) {
    let s1: Value<()> = Rc::new(RefCell::new(s1));
    let s2: Value<()> = Rc::new(RefCell::new(s2));
    let mut s1_: () = unimplemented!();
    let mut s2_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_12(s1: (), s2: ()) {
    let s1: Value<()> = Rc::new(RefCell::new(s1));
    let s2: Value<()> = Rc::new(RefCell::new(s2));
    let mut s1_: () = unimplemented!();
    let mut s2_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_13(s1: (), s2: ()) {
    let s1: Value<()> = Rc::new(RefCell::new(s1));
    let s2: Value<()> = Rc::new(RefCell::new(s2));
    let mut s1_: () = unimplemented!();
    let mut s2_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_string_14() {
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
}
pub fn test_string_15() {
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
}
pub fn test_string_16() {
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_12(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
}
pub fn test_string_17() {
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_13(
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrstabcdefghijklmnopqrst") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___operator_hijacker_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
}
pub fn test_18() -> () {
    ({ test_string_14() });
    ({ test_string_15() });
    ({ test_string_16() });
    ({ test_string_17() });
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_18() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait min_allocator_char_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char_Impl for Ptr<min_allocator_char_> {
    fn allocate(&self, mut n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait operator_hijackerImpl {
    fn operator_lt(&self, _a0: Ptr<operator_hijacker>) -> ();
    fn operator_eq(&self, _a0: Ptr<operator_hijacker>) -> ();
    fn operator_call(&self) -> ();
}
impl operator_hijackerImpl for Ptr<operator_hijacker> {
    fn operator_lt(&self, _a0: Ptr<operator_hijacker>) -> () {
        return true;
    }
    fn operator_eq(&self, _a0: Ptr<operator_hijacker>) -> () {
        return true;
    }
    fn operator_call(&self) -> () {
        return 42_();
    }
}
pub trait safe_allocator_char_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
}
impl safe_allocator_char_Impl for Ptr<safe_allocator_char_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        let memory: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                memset_5(
                    (*memory.borrow()).to_any(),
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        {
            ({
                let _value: Value<()> = Rc::new(RefCell::new(
                    ({
                        memset_5(
                            (p).to_any(),
                            0_(),
                            (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                        )
                    }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
