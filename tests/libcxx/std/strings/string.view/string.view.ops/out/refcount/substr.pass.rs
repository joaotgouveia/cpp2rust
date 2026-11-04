extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0(
    sv: std_basic_string_view_char__std_char_traits_char__,
    mut n: usize,
    mut pos: usize,
) {
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(sv));
    let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        <std_basic_string_view_char__std_char_traits_char__>::default(),
    ));
    if (pos > (({ (*sv.borrow()).size() }) as usize)) {
        return;
    }
    (*sv1.borrow_mut()) = ({ (*sv.borrow()).substr((pos as u64), (n as u64)) });
    let mut rlen: usize = ({
        let __tmp_1: Value<u64> = Rc::new(RefCell::new((n as u64)));
        (match (Rc::new(RefCell::new(
            ({ (*sv.borrow()).size() }).wrapping_sub((pos as u64)),
        ))
        .as_pointer(),)
        {
            (__tmp_0,) => {
                if CppLess::less(&(__tmp_0).clone(), &__tmp_1.as_pointer()) {
                    (__tmp_0).clone()
                } else {
                    __tmp_1.as_pointer()
                }
            }
        }
        .read())
    } as usize);
    assert!(({ ((({ (*sv1.borrow()).size() }) as usize) == rlen) }));
    let mut i: usize = 0_usize;
    'loop_: while (i < rlen) {
        assert!(
            ({
                (({ (*sv.borrow())[(pos as u64).wrapping_add((i as u64))] } as i32)
                    == ({ (*sv1.borrow())[i] } as i32))
            })
        );
        i.prefix_inc();
    }
}
pub fn test1_1(
    sv: std_basic_string_view_wchar_t__std_char_traits_wchar_t__,
    mut n: usize,
    mut pos: usize,
) {
    let sv: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(sv));
    let sv1: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> = Rc::new(
        RefCell::new(<std_basic_string_view_wchar_t__std_char_traits_wchar_t__>::default()),
    );
    if (pos > (({ (*sv.borrow()).size() }) as usize)) {
        return;
    }
    (*sv1.borrow_mut()) = ({ (*sv.borrow()).substr((pos as u64), (n as u64)) });
    let mut rlen: usize = ({
        let __tmp_3: Value<u64> = Rc::new(RefCell::new((n as u64)));
        (match (Rc::new(RefCell::new(
            ({ (*sv.borrow()).size() }).wrapping_sub((pos as u64)),
        ))
        .as_pointer(),)
        {
            (__tmp_2,) => {
                if CppLess::less(&(__tmp_2).clone(), &__tmp_3.as_pointer()) {
                    (__tmp_2).clone()
                } else {
                    __tmp_3.as_pointer()
                }
            }
        }
        .read())
    } as usize);
    assert!(({ ((({ (*sv1.borrow()).size() }) as usize) == rlen) }));
    let mut i: usize = 0_usize;
    'loop_: while (i < rlen) {
        assert!(
            ({
                ({ (*sv.borrow())[(pos as u64).wrapping_add((i as u64))] } == {
                    (*sv1.borrow())[i]
                })
            })
        );
        i.prefix_inc();
    }
}
pub fn test1_2(
    sv: std_basic_string_view_char16_t__std_char_traits_char16_t__,
    mut n: usize,
    mut pos: usize,
) {
    let sv: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(sv));
    let sv1: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> = Rc::new(
        RefCell::new(<std_basic_string_view_char16_t__std_char_traits_char16_t__>::default()),
    );
    if (pos > (({ (*sv.borrow()).size() }) as usize)) {
        return;
    }
    (*sv1.borrow_mut()) = ({ (*sv.borrow()).substr((pos as u64), (n as u64)) });
    let mut rlen: usize = ({
        let __tmp_5: Value<u64> = Rc::new(RefCell::new((n as u64)));
        (match (Rc::new(RefCell::new(
            ({ (*sv.borrow()).size() }).wrapping_sub((pos as u64)),
        ))
        .as_pointer(),)
        {
            (__tmp_4,) => {
                if CppLess::less(&(__tmp_4).clone(), &__tmp_5.as_pointer()) {
                    (__tmp_4).clone()
                } else {
                    __tmp_5.as_pointer()
                }
            }
        }
        .read())
    } as usize);
    assert!(({ ((({ (*sv1.borrow()).size() }) as usize) == rlen) }));
    let mut i: usize = 0_usize;
    'loop_: while (i < rlen) {
        assert!(
            ({
                (({ (*sv.borrow())[(pos as u64).wrapping_add((i as u64))] } as i32)
                    == ({ (*sv1.borrow())[i] } as i32))
            })
        );
        i.prefix_inc();
    }
}
pub fn test1_3(
    sv: std_basic_string_view_char32_t__std_char_traits_char32_t__,
    mut n: usize,
    mut pos: usize,
) {
    let sv: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(sv));
    let sv1: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> = Rc::new(
        RefCell::new(<std_basic_string_view_char32_t__std_char_traits_char32_t__>::default()),
    );
    if (pos > (({ (*sv.borrow()).size() }) as usize)) {
        return;
    }
    (*sv1.borrow_mut()) = ({ (*sv.borrow()).substr((pos as u64), (n as u64)) });
    let mut rlen: usize = ({
        let __tmp_7: Value<u64> = Rc::new(RefCell::new((n as u64)));
        (match (Rc::new(RefCell::new(
            ({ (*sv.borrow()).size() }).wrapping_sub((pos as u64)),
        ))
        .as_pointer(),)
        {
            (__tmp_6,) => {
                if CppLess::less(&(__tmp_6).clone(), &__tmp_7.as_pointer()) {
                    (__tmp_6).clone()
                } else {
                    __tmp_7.as_pointer()
                }
            }
        }
        .read())
    } as usize);
    assert!(({ ((({ (*sv1.borrow()).size() }) as usize) == rlen) }));
    let mut i: usize = 0_usize;
    'loop_: while (i < rlen) {
        assert!(
            ({
                ({ (*sv.borrow())[(pos as u64).wrapping_add((i as u64))] } == {
                    (*sv1.borrow())[i]
                })
            })
        );
        i.prefix_inc();
    }
}
pub fn test_4(mut s: Ptr<i8>) {
    let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({ (s).clone() }),
    ));
    ({ test1_0((*sv1.borrow()).clone(), 0_usize, 0_usize) });
    ({ test1_0((*sv1.borrow()).clone(), 1_usize, 0_usize) });
    ({ test1_0((*sv1.borrow()).clone(), 20_usize, 0_usize) });
    ({
        let _sv: std_basic_string_view_char__std_char_traits_char__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }) as usize);
        test1_0(_sv, _n, 0_usize)
    });
    ({ test1_0((*sv1.borrow()).clone(), 0_usize, 3_usize) });
    ({ test1_0((*sv1.borrow()).clone(), 2_usize, 3_usize) });
    ({ test1_0((*sv1.borrow()).clone(), 100_usize, 3_usize) });
    ({
        test1_0(
            (*sv1.borrow()).clone(),
            0_usize,
            (npos_5.with(|rc| *rc.borrow()) as usize),
        )
    });
    ({
        test1_0(
            (*sv1.borrow()).clone(),
            2_usize,
            (npos_5.with(|rc| *rc.borrow()) as usize),
        )
    });
    ({
        let _sv: std_basic_string_view_char__std_char_traits_char__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }) as usize);
        test1_0(_sv, _n, (npos_5.with(|rc| *rc.borrow()) as usize))
    });
    ({
        let _sv: std_basic_string_view_char__std_char_traits_char__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_0(_sv, _n, 0_usize)
    });
    ({
        let _sv: std_basic_string_view_char__std_char_traits_char__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_0(_sv, _n, 1_usize)
    });
    ({
        let _sv: std_basic_string_view_char__std_char_traits_char__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_0(_sv, _n, (npos_5.with(|rc| *rc.borrow()) as usize))
    });
}
pub fn test_6(mut s: Ptr<i32>) {
    let sv1: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1({ (s).clone() }),
        ));
    ({ test1_1((*sv1.borrow()).clone(), 0_usize, 0_usize) });
    ({ test1_1((*sv1.borrow()).clone(), 1_usize, 0_usize) });
    ({ test1_1((*sv1.borrow()).clone(), 20_usize, 0_usize) });
    ({
        let _sv: std_basic_string_view_wchar_t__std_char_traits_wchar_t__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }) as usize);
        test1_1(_sv, _n, 0_usize)
    });
    ({ test1_1((*sv1.borrow()).clone(), 0_usize, 3_usize) });
    ({ test1_1((*sv1.borrow()).clone(), 2_usize, 3_usize) });
    ({ test1_1((*sv1.borrow()).clone(), 100_usize, 3_usize) });
    ({
        test1_1(
            (*sv1.borrow()).clone(),
            0_usize,
            (npos_7.with(|rc| *rc.borrow()) as usize),
        )
    });
    ({
        test1_1(
            (*sv1.borrow()).clone(),
            2_usize,
            (npos_7.with(|rc| *rc.borrow()) as usize),
        )
    });
    ({
        let _sv: std_basic_string_view_wchar_t__std_char_traits_wchar_t__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }) as usize);
        test1_1(_sv, _n, (npos_7.with(|rc| *rc.borrow()) as usize))
    });
    ({
        let _sv: std_basic_string_view_wchar_t__std_char_traits_wchar_t__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_1(_sv, _n, 0_usize)
    });
    ({
        let _sv: std_basic_string_view_wchar_t__std_char_traits_wchar_t__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_1(_sv, _n, 1_usize)
    });
    ({
        let _sv: std_basic_string_view_wchar_t__std_char_traits_wchar_t__ = (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_1(_sv, _n, (npos_7.with(|rc| *rc.borrow()) as usize))
    });
}
pub fn test_8(mut s: Ptr<u16>) {
    let sv1: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_1({ (s).clone() }),
        ));
    ({ test1_2((*sv1.borrow()).clone(), 0_usize, 0_usize) });
    ({ test1_2((*sv1.borrow()).clone(), 1_usize, 0_usize) });
    ({ test1_2((*sv1.borrow()).clone(), 20_usize, 0_usize) });
    ({
        let _sv: std_basic_string_view_char16_t__std_char_traits_char16_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }) as usize);
        test1_2(_sv, _n, 0_usize)
    });
    ({ test1_2((*sv1.borrow()).clone(), 0_usize, 3_usize) });
    ({ test1_2((*sv1.borrow()).clone(), 2_usize, 3_usize) });
    ({ test1_2((*sv1.borrow()).clone(), 100_usize, 3_usize) });
    ({
        test1_2(
            (*sv1.borrow()).clone(),
            0_usize,
            (npos_9.with(|rc| *rc.borrow()) as usize),
        )
    });
    ({
        test1_2(
            (*sv1.borrow()).clone(),
            2_usize,
            (npos_9.with(|rc| *rc.borrow()) as usize),
        )
    });
    ({
        let _sv: std_basic_string_view_char16_t__std_char_traits_char16_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }) as usize);
        test1_2(_sv, _n, (npos_9.with(|rc| *rc.borrow()) as usize))
    });
    ({
        let _sv: std_basic_string_view_char16_t__std_char_traits_char16_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_2(_sv, _n, 0_usize)
    });
    ({
        let _sv: std_basic_string_view_char16_t__std_char_traits_char16_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_2(_sv, _n, 1_usize)
    });
    ({
        let _sv: std_basic_string_view_char16_t__std_char_traits_char16_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_2(_sv, _n, (npos_9.with(|rc| *rc.borrow()) as usize))
    });
}
pub fn test_10(mut s: Ptr<u32>) {
    let sv1: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
        Rc::new(RefCell::new(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_1({ (s).clone() }),
        ));
    ({ test1_3((*sv1.borrow()).clone(), 0_usize, 0_usize) });
    ({ test1_3((*sv1.borrow()).clone(), 1_usize, 0_usize) });
    ({ test1_3((*sv1.borrow()).clone(), 20_usize, 0_usize) });
    ({
        let _sv: std_basic_string_view_char32_t__std_char_traits_char32_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }) as usize);
        test1_3(_sv, _n, 0_usize)
    });
    ({ test1_3((*sv1.borrow()).clone(), 0_usize, 3_usize) });
    ({ test1_3((*sv1.borrow()).clone(), 2_usize, 3_usize) });
    ({ test1_3((*sv1.borrow()).clone(), 100_usize, 3_usize) });
    ({
        test1_3(
            (*sv1.borrow()).clone(),
            0_usize,
            (npos_11.with(|rc| *rc.borrow()) as usize),
        )
    });
    ({
        test1_3(
            (*sv1.borrow()).clone(),
            2_usize,
            (npos_11.with(|rc| *rc.borrow()) as usize),
        )
    });
    ({
        let _sv: std_basic_string_view_char32_t__std_char_traits_char32_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }) as usize);
        test1_3(_sv, _n, (npos_11.with(|rc| *rc.borrow()) as usize))
    });
    ({
        let _sv: std_basic_string_view_char32_t__std_char_traits_char32_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_3(_sv, _n, 0_usize)
    });
    ({
        let _sv: std_basic_string_view_char32_t__std_char_traits_char32_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_3(_sv, _n, 1_usize)
    });
    ({
        let _sv: std_basic_string_view_char32_t__std_char_traits_char32_t__ =
            (*sv1.borrow()).clone();
        let _n: usize = (({ (*sv1.borrow()).size() }).wrapping_add(1_u64) as usize);
        test1_3(_sv, _n, (npos_11.with(|rc| *rc.borrow()) as usize))
    });
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
    ({
        test_4 ( Ptr::<i8 >::from_string_literal(b"ABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDE" )  , )
    });
    ({ test_4(Ptr::<i8>::from_string_literal(b"ABCDE")) });
    ({ test_4(Ptr::<i8>::from_string_literal(b"a")) });
    ({ test_4(Ptr::<i8>::from_string_literal(b"")) });
    ({
        test_6(Ptr::<i32>::from_string_literal(&[
            65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32,
            68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32,
            66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32,
            69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32,
            67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32,
            65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32,
            68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32,
            66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32,
            69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32,
            67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32,
            65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32,
            68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32,
            66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32,
            69 as i32, 0 as i32,
        ]))
    });
    ({
        test_6(Ptr::<i32>::from_string_literal(&[
            65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
        ]))
    });
    ({ test_6(Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32])) });
    ({ test_6(Ptr::<i32>::from_string_literal(&[0 as i32])) });
    ({
        test_8(Ptr::<u16>::from_string_literal(&[
            65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16,
            68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16,
            66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16,
            69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16,
            67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16,
            65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16,
            68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16,
            66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16,
            69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16,
            67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16,
            65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16,
            68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16,
            66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16,
            69 as u16, 0 as u16,
        ]))
    });
    ({
        test_8(Ptr::<u16>::from_string_literal(&[
            65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
        ]))
    });
    ({ test_8(Ptr::<u16>::from_string_literal(&[97 as u16, 0 as u16])) });
    ({ test_8(Ptr::<u16>::from_string_literal(&[0 as u16])) });
    ({
        test_10(Ptr::<u32>::from_string_literal(&[
            65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32,
            68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32,
            66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32,
            69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32,
            67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32,
            65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32,
            68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32,
            66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32,
            69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32,
            67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32,
            65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32,
            68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32,
            66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32,
            69 as u32, 0 as u32,
        ]))
    });
    ({
        test_10(Ptr::<u32>::from_string_literal(&[
            65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
        ]))
    });
    ({ test_10(Ptr::<u32>::from_string_literal(&[97 as u32, 0 as u32])) });
    ({ test_10(Ptr::<u32>::from_string_literal(&[0 as u32])) });
    {
        let sv1: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_2(
                { Ptr::<i8>::from_string_literal(b"ABCDE") },
                { 5_u64 },
            ),
        ));
        {
            let sv2: Value<std_basic_string_view_char__std_char_traits_char__> =
                Rc::new(RefCell::new(({ (*sv1.borrow()).substr(0_u64, 3_u64) })));
            const _: () = assert!(true, "sv2.size() == 3");;
            const _: () = assert!(true, "sv2[0] == \'A\'");;
            const _: () = assert!(true, "sv2[1] == \'B\'");;
            const _: () = assert!(true, "sv2[2] == \'C\'");;
        }
        {
            let sv2: Value<std_basic_string_view_char__std_char_traits_char__> =
                Rc::new(RefCell::new(({ (*sv1.borrow()).substr(3_u64, 0_u64) })));
            const _: () = assert!(true, "sv2.size() == 0");;
        }
        {
            let sv2: Value<std_basic_string_view_char__std_char_traits_char__> =
                Rc::new(RefCell::new(({ (*sv1.borrow()).substr(3_u64, 3_u64) })));
            const _: () = assert!(true, "sv2.size() == 2");;
            const _: () = assert!(true, "sv2[0] == \'D\'");;
            const _: () = assert!(true, "sv2[1] == \'E\'");;
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
