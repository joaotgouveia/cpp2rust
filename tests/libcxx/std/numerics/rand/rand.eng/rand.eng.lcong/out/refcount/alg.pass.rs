extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    let e1 : Value<std_linear_congruential_engine_unsigned_long_long__25214903917__1__281474976710656_ > = Rc::new(RefCell::new(<std_linear_congruential_engine_unsigned_long_long__25214903917__1__281474976710656_  >::default() )) ;
    assert!(({ (({ (*e1.borrow_mut())() }) == 25214903918_u64) }));
    assert!(({ (({ (*e1.borrow_mut())() }) == 205774354444503_u64) }));
    assert!(({ (({ (*e1.borrow_mut())() }) == 158051849450892_u64) }));
    assert!(({ (({ (*e1.borrow_mut())() }) < (1_u64 << 48)) }));
    assert!(({ (({ (*e1.borrow_mut())() }) < (1_u64 << 48)) }));
    assert!(({ (({ (*e1.borrow_mut())() }) < (1_u64 << 48)) }));
    assert!(({ (({ (*e1.borrow_mut())() }) < (1_u64 << 48)) }));
    assert!(({ (({ (*e1.borrow_mut())() }) < (1_u64 << 48)) }));
    let e2 : Value<std_linear_congruential_engine_unsigned_long_long__4294967296__0__9223372036854775809_ > = Rc::new(RefCell::new(<std_linear_congruential_engine_unsigned_long_long__4294967296__0__9223372036854775809_  >::default() )) ;
    assert!(({ (({ (*e2.borrow_mut())() }) == (1_u64 << 32)) }));
    assert!(
        ({ (({ (*e2.borrow_mut())() }) == ((1_u64 << 63) as u64).wrapping_sub((1_u64 as u64))) })
    );
    assert!(
        ({
            (({ (*e2.borrow_mut())() })
                == ((1_u64 << 63) as u64).wrapping_sub((8589934591_u64 as u64)))
        })
    );
    assert!(({ (({ (*e2.borrow_mut())() }) < ((1_u64 << 63) as u64).wrapping_add(1_u64)) }));
    assert!(({ (({ (*e2.borrow_mut())() }) < ((1_u64 << 63) as u64).wrapping_add(1_u64)) }));
    assert!(({ (({ (*e2.borrow_mut())() }) < ((1_u64 << 63) as u64).wrapping_add(1_u64)) }));
    assert!(({ (({ (*e2.borrow_mut())() }) < ((1_u64 << 63) as u64).wrapping_add(1_u64)) }));
    assert!(({ (({ (*e2.borrow_mut())() }) < ((1_u64 << 63) as u64).wrapping_add(1_u64)) }));
    let e3 : Value<std_linear_congruential_engine_unsigned_long_long__402653185__74567__216172782113783808_ > = Rc::new(RefCell::new(<std_linear_congruential_engine_unsigned_long_long__402653185__74567__216172782113783808_  >::default() )) ;
    assert!(({ (({ (*e3.borrow_mut())() }) == 402727752_u64) }));
    assert!(({ (({ (*e3.borrow_mut())() }) == 162159612030764687_u64) }));
    assert!(({ (({ (*e3.borrow_mut())() }) == 108176466184989142_u64) }));
    assert!(({ (({ (*e3.borrow_mut())() }) < (3_u64 << 56)) }));
    assert!(({ (({ (*e3.borrow_mut())() }) < (3_u64 << 56)) }));
    assert!(({ (({ (*e3.borrow_mut())() }) < (3_u64 << 56)) }));
    assert!(({ (({ (*e3.borrow_mut())() }) < (3_u64 << 56)) }));
    assert!(({ (({ (*e3.borrow_mut())() }) < (3_u64 << 56)) }));
    let e4: Value<std_linear_congruential_engine_unsigned_int__65545__0__2147483647_> =
        Rc::new(RefCell::new(
            <std_linear_congruential_engine_unsigned_int__65545__0__2147483647_>::default(),
        ));
    assert!(({ (({ (*e4.borrow_mut())() }) == 65545_u32) }));
    assert!(({ (({ (*e4.borrow_mut())() }) == 1179731_u32) }));
    assert!(({ (({ (*e4.borrow_mut())() }) == 16057103_u32) }));
    assert!(({ (({ (*e4.borrow_mut())() }) < 2147483647_u32) }));
    assert!(({ (({ (*e4.borrow_mut())() }) < 2147483647_u32) }));
    assert!(({ (({ (*e4.borrow_mut())() }) < 2147483647_u32) }));
    assert!(({ (({ (*e4.borrow_mut())() }) < 2147483647_u32) }));
    assert!(({ (({ (*e4.borrow_mut())() }) < 2147483647_u32) }));
    let e5 : Value<std_linear_congruential_engine_unsigned_long_long__4294967297__0__2305843009213693951_ > = Rc::new(RefCell::new(<std_linear_congruential_engine_unsigned_long_long__4294967297__0__2305843009213693951_  >::default() )) ;
    assert!(({ (({ (*e5.borrow_mut())() }) == 4294967297_u64) }));
    assert!(({ (({ (*e5.borrow_mut())() }) == 8589934601_u64) }));
    assert!(({ (({ (*e5.borrow_mut())() }) == 47244640281_u64) }));
    assert!(
        ({ (({ (*e5.borrow_mut())() }) < ((1_u64 << 61) as u64).wrapping_sub((1_u64 as u64))) })
    );
    assert!(
        ({ (({ (*e5.borrow_mut())() }) < ((1_u64 << 61) as u64).wrapping_sub((1_u64 as u64))) })
    );
    assert!(
        ({ (({ (*e5.borrow_mut())() }) < ((1_u64 << 61) as u64).wrapping_sub((1_u64 as u64))) })
    );
    assert!(
        ({ (({ (*e5.borrow_mut())() }) < ((1_u64 << 61) as u64).wrapping_sub((1_u64 as u64))) })
    );
    assert!(
        ({ (({ (*e5.borrow_mut())() }) < ((1_u64 << 61) as u64).wrapping_sub((1_u64 as u64))) })
    );
    let e6: Value<std_linear_congruential_engine_unsigned_long_long__1__1__281474976710656_> =
        Rc::new(RefCell::new(
            <std_linear_congruential_engine_unsigned_long_long__1__1__281474976710656_>::default(),
        ));
    assert!(({ (({ (*e6.borrow_mut())() }) == 2_u64) }));
    assert!(({ (({ (*e6.borrow_mut())() }) == 3_u64) }));
    assert!(({ (({ (*e6.borrow_mut())() }) == 4_u64) }));
    assert!(({ (({ (*e6.borrow_mut())() }) < (1_u64 << 48)) }));
    assert!(({ (({ (*e6.borrow_mut())() }) < (1_u64 << 48)) }));
    assert!(({ (({ (*e6.borrow_mut())() }) < (1_u64 << 48)) }));
    assert!(({ (({ (*e6.borrow_mut())() }) < (1_u64 << 48)) }));
    assert!(({ (({ (*e6.borrow_mut())() }) < (1_u64 << 48)) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
