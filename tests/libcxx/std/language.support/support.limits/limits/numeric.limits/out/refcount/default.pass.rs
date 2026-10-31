extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    pub data_: i32,
}
impl A {
    pub fn new(i: Option<i32>) -> Self {
        let mut i: i32 = i.unwrap_or_else(|| 0);
        Self { data_: i }
    }
}
impl CppEq for A {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new(None) }
    }
}
pub fn operator_eq_0(x: Ptr<A>, y: Ptr<A>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
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
    const _: () = assert!(true, "std::numeric_limits<A>::is_specialized == false");;
    assert!(
        ({
            let _x: Value<A> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<A> = Rc::new(RefCell::new(A::new(None)));
            operator_eq_0(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Value<A> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<A> = Rc::new(RefCell::new(A::new(None)));
            operator_eq_0(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Value<A> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<A> = Rc::new(RefCell::new(A::new(None)));
            operator_eq_0(_x.as_pointer(), _y.as_pointer())
        })
    );
    const _: () = assert!(true, "std::numeric_limits<A>::digits == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::digits10 == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::max_digits10 == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_signed == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_integer == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_exact == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::radix == 0");;
    assert!(
        ({
            let _x: Value<A> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<A> = Rc::new(RefCell::new(A::new(None)));
            operator_eq_0(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Value<A> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<A> = Rc::new(RefCell::new(A::new(None)));
            operator_eq_0(_x.as_pointer(), _y.as_pointer())
        })
    );
    const _: () = assert!(true, "std::numeric_limits<A>::min_exponent == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::min_exponent10 == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::max_exponent == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::max_exponent10 == 0");;
    const _: () = assert!(true, "std::numeric_limits<A>::has_infinity == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::has_quiet_NaN == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::has_signaling_NaN == false");;
    const _: () = assert!(
        true,
        "std::numeric_limits<A>::has_denorm == std::denorm_absent"
    );;
    const _: () = assert!(true, "std::numeric_limits<A>::has_denorm_loss == false");;
    assert!(
        ({
            let _x: Value<A> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<A> = Rc::new(RefCell::new(A::new(None)));
            operator_eq_0(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Value<A> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<A> = Rc::new(RefCell::new(A::new(None)));
            operator_eq_0(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Value<A> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<A> = Rc::new(RefCell::new(A::new(None)));
            operator_eq_0(_x.as_pointer(), _y.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Value<A> = Rc::new(RefCell::new(unimplemented!()));
            let _y: Value<A> = Rc::new(RefCell::new(A::new(None)));
            operator_eq_0(_x.as_pointer(), _y.as_pointer())
        })
    );
    const _: () = assert!(true, "std::numeric_limits<A>::is_iec559 == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_bounded == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::is_modulo == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::traps == false");;
    const _: () = assert!(true, "std::numeric_limits<A>::tinyness_before == false");;
    const _: () = assert!(
        true,
        "std::numeric_limits<A>::round_style == std::round_toward_zero"
    );;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
