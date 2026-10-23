extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BorrowedRange {
    begin_: *mut (),
    end_: *mut (),
}
impl BorrowedRange {
    pub unsafe fn new(mut b: *mut (), mut e: *mut ()) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&self) -> *mut () {
        return self.begin_;
    }
    pub unsafe fn end(&self) -> *mut () {
        return self.end_;
    }
}
pub static mut enable_borrowed_range_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
pub unsafe fn test_1() -> () {
    let mut buff: [(); 8] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_()];
    {
        let mut range: BorrowedRange =
            BorrowedRange::new({ buff.as_mut_ptr() }, { buff.as_mut_ptr() });
        let mut subrange: () =
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                { &mut range },
                { 0_() },
            );
        unimplemented!();
    }
    {
        let mut range: BorrowedRange = BorrowedRange::new({ buff.as_mut_ptr() }, {
            buff.as_mut_ptr().offset((1_()) as isize)
        });
        let mut subrange: () =
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                { &mut range },
                { 1_() },
            );
        unimplemented!();
    }
    {
        let mut range: BorrowedRange = BorrowedRange::new({ buff.as_mut_ptr() }, {
            buff.as_mut_ptr().offset((2_()) as isize)
        });
        let mut subrange: () =
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                { &mut range },
                { 2_() },
            );
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut range: BorrowedRange = BorrowedRange::new({ buff.as_mut_ptr() }, {
            buff.as_mut_ptr().offset((8_()) as isize)
        });
        let mut subrange: () =
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                { &mut range },
                { 8_() },
            );
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_0);
}
