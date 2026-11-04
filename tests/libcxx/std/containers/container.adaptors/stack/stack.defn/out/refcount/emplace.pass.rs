extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct Emplaceable {
    #[offset(0)]
    int_: i32,
    #[offset(8)]
    double_: f64,
}
impl Emplaceable {
    pub fn new_1() -> Self {
        Self {
            int_: 0,
            double_: 0_f64,
        }
    }
    pub fn new_2(mut i: i32, mut d: f64) -> Self {
        Self {
            int_: i,
            double_: d,
        }
    }
    pub fn move_from(x: Ptr<Emplaceable>) -> Self {
        let __this: Emplaceable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(0);
        field!(x, double_).write(0_f64);
        __this
    }
}
impl CppLess for Emplaceable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Emplaceable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for Emplaceable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for Emplaceable {
    fn default() -> Self {
        { Emplaceable::new_1() }
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
    ({ test_return_type_0() });
    ({ test_return_type_1() });
    let q: Value<std_stack_Emplaceable__std_deque_Emplaceable__std_allocator_Emplaceable___> =
        Rc::new(RefCell::new(
            <std_stack_Emplaceable__std_deque_Emplaceable__std_allocator_Emplaceable___>::default(),
        ));
    let r1: Ptr<Emplaceable> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(1));
        let ___args_1: Value<f64> = Rc::new(RefCell::new(2.5_f64));
        (*q.borrow()).emplace(___args_0.as_pointer(), ___args_1.as_pointer())
    });
    ({ cpp2rust_assert_fail_2(({ (r1).clone() } == { ({ (*q.borrow()).top_15() }) })) });
    let r2: Ptr<Emplaceable> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(2));
        let ___args_1: Value<f64> = Rc::new(RefCell::new(3.5_f64));
        (*q.borrow()).emplace(___args_0.as_pointer(), ___args_1.as_pointer())
    });
    ({ cpp2rust_assert_fail_2(({ (r2).clone() } == { ({ (*q.borrow()).top_15() }) })) });
    let r3: Ptr<Emplaceable> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(3));
        let ___args_1: Value<f64> = Rc::new(RefCell::new(4.5_f64));
        (*q.borrow()).emplace(___args_0.as_pointer(), ___args_1.as_pointer())
    });
    ({ cpp2rust_assert_fail_2(({ (r3).clone() } == { ({ (*q.borrow()).top_15() }) })) });
    ({ cpp2rust_assert_fail_2((({ (*q.borrow()).size() }) == 3_u64)) });
    ({
        cpp2rust_assert_fail_2(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 3 }, { 4.5_f64 })));
                EmplaceableImpl::operator_eq(&({ (*q.borrow()).top_15() }), _x.as_pointer())
            }),
        )
    });
    return 0;
}
pub trait EmplaceableImpl {
    fn copy_assign(&self, _a0: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable>;
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool;
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool;
    fn get(&self) -> i32;
}
impl EmplaceableImpl for Ptr<Emplaceable> {
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(0);
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write(0_f64);
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
}
pub fn __cpp2rust_init_globals() {}
