extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
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
pub fn test_4(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        let mut n: () = unimplemented!();
        unimplemented!();
    }
}
pub fn test_5(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        let mut n: () = unimplemented!();
        unimplemented!();
    }
}
pub fn test_6(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        let mut pos: () = unimplemented!();
        let mut n: () = unimplemented!();
        unimplemented!();
    }
}
pub fn test_7(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        let mut pos: () = unimplemented!();
        let mut n: () = unimplemented!();
        unimplemented!();
    }
}
pub fn test_string_8() {
    {
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                1_(),
                1_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                2_(),
                2_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                2_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                2_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                2_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                4_(),
                4_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                4_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                4_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                4_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                5_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                5_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                5_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                5_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                6_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                6_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                6_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                6_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                1_(),
                1_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                5_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                5_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                5_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                5_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                9_(),
                9_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                9_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                9_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                9_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                10_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                10_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                10_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                10_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                11_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                11_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                11_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                11_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                1_(),
                1_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                10_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                10_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                10_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                10_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                19_(),
                19_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                19_(),
                15_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                19_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                19_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                20_(),
                20_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                20_(),
                15_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                20_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                20_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                21_(),
                20_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                21_(),
                15_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                21_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                21_(),
                0_(),
            )
        });
    }
    {
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 0_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 5_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 10_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 20_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                15_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_6(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
            )
        });
    }
}
pub fn test_string_9() {
    {
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                1_(),
                1_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                2_(),
                2_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                2_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                2_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                2_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                4_(),
                4_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                4_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                4_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                4_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                5_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                5_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                5_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                5_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                6_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                6_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                6_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                6_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                1_(),
                1_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                1_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                5_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                5_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                5_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                5_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                9_(),
                9_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                9_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                9_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                9_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                10_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                10_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                10_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                10_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                11_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                11_(),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                11_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                11_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                1_(),
                1_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                1_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                10_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                10_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                10_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                10_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                19_(),
                19_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                19_(),
                15_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                19_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                19_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                20_(),
                20_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                20_(),
                15_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                20_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                20_(),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b""),
                21_(),
                20_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                21_(),
                15_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                21_(),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_5(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                21_(),
                0_(),
            )
        });
    }
    {
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 0_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 5_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 10_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                5_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                0_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 20_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcde"),
                15_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcde"),
                10_(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_7(
                _s.as_pointer(),
                Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
                0_(),
            )
        });
    }
}
pub fn tests_10() -> () {
    ({ test_string_8() });
    ({ test_string_9() });
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
    ({ tests_10() });
    const _: () = assert!(true, "tests()");;
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
