extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NoCopy {
    pub v: (),
}
impl NoCopy {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn move_from(o: *mut NoCopy) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, o: *mut NoCopy) -> *mut NoCopy {
        self.v = (*o).v;
        (*o).v = 0_();
        return &mut (*(self as *mut NoCopy));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct PrivateCopy {
    pub v: (),
}
impl PrivateCopy {
    pub unsafe fn new() -> Self {
        let mut this = Self { v: 0_() };
        this
    }
    pub unsafe fn move_from(o: *mut PrivateCopy) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, o: *mut PrivateCopy) -> *mut PrivateCopy {
        self.v = (*o).v;
        (*o).v = 0_();
        return &mut (*(self as *mut PrivateCopy));
    }
}
impl Default for PrivateCopy {
    fn default() -> Self {
        unsafe { PrivateCopy::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct Immovable {
    pub v: (),
}
impl Immovable {
    pub unsafe fn new() -> Self {
        let mut this = Self { v: 0_() };
        this
    }
}
impl Default for Immovable {
    fn default() -> Self {
        unsafe { Immovable::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Container {
    pub inner: NoCopy,
    pub tag: (),
}
impl Container {
    pub unsafe fn move_from(_a0: *mut Container) -> Self {
        let mut this = Self {
            inner: NoCopy::move_from({ &mut (*_a0).inner }),
            tag: (*_a0).tag,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Container) -> *mut Container {
        (unsafe {
            let _o: *mut NoCopy = &mut (*_a0).inner;
            NoCopy::move_assign(&mut self.inner, _o)
        });
        self.tag = (*_a0).tag;
        return &mut (*(self as *mut Container));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct VecHolder {
    pub items: (),
}
impl VecHolder {
    pub unsafe fn move_from(_a0: *mut VecHolder) -> Self {
        let mut this = Self {
            items: unimplemented!(),
        };
        this
    }
}
pub unsafe fn bump_0(mut p: *mut NoCopy) {
    (*p).v.postfix_inc();
}
pub unsafe fn bump_ref_1(r: *mut Immovable) {
    (*r).v.postfix_inc();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: NoCopy = NoCopy::new({ 1_() });
    let mut b: NoCopy = NoCopy::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    (unsafe { NoCopy::move_assign(&mut a, &mut (*unimplemented!())) });
    unimplemented!();
    (unsafe { bump_0((&mut a as *mut NoCopy)) });
    unimplemented!();
    let mut p: PrivateCopy = PrivateCopy::new();
    p.v = 3_();
    let mut q: PrivateCopy = PrivateCopy::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    (unsafe { PrivateCopy::move_assign(&mut p, &mut (*unimplemented!())) });
    unimplemented!();
    let mut im: Immovable = Immovable::new();
    im.v = 4_();
    (unsafe { bump_ref_1(&mut im) });
    let mut pim: *mut Immovable = (&mut im as *mut Immovable);
    unimplemented!();
    let mut c: Container = Container {
        inner: NoCopy::new({ 6_() }),
        tag: 7_(),
    };
    let mut d: Container = Container::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    let mut h: VecHolder = <VecHolder>::default();
    let mut h2: VecHolder = VecHolder::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
