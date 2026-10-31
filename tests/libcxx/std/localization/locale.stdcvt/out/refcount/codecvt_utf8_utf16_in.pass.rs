extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHelper_wchar_t__4_ {}
impl TestHelper_wchar_t__4_ {
    pub fn test() {
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let w: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32])));
            let n: Value<Box<[i8]>> =
                Rc::new(RefCell::new(Box::new([-15_i8, -128_i8, -128_i8, -125_i8])));
            let mut wp: Ptr<i32> = Ptr::<i32>::null();
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let mut np: Ptr<i8> = Ptr::<i8>::null();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((2) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((4) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 55488));
            assert!(((*w.borrow())[(1) as usize] == 56323));
            (*n.borrow_mut())[(0) as usize] = -31_i8;
            (*n.borrow_mut())[(1) as usize] = -128_i8;
            (*n.borrow_mut())[(2) as usize] = -123_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((3) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 4101));
            (*n.borrow_mut())[(0) as usize] = -47_i8;
            (*n.borrow_mut())[(1) as usize] = -109_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((2) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 1107));
            (*n.borrow_mut())[(0) as usize] = 86_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((1) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 86));
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let w: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32])));
            let n: Value<Box<[i8]>> =
                Rc::new(RefCell::new(Box::new([-15_i8, -128_i8, -128_i8, -125_i8])));
            let mut wp: Ptr<i32> = Ptr::<i32>::null();
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let mut np: Ptr<i8> = Ptr::<i8>::null();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>) }));
            (*n.borrow_mut())[(0) as usize] = -31_i8;
            (*n.borrow_mut())[(1) as usize] = -128_i8;
            (*n.borrow_mut())[(2) as usize] = -123_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>) }));
            (*n.borrow_mut())[(0) as usize] = -47_i8;
            (*n.borrow_mut())[(1) as usize] = -109_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((2) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 1107));
            (*n.borrow_mut())[(0) as usize] = 86_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((1) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 86));
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let w: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32])));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
                -17_i8, -69_i8, -65_i8, -15_i8, -128_i8, -128_i8, -125_i8,
            ])));
            let mut wp: Ptr<i32> = Ptr::<i32>::null();
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let mut np: Ptr<i8> = Ptr::<i8>::null();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((2) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((7) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 55488));
            assert!(((*w.borrow())[(1) as usize] == 56323));
            (*n.borrow_mut())[(0) as usize] = -31_i8;
            (*n.borrow_mut())[(1) as usize] = -128_i8;
            (*n.borrow_mut())[(2) as usize] = -123_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((3) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 4101));
            (*n.borrow_mut())[(0) as usize] = -47_i8;
            (*n.borrow_mut())[(1) as usize] = -109_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((2) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 1107));
            (*n.borrow_mut())[(0) as usize] = 86_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((1) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 86));
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHelper_char32_t__4_ {}
impl TestHelper_char32_t__4_ {
    pub fn test() {
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let w: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([0_u32, 0_u32])));
            let n: Value<Box<[i8]>> =
                Rc::new(RefCell::new(Box::new([-15_i8, -128_i8, -128_i8, -125_i8])));
            let mut wp: Ptr<u32> = Ptr::<u32>::null();
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let mut np: Ptr<i8> = Ptr::<i8>::null();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((2) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((4) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 55488_u32));
            assert!(((*w.borrow())[(1) as usize] == 56323_u32));
            (*n.borrow_mut())[(0) as usize] = -31_i8;
            (*n.borrow_mut())[(1) as usize] = -128_i8;
            (*n.borrow_mut())[(2) as usize] = -123_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((3) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 4101_u32));
            (*n.borrow_mut())[(0) as usize] = -47_i8;
            (*n.borrow_mut())[(1) as usize] = -109_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((2) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 1107_u32));
            (*n.borrow_mut())[(0) as usize] = 86_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((1) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 86_u32));
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let w: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([0_u32, 0_u32])));
            let n: Value<Box<[i8]>> =
                Rc::new(RefCell::new(Box::new([-15_i8, -128_i8, -128_i8, -125_i8])));
            let mut wp: Ptr<u32> = Ptr::<u32>::null();
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let mut np: Ptr<i8> = Ptr::<i8>::null();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>) }));
            (*n.borrow_mut())[(0) as usize] = -31_i8;
            (*n.borrow_mut())[(1) as usize] = -128_i8;
            (*n.borrow_mut())[(2) as usize] = -123_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>) }));
            (*n.borrow_mut())[(0) as usize] = -47_i8;
            (*n.borrow_mut())[(1) as usize] = -109_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((2) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 1107_u32));
            (*n.borrow_mut())[(0) as usize] = 86_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((1) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 86_u32));
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let w: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([0_u32, 0_u32])));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
                -17_i8, -69_i8, -65_i8, -15_i8, -128_i8, -128_i8, -125_i8,
            ])));
            let mut wp: Ptr<u32> = Ptr::<u32>::null();
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let mut np: Ptr<i8> = Ptr::<i8>::null();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((2) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((7) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 55488_u32));
            assert!(((*w.borrow())[(1) as usize] == 56323_u32));
            (*n.borrow_mut())[(0) as usize] = -31_i8;
            (*n.borrow_mut())[(1) as usize] = -128_i8;
            (*n.borrow_mut())[(2) as usize] = -123_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((3) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 4101_u32));
            (*n.borrow_mut())[(0) as usize] = -47_i8;
            (*n.borrow_mut())[(1) as usize] = -109_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((2) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 1107_u32));
            (*n.borrow_mut())[(0) as usize] = 86_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u32>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((1) as isize) }));
            assert!(((*w.borrow())[(0) as usize] == 86_u32));
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHelper_char16_t__2_ {}
impl TestHelper_char16_t__2_ {
    pub fn test() {
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let w: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([0_u16, 0_u16])));
            let n: Value<Box<[i8]>> =
                Rc::new(RefCell::new(Box::new([-15_i8, -128_i8, -128_i8, -125_i8])));
            let mut wp: Ptr<u16> = Ptr::<u16>::null();
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let mut np: Ptr<i8> = Ptr::<i8>::null();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u16>) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>) }));
            (*n.borrow_mut())[(0) as usize] = -31_i8;
            (*n.borrow_mut())[(1) as usize] = -128_i8;
            (*n.borrow_mut())[(2) as usize] = -123_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u16>) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>) }));
            (*n.borrow_mut())[(0) as usize] = -47_i8;
            (*n.borrow_mut())[(1) as usize] = -109_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u16>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((2) as isize) }));
            assert!((((*w.borrow())[(0) as usize] as i32) == 1107));
            (*n.borrow_mut())[(0) as usize] = 86_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u16>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((1) as isize) }));
            assert!((((*w.borrow())[(0) as usize] as i32) == 86));
        }
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let w: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([0_u16, 0_u16])));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
                -17_i8, -69_i8, -65_i8, -15_i8, -128_i8, -128_i8, -125_i8,
            ])));
            let mut wp: Ptr<u16> = Ptr::<u16>::null();
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let mut np: Ptr<i8> = Ptr::<i8>::null();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u16>).offset((2) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((7) as isize) }));
            assert!((((*w.borrow())[(0) as usize] as i32) == 55488));
            assert!((((*w.borrow())[(1) as usize] as i32) == 56323));
            (*n.borrow_mut())[(0) as usize] = -31_i8;
            (*n.borrow_mut())[(1) as usize] = -128_i8;
            (*n.borrow_mut())[(2) as usize] = -123_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u16>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((3) as isize) }));
            assert!((((*w.borrow())[(0) as usize] as i32) == 4101));
            (*n.borrow_mut())[(0) as usize] = -47_i8;
            (*n.borrow_mut())[(1) as usize] = -109_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u16>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((2) as isize) }));
            assert!((((*w.borrow())[(0) as usize] as i32) == 1107));
            (*n.borrow_mut())[(0) as usize] = 86_i8;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(({ (wp).clone() } == { (w.as_pointer() as Ptr::<u16>).offset((1) as isize) }));
            assert!(({ (np).clone() } == { (n.as_pointer() as Ptr::<i8>).offset((1) as isize) }));
            assert!((((*w.borrow())[(0) as usize] as i32) == 86));
        }
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ TestHelper_wchar_t__4_::test() });
    ({ TestHelper_char32_t__4_::test() });
    ({ TestHelper_char16_t__2_::test() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
