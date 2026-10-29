extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn compare_paths_0(p1: Ptr<()>, p2: Ptr<()>) {
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
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    if ({ unimplemented!() } != { unimplemented!() }) {
        {
            unimplemented!();
            &(unimplemented!())
        };
    }
    let mut d1: () = unimplemented!();
    let mut d2: () = unimplemented!();
    if (d1 != d2) {
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
thread_local!(
    pub static test_paths_1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
    ])));
);
pub fn root_path_2() -> () {
    return unimplemented!();
}
pub fn nonexistent_path_3(file: Option<()>) -> () {
    let file: Value<()> = Rc::new(RefCell::new(file.unwrap_or_else(|| unimplemented!())));
    if (unimplemented!() > 3_()) && (unimplemented!() == 0_()) {
        unimplemented!();
    }
    let mut pos: () = unimplemented!();
    if (pos != npos_4.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut fd: () = unimplemented!();
    if (fd == -1_()) {
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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Destructor)]
#[byte_size(40)]
pub struct __gnu_test_scoped_file {
    #[offset(0)]
    #[byte_size(40)]
    pub path: (),
}
impl __gnu_test_scoped_file {
    pub fn new_1(p: Option<Ptr<()>>) -> Self {
        let mut __p_default: Option<Value<()>> = None;
        let p: Ptr<()> = p.unwrap_or_else(|| {
            __p_default
                .insert(Rc::new(RefCell::new(({ nonexistent_path_3(None) }))))
                .as_pointer()
        });
        let __this: __gnu_test_scoped_file = Self {
            path: unimplemented!(),
        };
        unimplemented!();
        __this
    }
    pub fn new_2(p: (), mut _a1: __gnu_test_scoped_file_adopt_file_t) -> Self {
        let p: Value<()> = Rc::new(RefCell::new(p));
        Self {
            path: unimplemented!(),
        }
    }
    pub fn move_from(_a0: Ptr<__gnu_test_scoped_file>) -> Self {
        Self {
            path: unimplemented!(),
        }
    }
}
impl Default for __gnu_test_scoped_file {
    fn default() -> Self {
        { __gnu_test_scoped_file::new_1(None) }
    }
}
pub fn permissions_are_testable_6(print_msg: Option<()>) -> () {
    let mut print_msg: () = print_msg.unwrap_or_else(|| true);
    let testable: Value<()> = Rc::new(RefCell::new(false));
    if (unimplemented!() != 0_()) {
        (*testable.borrow_mut()) = true;
    }
    if (print_msg) && (!(*testable.borrow())) {
        unimplemented!();
    }
    return (*testable.borrow());
}
thread_local!(
    static counter_5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn test01_7() {
    'loop_: for p in IterRange::new(
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<()>),
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<()>).offset((18_()) as isize),
    ) {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p.as_pointer(), copy.as_pointer()) });
    }
}
pub fn test02_8() {
    'loop_: for p in IterRange::new(
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<()>),
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<()>).offset((18_()) as isize),
    ) {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let move_: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p.as_pointer(), move_.as_pointer()) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_7() });
    ({ test02_8() });
    0
}
pub trait __gnu_test_scoped_fileImpl {
    fn destructor(&self);
    fn move_assign(&self, _a0: Ptr<__gnu_test_scoped_file>) -> Ptr<__gnu_test_scoped_file>;
}
impl __gnu_test_scoped_fileImpl for Ptr<__gnu_test_scoped_file> {
    fn destructor(&self) {
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    fn move_assign(&self, _a0: Ptr<__gnu_test_scoped_file>) -> Ptr<__gnu_test_scoped_file> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = test_paths_1.with(|_| ());
}
