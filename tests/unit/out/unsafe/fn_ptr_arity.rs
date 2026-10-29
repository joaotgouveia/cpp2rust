extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(
    mut a1: (),
    mut a2: (),
    mut a3: (),
    mut a4: (),
    mut a5: (),
    mut a6: (),
    mut a7: (),
    mut a8: (),
    mut a9: (),
    mut a10: (),
    mut a11: (),
    mut a12: (),
    mut a13: (),
    mut a14: (),
) -> () {
    return 22_();
}
pub unsafe fn wide_1(
    mut a1: (),
    mut a2: (),
    mut a3: (),
    mut a4: (),
    mut a5: (),
    mut a6: (),
    mut a7: (),
    mut a8: (),
    mut a9: (),
    mut a10: (),
    mut a11: (),
    mut a12: (),
    mut a13: (),
    mut a14: (),
    mut a15: (),
    mut a16: (),
    mut a17: (),
    mut a18: (),
    mut a19: (),
    mut a20: (),
    mut a21: (),
    mut a22: (),
) -> () {
    return ((a1) + (a22));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut f: Option<unsafe fn((), (), (), (), (), (), (), (), (), (), (), (), (), ()) -> ()> =
        (Some(foo_0));
    unimplemented!();
    let mut w: Option<
        unsafe fn(
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
            (),
        ) -> (),
    > = (Some(wide_1));
    unimplemented!();
    let mut l : FnPtr<fn(() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,) -> () > = FnPtr::<fn(() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,() ,) -> () >::new ( | mut a1 : () , mut a2 : () , mut a3 : () , mut a4 : () , mut a5 : () , mut a6 : () , mut a7 : () , mut a8 : () , mut a9 : () , mut a10 : () , mut a11 : () , mut a12 : () , mut a13 : () , mut a14 : () , mut a15 : () , mut a16 : () , mut a17 : () , mut a18 : () , | -> () { unsafe { return ( ( a1 ) * ( a18 ) )  ;
 } } ) ;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
