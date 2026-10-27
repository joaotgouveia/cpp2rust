extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_contiguous_container_asan_correct_0(_a0: Ptr<()>) -> () {
    return true;
}
thread_local!(
    pub static gCopyConstructorShouldThrow_1: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(8)]
pub struct CMyClass {
    #[offset(0)]
    fMagicValue: (),
    #[offset(4)]
    fTag: (),
}
impl CMyClass {
    pub fn new(mut tag: ()) -> Self {
        let __this: Value<CMyClass> = Rc::new(RefCell::new(Self {
            fMagicValue: kStartedConstructionMagicValue_2.with(|rc| *rc.borrow()),
            fTag: tag,
        }));
        let this: Ptr<CMyClass> = __this.as_pointer();
        field!(this, fMagicValue).write(kFinishedConstructionMagicValue_3.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(iOther: Ptr<CMyClass>) -> Self {
        let __this: Value<CMyClass> = Rc::new(RefCell::new(Self {
            fMagicValue: kStartedConstructionMagicValue_2.with(|rc| *rc.borrow()),
            fTag: iOther.with(|__s| __s.fTag),
        }));
        let this: Ptr<CMyClass> = __this.as_pointer();
        if gCopyConstructorShouldThrow_1.with(|rc| *rc.borrow()) {
            unimplemented!();
        }
        field!(this, fMagicValue).write(kFinishedConstructionMagicValue_3.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for CMyClass {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_4(
                Rc::new(RefCell::new(CMyClass {
                    fMagicValue: self.fMagicValue.clone(),
                    fTag: self.fTag.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(CMyClass {
                    fMagicValue: other.fMagicValue.clone(),
                    fTag: other.fTag.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for CMyClass {}
impl Clone for CMyClass {
    fn clone(&self) -> Self {
        let __src: Value<CMyClass> = Rc::new(RefCell::new(CMyClass {
            fMagicValue: self.fMagicValue.clone(),
            fTag: self.fTag.clone(),
        }));
        CMyClass::copy_from(__src.as_pointer())
    }
}
thread_local!(
    static kStartedConstructionMagicValue_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static kFinishedConstructionMagicValue_3: Value<()> = Rc::new(RefCell::new(12345_()));
);
impl CMyClass {}
impl CMyClass {}
pub fn operator_eq_4(lhs: Ptr<CMyClass>, rhs: Ptr<CMyClass>) -> () {
    return ({
        let _rhs: Ptr<CMyClass> = (rhs).clone();
        CMyClassImpl::equal(&lhs, _rhs)
    });
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
    let instance: Value<CMyClass> = Rc::new(RefCell::new(CMyClass::new({ 42_() })));
    let _dtor_instance = ScopedDestructor::new(&instance, |__p| __p.destructor());
    let vec_: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let vec2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait CMyClassImpl {
    fn destructor(&self);
    fn equal(&self, rhs: Ptr<CMyClass>) -> ();
    fn copy_assign(&self, _a0: Ptr<CMyClass>) -> Ptr<CMyClass>;
}
impl CMyClassImpl for Ptr<CMyClass> {
    fn equal(&self, rhs: Ptr<CMyClass>) -> () {
        return ({ (*self).with(|__s| __s.fTag) } == { rhs.with(|__s| __s.fTag) })
            && ({ (*self).with(|__s| __s.fMagicValue) } == { rhs.with(|__s| __s.fMagicValue) });
    }
    fn copy_assign(&self, _a0: Ptr<CMyClass>) -> Ptr<CMyClass> {
        field!((*self), fMagicValue).write({ _a0.with(|__s| __s.fMagicValue) });
        field!((*self), fTag).write({ _a0.with(|__s| __s.fTag) });
        return (*self).clone();
    }
    fn destructor(&self) {
        unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = gCopyConstructorShouldThrow_1.with(|_| ());
    let _ = kStartedConstructionMagicValue_2.with(|_| ());
    let _ = kFinishedConstructionMagicValue_3.with(|_| ());
}
