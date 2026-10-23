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
pub struct SafePointer {
    pub ptr: (),
}
impl SafePointer {
    pub unsafe fn inc(&mut self) {
        (*unimplemented!()).prefix_inc();
    }
    pub unsafe fn move_from(_a0: *mut SafePointer) -> Self {
        let mut this = Self {
            ptr: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut SafePointer) -> *mut SafePointer {
        unimplemented!();
        return &mut (*(self as *mut SafePointer));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub x: (),
    pub y: (),
}
impl Pair {
    pub unsafe fn inc(&mut self, mut k: ()) {
        self.x += k;
        self.y += k;
    }
}
pub unsafe fn DoStuffWithSafePointer_0(safe_ptr: *mut ()) {
    let mut x1: () = unimplemented!();
    let mut x2: () = unimplemented!();
    (*unimplemented!()) = 1_();
    unimplemented!();
    let mut raw_ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
    (*raw_ptr1).prefix_inc();
    unimplemented!();
    (unsafe { SafePointer::inc(&mut (*unimplemented!())) });
    (unsafe { SafePointer::inc(&mut (*unimplemented!())) });
    let mut x3: () = unimplemented!();
    let mut x4: () = unimplemented!();
    (*unimplemented!()) = ((*unimplemented!()) + (*unimplemented!()));
    unimplemented!();
    let mut raw_ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
    (*raw_ptr2) += 1_();
    let mut pair: () = unimplemented!();
    (unsafe { Pair::inc(&mut (*unimplemented!()), 10_()) });
    (*unimplemented!()) =
        (((*unimplemented!()) + ((*unimplemented!()).x)) + ((*unimplemented!()).y));
}
pub unsafe fn Consume_1(mut safe_ptr: ()) -> () {
    let mut x: () = unimplemented!();
    let mut p: () = std_unique_ptr_Pair__std_default_delete_Pair__::new({
        (Box::leak(Box::new(<Pair>::default())) as *mut Pair)
    });
    return ((*unimplemented!()) + ((*unimplemented!()).x));
}
pub unsafe fn RndStuff_2() {
    let mut x1: () = unimplemented!();
    let mut x2: () = std_unique_ptr_intarrarr__std_default_delete_intarrarr__::new_1({
        Box::leak((0..100_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr()
    });
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        (*unimplemented!()) = 1_();
        i.prefix_inc();
    }
    (unsafe {
        x2.reset(
            Box::leak((0..200_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr(),
        )
    });
    let mut i: () = 0_();
    'loop_: while ((i) < (200_())) {
        (*unimplemented!()) = 2_();
        i.prefix_inc();
    }
    let mut p2: *mut () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (200_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut x3: () = (unsafe { make_unique_3(10_()) });
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        (*unimplemented!()) = Pair { x: 1_(), y: 2_() };
        i.prefix_inc();
    }
    let mut p3_0: *mut Pair = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        unimplemented!();
        unimplemented!();
        (unsafe { Pair::inc(&mut (*unimplemented!()), 10_()) });
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    (unsafe {
        x3.reset(
            Box::leak(
                (0..50_())
                    .map(|_| <Pair>::default())
                    .collect::<Box<[Pair]>>(),
            )
            .as_mut_ptr(),
        )
    });
    let mut i: () = 0_();
    'loop_: while ((i) < (50_())) {
        (*unimplemented!()) = Pair { x: -1_(), y: -2_() };
        i.prefix_inc();
    }
    let mut p3_1: *mut Pair = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (50_())) {
        unimplemented!();
        unimplemented!();
        (unsafe { Pair::inc(&mut (*unimplemented!()), -10_()) });
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = unimplemented!();
    let mut safe_ptr: () = unimplemented!();
    (unsafe { DoStuffWithSafePointer_0(&mut safe_ptr) });
    unimplemented!();
    let mut p: () = std_unique_ptr_int__std_default_delete_int__::new({
        (Box::leak(Box::new(1_())) as *mut ())
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
