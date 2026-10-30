extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct NonCopy {
    pub data: (),
    pub tag: (),
}
impl NonCopy {
    pub unsafe fn move_from(_a0: *mut NonCopy) -> Self {
        let mut this = Self {
            data: unimplemented!(),
            tag: (*_a0).tag,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NonCopy) -> *mut NonCopy {
        unimplemented!();
        self.tag = (*_a0).tag;
        return &mut (*(self as *mut NonCopy));
    }
}
impl Default for NonCopy {
    fn default() -> Self {
        unsafe {
            NonCopy {
                data: unimplemented!(),
                tag: 0_(),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut arr: [NonCopy; 3] = std::array::from_fn::<_, 3, _>(|_| <NonCopy>::default());
    arr[(0_()) as usize].tag = 7_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
