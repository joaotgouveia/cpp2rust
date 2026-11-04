extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0() {
    let mut a : Ptr< i8  >  = Ptr::<i8 >::from_string_literal(b"13604817 711567 9760686 13278398 3323440 175548 5553651 3028863 10748297 2216688 275779 14778841 14438394 9483441 4229545 14657301 12636508 15978210 1653340 1718567 9272421 14302862 7940348 889045 0" )  ;
    let as_: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([3_u32, 5_u32, 7_u32])));
    let sseq: Value<std_seed_seq> = Rc::new(RefCell::new(std_seed_seq::new_1(
        { (as_.as_pointer() as Ptr<u32>) },
        { (as_.as_pointer() as Ptr<u32>).offset((3) as isize) },
    )));
    let e1: Value<std_subtract_with_carry_engine_unsigned_long__24__10__24_> =
        Rc::new(RefCell::new(
            std_subtract_with_carry_engine_unsigned_long__24__10__24_::new_1({ sseq.as_pointer() }),
        ));
    let os: Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    write!(os.as_pointer(), "{:}", (*e1.borrow_mut()),);
    assert!(
        match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_0,) => {
                a.with_c_str(|__s| {
                    (*__tmp_0.upgrade().deref())
                        [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
}
pub fn test2_1() {
    let mut a : Ptr< i8  >  = Ptr::<i8 >::from_string_literal(b"241408498702289 172342669275054 191026374555184 61020585639411 231929771458953 142769679250755 198672786411514 183712717244841 227473912549724 62843577252444 68782400568421 159248704678140 0" )  ;
    let as_: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([3_u32, 5_u32, 7_u32])));
    let sseq: Value<std_seed_seq> = Rc::new(RefCell::new(std_seed_seq::new_1(
        { (as_.as_pointer() as Ptr<u32>) },
        { (as_.as_pointer() as Ptr<u32>).offset((3) as isize) },
    )));
    let e1: Value<std_subtract_with_carry_engine_unsigned_long__48__5__12_> =
        Rc::new(RefCell::new(
            std_subtract_with_carry_engine_unsigned_long__48__5__12_::new_1({ sseq.as_pointer() }),
        ));
    let os: Value<std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_ostringstream_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    write!(os.as_pointer(), "{:}", (*e1.borrow_mut()),);
    assert!(
        match (Rc::new(RefCell::new(({ (*os.borrow()).str_16() }))).as_pointer(),) {
            (__tmp_1,) => {
                a.with_c_str(|__s| {
                    (*__tmp_1.upgrade().deref())
                        [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
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
    ({ test1_0() });
    ({ test2_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
