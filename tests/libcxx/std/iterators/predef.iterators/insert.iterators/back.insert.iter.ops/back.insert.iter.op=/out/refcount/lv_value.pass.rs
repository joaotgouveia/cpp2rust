extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(44)]
pub struct ConstexprFixedCapacityDeque_int__10_ {
    #[offset(0)]
    #[byte_size(40)]
    data_: Box<[i32]>,
    #[offset(40)]
    size_: i32,
}
impl Default for ConstexprFixedCapacityDeque_int__10_ {
    fn default() -> Self {
        {
            ConstexprFixedCapacityDeque_int__10_ {
                data_: (0..10).map(|_| 0_i32).collect::<Box<[i32]>>(),
                size_: 0,
            }
        }
    }
}
pub fn test_0(c: Vec<Copyable>) -> bool {
    let c: Value<Vec<Copyable>> = Rc::new(RefCell::new(c));
    let v: Value<Copyable> = Rc::new(RefCell::new(Copyable::new()));
    let _dtor_v = ScopedDestructor::new(&v, |__p| __p.destructor());
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!(
        ({
            let _x: Ptr<Copyable> = c.decay().to_last();
            operator_eq_1(_x, v.as_pointer())
        })
    );
    return true;
}
pub fn test_2(c: ConstexprFixedCapacityDeque_int__10_) -> bool {
    let c: Value<ConstexprFixedCapacityDeque_int__10_> = Rc::new(RefCell::new(c));
    let mut v: i32 = 0_i32;
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!(((({ ConstexprFixedCapacityDeque_int__10_Impl::back(&c.as_pointer(),) }).read()) == v));
    return true;
}
pub fn operator_eq_1(x: Ptr<Copyable>, y: Ptr<Copyable>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct Copyable {
    #[offset(0)]
    data_: i32,
}
impl Copyable {
    pub fn new() -> Self {
        Self { data_: 0 }
    }
}
impl CppEq for Copyable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for Copyable {
    fn default() -> Self {
        { Copyable::new() }
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
    ({ test_0(Vec::new()) });
    ({ test_2(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0;
}
pub trait ConstexprFixedCapacityDeque_int__10_Impl {
    fn begin_2(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn end_3(&self) -> Ptr<i32>;
    fn begin_4(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn end_5(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn size(&self) -> usize {
        unimplemented!()
    }
    fn front(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back(&self) -> Ptr<i32>;
    fn insert(&self, pos: Ptr<i32>, t: i32) -> Ptr<i32>;
    fn push_back(&self, t: i32);
    fn push_front(&self, mut t: i32) {
        unimplemented!()
    }
}
impl ConstexprFixedCapacityDeque_int__10_Impl for Ptr<ConstexprFixedCapacityDeque_int__10_> {
    fn end_3(&self) -> Ptr<i32> {
        return (array_field_ptr!((*self), data_) as Ptr<i32>)
            .offset(((*self).with(|__s| __s.size_)) as isize);
    }
    fn back(&self) -> Ptr<i32> {
        assert!(((*self).with(|__s| __s.size_) >= 1));
        return (array_field_ptr!((*self), data_) as Ptr<i32>)
            .offset(((*self).with(|__s| __s.size_) - 1) as isize);
    }
    fn insert(&self, mut pos: Ptr<i32>, mut t: i32) -> Ptr<i32> {
        let mut i: i32 =
            ((((pos).clone() - (array_field_ptr!((*self), data_) as Ptr<i32>)) as i64) as i32);
        if (i != (*self).with(|__s| __s.size_)) {
            unimplemented!();
        }
        array_elem!((*self), data_, i).write(t);
        {
            field!((*self), size_).with_mut(|__v| *__v = *__v + 1)
        };
        return (array_field_ptr!((*self), data_) as Ptr<i32>).offset((i) as isize);
    }
    fn push_back(&self, mut t: i32) {
        ({
            let _pos: Ptr<i32> = ({ ConstexprFixedCapacityDeque_int__10_Impl::end_3(self) });
            ConstexprFixedCapacityDeque_int__10_Impl::insert(self, _pos, t)
        });
    }
}
pub trait CopyableImpl {
    fn destructor(&self);
}
impl CopyableImpl for Ptr<Copyable> {
    fn destructor(&self) {
        field!((*self), data_).write(-1_i32);
    }
}
pub fn __cpp2rust_init_globals() {}
