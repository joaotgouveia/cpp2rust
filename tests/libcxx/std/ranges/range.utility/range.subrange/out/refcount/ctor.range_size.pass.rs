extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct BorrowedRange {
    #[offset(0)]
    #[byte_size(8)]
    begin_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    end_: Ptr<()>,
}
impl BorrowedRange {
    pub fn new(mut b: Ptr<()>, mut e: Ptr<()>) -> Self {
        Self {
            begin_: (b).clone(),
            end_: (e).clone(),
        }
    }
}
thread_local!(
    pub static enable_borrowed_range_0: Value<()> = Rc::new(RefCell::new(true));
);
pub fn test_1() -> () {
    let buff: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
    ])));
    {
        let range: Value<BorrowedRange> = Rc::new(RefCell::new(BorrowedRange::new(
            { (buff.as_pointer() as Ptr<()>) },
            { (buff.as_pointer() as Ptr<()>) },
        )));
        let subrange: Value<()> = Rc::new(RefCell::new(
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                { range.as_pointer() },
                { 0_() },
            ),
        ));
        unimplemented!();
    }
    {
        let range: Value<BorrowedRange> = Rc::new(RefCell::new(BorrowedRange::new(
            { (buff.as_pointer() as Ptr<()>) },
            { (buff.as_pointer() as Ptr<()>).offset((1_()) as isize) },
        )));
        let subrange: Value<()> = Rc::new(RefCell::new(
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                { range.as_pointer() },
                { 1_() },
            ),
        ));
        unimplemented!();
    }
    {
        let range: Value<BorrowedRange> = Rc::new(RefCell::new(BorrowedRange::new(
            { (buff.as_pointer() as Ptr<()>) },
            { (buff.as_pointer() as Ptr<()>).offset((2_()) as isize) },
        )));
        let subrange: Value<()> = Rc::new(RefCell::new(
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                { range.as_pointer() },
                { 2_() },
            ),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let range: Value<BorrowedRange> = Rc::new(RefCell::new(BorrowedRange::new(
            { (buff.as_pointer() as Ptr<()>) },
            { (buff.as_pointer() as Ptr<()>).offset((8_()) as isize) },
        )));
        let subrange: Value<()> = Rc::new(RefCell::new(
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                { range.as_pointer() },
                { 8_() },
            ),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait BorrowedRangeImpl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
}
impl BorrowedRangeImpl for Ptr<BorrowedRange> {
    fn begin(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.begin_.clone());
    }
    fn end(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.end_.clone());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = enable_borrowed_range_0.with(|_| ());
}
