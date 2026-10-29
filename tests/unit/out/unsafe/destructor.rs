extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut global_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct S {}
impl S {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)).postfix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Defaulted {
    pub s: S,
}
impl Defaulted {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Middle {
    pub s: S,
}
impl Middle {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Outer {
    pub m: Middle,
}
impl Outer {
    pub unsafe fn destructor(&mut self) {
        Middle::destructor(&mut self.m);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct ArrayMember {
    pub items: [S; 3],
}
impl ArrayMember {
    pub unsafe fn destructor(&mut self) {
        for __e in self.items.iter_mut() {
            S::destructor(__e);
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct EmptyBody {
    pub s: S,
}
impl EmptyBody {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Templated_char_ {
    pub v: (),
}
impl Templated_char_ {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0))
                .wrapping_add((::std::mem::size_of::<()>() as ()));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Templated_int_ {
    pub v: (),
}
impl Templated_int_ {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0))
                .wrapping_add((::std::mem::size_of::<()>() as ()));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Copied {
    pub v: (),
}
impl Copied {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)).postfix_inc();
    }
}
pub static mut order_1: std::cell::LazyCell<[(); 3]> =
    std::cell::LazyCell::new(|| unsafe { [unimplemented!(); 3] });
pub static mut order_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Tagged {
    pub tag: (),
}
impl Tagged {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut order_1))
            [((*std::cell::LazyCell::force_mut(&mut *&raw mut order_count_2)).postfix_inc())
                as usize] = self.tag;
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Ordered {
    pub first: Tagged,
    pub dummy1: (),
    pub second: Tagged,
    pub dummy2: (),
    pub third: Tagged,
}
impl Ordered {
    pub unsafe fn destructor(&mut self) {
        Tagged::destructor(&mut self.third);
        Tagged::destructor(&mut self.second);
        Tagged::destructor(&mut self.first);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    {
        let mut s: S = <S>::default();
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
    }
    unimplemented!();
    {
        let mut s: S = <S>::default();
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
    }
    unimplemented!();
    {
        let mut d: Defaulted = <Defaulted>::default();
        let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Defaulted::destructor);
    }
    unimplemented!();
    {
        let mut o: Outer = <Outer>::default();
        let _dtor_o = ScopedDestructorUnsafe::new(&raw mut o, Outer::destructor);
    }
    unimplemented!();
    {
        let mut am: ArrayMember = <ArrayMember>::default();
        let _dtor_am = ScopedDestructorUnsafe::new(&raw mut am, ArrayMember::destructor);
    }
    unimplemented!();
    {
        let mut e: EmptyBody = <EmptyBody>::default();
        let _dtor_e = ScopedDestructorUnsafe::new(&raw mut e, EmptyBody::destructor);
    }
    unimplemented!();
    {
        let mut tc: Templated_char_ = <Templated_char_>::default();
        let _dtor_tc = ScopedDestructorUnsafe::new(&raw mut tc, Templated_char_::destructor);
        let mut ti: Templated_int_ = <Templated_int_>::default();
        let _dtor_ti = ScopedDestructorUnsafe::new(&raw mut ti, Templated_int_::destructor);
    }
    unimplemented!();
    {
        let mut a: Copied = Copied { v: 5_() };
        let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Copied::destructor);
        let mut b: Copied = a.clone();
        let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Copied::destructor);
        unimplemented!();
    }
    unimplemented!();
    {
        let mut o: Ordered = Ordered {
            first: Tagged { tag: 1_() },
            dummy1: 0_(),
            second: Tagged { tag: 2_() },
            dummy2: 0_(),
            third: Tagged { tag: 3_() },
        };
        let _dtor_o = ScopedDestructorUnsafe::new(&raw mut o, Ordered::destructor);
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut before: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0));
    {
        let mut cs: S = <S>::default();
        let _dtor_cs = ScopedDestructorUnsafe::new(&raw mut cs, S::destructor);
        &(cs);
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_0);
    std::cell::LazyCell::force(&*&raw const order_1);
    std::cell::LazyCell::force(&*&raw const order_count_2);
}
