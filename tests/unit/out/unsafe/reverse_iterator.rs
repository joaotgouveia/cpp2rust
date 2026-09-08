extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Foo {
    pub v: i32,
}
impl Foo {
    pub unsafe fn get(&self) -> i32 {
        return self.v;
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a1: [Foo; 5] = [
        Foo { v: 10 },
        Foo { v: 20 },
        Foo { v: 30 },
        Foo { v: 40 },
        Foo { v: 50 },
    ];
    let mut def: *mut Foo = Default::default();
    let mut first: *mut Foo = a1.as_mut_ptr().offset((5) as isize);
    assert!(((first) == (a1.as_mut_ptr().offset((5) as isize))));
    let ref_: *mut Foo = &mut (*first.offset(-1)) as *mut Foo;
    assert!(((unsafe { Foo::get(&(*ref_),) }) == (50)));
    assert!(((unsafe { Foo::get(&(*(first.offset(-1)).cast_const()),) }) == (50)));
    first.prefix_dec();
    assert!(((unsafe { Foo::get(&(*(first.offset(-1)).cast_const()),) }) == (40)));
    let mut inc: *mut Foo = first.postfix_dec();
    assert!(((unsafe { Foo::get(&(*(inc.offset(-1)).cast_const()),) }) == (40)));
    assert!(((unsafe { Foo::get(&(*(first.offset(-1)).cast_const()),) }) == (30)));
    let mut n: isize = 2_isize;
    let mut plus: *mut Foo = first.offset(Into::<isize>::into(-n));
    assert!(((unsafe { Foo::get(&(*(plus.offset(-1)).cast_const()),) }) == (10)));
    let mut minus: *mut Foo = plus.offset(Into::<isize>::into(n));
    assert!(((unsafe { Foo::get(&(*(minus.offset(-1)).cast_const()),) }) == (30)));
    assert!(minus == first);
    return 0;
}
