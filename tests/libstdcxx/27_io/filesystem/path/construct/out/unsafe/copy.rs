extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn compare_paths_0(p1: *const (), p2: *const ()) {
    if !(unimplemented!()) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ((unimplemented!()) != (unimplemented!())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    let mut d1: () = unimplemented!();
    let mut d2: () = unimplemented!();
    if ((d1) != (d2)) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
}
pub static mut test_paths_1: std::cell::LazyCell<[(); 18]> = std::cell::LazyCell::new(|| unsafe {
    [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ]
});
pub unsafe fn root_path_2() -> () {
    return unimplemented!();
}
pub unsafe fn nonexistent_path_3(mut file: Option<()>) -> () {
    let mut file: () = file.unwrap_or_else(|| unsafe { unimplemented!() });
    if ((unimplemented!()) > (3_())) && ((unimplemented!()) == (0_())) {
        unimplemented!();
    }
    let mut pos: () = unimplemented!();
    if ((pos) != (unimplemented!())) {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: () = unimplemented!();
    unimplemented!();
    let mut fd: () = unimplemented!();
    if ((fd) == (-1_())) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub type __gnu_test_scoped_file_adopt_file_t = ();
pub const __gnu_test_scoped_file_adopt_file_t_adopt_file: __gnu_test_scoped_file_adopt_file_t = 0;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_scoped_file {
    pub path: (),
}
impl __gnu_test_scoped_file {
    pub unsafe fn new_1(p: Option<*const ()>) -> Self {
        let mut __p_default: Option<()> = None;
        let mut p: *const () = p.unwrap_or_else(|| unsafe {
            __p_default.insert((unsafe { nonexistent_path_3(None) })) as *const ()
        });
        let mut this = Self {
            path: unimplemented!(),
        };
        unimplemented!();
        this
    }
    pub unsafe fn new_2(mut p: (), mut _a1: __gnu_test_scoped_file_adopt_file_t) -> Self {
        let mut this = Self {
            path: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    pub unsafe fn move_from(_a0: *mut __gnu_test_scoped_file) -> Self {
        let mut this = Self {
            path: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_test_scoped_file,
    ) -> *mut __gnu_test_scoped_file {
        unimplemented!();
        return &mut (*(self as *mut __gnu_test_scoped_file));
    }
}
impl Default for __gnu_test_scoped_file {
    fn default() -> Self {
        unsafe { __gnu_test_scoped_file::new_1(None) }
    }
}
pub unsafe fn permissions_are_testable_5(mut print_msg: Option<()>) -> () {
    let mut print_msg: () = print_msg.unwrap_or_else(|| unsafe { true });
    let mut testable: () = false;
    if ((unimplemented!()) != (0_())) {
        testable = true;
    }
    if (print_msg) && (!(testable)) {
        unimplemented!();
    }
    return testable;
}
static mut counter_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn test01_6() {
    'loop_: for p in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_paths_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_paths_1))
            .as_ptr()
            .offset((18_()) as isize),
    ) {
        let p: () = unimplemented!();
        let mut copy: () = unimplemented!();
        (unsafe { compare_paths_0(&p, &copy) });
    }
}
pub unsafe fn test02_7() {
    'loop_: for p in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_paths_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_paths_1))
            .as_ptr()
            .offset((18_()) as isize),
    ) {
        let p: () = unimplemented!();
        let mut copy: () = unimplemented!();
        let mut move_: () = unimplemented!();
        (unsafe { compare_paths_0(&p, &move_) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_6() });
    (unsafe { test02_7() });
    0
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_paths_1);
}
