extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_contiguous_container_asan_correct_0(_a0: *const Vec<CMyClass>) -> bool {
    return true;
}
pub static mut gCopyConstructorShouldThrow_1: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct CMyClass {
    fMagicValue: i32,
    fTag: i32,
}
impl CMyClass {
    pub unsafe fn new(mut tag: i32) -> Self {
        let mut this = Self {
            fMagicValue: (*std::cell::LazyCell::force_mut(
                &mut *&raw mut kStartedConstructionMagicValue_2,
            )),
            fTag: tag,
        };
        this.fMagicValue =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut kFinishedConstructionMagicValue_3));
        this
    }
    pub unsafe fn copy_from(iOther: *const CMyClass) -> Self {
        let mut this = Self {
            fMagicValue: (*std::cell::LazyCell::force_mut(
                &mut *&raw mut kStartedConstructionMagicValue_2,
            )),
            fTag: (*iOther).fTag,
        };
        if (*std::cell::LazyCell::force_mut(&mut *&raw mut gCopyConstructorShouldThrow_1)) {
            std::process::abort();
        }
        this.fMagicValue =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut kFinishedConstructionMagicValue_3));
        this
    }
    pub unsafe fn equal(&self, rhs: *const CMyClass) -> bool {
        return ((self.fTag) == ((*rhs).fTag)) && ((self.fMagicValue) == ((*rhs).fMagicValue));
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const CMyClass) -> *mut CMyClass {
        self.fMagicValue = (*_a0).fMagicValue;
        self.fTag = (*_a0).fTag;
        return &mut (*(self as *mut CMyClass));
    }
}
impl std::cmp::PartialEq for CMyClass {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self as *const CMyClass, other as *const CMyClass) }
    }
}
impl std::cmp::Eq for CMyClass {}
impl Clone for CMyClass {
    fn clone(&self) -> Self {
        unsafe { CMyClass::copy_from(self as *const CMyClass) }
    }
}
static mut kStartedConstructionMagicValue_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
static mut kFinishedConstructionMagicValue_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 12345 });
impl CMyClass {}
impl CMyClass {}
impl CMyClass {
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.fMagicValue)
                == (*std::cell::LazyCell::force_mut(
                    &mut *&raw mut kFinishedConstructionMagicValue_3
                )))
        );
    }
}
pub unsafe fn operator_eq_4(lhs: *const CMyClass, rhs: *const CMyClass) -> bool {
    return (unsafe {
        let _rhs: *const CMyClass = rhs;
        CMyClass::equal(&(*lhs), _rhs)
    });
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    let mut instance: CMyClass = CMyClass::new({ 42 });
    let _dtor_instance = ScopedDestructorUnsafe::new(&raw mut instance, CMyClass::destructor);
    let mut vec_: Vec<CMyClass> = Vec::new();
    {
        let a0_clone = instance.clone();
        vec_.push(a0_clone)
    };
    let mut vec2: Vec<CMyClass> = vec_.clone();
    assert!((unsafe { is_contiguous_container_asan_correct_0(&vec_,) }));
    assert!((unsafe { is_contiguous_container_asan_correct_0(&vec2,) }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const gCopyConstructorShouldThrow_1);
    std::cell::LazyCell::force(&*&raw const kStartedConstructionMagicValue_2);
    std::cell::LazyCell::force(&*&raw const kFinishedConstructionMagicValue_3);
}
