extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_skipws_0(is: Ptr<()>) -> bool {
    return (((unimplemented!()) as i32) != 0);
}
pub fn is_skipws_1(is: Ptr<()>) -> bool {
    return (((unimplemented!()) as i32) != 0);
}
pub fn both_ways_2(mut p: Ptr<i8>) {
    let mut str: Vec<i8> = {
        let mut __bytes = p.to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    };
    let q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut skippingws: bool = ({ is_skipws_0(ss.as_pointer()) });
    (&(skippingws));
    unimplemented!();
    unimplemented!();
}
pub fn round_trip_3(mut p: Ptr<i8>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut skippingws: bool = ({ is_skipws_0(ss.as_pointer()) });
    unimplemented!();
    let mut s: Vec<i8> = vec![i8::default()];
    unimplemented!();
    assert!(p.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
    assert!(((skippingws as i32) == (({ is_skipws_0(ss.as_pointer(),) }) as i32)));
}
pub fn round_trip_4(mut p: Ptr<i32>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut skippingws: bool = ({ is_skipws_1(ss.as_pointer()) });
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(p.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
    assert!(((skippingws as i32) == (({ is_skipws_1(ss.as_pointer(),) }) as i32)));
}
pub fn round_trip_ws_5(mut p: Ptr<i8>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    noskipws_refcount(ss.as_pointer());
    let mut skippingws: bool = ({ is_skipws_0(ss.as_pointer()) });
    unimplemented!();
    let mut s: Vec<i8> = vec![i8::default()];
    unimplemented!();
    assert!(p.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
    assert!(((skippingws as i32) == (({ is_skipws_0(ss.as_pointer(),) }) as i32)));
}
pub fn round_trip_ws_6(mut p: Ptr<i32>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    noskipws_refcount(ss.as_pointer());
    let mut skippingws: bool = ({ is_skipws_1(ss.as_pointer()) });
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(p.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
    assert!(((skippingws as i32) == (({ is_skipws_1(ss.as_pointer(),) }) as i32)));
}
pub fn round_trip_d_7(mut p: Ptr<i8>, mut delim: i8) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut d: i8 = delim;
    unimplemented!();
    let mut s: Vec<i8> = vec![i8::default()];
    unimplemented!();
    assert!(p.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
}
pub fn round_trip_d_8(mut p: Ptr<i32>, mut delim: i8) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut d: i32 = (delim as i32);
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(p.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
}
pub fn round_trip_e_9(mut p: Ptr<i8>, mut escape: i8) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut e: i8 = escape;
    unimplemented!();
    let mut s: Vec<i8> = vec![i8::default()];
    unimplemented!();
    assert!(p.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
}
pub fn round_trip_e_10(mut p: Ptr<i32>, mut escape: i8) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut e: i32 = (escape as i32);
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    assert!(p.with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s));
}
pub fn quote_11(mut p: Ptr<i8>, delim: Option<i8>, escape: Option<i8>) -> Vec<i8> {
    let mut delim: i8 = delim.unwrap_or_else(|| ('\"' as i8));
    let mut escape: i8 = escape.unwrap_or_else(|| ('\\' as i8));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut d: i8 = delim;
    let mut e: i8 = escape;
    unimplemented!();
    let mut s: Vec<i8> = vec![i8::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub fn quote_12(mut p: Ptr<i32>, delim: Option<i8>, escape: Option<i8>) -> Vec<i32> {
    let mut delim: i8 = delim.unwrap_or_else(|| ('\"' as i8));
    let mut escape: i8 = escape.unwrap_or_else(|| ('\\' as i8));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut d: i32 = (delim as i32);
    let mut e: i32 = (escape as i32);
    unimplemented!();
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub fn unquote_13(mut p: Ptr<i8>, delim: Option<i8>, escape: Option<i8>) -> Vec<i8> {
    let mut delim: i8 = delim.unwrap_or_else(|| ('\"' as i8));
    let mut escape: i8 = escape.unwrap_or_else(|| ('\\' as i8));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let __s = ss.as_pointer();
        let _ = p.with_c_str(|__cs| {
            __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>())
        });
        __s
    };
    let mut d: i8 = delim;
    let mut e: i8 = escape;
    let mut s: Vec<i8> = vec![i8::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub fn unquote_14(mut p: Ptr<i32>, delim: Option<i8>, escape: Option<i8>) -> Vec<i32> {
    let mut delim: i8 = delim.unwrap_or_else(|| ('\"' as i8));
    let mut escape: i8 = escape.unwrap_or_else(|| ('\\' as i8));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut d: i32 = (delim as i32);
    let mut e: i32 = (escape as i32);
    let mut s: Vec<i32> = vec![i32::default()];
    unimplemented!();
    return std::mem::take(&mut s);
}
pub fn test_padding_15() {
    {
        let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(
            Ptr::<i8>::from_string_literal(b"`abc`!!!!!").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    {
        let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(
            Ptr::<i8>::from_string_literal(b"!!!!!`abc`").with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
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
    ({ both_ways_2(Ptr::<i8>::from_string_literal(b"")) });
    ({ round_trip_3(Ptr::<i8>::from_string_literal(b"")) });
    ({ round_trip_ws_5(Ptr::<i8>::from_string_literal(b"")) });
    ({ round_trip_d_7(Ptr::<i8>::from_string_literal(b""), ('q' as i8)) });
    ({ round_trip_e_9(Ptr::<i8>::from_string_literal(b""), ('q' as i8)) });
    ({ round_trip_4(Ptr::<i32>::from_string_literal(&[0 as i32])) });
    ({ round_trip_ws_6(Ptr::<i32>::from_string_literal(&[0 as i32])) });
    ({ round_trip_d_8(Ptr::<i32>::from_string_literal(&[0 as i32]), ('q' as i8)) });
    ({ round_trip_e_10(Ptr::<i32>::from_string_literal(&[0 as i32]), ('q' as i8)) });
    ({ round_trip_3(Ptr::<i8>::from_string_literal(b"Hi")) });
    ({ round_trip_ws_5(Ptr::<i8>::from_string_literal(b"Hi")) });
    ({ round_trip_d_7(Ptr::<i8>::from_string_literal(b"Hi"), ('!' as i8)) });
    ({ round_trip_e_9(Ptr::<i8>::from_string_literal(b"Hi"), ('!' as i8)) });
    assert!(Ptr::<i8>::from_string_literal(b"!Hi!").with_c_str(|__s| {
        ({
            quote_11(
                Ptr::<i8>::from_string_literal(b"Hi"),
                Some(('!' as i8)),
                None,
            )
        })[..({
            quote_11(
                Ptr::<i8>::from_string_literal(b"Hi"),
                Some(('!' as i8)),
                None,
            )
        })
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(
        Ptr::<i8>::from_string_literal(b"!Hi\\!!").with_c_str(|__s| ({
            quote_11(
                Ptr::<i8>::from_string_literal(b"Hi!"),
                Some(('!' as i8)),
                None,
            )
        })[..({
            quote_11(
                Ptr::<i8>::from_string_literal(b"Hi!"),
                Some(('!' as i8)),
                None,
            )
        })
        .len()
        .saturating_sub(1)]
            == *__s)
    );
    ({
        round_trip_4(Ptr::<i32>::from_string_literal(&[
            72 as i32, 105 as i32, 0 as i32,
        ]))
    });
    ({
        round_trip_ws_6(Ptr::<i32>::from_string_literal(&[
            72 as i32, 105 as i32, 0 as i32,
        ]))
    });
    ({
        round_trip_d_8(
            Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 0 as i32]),
            ('!' as i8),
        )
    });
    ({
        round_trip_e_10(
            Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 0 as i32]),
            ('!' as i8),
        )
    });
    assert!(
        Ptr::<i32>::from_string_literal(&[33 as i32, 72 as i32, 105 as i32, 33 as i32, 0 as i32,])
            .with_c_str(|__s| ({
                quote_12(
                    Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 0 as i32]),
                    Some(('!' as i8)),
                    None,
                )
            })[..({
                quote_12(
                    Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 0 as i32]),
                    Some(('!' as i8)),
                    None,
                )
            })
            .len()
            .saturating_sub(1)]
                == *__s)
    );
    assert!(
        Ptr::<i32>::from_string_literal(&[
            33 as i32, 72 as i32, 105 as i32, 92 as i32, 33 as i32, 33 as i32, 0 as i32,
        ])
        .with_c_str(|__s| ({
            quote_12(
                Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 33 as i32, 0 as i32]),
                Some(('!' as i8)),
                None,
            )
        })[..({
            quote_12(
                Ptr::<i32>::from_string_literal(&[72 as i32, 105 as i32, 33 as i32, 0 as i32]),
                Some(('!' as i8)),
                None,
            )
        })
        .len()
        .saturating_sub(1)]
            == *__s)
    );
    ({ round_trip_3(Ptr::<i8>::from_string_literal(b"Hi Mom")) });
    ({ round_trip_ws_5(Ptr::<i8>::from_string_literal(b"Hi Mom")) });
    ({
        round_trip_4(Ptr::<i32>::from_string_literal(&[
            72 as i32, 105 as i32, 32 as i32, 77 as i32, 111 as i32, 109 as i32, 0 as i32,
        ]))
    });
    ({
        round_trip_ws_6(Ptr::<i32>::from_string_literal(&[
            72 as i32, 105 as i32, 32 as i32, 77 as i32, 111 as i32, 109 as i32, 0 as i32,
        ]))
    });
    assert!(Ptr::<i8>::from_string_literal(b"\"\"").with_c_str(|__s| {
        ({ quote_11(Ptr::<i8>::from_string_literal(b""), None, None) })[..({
            quote_11(Ptr::<i8>::from_string_literal(b""), None, None)
        })
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(Ptr::<i8>::from_string_literal(b"\"a\"").with_c_str(|__s| {
        ({ quote_11(Ptr::<i8>::from_string_literal(b"a"), None, None) })[..({
            quote_11(Ptr::<i8>::from_string_literal(b"a"), None, None)
        })
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(
        Ptr::<i32>::from_string_literal(&[34 as i32, 34 as i32, 0 as i32,]).with_c_str(|__s| ({
            quote_12(Ptr::<i32>::from_string_literal(&[0 as i32]), None, None)
        })[..({ quote_12(Ptr::<i32>::from_string_literal(&[0 as i32,]), None, None,) })
            .len()
            .saturating_sub(1)]
            == *__s)
    );
    assert!(
        Ptr::<i32>::from_string_literal(&[34 as i32, 97 as i32, 34 as i32, 0 as i32,]).with_c_str(
            |__s| ({
                quote_12(
                    Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]),
                    None,
                    None,
                )
            })[..({
                quote_12(
                    Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32]),
                    None,
                    None,
                )
            })
            .len()
            .saturating_sub(1)]
                == *__s
        )
    );
    assert!(Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
        ({ unquote_13(Ptr::<i8>::from_string_literal(b"\"abc"), None, None) })[..({
            unquote_13(Ptr::<i8>::from_string_literal(b"\"abc"), None, None)
        })
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(
        Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32,]).with_c_str(
            |__s| ({
                unquote_14(
                    Ptr::<i32>::from_string_literal(&[
                        34 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32,
                    ]),
                    None,
                    None,
                )
            })[..({
                unquote_14(
                    Ptr::<i32>::from_string_literal(&[
                        34 as i32, 97 as i32, 98 as i32, 99 as i32, 0 as i32,
                    ]),
                    None,
                    None,
                )
            })
            .len()
            .saturating_sub(1)]
                == *__s
        )
    );
    assert!(Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
        ({ unquote_13(Ptr::<i8>::from_string_literal(b"abc"), None, None) })[..({
            unquote_13(Ptr::<i8>::from_string_literal(b"abc"), None, None)
        })
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
        ({ unquote_13(Ptr::<i8>::from_string_literal(b"abc def"), None, None) })[..({
            unquote_13(Ptr::<i8>::from_string_literal(b"abc def"), None, None)
        })
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(
        Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32,]).with_c_str(
            |__s| ({
                unquote_14(
                    Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32]),
                    None,
                    None,
                )
            })[..({
                unquote_14(
                    Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32]),
                    None,
                    None,
                )
            })
            .len()
            .saturating_sub(1)]
                == *__s
        )
    );
    assert!(
        Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32,]).with_c_str(
            |__s| ({
                unquote_14(
                    Ptr::<i32>::from_string_literal(&[
                        97 as i32, 98 as i32, 99 as i32, 32 as i32, 100 as i32, 101 as i32,
                        102 as i32, 0 as i32,
                    ]),
                    None,
                    None,
                )
            })[..({
                unquote_14(
                    Ptr::<i32>::from_string_literal(&[
                        97 as i32, 98 as i32, 99 as i32, 32 as i32, 100 as i32, 101 as i32,
                        102 as i32, 0 as i32,
                    ]),
                    None,
                    None,
                )
            })
            .len()
            .saturating_sub(1)]
                == *__s
        )
    );
    assert!(Ptr::<i8>::from_string_literal(b"").with_c_str(|__s| {
        ({ unquote_13(Ptr::<i8>::from_string_literal(b""), None, None) })[..({
            unquote_13(Ptr::<i8>::from_string_literal(b""), None, None)
        })
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(
        Ptr::<i32>::from_string_literal(&[0 as i32,]).with_c_str(|__s| ({
            unquote_14(Ptr::<i32>::from_string_literal(&[0 as i32]), None, None)
        })[..({ unquote_14(Ptr::<i32>::from_string_literal(&[0 as i32,]), None, None,) })
            .len()
            .saturating_sub(1)]
            == *__s)
    );
    ({ test_padding_15() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
