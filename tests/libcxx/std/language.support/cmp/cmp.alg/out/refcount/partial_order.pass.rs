extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
pub fn has_partial_order_1(t: Ptr<()>, u: Ptr<()>) -> () {
    return true;
}
pub fn has_partial_order_2(t: Ptr<Ptr<()>>, u: Ptr<Ptr<()>>) -> () {
    return true;
}
pub fn has_partial_order_3(t: Ptr<Ptr<()>>, u: Ptr<Ptr<()>>) -> () {
    return true;
}
pub fn has_partial_order_4(t: Ptr<N11_A>, u: Ptr<N11_A>) -> () {
    return true;
}
pub fn has_partial_order_5(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N11_A {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N11_B {}
pub fn partial_order_6(_a0: Ptr<N11_A>, _a1: Ptr<N11_A>) -> () {
    return unimplemented!();
}
pub fn test_1_1_7() {
    const _: () = assert!(true, "has_partial_order(1, 2)");;
    const _: () = assert!(true, "!has_partial_order(1, (short)2)");;
    const _: () = assert!(true, "!has_partial_order(1, 2.0)");;
    const _: () = assert!(true, "!has_partial_order(1.0f, 2.0)");;
    const _: () = assert!(true, "has_partial_order((int*)nullptr, (int*)nullptr)");;
    const _: () = assert!(
        true,
        "!has_partial_order((int*)nullptr, (const int*)nullptr)"
    );;
    const _: () = assert!(
        true,
        "!has_partial_order((const int*)nullptr, (int*)nullptr)"
    );;
    const _: () = assert!(
        true,
        "has_partial_order((const int*)nullptr, (const int*)nullptr)"
    );;
    let a: Value<N11_A> = Rc::new(RefCell::new(<N11_A>::default()));
    let mut b: N11_B = <N11_B>::default();
    const _: () = assert!(true, "has_partial_order(a, a)");;
    const _: () = assert!(true, "!has_partial_order(a, b)");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N12_A {}
pub fn partial_order_8(_a0: Ptr<N12_A>, _a1: Ptr<N12_A>) -> () {
    return unimplemented!();
}
pub fn partial_order_9(_a0: Ptr<N12_A>, _a1: Ptr<N12_A>) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N12_B {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N12_PartialOrder {}
pub fn partial_order_10(lhs: Ptr<N12_C>, _a1: Ptr<N12_C>) -> N12_PartialOrder {
    field!(lhs, touched).write(true);
    return <N12_PartialOrder>::default();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct N12_C {
    #[offset(0)]
    pub touched: (),
}
impl Default for N12_C {
    fn default() -> Self {
        { N12_C { touched: false } }
    }
}
pub fn test_1_2_11() {
    let a: Value<N12_A> = Rc::new(RefCell::new(<N12_A>::default()));
    unimplemented!();
    unimplemented!();
    let mut b: N12_B = <N12_B>::default();
    const _: () = assert!(true, "!has_partial_order(b, b)");;
    let c1: Value<N12_C> = Rc::new(RefCell::new(<N12_C>::default()));
    let c2: Value<N12_C> = Rc::new(RefCell::new(<N12_C>::default()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N13_A {}
impl std::cmp::Ord for N13_A {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_12(
                Rc::new(RefCell::new(N13_A {})).as_pointer(),
                Rc::new(RefCell::new(N13_A {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for N13_A {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for N13_A {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_cmp_12(
                Rc::new(RefCell::new(N13_A {})).as_pointer(),
                Rc::new(RefCell::new(N13_A {})).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for N13_A {}
pub fn operator_cmp_13(_a0: Ptr<N13_A>, _a1: Ptr<N13_A>) -> () {
    return unimplemented!();
}
pub fn operator_cmp_12(_a0: Ptr<N13_A>, _a1: Ptr<N13_A>) -> () {
    return unimplemented!();
}
const _: () = assert!(true, "std::three_way_comparable<A>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N13_B {}
const _: () = assert!(!(false), "!std::three_way_comparable<B>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct N13_C {
    #[offset(0)]
    #[byte_size(8)]
    pub touched: Ptr<()>,
}
impl std::cmp::Ord for N13_C {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            N13_CImpl::operator_cmp(
                &Rc::new(RefCell::new(N13_C {
                    touched: self.touched.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(N13_C {
                    touched: other.touched.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for N13_C {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for N13_C {
    fn eq(&self, other: &Self) -> bool {
        {
            N13_CImpl::operator_cmp(
                &Rc::new(RefCell::new(N13_C {
                    touched: self.touched.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(N13_C {
                    touched: other.touched.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for N13_C {}
const _: () = assert!(true, "std::three_way_comparable<C>");
pub fn test_1_3_14() -> () {
    let a: Value<N13_A> = Rc::new(RefCell::new(<N13_A>::default()));
    unimplemented!();
    unimplemented!();
    let mut b: N13_B = <N13_B>::default();
    const _: () = assert!(true, "!has_partial_order(b, b)");;
    let c1_touched: Value<()> = Rc::new(RefCell::new(false));
    let c2_touched: Value<()> = Rc::new(RefCell::new(false));
    let c1: Value<N13_C> = Rc::new(RefCell::new(N13_C {
        touched: (c1_touched.as_pointer()),
    }));
    let c2: Value<N13_C> = Rc::new(RefCell::new(N13_C {
        touched: (c2_touched.as_pointer()),
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let nan: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N14_A {}
pub fn weak_order_15(_a0: Ptr<N14_A>, _a1: Ptr<N14_A>) -> () {
    return unimplemented!();
}
pub fn weak_order_16(_a0: Ptr<N14_A>, _a1: Ptr<N14_A>) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N14_B {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N14_StrongOrder {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N14_C {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N14_WeakOrder {}
pub fn weak_order_17(lhs: Ptr<N14_D>, _a1: Ptr<N14_D>) -> N14_WeakOrder {
    field!(lhs, touched).write(true);
    return <N14_WeakOrder>::default();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct N14_D {
    #[offset(0)]
    pub touched: (),
}
impl Default for N14_D {
    fn default() -> Self {
        { N14_D { touched: false } }
    }
}
pub fn test_1_4_18() -> () {
    let a: Value<N14_A> = Rc::new(RefCell::new(<N14_A>::default()));
    unimplemented!();
    unimplemented!();
    let mut b: N14_B = <N14_B>::default();
    const _: () = assert!(true, "!has_partial_order(b, b)");;
    let mut c: N14_C = <N14_C>::default();
    const _: () = assert!(true, "!has_partial_order(c, c)");;
    let d1: Value<N14_D> = Rc::new(RefCell::new(<N14_D>::default()));
    let d2: Value<N14_D> = Rc::new(RefCell::new(<N14_D>::default()));
    const _: () = assert!(true, "");;
    unimplemented!();
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
    ({ test_1_1_7() });
    ({ test_1_2_11() });
    ({ test_1_3_14() });
    ({ test_1_4_18() });
    const _: () = assert!(true, "test_1_3()");;
    const _: () = assert!(true, "test_1_4()");;
    return 0_();
}
pub trait N12_PartialOrderImpl {
    fn to_(&self) -> ();
}
impl N12_PartialOrderImpl for Ptr<N12_PartialOrder> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait N13_BImpl {
    fn operator_cmp(&self, _a0: Ptr<N13_B>) -> () {
        unimplemented!()
    }
}
impl N13_BImpl for Ptr<N13_B> {}
pub trait N13_CImpl {
    fn operator_eq(&self, _a0: Ptr<N13_C>) -> () {
        unimplemented!()
    }
    fn operator_cmp(&self, rhs: Ptr<N13_C>) -> ();
}
impl N13_CImpl for Ptr<N13_C> {
    fn operator_cmp(&self, rhs: Ptr<N13_C>) -> () {
        rhs.with(|__s| __s.touched.clone()).write(true);
        return unimplemented!();
    }
}
pub trait N14_StrongOrderImpl {
    fn to_(&self) -> ();
}
impl N14_StrongOrderImpl for Ptr<N14_StrongOrder> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait N14_WeakOrderImpl {
    fn to__1(&self) -> ();
}
impl N14_WeakOrderImpl for Ptr<N14_WeakOrder> {
    fn to__1(&self) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
