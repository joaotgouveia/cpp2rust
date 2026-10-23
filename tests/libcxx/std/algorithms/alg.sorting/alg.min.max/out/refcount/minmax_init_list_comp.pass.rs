extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct binary_counting_predicate_bool__ptr__int__int___int__int_ {
    #[offset(0)]
    #[byte_size(8)]
    p_: FnPtr<fn((), ()) -> ()>,
    #[offset(8)]
    count_: (),
}
impl binary_counting_predicate_bool__ptr__int__int___int__int_ {
    pub fn new(mut p: FnPtr<fn((), ()) -> ()>) -> Self {
        Self {
            p_: (p).clone(),
            count_: 0_(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct binary_counting_predicate_std_greater_int___int__int_ {
    #[offset(0)]
    #[byte_size(1)]
    p_: (),
    #[offset(8)]
    count_: (),
}
impl binary_counting_predicate_std_greater_int___int__int_ {
    pub fn new(p: ()) -> Self {
        let p: Value<()> = Rc::new(RefCell::new(p));
        Self {
            p_: (*p.borrow()).clone(),
            count_: 0_(),
        }
    }
}
impl Default for binary_counting_predicate_std_greater_int___int__int_ {
    fn default() -> Self {
        {
            binary_counting_predicate_std_greater_int___int__int_ {
                p_: unimplemented!(),
                count_: unimplemented!(),
            }
        }
    }
}
pub fn all_equal_0(mut _a0: (), mut _a1: ()) -> () {
    return false;
}
pub fn test_all_equal_1(il: ()) {
    let il: Value<()> = Rc::new(RefCell::new(il));
    let pred: Value<binary_counting_predicate_bool__ptr__int__int___int__int_> =
        Rc::new(RefCell::new(
            binary_counting_predicate_bool__ptr__int__int___int__int_::new({
                FnPtr::<fn((), ()) -> ()>::new(all_equal_0)
            }),
        ));
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut ptr: Ptr<()> = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let pred: Value<binary_counting_predicate_std_greater_int___int__int_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) }),
    ));
    unimplemented!();
    unimplemented!();
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    ({ test_all_equal_1(unimplemented!()) });
    {
        const _: () = assert!(
            true,
            "(std::minmax({{1, 2, 3}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{1, 3, 2}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{2, 1, 3}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{2, 3, 1}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{3, 1, 2}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
        const _: () = assert!(
            true,
            "(std::minmax({{3, 2, 1}}, std::greater<int>()) == std::pair<int, int>(3, 1))"
        );;
    }
    return 0_();
}
pub trait binary_counting_predicate_bool__ptr__int__int___int__int_Impl {
    fn operator_call(&self, a1: Ptr<()>, a2: Ptr<()>) -> ();
    fn count(&self) -> ();
    fn reset(&self) {
        unimplemented!()
    }
}
impl binary_counting_predicate_bool__ptr__int__int___int__int_Impl
    for Ptr<binary_counting_predicate_bool__ptr__int__int___int__int_>
{
    fn operator_call(&self, a1: Ptr<()>, a2: Ptr<()>) -> () {
        field!((*self), count_).with_mut(|__v| __v.prefix_inc());
        return ({
            let _arg0: () = (a1.read());
            let _arg1: () = (a2.read());
            (*self).with(|__s| __s.p_.clone()).call(_arg0, _arg1)
        });
    }
    fn count(&self) -> () {
        return (*self).with(|__s| __s.count_);
    }
}
pub trait binary_counting_predicate_std_greater_int___int__int_Impl {
    fn operator_call(&self, a1: Ptr<()>, a2: Ptr<()>) -> ();
    fn count(&self) -> ();
    fn reset(&self) {
        unimplemented!()
    }
}
impl binary_counting_predicate_std_greater_int___int__int_Impl
    for Ptr<binary_counting_predicate_std_greater_int___int__int_>
{
    fn operator_call(&self, a1: Ptr<()>, a2: Ptr<()>) -> () {
        field!((*self), count_).with_mut(|__v| __v.prefix_inc());
        return unimplemented!();
    }
    fn count(&self) -> () {
        return (*self).with(|__s| __s.count_);
    }
}
pub fn __cpp2rust_init_globals() {}
