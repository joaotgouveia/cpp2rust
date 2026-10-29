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
    pub ptr: std_unique_ptr_int__std_default_delete_int__,
}
impl SafePointer {
    pub unsafe fn inc(&mut self) {
        (*self.ptr).prefix_inc();
    }
    pub unsafe fn move_from(_a0: *mut SafePointer) -> Self {
        let mut this = Self { ptr: (*_a0).ptr };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut SafePointer) -> *mut SafePointer {
        self.ptr = ((*_a0).ptr).clone();
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
pub unsafe fn DoStuffWithSafePointer_0(
    safe_ptr: *mut std_unique_ptr_SafePointer__std_default_delete_SafePointer__,
) {
    let mut x1: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 0_();
        make_unique_1(&mut ___args)
    });
    let mut x2: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 0_();
        make_unique_1(&mut ___args)
    });
    (*x2) = 1_();
    x1 = (x2).clone();
    let mut raw_ptr1: *mut () = (&mut (*x1) as *mut ());
    (*raw_ptr1).prefix_inc();
    (*(*safe_ptr)).ptr = (x1).clone();
    (unsafe { SafePointer::inc(&mut (*(*safe_ptr))) });
    (unsafe { SafePointer::inc(&mut (*(*safe_ptr))) });
    let mut x3: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 10_();
        make_unique_1(&mut ___args)
    });
    let mut x4: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 20_();
        make_unique_1(&mut ___args)
    });
    (*x3) = ((*x3) + (*x4));
    x4 = (x3).clone();
    let mut raw_ptr2: *mut () = (&mut (*x4) as *mut ());
    (*raw_ptr2) += 1_();
    let mut pair: std_unique_ptr_Pair__std_default_delete_Pair__ = (unsafe {
        let mut ___args: Pair = Pair {
            x: (*raw_ptr2),
            y: 5_(),
        };
        make_unique_2(&mut ___args)
    });
    (unsafe { Pair::inc(&mut (*pair), 10_()) });
    (*(*(*safe_ptr)).ptr) = (((*(*(*safe_ptr)).ptr) + ((*pair).x)) + ((*pair).y));
}
pub unsafe fn Consume_3(
    mut safe_ptr: std_unique_ptr_SafePointer__std_default_delete_SafePointer__,
) -> () {
    let mut x: std_unique_ptr_SafePointer__std_default_delete_SafePointer__ = safe_ptr;
    let mut p: std_unique_ptr_Pair__std_default_delete_Pair__ =
        std_unique_ptr_Pair__std_default_delete_Pair__::new_1({
            (Box::leak(Box::new(<Pair>::default())) as *mut Pair)
        });
    return ((*(*x).ptr) + ((*p).x));
}
pub unsafe fn RndStuff_4() {
    let mut x1: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        <std_unique_ptr_intarrarr__std_default_delete_intarrarr__>::default();
    let mut x2: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        std_unique_ptr_intarrarr__std_default_delete_intarrarr__::new_1({
            Box::leak((0..100_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr()
        });
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        x2[(i) as usize] = 1_();
        i.prefix_inc();
    }
    (unsafe {
        x2.reset(
            Box::leak((0..200_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr(),
        )
    });
    let mut i: () = 0_();
    'loop_: while ((i) < (200_())) {
        x2[(i) as usize] = 2_();
        i.prefix_inc();
    }
    let mut p2: *mut () = (unsafe { x2.get() });
    let mut i: () = 0_();
    'loop_: while ((i) < (200_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut x3: std_unique_ptr_Pairarrarr__std_default_delete_Pairarrarr__ =
        (unsafe { make_unique_5(10_()) });
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        x3[(i) as usize] = Pair { x: 1_(), y: 2_() };
        i.prefix_inc();
    }
    let mut p3_0: *mut Pair = (unsafe { x3.get() });
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        unimplemented!();
        unimplemented!();
        (unsafe { Pair::inc(&mut x3[(i) as usize], 10_()) });
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
        x3[(i) as usize] = Pair { x: -1_(), y: -2_() };
        i.prefix_inc();
    }
    let mut p3_1: *mut Pair = (unsafe { x3.get() });
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (50_())) {
        unimplemented!();
        unimplemented!();
        (unsafe { Pair::inc(&mut x3[(i) as usize], -10_()) });
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn unique_ptr_that_wraps_container_6() {
    let mut v : std_unique_ptr_std_basic_string_char__std_char_traits_char___std_allocator_char__arrarr__std_default_delete_std_basic_string_char__std_char_traits_char___std_allocator_char__arrarr__ = ( unsafe { make_unique_7 ( 2_() , ) } )  ;
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: std_unique_ptr_int__std_default_delete_int__ = (unsafe {
        let mut ___args: () = 0_();
        make_unique_1(&mut ___args)
    });
    let mut safe_ptr: std_unique_ptr_SafePointer__std_default_delete_SafePointer__ = (unsafe {
        let mut ___args: SafePointer = SafePointer { ptr: x };
        make_unique_8(&mut ___args)
    });
    (unsafe { DoStuffWithSafePointer_0(&mut safe_ptr) });
    unimplemented!();
    let mut p: std_unique_ptr_int__std_default_delete_int__ =
        std_unique_ptr_int__std_default_delete_int__::new_1({
            (Box::leak(Box::new(1_())) as *mut ())
        });
    (unsafe { p.reset(Some((Box::leak(Box::new(2_())) as *mut ()))) });
    unimplemented!();
    (unsafe { p.reset(None) });
    unimplemented!();
    (unsafe { unique_ptr_that_wraps_container_6() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
