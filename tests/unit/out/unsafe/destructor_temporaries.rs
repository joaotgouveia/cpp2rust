extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut destroyed_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut last_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct S {
    pub v: (),
}
impl S {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)).postfix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) = self.v;
    }
    pub unsafe fn get(&self) -> () {
        return self.v;
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Outer {
    pub inner: S,
    pub w: (),
}
impl Outer {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.inner);
    }
}
pub unsafe fn make_2(mut v: ()) -> S {
    return S { v: v };
}
pub unsafe fn make_outer_3() -> Outer {
    return Outer {
        inner: S { v: 7_() },
        w: 8_(),
    };
}
pub unsafe fn use_4(s: *const S) -> () {
    unimplemented!();
    return (*s).v;
}
pub unsafe fn value_5(s: *const S) -> () {
    return (*s).v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { make_2(1_()) });
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    &(unsafe { make_2(2_()) });
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    let mut x: () = (unsafe { make_2(3_()) }).v;
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    x = (unsafe { S::get(&(unsafe { make_2(4_()) })) });
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    x = (unsafe { make_outer_3() }).inner.v;
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    x = S { v: 5_() }.v;
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    x = (unsafe {
        let mut _s: S = (unsafe { make_2(6_()) });
        use_4(&mut _s)
    });
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    x = ((unsafe {
        let mut _s: S = S { v: 1_() };
        value_5(&mut _s)
    }) + (unsafe {
        let mut _s: S = S { v: 2_() };
        value_5(&mut _s)
    }));
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    x = (if ((x) == (3_())) {
        (unsafe { make_2(9_()) })
    } else {
        S { v: 10_() }
    })
    .v;
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    {
        let mut __tmp_0: S = (unsafe { make_2(11_()) });
        let r: *const S = &mut __tmp_0;
        unimplemented!();
    }
    unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0_();
    {
        let mut s: S = S { v: 12_() };
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
        s = (unsafe { make_2(13_()) });
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destroyed_0);
    std::cell::LazyCell::force(&*&raw const last_1);
}
