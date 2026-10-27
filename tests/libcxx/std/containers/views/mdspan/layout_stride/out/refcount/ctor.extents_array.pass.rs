extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntType {
    #[offset(0)]
    pub val: (),
}
impl IntType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        {
            IntTypeImpl::operator_eq(
                &Rc::new(RefCell::new(IntType {
                    val: self.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(IntType {
                    val: other.val.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for IntType {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntTypeNC {
    #[offset(0)]
    pub val: (),
}
impl IntTypeNC {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
pub fn test_construction_0(e: (), s: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let s: Value<()> = Rc::new(RefCell::new(s));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == 0_()) {
            expected_size = 0_();
            break;
        }
        expected_size += ((unimplemented!() - 1_()) * ((unimplemented!().read()) as ()));
        r.postfix_inc();
    }
    unimplemented!();
    let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    {}
}
pub fn test_construction_1(e: (), s: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let s: Value<()> = Rc::new(RefCell::new(s));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == 0_()) {
            expected_size = 0_();
            break;
        }
        {
            let rhs_0 = (expected_size).wrapping_add(
                ((unimplemented!()).wrapping_sub(1_())).wrapping_mul((unimplemented!().read())),
            );
            expected_size = rhs_0
        };
        r.postfix_inc();
    }
    unimplemented!();
    let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub fn test_construction_2(e: (), s: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let s: Value<()> = Rc::new(RefCell::new(s));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == 0_()) {
            expected_size = 0_();
            break;
        }
        {
            let rhs_0 = (expected_size).wrapping_add(
                ((unimplemented!()).wrapping_sub(1_())).wrapping_mul((unimplemented!().read())),
            );
            expected_size = rhs_0
        };
        r.postfix_inc();
    }
    unimplemented!();
    let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub fn test_construction_3(e: (), s: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let s: Value<()> = Rc::new(RefCell::new(s));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == 0_()) {
            expected_size = 0_();
            break;
        }
        expected_size +=
            ((unimplemented!() - 1_()) * (({ IntTypeImpl::to__4(&unimplemented!()) }) as ()));
        r.postfix_inc();
    }
    unimplemented!();
    let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub fn test_construction_4(e: (), s: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let s: Value<()> = Rc::new(RefCell::new(s));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == 0_()) {
            expected_size = 0_();
            break;
        }
        {
            let rhs_0 = (expected_size).wrapping_add(
                ((unimplemented!()).wrapping_sub(1_())).wrapping_mul((unimplemented!().read())),
            );
            expected_size = rhs_0
        };
        r.postfix_inc();
    }
    unimplemented!();
    let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub fn test_construction_5(e: (), s: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let s: Value<()> = Rc::new(RefCell::new(s));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == 0_()) {
            expected_size = 0_();
            break;
        }
        expected_size += ((unimplemented!() - 1_()) * (unimplemented!().read()));
        r.postfix_inc();
    }
    unimplemented!();
    let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub fn test_construction_6(e: (), s: ()) {
    let e: Value<()> = Rc::new(RefCell::new(e));
    let s: Value<()> = Rc::new(RefCell::new(s));
    const _: () = assert!(true, "");;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == 0_()) {
            expected_size = 0_();
            break;
        }
        expected_size += ((unimplemented!() - 1_()) * (unimplemented!().read()));
        r.postfix_inc();
    }
    unimplemented!();
    let strides: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            r.postfix_inc();
        }
    }
}
pub fn test_7() -> () {
    let mut D: () = dynamic_extent_8.with(|rc| *rc.borrow());
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_construction_0(unimplemented!(), (*s.borrow()).clone()) });
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_construction_1(unimplemented!(), (*s.borrow()).clone()) });
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_construction_1(unimplemented!(), (*s.borrow()).clone()) });
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_construction_2(unimplemented!(), (*s.borrow()).clone()) });
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_construction_3(unimplemented!(), (*s.borrow()).clone()) });
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_construction_4(unimplemented!(), (*s.borrow()).clone()) });
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_construction_5(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_5(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_5(unimplemented!(), (*s.borrow()).clone()) });
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_construction_6(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_6(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_6(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_6(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_6(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_6(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_6(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_6(unimplemented!(), (*s.borrow()).clone()) });
        ({ test_construction_6(unimplemented!(), (*s.borrow()).clone()) });
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 2>, std::array<int, 3>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 2>, std::array<int, 1>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 3>, std::array<int, 2>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 2>, std::array<IntType, 2>>"
        );;
    }
    {
        const _: () = assert!(true, "std::is_convertible_v<IntType, unsigned char>");;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<mapping_t, std::dextents<int, 2>, std::array<IntType, 2>>"
        );;
    }
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
    ({ test_7() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait IntTypeImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeImpl for Ptr<IntType> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait IntTypeNCImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeNCImpl for Ptr<IntTypeNC> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub fn __cpp2rust_init_globals() {}
