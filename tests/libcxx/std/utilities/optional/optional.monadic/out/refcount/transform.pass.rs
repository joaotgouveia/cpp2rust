extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_int___std_optional_int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LVal {}
impl Callable1<Ptr<i32>, i32> for LVal {
    fn call(&self, a1: Ptr<i32>) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            LValImpl::operator_call_1(__it, a1)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CLVal {}
impl Callable1<Ptr<i32>, i32> for CLVal {
    fn call(&self, a1: Ptr<i32>) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CLValImpl::operator_call_2(__it, a1)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RVal {}
impl Callable1<Ptr<i32>, i32> for RVal {
    fn call(&self, a1: Ptr<i32>) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            RValImpl::operator_call_3(__it, a1)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CRVal {}
impl Callable1<Ptr<i32>, i32> for CRVal {
    fn call(&self, a1: Ptr<i32>) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CRValImpl::operator_call_4(__it, a1)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RefQual {}
impl Callable1<i32, i32> for RefQual {
    fn call(&self, a1: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            RefQualImpl::operator_call_1(__it, a1)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CRefQual {}
impl Callable1<i32, i32> for CRefQual {
    fn call(&self, a1: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CRefQualImpl::operator_call_2(__it, a1)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RVRefQual {}
impl Callable1<i32, i32> for RVRefQual {
    fn call(&self, a1: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            RVRefQualImpl::operator_call_3(__it, a1)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RVCRefQual {}
impl Callable1<i32, i32> for RVCRefQual {
    fn call(&self, a1: i32) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            RVCRefQualImpl::operator_call_4(__it, a1)
        })
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct NoCopy {}
impl NoCopy {
    pub fn copy_from(_a0: Ptr<NoCopy>) -> Self {
        let __this: NoCopy = Self {};
        assert!(false);
        __this
    }
}
impl Callable1<Ptr<NoCopy>, i32> for NoCopy {
    fn call(&self, a1: Ptr<NoCopy>) -> i32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            NoCopyImpl::operator_call(__it, a1)
        })
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
            assert!((unimplemented!() as bool));
            const _: () = assert!(true, "");;
        }
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut l: RefQual = <RefQual>::default();
            assert!((unimplemented!() as bool));
            const _: () = assert!(true, "");;
        }
    }
    {
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() as bool));
            const _: () = assert!(true, "");;
        }
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut l: CRefQual = <CRefQual>::default();
            assert!((unimplemented!() as bool));
            const _: () = assert!(true, "");;
        }
    }
    {
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() as bool));
            const _: () = assert!(true, "");;
        }
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() as bool));
            const _: () = assert!(true, "");;
        }
    }
    {
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() as bool));
            const _: () = assert!(true, "");;
        }
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut l: RVCRefQual = <RVCRefQual>::default();
            assert!((unimplemented!() as bool));
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
        |x: Ptr<NonConst>| -> i32 {
            return ({ NonConstImpl::non_const(&x) });
        },
        |x: Ptr<NonConst>| -> i32 {
            return ({ NonConstImpl::non_const(&x) });
        }
    )));
    &(unimplemented!());
    &(unimplemented!());
}
pub fn test_3() -> bool {
    ({ test_sfinae_2() });
    ({ test_val_types_1() });
    let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let copt: Ptr<()> = opt.as_pointer();
    let never_called: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> i32>::new(|mut _a0: i32| -> i32 {
            {
                assert!(false);
                return 0;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_3() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait CLValImpl {
    fn operator_call_2(&self, _a0: Ptr<i32>) -> i32;
}
impl CLValImpl for Ptr<CLVal> {
    fn operator_call_2(&self, _a0: Ptr<i32>) -> i32 {
        return 1;
    }
}
pub trait CRValImpl {
    fn operator_call_4(&self, _a0: Ptr<i32>) -> i32;
}
impl CRValImpl for Ptr<CRVal> {
    fn operator_call_4(&self, _a0: Ptr<i32>) -> i32 {
        return 1;
    }
}
pub trait CRefQualImpl {
    fn operator_call_2(&self, _a0: i32) -> i32;
}
impl CRefQualImpl for Ptr<CRefQual> {
    fn operator_call_2(&self, mut _a0: i32) -> i32 {
        return 1;
    }
}
pub trait LValImpl {
    fn operator_call_1(&self, _a0: Ptr<i32>) -> i32;
}
impl LValImpl for Ptr<LVal> {
    fn operator_call_1(&self, _a0: Ptr<i32>) -> i32 {
        return 1;
    }
}
pub trait NoCopyImpl {
    fn operator_call(&self, _a0: Ptr<NoCopy>) -> i32;
    fn copy_assign(&self, _a0: Ptr<NoCopy>) -> Ptr<NoCopy>;
}
impl NoCopyImpl for Ptr<NoCopy> {
    fn operator_call(&self, _a0: Ptr<NoCopy>) -> i32 {
        return 1;
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
    fn non_const(&self) -> i32;
}
impl NonConstImpl for Ptr<NonConst> {
    fn non_const(&self) -> i32 {
        return 1;
    }
}
pub trait RVCRefQualImpl {
    fn operator_call_4(&self, _a0: i32) -> i32;
}
impl RVCRefQualImpl for Ptr<RVCRefQual> {
    fn operator_call_4(&self, mut _a0: i32) -> i32 {
        return 1;
    }
}
pub trait RVRefQualImpl {
    fn operator_call_3(&self, _a0: i32) -> i32;
}
impl RVRefQualImpl for Ptr<RVRefQual> {
    fn operator_call_3(&self, mut _a0: i32) -> i32 {
        return 1;
    }
}
pub trait RValImpl {
    fn operator_call_3(&self, _a0: Ptr<i32>) -> i32;
}
impl RValImpl for Ptr<RVal> {
    fn operator_call_3(&self, _a0: Ptr<i32>) -> i32 {
        return 1;
    }
}
pub trait RefQualImpl {
    fn operator_call_1(&self, _a0: i32) -> i32;
}
impl RefQualImpl for Ptr<RefQual> {
    fn operator_call_1(&self, mut _a0: i32) -> i32 {
        return 1;
    }
}
pub fn __cpp2rust_init_globals() {}
