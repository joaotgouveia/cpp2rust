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
pub struct detail_parse_integer_impl_int_ {}
impl Callable1<Ptr<Vec<i8>>, i32> for detail_parse_integer_impl_int_ {
    fn call(&self, a1: Ptr<Vec<i8>>) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            detail_parse_integer_impl_int_Impl::operator_call_1_char(__it, a1)
        })
    }
}
impl CppMove for detail_parse_integer_impl_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_long_ {}
impl Callable1<Ptr<Vec<i8>>, i64> for detail_parse_integer_impl_long_ {
    fn call(&self, a1: Ptr<Vec<i8>>) -> i64 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            detail_parse_integer_impl_long_Impl::operator_call_1_char(__it, a1)
        })
    }
}
impl CppMove for detail_parse_integer_impl_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_long_long_ {}
impl Callable1<Ptr<Vec<i8>>, i64> for detail_parse_integer_impl_long_long_ {
    fn call(&self, a1: Ptr<Vec<i8>>) -> i64 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            detail_parse_integer_impl_long_long_Impl::operator_call_1_char(__it, a1)
        })
    }
}
impl CppMove for detail_parse_integer_impl_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_unsigned_int_ {}
impl Callable1<Ptr<Vec<i8>>, u32> for detail_parse_integer_impl_unsigned_int_ {
    fn call(&self, a1: Ptr<Vec<i8>>) -> u32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            detail_parse_integer_impl_unsigned_int_Impl::operator_call_1_char(__it, a1)
        })
    }
}
impl CppMove for detail_parse_integer_impl_unsigned_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_unsigned_long_ {}
impl Callable1<Ptr<Vec<i8>>, u64> for detail_parse_integer_impl_unsigned_long_ {
    fn call(&self, a1: Ptr<Vec<i8>>) -> u64 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            detail_parse_integer_impl_unsigned_long_Impl::operator_call_1_char(__it, a1)
        })
    }
}
impl CppMove for detail_parse_integer_impl_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_parse_integer_impl_unsigned_long_long_ {}
impl Callable1<Ptr<Vec<i8>>, u64> for detail_parse_integer_impl_unsigned_long_long_ {
    fn call(&self, a1: Ptr<Vec<i8>>) -> u64 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            detail_parse_integer_impl_unsigned_long_long_Impl::operator_call_1_char(__it, a1)
        })
    }
}
impl CppMove for detail_parse_integer_impl_unsigned_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn parse_integer_5(str: Ptr<Vec<i8>>) -> i32 {
    return ({
        detail_parse_integer_impl_int_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(<detail_parse_integer_impl_int_>::default())).as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_6(str: Ptr<Vec<i8>>) -> i64 {
    return ({
        detail_parse_integer_impl_long_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(<detail_parse_integer_impl_long_>::default())).as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_7(str: Ptr<Vec<i8>>) -> i64 {
    return ({
        detail_parse_integer_impl_long_long_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_long_long_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_8(str: Ptr<Vec<i8>>) -> u32 {
    return ({
        detail_parse_integer_impl_unsigned_int_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_unsigned_int_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_9(str: Ptr<Vec<i8>>) -> u64 {
    return ({
        detail_parse_integer_impl_unsigned_long_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_unsigned_long_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn parse_integer_10(str: Ptr<Vec<i8>>) -> u64 {
    return ({
        detail_parse_integer_impl_unsigned_long_long_Impl::operator_call_1_char(
            &Rc::new(RefCell::new(
                <detail_parse_integer_impl_unsigned_long_long_>::default(),
            ))
            .as_pointer(),
            (str).clone(),
        )
    });
}
pub fn test_signed_11() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_12(0) })));
        assert!(({ (((*s.borrow()).len() - 1) == 1_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_12(12345) })));
        assert!(({ (((*s.borrow()).len() - 1) == 5_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"12345").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_12(-12345_i32) })));
        assert!(({ (((*s.borrow()).len() - 1) == 6_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"-12345").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_12(i32::MAX) })));
        assert!(
            ({
                (((*s.borrow()).len() - 1) == ((digits10_13.with(|rc| *rc.borrow()) + 1) as usize))
            })
        );
        let mut t: i32 = ({ parse_integer_5((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i32::MAX));
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_12(i32::MIN) })));
        let mut t: i32 = ({ parse_integer_5((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i32::MIN));
    }
}
pub fn test_signed_14() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_15(0_i64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 1_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_15(12345_i64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 5_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"12345").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_15((-12345_i32 as i64)) })));
        assert!(({ (((*s.borrow()).len() - 1) == 6_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"-12345").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_15(i64::MAX) })));
        assert!(
            ({
                (((*s.borrow()).len() - 1) == ((digits10_16.with(|rc| *rc.borrow()) + 1) as usize))
            })
        );
        let mut t: i64 = ({ parse_integer_6((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i64::MAX));
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_15(i64::MIN) })));
        let mut t: i64 = ({ parse_integer_6((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i64::MIN));
    }
}
pub fn test_signed_17() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_18(0_i64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 1_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_18(12345_i64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 5_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"12345").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_18((-12345_i32 as i64)) })));
        assert!(({ (((*s.borrow()).len() - 1) == 6_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"-12345").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_18(i64::MAX) })));
        assert!(
            ({
                (((*s.borrow()).len() - 1) == ((digits10_19.with(|rc| *rc.borrow()) + 1) as usize))
            })
        );
        let mut t: i64 = ({ parse_integer_7((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i64::MAX));
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_18(i64::MIN) })));
        let mut t: i64 = ({ parse_integer_7((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == i64::MIN));
    }
}
pub fn test_unsigned_20() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_21(0_u32) })));
        assert!(({ (((*s.borrow()).len() - 1) == 1_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_21(12345_u32) })));
        assert!(({ (((*s.borrow()).len() - 1) == 5_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"12345").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_21(u32::MAX) })));
        assert!(
            ({
                (((*s.borrow()).len() - 1) == ((digits10_22.with(|rc| *rc.borrow()) + 1) as usize))
            })
        );
        let mut t: u32 = ({ parse_integer_8((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == u32::MAX));
    }
}
pub fn test_unsigned_23() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_24(0_u64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 1_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_24(12345_u64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 5_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"12345").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_24(u64::MAX) })));
        assert!(
            ({
                (((*s.borrow()).len() - 1) == ((digits10_25.with(|rc| *rc.borrow()) + 1) as usize))
            })
        );
        let mut t: u64 = ({ parse_integer_9((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == u64::MAX));
    }
}
pub fn test_unsigned_26() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_27(0_u64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 1_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_27(12345_u64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 5_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"12345").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(
            ({ to_string_27(({ std_numeric_limits_unsigned_long_long_::max() })) }),
        ));
        assert!(
            ({
                (((*s.borrow()).len() - 1) == ((digits10_28.with(|rc| *rc.borrow()) + 1) as usize))
            })
        );
        let mut t: u64 = ({ parse_integer_10((s.as_pointer()).reinterpret_cast::<Vec<i8>>()) });
        assert!((t == ({ std_numeric_limits_unsigned_long_long_::max() })));
    }
}
pub fn test_float_29() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_30(0_f32) })));
        assert!(({ (((*s.borrow()).len() - 1) == 8_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"0.000000").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_30(12345_f32) })));
        assert!(({ (((*s.borrow()).len() - 1) == 12_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"12345.000000").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_30((-12345_i32 as f32)) })));
        assert!(({ (((*s.borrow()).len() - 1) == 13_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"-12345.000000").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
}
pub fn test_float_31() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_32(0_f64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 8_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"0.000000").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_32(12345_f64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 12_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"12345.000000").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_32((-12345_i32 as f64)) })));
        assert!(({ (((*s.borrow()).len() - 1) == 13_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"-12345.000000").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
}
pub fn test_float_33() {
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_34(0_f64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 8_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"0.000000").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_34(12345_f64) })));
        assert!(({ (((*s.borrow()).len() - 1) == 12_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"12345.000000").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
    }
    {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new(({ to_string_34((-12345_i32 as f64)) })));
        assert!(({ (((*s.borrow()).len() - 1) == 13_usize) }));
        assert!(
            (((s.decay()
                .offset(({ ((*s.borrow()).len() - 1) }) as isize)
                .read()) as i32)
                == 0)
        );
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"-12345.000000").with_c_str(|__s| {
                    (*s.borrow())[..(*s.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_signed_11() });
    ({ test_signed_14() });
    ({ test_signed_17() });
    ({ test_unsigned_20() });
    ({ test_unsigned_23() });
    ({ test_unsigned_26() });
    ({ test_float_29() });
    ({ test_float_31() });
    ({ test_float_33() });
    return 0;
}
pub trait detail_parse_integer_impl_int_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i32;
}
impl detail_parse_integer_impl_int_Impl for Ptr<detail_parse_integer_impl_int_> {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i32 {
        return ({ stoi_0((str).reinterpret_cast::<Vec<i8>>(), None, None) });
    }
}
pub trait detail_parse_integer_impl_long_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i64;
}
impl detail_parse_integer_impl_long_Impl for Ptr<detail_parse_integer_impl_long_> {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i64 {
        return ({ stol_1((str).reinterpret_cast::<Vec<i8>>(), None, None) });
    }
}
pub trait detail_parse_integer_impl_long_long_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i64;
}
impl detail_parse_integer_impl_long_long_Impl for Ptr<detail_parse_integer_impl_long_long_> {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> i64 {
        return ({ stoll_2((str).reinterpret_cast::<Vec<i8>>(), None, None) });
    }
}
pub trait detail_parse_integer_impl_unsigned_int_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u32;
}
impl detail_parse_integer_impl_unsigned_int_Impl for Ptr<detail_parse_integer_impl_unsigned_int_> {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u32 {
        return (({ stoul_3((str).reinterpret_cast::<Vec<i8>>(), None, None) }) as u32);
    }
}
pub trait detail_parse_integer_impl_unsigned_long_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u64;
}
impl detail_parse_integer_impl_unsigned_long_Impl
    for Ptr<detail_parse_integer_impl_unsigned_long_>
{
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u64 {
        return ({ stoul_3((str).reinterpret_cast::<Vec<i8>>(), None, None) });
    }
}
pub trait detail_parse_integer_impl_unsigned_long_long_Impl {
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u64;
}
impl detail_parse_integer_impl_unsigned_long_long_Impl
    for Ptr<detail_parse_integer_impl_unsigned_long_long_>
{
    fn operator_call_1_char(&self, str: Ptr<Vec<i8>>) -> u64 {
        return ({ stoull_4((str).reinterpret_cast::<Vec<i8>>(), None, None) });
    }
}
pub fn __cpp2rust_init_globals() {}
