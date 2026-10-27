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
    data_: Box<[()]>,
    #[offset(40)]
    size_: (),
}
impl Default for ConstexprFixedCapacityDeque_int__10_ {
    fn default() -> Self {
        {
            ConstexprFixedCapacityDeque_int__10_ {
                data_: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                size_: 0_(),
            }
        }
    }
}
pub fn test_0(c: ()) -> () {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_1(c: ConstexprFixedCapacityDeque_int__10_) -> () {
    let c: Value<ConstexprFixedCapacityDeque_int__10_> = Rc::new(RefCell::new(c));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
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
    ({ test_0(unimplemented!()) });
    ({ test_1(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0_();
}
pub trait ConstexprFixedCapacityDeque_int__10_Impl {
    fn begin_2(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end_3(&self) -> Ptr<()>;
    fn begin_4(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end_5(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
    fn front(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn back(&self) -> Ptr<()>;
    fn insert(&self, pos: Ptr<()>, t: ()) -> Ptr<()>;
    fn push_back(&self, t: ());
    fn push_front(&self, mut t: ()) {
        unimplemented!()
    }
}
impl ConstexprFixedCapacityDeque_int__10_Impl for Ptr<ConstexprFixedCapacityDeque_int__10_> {
    fn end_3(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), data_) as Ptr<()>)
            .offset(((*self).with(|__s| __s.size_)) as isize);
    }
    fn back(&self) -> Ptr<()> {
        unimplemented!();
        return (array_field_ptr!((*self), data_) as Ptr<()>)
            .offset(((*self).with(|__s| __s.size_) - 1_()) as isize);
    }
    fn insert(&self, mut pos: Ptr<()>, mut t: ()) -> Ptr<()> {
        let mut i: () = ((pos).clone() - (array_field_ptr!((*self), data_) as Ptr<()>)) as ();
        if (i != (*self).with(|__s| __s.size_)) {
            unimplemented!();
        }
        array_elem!((*self), data_, i).write(t);
        {
            field!((*self), size_).with_mut(|__v| *__v = *__v + 1_())
        };
        return (array_field_ptr!((*self), data_) as Ptr<()>).offset((i) as isize);
    }
    fn push_back(&self, mut t: ()) {
        ({
            let _pos: Ptr<()> = ({ ConstexprFixedCapacityDeque_int__10_Impl::end_3(self) });
            ConstexprFixedCapacityDeque_int__10_Impl::insert(self, _pos, t)
        });
    }
}
pub fn __cpp2rust_init_globals() {}
