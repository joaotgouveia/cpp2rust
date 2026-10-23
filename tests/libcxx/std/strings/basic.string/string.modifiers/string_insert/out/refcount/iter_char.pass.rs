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
pub fn operator_sub_5(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
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
pub fn operator_sub_6(
    mut x: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_7(
    mut x: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
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
impl std::cmp::PartialEq for min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_7(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_char__std_integral_constant_unsigned_long__0__ {}
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
pub fn is_string_asan_correct_9(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_10(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_11(_a0: Ptr<()>) -> () {
    return true;
}
pub fn test_12(s: Ptr<()>, p: (), mut c: (), mut expected: ()) {
    let p: Value<()> = Rc::new(RefCell::new(p));
    unimplemented!();
    let mut sufficient_cap: () = ({ unimplemented!() } < { unimplemented!() });
    let mut pos: () = (unimplemented!() as ());
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if sufficient_cap {
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_13(s: Ptr<()>, p: (), mut c: (), mut expected: ()) {
    let p: Value<()> = Rc::new(RefCell::new(p));
    unimplemented!();
    let mut sufficient_cap: () = ({ unimplemented!() } < { unimplemented!() });
    let mut pos: () = unimplemented!();
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if sufficient_cap {
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_14(s: Ptr<()>, p: (), mut c: (), mut expected: ()) {
    let p: Value<()> = Rc::new(RefCell::new(p));
    unimplemented!();
    let mut sufficient_cap: () = ({ unimplemented!() } < { unimplemented!() });
    let mut pos: () = (unimplemented!() as ());
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if sufficient_cap {
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_string_15() {
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_0.as_pointer() })
        };
        test_12(_s, _p, ('1' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_1.as_pointer() })
        };
        test_12(_s, _p, ('a' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_2.as_pointer() })
        };
        test_12(_s, _p, ('b' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_3.as_pointer() })
        };
        test_12(_s, _p, ('c' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_4.as_pointer() })
        };
        test_12(_s, _p, ('d' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_5.as_pointer() })
        };
        test_12(_s, _p, ('2' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_6.as_pointer() })
        };
        test_12(_s, _p, ('3' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_7: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_7.as_pointer() })
        };
        test_12(_s, _p, ('4' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_8.as_pointer() })
        };
        test_12(_s, _p, ('5' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_9.as_pointer() })
        };
        test_12(_s, _p, ('6' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_10.as_pointer() })
        };
        test_12(_s, _p, ('7' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_11.as_pointer() })
        };
        test_12(_s, _p, ('A' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_12: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_12.as_pointer() })
        };
        test_12(_s, _p, ('B' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_13: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_13.as_pointer() })
        };
        test_12(_s, _p, ('C' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_14: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_14.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_15: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_15.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_16: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_16.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_17: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_17.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_18: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_18.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_19: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_19.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_20: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_20.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_21: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_21.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_22: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_22.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_23: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_23.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_24: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_24.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_25: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_25.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_26: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_26.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_27: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_27.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_28: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_28.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_29: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_29.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_30: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_30.as_pointer() })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
}
pub fn test_string_16() {
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_31: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_31.as_pointer() },
            )
        };
        test_13(_s, _p, ('1' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_32: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_32.as_pointer() },
            )
        };
        test_13(_s, _p, ('a' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_33: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_33.as_pointer() },
            )
        };
        test_13(_s, _p, ('b' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_34: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_34.as_pointer() },
            )
        };
        test_13(_s, _p, ('c' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_35: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_35.as_pointer() },
            )
        };
        test_13(_s, _p, ('d' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_36: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_36.as_pointer() },
            )
        };
        test_13(_s, _p, ('2' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_37: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_37.as_pointer() },
            )
        };
        test_13(_s, _p, ('3' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_38: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_38.as_pointer() },
            )
        };
        test_13(_s, _p, ('4' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_39: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_39.as_pointer() },
            )
        };
        test_13(_s, _p, ('5' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_40: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_40.as_pointer() },
            )
        };
        test_13(_s, _p, ('6' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_41: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_41.as_pointer() },
            )
        };
        test_13(_s, _p, ('7' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_42: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_42.as_pointer() },
            )
        };
        test_13(_s, _p, ('A' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_43: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_43.as_pointer() },
            )
        };
        test_13(_s, _p, ('B' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_44: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_44.as_pointer() },
            )
        };
        test_13(_s, _p, ('C' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_45: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_45.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_46: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_46.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_47: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_47.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_48: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_48.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_49: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_49.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_50: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_50.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_51: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_51.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_52: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_52.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_53: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_53.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_54: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_54.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_55: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_55.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_56: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_56.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_57: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_57.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_58: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_58.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_59: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_59.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_60: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_60.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_61: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { __tmp_61.as_pointer() },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
}
pub fn test_string_17() {
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_62: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_62.as_pointer() })
        };
        test_14(_s, _p, ('1' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_63: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_63.as_pointer() })
        };
        test_14(_s, _p, ('a' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_64: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_64.as_pointer() })
        };
        test_14(_s, _p, ('b' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_65: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_65.as_pointer() })
        };
        test_14(_s, _p, ('c' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_66: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_66.as_pointer() })
        };
        test_14(_s, _p, ('d' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_67: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_67.as_pointer() })
        };
        test_14(_s, _p, ('2' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_68: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_68.as_pointer() })
        };
        test_14(_s, _p, ('3' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_69: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_69.as_pointer() })
        };
        test_14(_s, _p, ('4' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_70: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_70.as_pointer() })
        };
        test_14(_s, _p, ('5' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_71: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_71.as_pointer() })
        };
        test_14(_s, _p, ('6' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_72: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_72.as_pointer() })
        };
        test_14(_s, _p, ('7' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_73: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_73.as_pointer() })
        };
        test_14(_s, _p, ('A' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_74: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_74.as_pointer() })
        };
        test_14(_s, _p, ('B' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_75: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_75.as_pointer() })
        };
        test_14(_s, _p, ('C' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_76: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_76.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_77: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_77.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_78: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_78.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_79: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_79.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_80: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_80.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_81: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_81.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_82: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_82.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_83: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_83.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_84: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_84.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_85: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_85.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_86: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_86.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_87: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_87.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_88: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_88.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_89: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_89.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_90: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_90.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_91: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_91.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    ({
        let _s: Ptr<()> = s.as_pointer();
        let _p: () = {
            let __tmp_92: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            std___wrap_iter_const_char_ptr_::new_1({ __tmp_92.as_pointer() })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
}
pub fn test_18() -> () {
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
    fn operator_deref(&self) -> Ptr<()>;
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
        n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__;
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_char__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_char__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
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
                memset_8(
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
                        memset_8(
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
