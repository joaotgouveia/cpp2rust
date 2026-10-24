extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_padding_0() {
    {
        let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        write!(
            ss.as_pointer(),
            "{:}{:unimplemented!()}{:}",
            FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(libcc2rs::left_refcount),
            unimplemented!(),
            unimplemented!(),
        );
        unimplemented!();
    }
    {
        let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        write!(
            ss.as_pointer(),
            "{:}{:unimplemented!()}{:}",
            FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(libcc2rs::right_refcount),
            unimplemented!(),
            unimplemented!(),
        );
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ both_ways_3(Ptr::<()>::from_string_literal(b"")) });
    ({ round_trip_4(Ptr::<()>::from_string_literal(b"")) });
    ({ round_trip_ws_5(Ptr::<()>::from_string_literal(b"")) });
    ({ round_trip_d_6(Ptr::<()>::from_string_literal(b""), ('q' as ())) });
    ({ round_trip_e_7(Ptr::<()>::from_string_literal(b""), ('q' as ())) });
    ({ round_trip_8(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({ round_trip_ws_9(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({ round_trip_d_10(Ptr::<()>::from_string_literal(&[0 as ()]), ('q' as ())) });
    ({ round_trip_e_11(Ptr::<()>::from_string_literal(&[0 as ()]), ('q' as ())) });
    ({ round_trip_4(Ptr::<()>::from_string_literal(b"Hi")) });
    ({ round_trip_ws_5(Ptr::<()>::from_string_literal(b"Hi")) });
    ({ round_trip_d_6(Ptr::<()>::from_string_literal(b"Hi"), ('!' as ())) });
    ({ round_trip_e_7(Ptr::<()>::from_string_literal(b"Hi"), ('!' as ())) });
    unimplemented!();
    unimplemented!();
    ({
        round_trip_8(Ptr::<()>::from_string_literal(&[
            72 as (), 105 as (), 0 as (),
        ]))
    });
    ({
        round_trip_ws_9(Ptr::<()>::from_string_literal(&[
            72 as (), 105 as (), 0 as (),
        ]))
    });
    ({
        round_trip_d_10(
            Ptr::<()>::from_string_literal(&[72 as (), 105 as (), 0 as ()]),
            ('!' as ()),
        )
    });
    ({
        round_trip_e_11(
            Ptr::<()>::from_string_literal(&[72 as (), 105 as (), 0 as ()]),
            ('!' as ()),
        )
    });
    unimplemented!();
    unimplemented!();
    ({ round_trip_4(Ptr::<()>::from_string_literal(b"Hi Mom")) });
    ({ round_trip_ws_5(Ptr::<()>::from_string_literal(b"Hi Mom")) });
    ({
        round_trip_8(Ptr::<()>::from_string_literal(&[
            72 as (), 105 as (), 32 as (), 77 as (), 111 as (), 109 as (), 0 as (),
        ]))
    });
    ({
        round_trip_ws_9(Ptr::<()>::from_string_literal(&[
            72 as (), 105 as (), 32 as (), 77 as (), 111 as (), 109 as (), 0 as (),
        ]))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ test_padding_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
