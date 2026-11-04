extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static gCopyConstructorShouldThrow_0: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Destructor, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct CMyClass {
    #[offset(0)]
    fMagicValue: i32,
    #[offset(4)]
    fTag: i32,
}
impl CMyClass {
    pub fn new(mut tag: i32) -> Self {
        let __this: Value<CMyClass> = Rc::new(RefCell::new(Self {
            fMagicValue: kStartedConstructionMagicValue_1.with(|rc| *rc.borrow()),
            fTag: tag,
        }));
        let this: Ptr<CMyClass> = __this.as_pointer();
        field!(this, fMagicValue).write(kFinishedConstructionMagicValue_2.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(iOther: Ptr<CMyClass>) -> Self {
        let __this: Value<CMyClass> = Rc::new(RefCell::new(Self {
            fMagicValue: kStartedConstructionMagicValue_1.with(|rc| *rc.borrow()),
            fTag: iOther.with(|__s| __s.fTag),
        }));
        let this: Ptr<CMyClass> = __this.as_pointer();
        if gCopyConstructorShouldThrow_0.with(|rc| *rc.borrow()) {
            std::process::abort();
        }
        field!(this, fMagicValue).write(kFinishedConstructionMagicValue_2.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for CMyClass {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for CMyClass {
    fn clone(&self) -> Self {
        let __src: Value<CMyClass> = Rc::new(RefCell::new(CMyClass {
            fMagicValue: self.fMagicValue.clone(),
            fTag: self.fTag.clone(),
        }));
        CMyClass::copy_from(__src.as_pointer())
    }
}
impl CppMove for CMyClass {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    static kStartedConstructionMagicValue_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static kFinishedConstructionMagicValue_2: Value<i32> = Rc::new(RefCell::new(12345));
);
impl CMyClass {}
impl CMyClass {}
pub fn operator_eq_3(lhs: Ptr<CMyClass>, rhs: Ptr<CMyClass>) -> bool {
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    let instance: Value<CMyClass> = Rc::new(RefCell::new(CMyClass::new({ 42 })));
    let _dtor_instance = ScopedDestructor::new(&instance, |__p| __p.destructor());
    let vec_: Value<Vec<CMyClass>> = Rc::new(RefCell::new(Vec::new()));
    {
        let a0_clone = (*instance.borrow()).clone();
        (*vec_.borrow_mut()).push(a0_clone)
    };
    let vec2: Value<Vec<CMyClass>> = Rc::new(RefCell::new((*vec_.borrow()).clone()));
    assert!(({ is_contiguous_container_asan_correct_4(vec_.as_pointer(),) }));
    assert!(({ is_contiguous_container_asan_correct_4(vec2.as_pointer(),) }));
    return 0;
}
pub trait CMyClassImpl {
    fn destructor(&self);
    fn equal(&self, rhs: Ptr<CMyClass>) -> bool;
    fn copy_assign(&self, _a0: Ptr<CMyClass>) -> Ptr<CMyClass>;
}
impl CMyClassImpl for Ptr<CMyClass> {
    fn equal(&self, rhs: Ptr<CMyClass>) -> bool {
        return ({ (*self).with(|__s| __s.fTag) } == { rhs.with(|__s| __s.fTag) })
            && ({ (*self).with(|__s| __s.fMagicValue) } == { rhs.with(|__s| __s.fMagicValue) });
    }
    fn copy_assign(&self, _a0: Ptr<CMyClass>) -> Ptr<CMyClass> {
        field!((*self), fMagicValue).write({ _a0.with(|__s| __s.fMagicValue) });
        field!((*self), fTag).write({ _a0.with(|__s| __s.fTag) });
        return (*self).clone();
    }
    fn destructor(&self) {
        assert!(
            ((*self).with(|__s| __s.fMagicValue)
                == kFinishedConstructionMagicValue_2.with(|rc| *rc.borrow()))
        );
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = gCopyConstructorShouldThrow_0.with(|_| ());
    let _ = kStartedConstructionMagicValue_1.with(|_| ());
    let _ = kFinishedConstructionMagicValue_2.with(|_| ());
}
