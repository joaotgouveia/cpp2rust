extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Rep {
    #[offset(0)]
    data_: i32,
}
impl Rep {
    pub fn new_1() -> Self {
        Self { data_: -1_i32 }
    }
    pub fn new_2(mut i: i32) -> Self {
        Self { data_: i }
    }
}
impl CppEq for Rep {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        RepImpl::operator_eq_4(__lhs, __rhs.clone())
    }
}
impl CppMove for Rep {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Rep {
    fn default() -> Self {
        { Rep::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotARep {}
impl CppMove for NotARep {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i64, to_i64_2)]
#[byte_size(1)]
pub struct RepConstConvertibleLWG3050 {}
impl CppMove for RepConstConvertibleLWG3050 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    assert!((({ std_chrono_duration_values_int_::max() }) == i32::MAX));
    assert!((({ std_chrono_duration_values_double_::max() }) == f64::MAX));
    assert!(
        ({
            let _r: Value<Rep> = Rc::new(RefCell::new(({ std_numeric_limits_Rep_::max() })));
            RepImpl::operator_eq_4(
                &Rc::new(RefCell::new(({ std_chrono_duration_values_Rep_::max() }))).as_pointer(),
                _r.as_pointer(),
            )
        })
    );
    const _: () = assert!(
        true,
        "std::chrono::duration_values<int>::max() ==\n           std::numeric_limits<int>::max()"
    );;
    const _: () = assert!(
        true,
        "std::chrono::duration_values<double>::max() ==\n           std::numeric_limits<double>::max()"
    );;
    const _: () = assert!(
        true,
        "std::chrono::duration_values<Rep>::max() ==\n           std::numeric_limits<Rep>::max()"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return 0;
}
pub trait RepImpl {
    fn operator_eq_3(&self, i: i32) -> bool;
    fn operator_eq_4(&self, r: Ptr<Rep>) -> bool;
    fn operator_mul_assign(&self, x: Rep) -> Ptr<Rep>;
    fn operator_div_assign(&self, x: Rep) -> Ptr<Rep>;
}
impl RepImpl for Ptr<Rep> {
    fn operator_eq_3(&self, mut i: i32) -> bool {
        return ((*self).with(|__s| __s.data_) == i);
    }
    fn operator_eq_4(&self, r: Ptr<Rep>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } == { r.with(|__s| __s.data_) });
    }
    fn operator_mul_assign(&self, mut x: Rep) -> Ptr<Rep> {
        {
            let __rhs = { x.data_ };
            field!((*self), data_).with_mut(|__v| *__v = *__v * __rhs)
        };
        return (*self).clone();
    }
    fn operator_div_assign(&self, mut x: Rep) -> Ptr<Rep> {
        {
            let __rhs = { x.data_ };
            field!((*self), data_).with_mut(|__v| *__v = *__v / __rhs)
        };
        return (*self).clone();
    }
}
pub trait RepConstConvertibleLWG3050Impl {
    fn to_i64_2(&self) -> i64;
}
impl RepConstConvertibleLWG3050Impl for Ptr<RepConstConvertibleLWG3050> {
    fn to_i64_2(&self) -> i64 {
        return 2_i64;
    }
}
pub fn __cpp2rust_init_globals() {}
