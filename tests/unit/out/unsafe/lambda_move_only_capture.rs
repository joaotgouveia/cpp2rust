extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct Owner {
    pub p: *mut (),
}
impl Owner {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            p: (Box::leak(Box::new(v)) as *mut ()),
        };
        this
    }
    pub unsafe fn move_from(o: *mut Owner) -> Self {
        let mut this = Self { p: (*o).p };
        (*o).p = std::ptr::null_mut();
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
    pub unsafe fn take(&mut self) -> FnPtr<fn() -> ()> {
        return lambda_unsafe!(
            {
                let self_: Owner = Owner::move_from({ &mut (*(self as *mut Owner)) });
            },
            || -> () {
                return (*self_.p);
            }
        );
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut o: Owner = Owner::new({ 5_() });
    let _dtor_o = ScopedDestructorUnsafe::new(&raw mut o, Owner::destructor);
    let mut f: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let h: Owner = Owner::move_from({ &mut o });
        },
        || -> () {
            return (*h.p);
        }
    );
    let _dtor_f = ScopedDestructorUnsafe::new(&raw mut f, |__f| __f.destroy());
    unimplemented!();
    unimplemented!();
    let mut g: () = f.move_from();
    let _dtor_g = ScopedDestructorUnsafe::new(&raw mut g, |__f| __f.destroy());
    unimplemented!();
    let mut total: () = 0_();
    let mut consume: FnPtr<fn()> = lambda_unsafe!(
        {
            let h: Owner = Owner::new({ 7_() });
            let total: *mut () = &mut total;
        },
        || {
            (*total) += (*h.p);
            (*h.p) = 0_();
        }
    );
    let _dtor_consume = ScopedDestructorUnsafe::new(&raw mut consume, |__f| __f.destroy());
    (unsafe { consume.call() });
    (unsafe { consume.call() });
    unimplemented!();
    let mut o2: Owner = Owner::new({ 9_() });
    let _dtor_o2 = ScopedDestructorUnsafe::new(&raw mut o2, Owner::destructor);
    let mut t: FnPtr<fn() -> ()> = (unsafe { Owner::take(&mut o2) });
    let _dtor_t = ScopedDestructorUnsafe::new(&raw mut t, |__f| __f.destroy());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
