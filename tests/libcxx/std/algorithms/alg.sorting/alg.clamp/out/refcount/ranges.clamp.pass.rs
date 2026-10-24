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
pub struct NoComp {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CreateNoComp {}
const _: () = assert!(true, "HasClamp<int, std::ranges::less, std::identity>");
const _: () = assert!(!(false), "!HasClamp<NoComp>");
const _: () = assert!(!(false), "!HasClamp<int, NoComp>");
const _: () = assert!(!(false), "!HasClamp<int, std::ranges::less, CreateNoComp>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct EnsureValueCategoryComp {}
pub fn test_0() -> () {
    {
        let val: Value<()> = Rc::new(RefCell::new(2_()));
        let low: Value<()> = Rc::new(RefCell::new(1_()));
        let high: Value<()> = Rc::new(RefCell::new(3_()));
        let ret: Ptr<()> = ({
            (*clamp_1.with(Value::clone).borrow_mut())(
                val.as_pointer(),
                low.as_pointer(),
                high.as_pointer(),
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let val: Value<()> = Rc::new(RefCell::new(10_()));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let val: Value<S_2> = Rc::new(RefCell::new(S_2 { i: 10_() }));
        let low: Value<S_2> = Rc::new(RefCell::new(S_2 { i: 20_() }));
        let high: Value<S_2> = Rc::new(RefCell::new(S_2 { i: 30_() }));
        let proj: Value<FnPtr<fn(Ptr<S_2>) -> Ptr<()>>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<S_2>) -> Ptr<()>>::new(
                |s: Ptr<S_2>| -> Ptr<()> {
                    {
                        return field_ptr!(s, i);
                    }
                },
            )));
        unimplemented!();
    }
    {
        const _: () = assert!(
            true,
            "std::indirect_strict_weak_order<EnsureValueCategoryComp, std::projected<const int*, MoveProj>>"
        );;
        unimplemented!();
    }
    {
        let counter: Value<()> = Rc::new(RefCell::new(0_()));
        let projection_function: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
            {
                let counter: Ptr<()> = counter.as_pointer();
            },
            |value: ()| -> () {
                let value: Value<()> = Rc::new(RefCell::new(value));
                counter.with_mut(|__v| __v.postfix_inc());
                return (*value.borrow());
            }
        )));
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_2 {
    #[offset(0)]
    pub i: (),
}
impl std::cmp::PartialEq for S_2 {
    fn eq(&self, other: &Self) -> bool {
        {
            S_2Impl::operator_eq(
                &Rc::new(RefCell::new(S_2 { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(S_2 { i: other.i.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for S_2 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MoveProj_3 {}
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait CreateNoCompImpl {
    fn operator_call(&self, _a0: ()) -> NoComp;
}
impl CreateNoCompImpl for Ptr<CreateNoComp> {
    fn operator_call(&self, mut _a0: ()) -> NoComp {
        return <NoComp>::default();
    }
}
pub trait EnsureValueCategoryCompImpl {
    fn operator_call_1(&self, x: Ptr<()>, y: Ptr<()>) -> ();
    fn operator_call_2(&self, x: Ptr<()>, y: Ptr<()>) -> ();
    fn operator_call_3(&self, x: Ptr<()>, y: Ptr<()>) -> ();
    fn operator_call_4(&self, x: Ptr<()>, y: Ptr<()>) -> ();
}
impl EnsureValueCategoryCompImpl for Ptr<EnsureValueCategoryComp> {
    fn operator_call_1(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        return ({ (x.read()) } < { (y.read()) });
    }
    fn operator_call_2(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        return ({ (x.read()) } < { (y.read()) });
    }
    fn operator_call_3(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        return ({ (x.read()) } < { (y.read()) });
    }
    fn operator_call_4(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        return ({ (x.read()) } < { (y.read()) });
    }
}
pub trait MoveProj_3Impl {
    fn operator_call(&self, x: Ptr<()>) -> Ptr<()>;
}
impl MoveProj_3Impl for Ptr<MoveProj_3> {
    fn operator_call(&self, x: Ptr<()>) -> Ptr<()> {
        return unimplemented!();
    }
}
pub trait S_2Impl {
    fn operator_eq(&self, other: Ptr<S_2>) -> ();
}
impl S_2Impl for Ptr<S_2> {
    fn operator_eq(&self, other: Ptr<S_2>) -> () {
        return ({ (*self).with(|__s| __s.i) } == { other.with(|__s| __s.i) });
    }
}
pub fn __cpp2rust_init_globals() {}
