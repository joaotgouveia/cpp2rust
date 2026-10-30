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
pub struct test_macros_detail_is_same_std_optional_int___std_optional_int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LVal {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CLVal {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RVal {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CRVal {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RefQual {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CRefQual {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RVRefQual {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RVCRefQual {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct NoCopy {}
impl NoCopy {
    pub fn copy_from(_a0: Ptr<NoCopy>) -> Self {
        let __this: NoCopy = Self {};
        unimplemented!();
        __this
    }
}
impl Clone for NoCopy {
    fn clone(&self) -> Self {
        let __src: Value<NoCopy> = Rc::new(RefCell::new(NoCopy {}));
        NoCopy::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoMove {}
pub fn test_val_types_1() {
    {
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut l: RefQual = <RefQual>::default();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
    }
    {
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut l: CRefQual = <CRefQual>::default();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
    }
    {
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            const _: () = assert!(true, "");;
        }
    }
    {
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut l: RVCRefQual = <RVCRefQual>::default();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonConst {}
pub fn test_sfinae_2() {
    let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let l: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |x: Ptr<NonConst>| -> () {
            return ({ NonConstImpl::non_const(&x) });
        },
        |x: Ptr<NonConst>| -> () {
            return ({ NonConstImpl::non_const(&x) });
        }
    )));
    &(unimplemented!());
    &(unimplemented!());
}
pub fn test_3() -> () {
    ({ test_sfinae_2() });
    ({ test_val_types_1() });
    let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let copt: Ptr<()> = opt.as_pointer();
    let never_called: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn(()) -> ()>::new(|mut _a0: ()| -> () {
            {
                unimplemented!();
                return 0_();
            }
        }),
    ));
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    let nc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cnc: Ptr<()> = nc.as_pointer();
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
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
    ({ test_3() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait CLValImpl {
    fn operator_call_2(&self, _a0: Ptr<()>) -> ();
}
impl CLValImpl for Ptr<CLVal> {
    fn operator_call_2(&self, _a0: Ptr<()>) -> () {
        return 1_();
    }
}
pub trait CRValImpl {
    fn operator_call_4(&self, _a0: Ptr<()>) -> ();
}
impl CRValImpl for Ptr<CRVal> {
    fn operator_call_4(&self, _a0: Ptr<()>) -> () {
        return 1_();
    }
}
pub trait CRefQualImpl {
    fn operator_call_2(&self, _a0: ()) -> ();
}
impl CRefQualImpl for Ptr<CRefQual> {
    fn operator_call_2(&self, mut _a0: ()) -> () {
        return 1_();
    }
}
pub trait LValImpl {
    fn operator_call_1(&self, _a0: Ptr<()>) -> ();
}
impl LValImpl for Ptr<LVal> {
    fn operator_call_1(&self, _a0: Ptr<()>) -> () {
        return 1_();
    }
}
pub trait NoCopyImpl {
    fn operator_call(&self, _a0: Ptr<NoCopy>) -> ();
    fn copy_assign(&self, _a0: Ptr<NoCopy>) -> Ptr<NoCopy>;
}
impl NoCopyImpl for Ptr<NoCopy> {
    fn operator_call(&self, _a0: Ptr<NoCopy>) -> () {
        return 1_();
    }
    fn copy_assign(&self, _a0: Ptr<NoCopy>) -> Ptr<NoCopy> {
        return (*self).clone();
    }
}
pub trait NoMoveImpl {
    fn operator_call(&self, _a0: Ptr<NoCopy>) -> NoMove;
}
impl NoMoveImpl for Ptr<NoMove> {
    fn operator_call(&self, _a0: Ptr<NoCopy>) -> NoMove {
        return <NoMove>::default();
    }
}
pub trait NonConstImpl {
    fn non_const(&self) -> ();
}
impl NonConstImpl for Ptr<NonConst> {
    fn non_const(&self) -> () {
        return 1_();
    }
}
pub trait RVCRefQualImpl {
    fn operator_call_4(&self, _a0: ()) -> ();
}
impl RVCRefQualImpl for Ptr<RVCRefQual> {
    fn operator_call_4(&self, mut _a0: ()) -> () {
        return 1_();
    }
}
pub trait RVRefQualImpl {
    fn operator_call_3(&self, _a0: ()) -> ();
}
impl RVRefQualImpl for Ptr<RVRefQual> {
    fn operator_call_3(&self, mut _a0: ()) -> () {
        return 1_();
    }
}
pub trait RValImpl {
    fn operator_call_3(&self, _a0: Ptr<()>) -> ();
}
impl RValImpl for Ptr<RVal> {
    fn operator_call_3(&self, _a0: Ptr<()>) -> () {
        return 1_();
    }
}
pub trait RefQualImpl {
    fn operator_call_1(&self, _a0: ()) -> ();
}
impl RefQualImpl for Ptr<RefQual> {
    fn operator_call_1(&self, mut _a0: ()) -> () {
        return 1_();
    }
}
pub fn __cpp2rust_init_globals() {}
