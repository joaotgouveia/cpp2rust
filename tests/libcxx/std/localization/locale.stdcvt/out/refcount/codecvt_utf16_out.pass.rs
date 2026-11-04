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
pub struct TestHelper_char32_t__4_ {}
impl TestHelper_char32_t__4_ {
    pub fn test() {
        {
            let c: Value<std_codecvt_utf16_char32_t__1114111__0_> = Rc::new(RefCell::new(
                <std_codecvt_utf16_char32_t__1114111__0_>::default(),
            ));
            let w: Value<u32> = Rc::new(RefCell::new(262147_u32));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-40_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-64_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (16_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (5_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 86_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (3_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_char32_t__4096__0_> = Rc::new(RefCell::new(
                <std_codecvt_utf16_char32_t__4096__0_>::default(),
            ));
            let w: Value<u32> = Rc::new(RefCell::new(262147_u32));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
            assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
            assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
            assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
            assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 86_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_char32_t__1114111__std_generate_header_> =
                Rc::new(RefCell::new(
                    <std_codecvt_utf16_char32_t__1114111__std_generate_header_>::default(),
                ));
            let w: Value<u32> = Rc::new(RefCell::new(262147_u32));
            let n: Value<Box<[i8]>> =
                Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((6) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-40_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-64_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (16_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (5_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 86_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (3_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_char32_t__1114111__std_little_endian_> = Rc::new(
                RefCell::new(<std_codecvt_utf16_char32_t__1114111__std_little_endian_>::default()),
            );
            let w: Value<u32> = Rc::new(RefCell::new(262147_u32));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-40_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-64_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (16_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (5_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 86_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (3_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_char32_t__4096__std_little_endian_> = Rc::new(
                RefCell::new(<std_codecvt_utf16_char32_t__4096__std_little_endian_>::default()),
            );
            let w: Value<u32> = Rc::new(RefCell::new(262147_u32));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
            assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
            assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
            assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
            assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 86_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_char32_t__1114111__3_> = Rc::new(RefCell::new(
                <std_codecvt_utf16_char32_t__1114111__3_>::default(),
            ));
            let w: Value<u32> = Rc::new(RefCell::new(262147_u32));
            let n: Value<Box<[i8]>> =
                Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((6) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-40_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-64_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (16_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (5_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 86_u32;
            r = ({
                let ___from: Ptr<u32> = (w.as_pointer());
                let ___from_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (3_i8 as i32)) }));
        }
    }
}
impl CppMove for TestHelper_char32_t__4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHelper_wchar_t__4_ {}
impl TestHelper_wchar_t__4_ {
    pub fn test() {
        {
            let c: Value<std_codecvt_utf16_wchar_t__1114111__0_> = Rc::new(RefCell::new(
                <std_codecvt_utf16_wchar_t__1114111__0_>::default(),
            ));
            let w: Value<i32> = Rc::new(RefCell::new(262147_i32));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-40_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-64_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (16_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (5_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 86_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (3_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_wchar_t__4096__0_> = Rc::new(RefCell::new(
                <std_codecvt_utf16_wchar_t__4096__0_>::default(),
            ));
            let w: Value<i32> = Rc::new(RefCell::new(262147_i32));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
            assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
            assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
            assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
            assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 86_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_wchar_t__1114111__std_generate_header_> = Rc::new(
                RefCell::new(<std_codecvt_utf16_wchar_t__1114111__std_generate_header_>::default()),
            );
            let w: Value<i32> = Rc::new(RefCell::new(262147_i32));
            let n: Value<Box<[i8]>> =
                Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((6) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-40_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-64_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (16_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (5_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 86_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (3_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_wchar_t__1114111__std_little_endian_> = Rc::new(
                RefCell::new(<std_codecvt_utf16_wchar_t__1114111__std_little_endian_>::default()),
            );
            let w: Value<i32> = Rc::new(RefCell::new(262147_i32));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-40_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-64_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (16_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (5_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 86_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (3_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_wchar_t__4096__std_little_endian_> = Rc::new(
                RefCell::new(<std_codecvt_utf16_wchar_t__4096__std_little_endian_>::default()),
            );
            let w: Value<i32> = Rc::new(RefCell::new(262147_i32));
            let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
            assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
            assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
            assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
            assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
            (*w.borrow_mut()) = 86_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((2) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (0_i8 as i32)) }));
        }
        {
            let c: Value<std_codecvt_utf16_wchar_t__1114111__3_> = Rc::new(RefCell::new(
                <std_codecvt_utf16_wchar_t__1114111__3_>::default(),
            ));
            let w: Value<i32> = Rc::new(RefCell::new(262147_i32));
            let n: Value<Box<[i8]>> =
                Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8])));
            let wp: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
            let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
            let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            let mut r: std_codecvt_base_result = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((6) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (-40_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (-64_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 4101_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (16_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (5_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 1107_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (4_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (83_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (3_i8 as i32)) }));
            (*w.borrow_mut()) = 86_i32;
            r = ({
                let ___from: Ptr<i32> = (w.as_pointer());
                let ___from_end: Ptr<i32> = (w.as_pointer()).offset((1) as isize);
                let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
                let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
                ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                    m.as_pointer(),
                    ___from,
                    ___from_end,
                    wp.as_pointer(),
                    ___to,
                    ___to_end,
                    np.as_pointer(),
                )
            });
            assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
            assert!(
                ({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) })
            );
            assert!(
                ({
                    ({ (*np.borrow()).clone() } == {
                        (n.as_pointer() as Ptr<i8>).offset((4) as isize)
                    })
                })
            );
            assert!(({ (((*n.borrow())[(1) as usize] as i32) == (-2_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(0) as usize] as i32) == (-1_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(3) as usize] as i32) == (0_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(2) as usize] as i32) == (86_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(5) as usize] as i32) == (-36_i8 as i32)) }));
            assert!(({ (((*n.borrow())[(4) as usize] as i32) == (3_i8 as i32)) }));
        }
    }
}
impl CppMove for TestHelper_wchar_t__4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    ({ TestHelper_char32_t__4_::test() });
    ({ TestHelper_wchar_t__4_::test() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
