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
pub fn test_9(s: (), mut pos1: (), mut n1: (), mut str: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let mut old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if (pos1 <= old_size) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut xlen: () = (unimplemented!().read());
        let mut rlen: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_10(s: (), mut pos1: (), mut n1: (), mut str: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let mut old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if (pos1 <= old_size) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut xlen: () = (unimplemented!().read());
        let mut rlen: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_11(s: (), mut pos1: (), mut n1: (), mut str: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let mut old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if (pos1 <= old_size) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut xlen: () = (unimplemented!().read());
        let mut rlen: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test0_12() -> () {
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
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
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
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
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345bcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
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
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
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
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345bcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ade") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345de") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890de") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890de") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ae") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890cde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345de") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890de") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890de") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abe") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd1234567890e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    return true;
}
pub fn test0_13() -> () {
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
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
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
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
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345bcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
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
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
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
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345bcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ade") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345de") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890de") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890de") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ae") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890cde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345de") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890de") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890de") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abe") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd1234567890e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    return true;
}
pub fn test0_14() -> () {
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
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
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
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
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345bcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
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
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
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
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            0_(),
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345bcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890bcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ade") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345de") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890de") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890de") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ae") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            1_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890cde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345de") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890de") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890de") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abe") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            2_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ab12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd1234567890e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345678901234567890e") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            4_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcd12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    return true;
}
pub fn test1_15() -> () {
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
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
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
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
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345cdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890cdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890cdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"afghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aj") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdeghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345ghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890ghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890ghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdehij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345hij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890hij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890hij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdej") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi1234567890j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    return true;
}
pub fn test1_16() -> () {
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
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
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
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
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345cdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890cdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890cdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"afghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aj") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdeghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345ghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890ghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890ghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdehij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345hij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890hij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890hij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdej") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi1234567890j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    return true;
}
pub fn test1_17() -> () {
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
            6_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
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
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
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
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            0_(),
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890bcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345cdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890cdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890cdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"afghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aj") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            8_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            1_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890fghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdeghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345ghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890ghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890ghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdehij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345hij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890hij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890hij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdej") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            5_(),
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcde12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi1234567890j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345678901234567890j") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            9_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghi12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
            11_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    return true;
}
pub fn test2_18() -> () {
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
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
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
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
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345cdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890cdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890cdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"at") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijlmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345lmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890lmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890lmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijpqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345pqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890pqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890pqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijt") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs1234567890t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst1234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    return true;
}
pub fn test2_19() -> () {
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
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
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
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
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345cdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890cdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890cdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"at") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijlmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345lmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890lmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890lmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijpqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345pqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890pqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890pqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijt") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs1234567890t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst1234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    return true;
}
pub fn test2_20() -> () {
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
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
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
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
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            0_(),
            21_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890bcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345cdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890cdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890cdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"at") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            18_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            1_(),
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"a12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890klmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijlmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345lmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890lmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890lmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijpqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345pqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890pqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890pqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijt") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            10_(),
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghij12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs1234567890t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345678901234567890t") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            19_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrs12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst1234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            20_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst12345678901234567890") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    ({
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst") },
                None,
            ),
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"can\'t happen") },
                None,
            ),
        )
    });
    return true;
}
pub fn test_21() {
    {
        ({ test0_12() });
        ({ test1_15() });
        ({ test2_18() });
        const _: () = assert!(true, "test0<S>()");;
        const _: () = assert!(true, "test1<S>()");;
        const _: () = assert!(true, "test2<S>()");;
    }
    {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { Ptr::<()>::from_string_literal(b" ") },
            None,
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_22() {
    {
        ({ test0_13() });
        ({ test1_16() });
        ({ test2_19() });
        const _: () = assert!(true, "test0<S>()");;
        const _: () = assert!(true, "test1<S>()");;
        const _: () = assert!(true, "test2<S>()");;
    }
    {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { Ptr::<()>::from_string_literal(b" ") },
            None,
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_23() {
    {
        ({ test0_14() });
        ({ test1_17() });
        ({ test2_20() });
        const _: () = assert!(true, "test0<S>()");;
        const _: () = assert!(true, "test1<S>()");;
        const _: () = assert!(true, "test2<S>()");;
    }
    {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { Ptr::<()>::from_string_literal(b" ") },
            None,
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
    ({ test_21() });
    ({ test_22() });
    ({ test_23() });
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
