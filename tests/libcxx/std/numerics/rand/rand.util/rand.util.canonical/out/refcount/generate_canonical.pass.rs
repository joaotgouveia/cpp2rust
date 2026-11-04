extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn truncate_fp_0(mut val: f64) -> f64 {
    let mut sink: f64 = val;
    return sink;
}
pub fn truncate_fp_1(mut val: f64) -> f64 {
    let mut sink: f64 = val;
    return sink;
}
pub fn truncate_fp_2(mut val: f32) -> f32 {
    let mut sink: f32 = val;
    return sink;
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
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f32 = ({ generate_canonical_3(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_2 ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) / ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) + 1_f32 ) ) )  , )
            }))
        );
    }
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f32 = ({ generate_canonical_4(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_2 ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) / ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) + 1_f32 ) ) )  , )
            }))
        );
    }
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f32 = ({ generate_canonical_5(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_2 ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) / ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) + 1_f32 ) ) )  , )
            }))
        );
    }
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f32 = ({ generate_canonical_6(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_2 ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) / ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) + 1_f32 ) ) )  , )
            }))
        );
    }
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f32 = ({ generate_canonical_7(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_2 ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) / ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f32 ) + 1_f32 ) ) )  , )
            }))
        );
    }
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f64 = ({ generate_canonical_8(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_1 ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) / ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) )  , )
            }))
        );
    }
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f64 = ({ generate_canonical_9(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_1 ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) / ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) )  , )
            }))
        );
    }
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f64 = ({ generate_canonical_10(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_1 ( ( ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + ( ( ( ( 282475249_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) * ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) ) ) ) / ( ( ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) * ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) ) ) )  , )
            }))
        );
    }
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f64 = ({ generate_canonical_11(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_1 ( ( ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + ( ( ( ( 282475249_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) * ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) ) ) ) / ( ( ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) * ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) ) ) )  , )
            }))
        );
    }
    {
        let r: Value<std_linear_congruential_engine_unsigned_long__16807__0__2147483647_> =
            Rc::new(RefCell::new(
                <std_linear_congruential_engine_unsigned_long__16807__0__2147483647_>::default(),
            ));
        let mut f: f64 = ({ generate_canonical_12(r.as_pointer()) });
        assert!(
            (f == ({
                truncate_fp_1 ( ( ( ( ( ( ( 16807_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + ( ( ( ( 282475249_u64 ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) * ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) ) ) ) / ( ( ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) * ( ( ( ( ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::max ( ) } )  as u64 ) ) . wrapping_sub ( ( ( { std_linear_congruential_engine_unsigned_long__16807__0__2147483647_::min ( ) } )  as u64 ) ) ) as f64 ) + 1_f64 ) ) ) ) )  , )
            }))
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
