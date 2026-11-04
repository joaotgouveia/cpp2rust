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
    let l: Value<std_locale> = Rc::new(RefCell::new(std_locale::new_1(
        { ({ std_locale::classic() }) },
        { Ptr::alloc(<std_codecvt_utf16_char32_t__1114111__0_>::default()) },
    )));
    let f32_8: Ptr<std_codecvt_char32_t__char____mbstate_t_> =
        ({ use_facet_0(({ std_locale::classic() })) });
    let f32_16: Ptr<std_codecvt_utf16_char32_t__1114111__0_> = ({ use_facet_1(l.as_pointer()) });
    let f16_8: Ptr<std_codecvt_char16_t__char____mbstate_t_> =
        ({ use_facet_2(({ std_locale::classic() })) });
    let mbs32_8: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mbs16_8: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mbs32_16: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let c32p: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
    let c16p: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::null()));
    let c8p: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
    let c_c32p: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
    let c_c16p: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::null()));
    let c_c8p: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
    let c32: Value<u32> = Rc::new(RefCell::new(0_u32));
    let c16: Value<Box<[u16]>> =
        Rc::new(RefCell::new((0..2).map(|_| 0_u16).collect::<Box<[u16]>>()));
    let c16c: Value<Box<[i8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let c16cp: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
    let c8: Value<Box<[i8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let f32_8t: Ptr<std_codecvt_char32_t__char8_t____mbstate_t_> =
        ({ use_facet_3(({ std_locale::classic() })) });
    let f16_8t: Ptr<std_codecvt_char16_t__char8_t____mbstate_t_> =
        ({ use_facet_4(({ std_locale::classic() })) });
    let mbs32_8t: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mbs16_8t: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let c8tp: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    let c_c8tp: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    let c8t: Value<Box<[u8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let c32x: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*c32x.borrow()) < 1114115_u32) {
        if ((55296_u32 <= (*c32x.borrow())) && ((*c32x.borrow()) < 57344_u32))
            || ((*c32x.borrow()) >= 1114112_u32)
        {
            assert!(
                ({
                    ((({
                        let ___from: Ptr<u32> = (c32x.as_pointer());
                        let ___from_end: Ptr<u32> = (c32x.as_pointer()).offset((1) as isize);
                        let ___to: Ptr<i8> = (c16c.as_pointer() as Ptr<i8>).offset((0) as isize);
                        let ___to_end: Ptr<i8> =
                            (c16c.as_pointer() as Ptr<i8>).offset((4) as isize);
                        ((*f32_16.upgrade().deref())
                            as std___codecvt_abstract_base_char32_t__char____mbstate_t_)
                            .out(
                                mbs32_16.as_pointer(),
                                ___from,
                                ___from_end,
                                c_c32p.as_pointer(),
                                ___to,
                                ___to_end,
                                c16cp.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_error as i32))
                })
            );
            assert!(
                ({
                    ((({
                        let ___from: Ptr<u32> = (c32x.as_pointer());
                        let ___from_end: Ptr<u32> = (c32x.as_pointer()).offset((1) as isize);
                        let ___to: Ptr<i8> = (c8.as_pointer() as Ptr<i8>);
                        let ___to_end: Ptr<i8> = (c8.as_pointer() as Ptr<i8>).offset((4) as isize);
                        ((*f32_8.upgrade().deref())
                            as std___codecvt_abstract_base_char32_t__char____mbstate_t_)
                            .out(
                                mbs32_8.as_pointer(),
                                ___from,
                                ___from_end,
                                c_c32p.as_pointer(),
                                ___to,
                                ___to_end,
                                c8p.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_error as i32))
                })
            );
            assert!(
                ({
                    ((({
                        let ___from: Ptr<u32> = (c32x.as_pointer());
                        let ___from_end: Ptr<u32> = (c32x.as_pointer()).offset((1) as isize);
                        let ___to: Ptr<u8> = (c8t.as_pointer() as Ptr<u8>);
                        let ___to_end: Ptr<u8> = (c8t.as_pointer() as Ptr<u8>).offset((4) as isize);
                        ((*f32_8t.upgrade().deref())
                            as std___codecvt_abstract_base_char32_t__char8_t____mbstate_t_)
                            .out(
                                mbs32_8t.as_pointer(),
                                ___from,
                                ___from_end,
                                c_c32p.as_pointer(),
                                ___to,
                                ___to_end,
                                c8tp.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_error as i32))
                })
            );
        } else {
            assert!(
                ({
                    ((({
                        let ___from: Ptr<u32> = (c32x.as_pointer());
                        let ___from_end: Ptr<u32> = (c32x.as_pointer()).offset((1) as isize);
                        let ___to: Ptr<i8> = (c16c.as_pointer() as Ptr<i8>);
                        let ___to_end: Ptr<i8> =
                            (c16c.as_pointer() as Ptr<i8>).offset((4) as isize);
                        ((*f32_16.upgrade().deref())
                            as std___codecvt_abstract_base_char32_t__char____mbstate_t_)
                            .out(
                                mbs32_16.as_pointer(),
                                ___from,
                                ___from_end,
                                c_c32p.as_pointer(),
                                ___to,
                                ___to_end,
                                c16cp.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            assert!(({ (((*c_c32p.borrow()).clone() - (c32x.as_pointer())) as i64 == 1_i64) }));
            if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c16cp.borrow()).clone() - (c16c.as_pointer() as Ptr<i8>)) as i64
                            == 2_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c16cp.borrow()).clone() - (c16c.as_pointer() as Ptr<i8>)) as i64
                            == 4_i64)
                    })
                );
            }
            let mut i: i32 = 0;
            'loop_: while ({ (i as i64) } < {
                ((((*c16cp.borrow()).clone() - (c16c.as_pointer() as Ptr<i8>)) as i64) / 2_i64)
            }) {
                (*c16.borrow_mut())[(i) as usize] = {
                    ((((((*c16c.borrow())[(2 * i) as usize] as u8) as i32) << 8)
                        | (((*c16c.borrow())[((2 * i) + 1) as usize] as u8) as i32))
                        as u16)
                };
                i.prefix_inc();
            }
            (*c_c16p.borrow_mut()) = (c16.as_pointer() as Ptr<u16>).offset(
                ((((*c16cp.borrow()).clone() - (c16c.as_pointer() as Ptr<i8>)) as i64) / 2_i64)
                    as isize,
            );
            assert!(
                ({
                    ((({
                        let ___from_end: Ptr<u16> = (*c_c16p.borrow()).clone();
                        let ___from_next: Ptr<Ptr<u16>> = c_c16p.as_pointer();
                        let ___to: Ptr<i8> = (c8.as_pointer() as Ptr<i8>);
                        let ___to_end: Ptr<i8> = (c8.as_pointer() as Ptr<i8>).offset((4) as isize);
                        ((*f16_8.upgrade().deref())
                            as std___codecvt_abstract_base_char16_t__char____mbstate_t_)
                            .out(
                                mbs16_8.as_pointer(),
                                (c16.as_pointer() as Ptr<u16>),
                                ___from_end,
                                ___from_next,
                                ___to,
                                ___to_end,
                                c8p.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c_c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64
                            == 1_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c_c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64
                            == 2_i64)
                    })
                );
            }
            if ((*c32x.borrow()) < 128_u32) {
                assert!(
                    ({
                        (((*c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 1_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 2048_u32) {
                assert!(
                    ({
                        (((*c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 2_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 3_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 4_i64)
                    })
                );
            }
            (*c_c8p.borrow_mut()) = (*c8p.borrow()).clone();
            assert!(
                ({
                    ((({
                        let ___from_end: Ptr<i8> = (*c_c8p.borrow()).clone();
                        let ___from_next: Ptr<Ptr<i8>> = c_c8p.as_pointer();
                        let ___to: Ptr<u32> = (c32.as_pointer());
                        let ___to_end: Ptr<u32> = (c32.as_pointer()).offset((1) as isize);
                        ((*f32_8.upgrade().deref())
                            as std___codecvt_abstract_base_char32_t__char____mbstate_t_)
                            .in_(
                                mbs32_8.as_pointer(),
                                (c8.as_pointer() as Ptr<i8>),
                                ___from_end,
                                ___from_next,
                                ___to,
                                ___to_end,
                                c32p.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            if ((*c32x.borrow()) < 128_u32) {
                assert!(
                    ({
                        (((*c_c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 1_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 2048_u32) {
                assert!(
                    ({
                        (((*c_c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 2_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c_c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 3_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c_c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 4_i64)
                    })
                );
            }
            assert!(({ (((*c32p.borrow()).clone() - (c32.as_pointer())) as i64 == 1_i64) }));
            assert!(({ ((*c32.borrow()) == (*c32x.borrow())) }));
            assert!(
                ({
                    ((({
                        let ___from: Ptr<u32> = (c32x.as_pointer());
                        let ___from_end: Ptr<u32> = (c32x.as_pointer()).offset((1) as isize);
                        let ___to: Ptr<i8> = (c8.as_pointer() as Ptr<i8>);
                        let ___to_end: Ptr<i8> = (c8.as_pointer() as Ptr<i8>).offset((4) as isize);
                        ((*f32_8.upgrade().deref())
                            as std___codecvt_abstract_base_char32_t__char____mbstate_t_)
                            .out(
                                mbs32_8.as_pointer(),
                                ___from,
                                ___from_end,
                                c_c32p.as_pointer(),
                                ___to,
                                ___to_end,
                                c8p.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            assert!(({ (((*c_c32p.borrow()).clone() - (c32x.as_pointer())) as i64 == 1_i64) }));
            if ((*c32x.borrow()) < 128_u32) {
                assert!(
                    ({
                        (((*c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 1_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 2048_u32) {
                assert!(
                    ({
                        (((*c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 2_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 3_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 4_i64)
                    })
                );
            }
            (*c_c8p.borrow_mut()) = (*c8p.borrow()).clone();
            assert!(
                ({
                    ((({
                        let ___from_end: Ptr<i8> = (*c_c8p.borrow()).clone();
                        let ___from_next: Ptr<Ptr<i8>> = c_c8p.as_pointer();
                        let ___to: Ptr<u16> = (c16.as_pointer() as Ptr<u16>);
                        let ___to_end: Ptr<u16> =
                            (c16.as_pointer() as Ptr<u16>).offset((2) as isize);
                        ((*f16_8.upgrade().deref())
                            as std___codecvt_abstract_base_char16_t__char____mbstate_t_)
                            .in_(
                                mbs16_8.as_pointer(),
                                (c8.as_pointer() as Ptr<i8>),
                                ___from_end,
                                ___from_next,
                                ___to,
                                ___to_end,
                                c16p.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            if ((*c32x.borrow()) < 128_u32) {
                assert!(
                    ({
                        (((*c_c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 1_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 2048_u32) {
                assert!(
                    ({
                        (((*c_c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 2_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c_c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 3_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c_c8p.borrow()).clone() - (c8.as_pointer() as Ptr<i8>)) as i64 == 4_i64)
                    })
                );
            }
            if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64
                            == 1_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64
                            == 2_i64)
                    })
                );
            }
            let mut i: i32 = 0;
            'loop_: while ({ (i as i64) } < {
                ((*c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64
            }) {
                (*c16c.borrow_mut())[(2 * i) as usize] =
                    { ((((*c16.borrow())[(i) as usize] as i32) >> 8) as i8) };
                (*c16c.borrow_mut())[((2 * i) + 1) as usize] =
                    { ((*c16.borrow())[(i) as usize] as i8) };
                i.prefix_inc();
            }
            let c_c16cp: Value<Ptr<i8>> =
                Rc::new(RefCell::new((c16c.as_pointer() as Ptr<i8>).offset(
                    ((((*c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64) * 2_i64)
                        as isize,
                )));
            assert!(
                ({
                    ((({
                        let ___from_end: Ptr<i8> = (*c_c16cp.borrow()).clone();
                        let ___from_next: Ptr<Ptr<i8>> = c_c16cp.as_pointer();
                        let ___to: Ptr<u32> = (c32.as_pointer());
                        let ___to_end: Ptr<u32> = (c32.as_pointer()).offset((1) as isize);
                        ((*f32_16.upgrade().deref())
                            as std___codecvt_abstract_base_char32_t__char____mbstate_t_)
                            .in_(
                                mbs32_16.as_pointer(),
                                (c16c.as_pointer() as Ptr<i8>),
                                ___from_end,
                                ___from_next,
                                ___to,
                                ___to_end,
                                c32p.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c_c16cp.borrow()).clone() - (c16c.as_pointer() as Ptr<i8>)) as i64
                            == 2_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c_c16cp.borrow()).clone() - (c16c.as_pointer() as Ptr<i8>)) as i64
                            == 4_i64)
                    })
                );
            }
            assert!(({ (((*c32p.borrow()).clone() - (c32.as_pointer())) as i64 == 1_i64) }));
            assert!(({ ((*c32.borrow()) == (*c32x.borrow())) }));
            assert!(
                ({
                    ((({
                        let ___from: Ptr<u32> = (c32x.as_pointer());
                        let ___from_end: Ptr<u32> = (c32x.as_pointer()).offset((1) as isize);
                        let ___to: Ptr<u8> = (c8t.as_pointer() as Ptr<u8>);
                        let ___to_end: Ptr<u8> = (c8t.as_pointer() as Ptr<u8>).offset((4) as isize);
                        ((*f32_8t.upgrade().deref())
                            as std___codecvt_abstract_base_char32_t__char8_t____mbstate_t_)
                            .out(
                                mbs32_8t.as_pointer(),
                                ___from,
                                ___from_end,
                                c_c32p.as_pointer(),
                                ___to,
                                ___to_end,
                                c8tp.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            assert!(({ (((*c_c32p.borrow()).clone() - (c32x.as_pointer())) as i64 == 1_i64) }));
            if ((*c32x.borrow()) < 128_u32) {
                assert!(
                    ({
                        (((*c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64 == 1_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 2048_u32) {
                assert!(
                    ({
                        (((*c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64 == 2_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64 == 3_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64 == 4_i64)
                    })
                );
            }
            (*c_c8tp.borrow_mut()) = (*c8tp.borrow()).clone();
            assert!(
                ({
                    ((({
                        let ___from_end: Ptr<u8> = (*c_c8tp.borrow()).clone();
                        let ___from_next: Ptr<Ptr<u8>> = c_c8tp.as_pointer();
                        let ___to: Ptr<u16> = (c16.as_pointer() as Ptr<u16>);
                        let ___to_end: Ptr<u16> =
                            (c16.as_pointer() as Ptr<u16>).offset((2) as isize);
                        ((*f16_8t.upgrade().deref())
                            as std___codecvt_abstract_base_char16_t__char8_t____mbstate_t_)
                            .in_(
                                mbs16_8t.as_pointer(),
                                (c8t.as_pointer() as Ptr<u8>),
                                ___from_end,
                                ___from_next,
                                ___to,
                                ___to_end,
                                c16p.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            if ((*c32x.borrow()) < 128_u32) {
                assert!(
                    ({
                        (((*c_c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64
                            == 1_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 2048_u32) {
                assert!(
                    ({
                        (((*c_c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64
                            == 2_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c_c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64
                            == 3_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c_c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64
                            == 4_i64)
                    })
                );
            }
            if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64
                            == 1_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64
                            == 2_i64)
                    })
                );
            }
            (*c_c16p.borrow_mut()) = (*c16p.borrow()).clone();
            assert!(
                ({
                    ((({
                        let ___from_end: Ptr<u16> = (*c_c16p.borrow()).clone();
                        let ___from_next: Ptr<Ptr<u16>> = c_c16p.as_pointer();
                        let ___to: Ptr<u8> = (c8t.as_pointer() as Ptr<u8>);
                        let ___to_end: Ptr<u8> = (c8t.as_pointer() as Ptr<u8>).offset((4) as isize);
                        ((*f16_8t.upgrade().deref())
                            as std___codecvt_abstract_base_char16_t__char8_t____mbstate_t_)
                            .out(
                                mbs16_8t.as_pointer(),
                                (c16.as_pointer() as Ptr<u16>),
                                ___from_end,
                                ___from_next,
                                ___to,
                                ___to_end,
                                c8tp.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c_c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64
                            == 1_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c_c16p.borrow()).clone() - (c16.as_pointer() as Ptr<u16>)) as i64
                            == 2_i64)
                    })
                );
            }
            if ((*c32x.borrow()) < 128_u32) {
                assert!(
                    ({
                        (((*c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64 == 1_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 2048_u32) {
                assert!(
                    ({
                        (((*c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64 == 2_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64 == 3_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64 == 4_i64)
                    })
                );
            }
            (*c_c8tp.borrow_mut()) = (*c8tp.borrow()).clone();
            assert!(
                ({
                    ((({
                        let ___from_end: Ptr<u8> = (*c_c8tp.borrow()).clone();
                        let ___from_next: Ptr<Ptr<u8>> = c_c8tp.as_pointer();
                        let ___to: Ptr<u32> = (c32.as_pointer());
                        let ___to_end: Ptr<u32> = (c32.as_pointer()).offset((1) as isize);
                        ((*f32_8t.upgrade().deref())
                            as std___codecvt_abstract_base_char32_t__char8_t____mbstate_t_)
                            .in_(
                                mbs32_8t.as_pointer(),
                                (c8t.as_pointer() as Ptr<u8>),
                                ___from_end,
                                ___from_next,
                                ___to,
                                ___to_end,
                                c32p.as_pointer(),
                            )
                    }) as i32)
                        == (std_codecvt_base_result_ok as i32))
                })
            );
            if ((*c32x.borrow()) < 128_u32) {
                assert!(
                    ({
                        (((*c_c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64
                            == 1_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 2048_u32) {
                assert!(
                    ({
                        (((*c_c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64
                            == 2_i64)
                    })
                );
            } else if ((*c32x.borrow()) < 65536_u32) {
                assert!(
                    ({
                        (((*c_c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64
                            == 3_i64)
                    })
                );
            } else {
                assert!(
                    ({
                        (((*c_c8tp.borrow()).clone() - (c8t.as_pointer() as Ptr<u8>)) as i64
                            == 4_i64)
                    })
                );
            }
            assert!(({ (((*c32p.borrow()).clone() - (c32.as_pointer())) as i64 == 1_i64) }));
            assert!(({ ((*c32.borrow()) == (*c32x.borrow())) }));
        }
        (*c32x.borrow_mut()).prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
