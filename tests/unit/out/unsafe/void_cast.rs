extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn unused_param_0(mut x: ()) {
    &(x);
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NonTrivial {
    pub data: (),
}
impl NonTrivial {
    pub unsafe fn move_from(_a0: *mut NonTrivial) -> Self {
        let mut this = Self {
            data: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NonTrivial) -> *mut NonTrivial {
        unimplemented!();
        return &mut (*(self as *mut NonTrivial));
    }
}
pub unsafe fn unused_ref_param_1(x: *const NonTrivial) {
    &(*x);
}
pub unsafe fn unused_ptr_param_2(mut p: *const NonTrivial) {
    &(*p);
}
pub static mut side_effect_counter_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn bump_and_return_4() -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut side_effect_counter_3)).prefix_inc();
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut side_effect_counter_3));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub field: (),
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NonCopyable {
    pub value: (),
}
impl NonCopyable {
    pub unsafe fn move_from(_a0: *mut NonCopyable) -> Self {
        let mut this = Self {
            value: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NonCopyable) -> *mut NonCopyable {
        unimplemented!();
        return &mut (*(self as *mut NonCopyable));
    }
}
pub unsafe fn unused_noncopyable_param_5(x: *const NonCopyable) {
    &(*x);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { unused_param_0(42_()) });
    let mut y: () = 5_();
    &(y);
    let mut z: () = {
        &(y);
        7_()
    };
    unimplemented!();
    let mut counter: () = 0_();
    let mut w: () = {
        {
            &(counter);
            counter = 3_()
        };
        counter
    };
    unimplemented!();
    unimplemented!();
    &(unsafe { bump_and_return_4() });
    unimplemented!();
    let mut v: () = {
        &(unsafe { bump_and_return_4() });
        99_()
    };
    unimplemented!();
    unimplemented!();
    &(0_());
    &(0_());
    &(y);
    (&(0_()));
    (&(y));
    let mut err: () = 0_();
    (&(err = 42_()));
    unimplemented!();
    let mut chosen: () = {
        &(err = 7_());
        123_()
    };
    unimplemented!();
    unimplemented!();
    &(bump_and_return_4);
    unimplemented!();
    &(Some(bump_and_return_4));
    unimplemented!();
    &(std::mem::transmute::<Option<unsafe fn() -> ()>, Option<unsafe fn() -> ()>>(
        (Some(bump_and_return_4)),
    ));
    unimplemented!();
    let mut storage: () = 11_();
    let mut p: *mut () = (&mut storage as *mut ());
    &(*p);
    &(p);
    let mut arr: [(); 3] = [1_(), 2_(), 3_()];
    &(arr[(1_()) as usize]);
    let mut h: Holder = Holder { field: 17_() };
    &(h.field);
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    &((*hp).field);
    let mut nt: NonTrivial = <NonTrivial>::default();
    (unsafe { unused_ref_param_1(&nt) });
    (unsafe { unused_ptr_param_2((&mut nt as *mut NonTrivial).cast_const()) });
    let mut g: NonCopyable = NonCopyable {
        value: unimplemented!(),
    };
    (&(g));
    &(g);
    (unsafe { unused_noncopyable_param_5(&g) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const side_effect_counter_3);
}
