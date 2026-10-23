extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct CopyOnly {}
impl CopyOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const CopyOnly) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const CopyOnly) -> *mut CopyOnly {
        return &mut (*(self as *mut CopyOnly));
    }
}
impl Clone for CopyOnly {
    fn clone(&self) -> Self {
        unsafe { CopyOnly::copy_from(self as *const CopyOnly) }
    }
}
impl Default for CopyOnly {
    fn default() -> Self {
        unsafe { CopyOnly::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct NoexceptMoveOnly {}
impl NoexceptMoveOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_from(_a0: *mut NoexceptMoveOnly) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NoexceptMoveOnly) -> *mut NoexceptMoveOnly {
        return &mut (*(self as *mut NoexceptMoveOnly));
    }
}
impl Default for NoexceptMoveOnly {
    fn default() -> Self {
        unsafe { NoexceptMoveOnly::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe)]
pub struct NotMoveConstructible {}
impl NotMoveConstructible {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut NotMoveConstructible,
    ) -> *mut NotMoveConstructible {
        return &mut (*(self as *mut NotMoveConstructible));
    }
}
impl Default for NotMoveConstructible {
    fn default() -> Self {
        unsafe { NotMoveConstructible::new() }
    }
}
pub unsafe fn can_swap_0() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut value_1));
}
pub unsafe fn can_swap_2() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut value_1));
}
pub unsafe fn can_swap_3() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut value_4));
}
pub unsafe fn test_unique_ptr_5() {
    let mut i: [(); 3] = std::array::from_fn::<_, 3, _>(|_| unimplemented!());
    let mut k: () = 0_();
    'loop_: while ((k) < (3_())) {
        unimplemented!();
        k.prefix_inc();
    }
    let mut j: [(); 3] = std::array::from_fn::<_, 3, _>(|_| unimplemented!());
    let mut k: () = 0_();
    'loop_: while ((k) < (3_())) {
        unimplemented!();
        k.prefix_inc();
    }
    (unsafe { swap_6(&mut i, &mut j) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_7() -> () {
    {
        let mut i: [(); 3] = [1_(), 2_(), 3_()];
        let mut j: [(); 3] = [4_(), 5_(), 6_()];
        (unsafe { swap_8(&mut i, &mut j) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [[(); 2]; 2] = [[0_(), 1_()], [2_(), 3_()]];
        let mut b: [[(); 2]; 2] = [[9_(), 8_()], [7_(), 6_()]];
        (unsafe { swap_9(&mut a, &mut b) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [[(); 3]; 3] = [[0_(), 1_(), 2_()], [3_(), 4_(), 5_()], [6_(), 7_(), 8_()]];
        let mut b: [[(); 3]; 3] = [[9_(), 8_(), 7_()], [6_(), 5_(), 4_()], [3_(), 2_(), 1_()]];
        (unsafe { swap_10(&mut a, &mut b) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "can_swap<CA&>()");;
        const _: () = assert!(true, "can_swap<MA&>()");;
        const _: () = assert!(true, "!can_swap<NA&>()");;
        let mut ca: [CopyOnly; 42] = std::array::from_fn::<_, 42, _>(|_| CopyOnly::new());
        let mut ma: [NoexceptMoveOnly; 42] =
            std::array::from_fn::<_, 42, _>(|_| NoexceptMoveOnly::new());
        const _: () = assert!(true, "!noexcept(std::swap(ca, ca))");;
        const _: () = assert!(true, "noexcept(std::swap(ma, ma))");;
    }
    {
        (unsafe { test_unique_ptr_5() });
    }
    return true;
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_7() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
