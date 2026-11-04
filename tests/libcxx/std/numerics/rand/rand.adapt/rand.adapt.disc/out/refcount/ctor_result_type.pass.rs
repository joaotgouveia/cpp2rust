extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0() {
    let mut a : Ptr< i8  >  = Ptr::<i8 >::from_string_literal(b"15136306 8587749 2346244 16479026 15515802 9510553 16090340 14501685 13839944 10789678 11581259 9590790 5840316 5953700 13398366 8134459 16629731 6851902 15583892 1317475 4231148 9092691 5707268 2355175 0 0" )  ;
    let e1 : Value<std_discard_block_engine_std_subtract_with_carry_engine_unsigned_long__24__10__24___223__23_ > = Rc::new(RefCell::new(std_discard_block_engine_std_subtract_with_carry_engine_unsigned_long__24__10__24___223__23_ :: new_1 ( { 0_u64  } , )  )) ;
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
    let mut a : Ptr< i8  >  = Ptr::<i8 >::from_string_literal(b"10880375256626 126660097854724 33643165434010 78293780235492 179418984296008 96783156950859 238199764491708 34339434557790 155299155394531 29014415493780 209265474179052 263777435457028 0 0" )  ;
    let e1 : Value<std_discard_block_engine_std_subtract_with_carry_engine_unsigned_long__48__5__12___389__11_ > = Rc::new(RefCell::new(std_discard_block_engine_std_subtract_with_carry_engine_unsigned_long__48__5__12___389__11_ :: new_1 ( { 0_u64  } , )  )) ;
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
