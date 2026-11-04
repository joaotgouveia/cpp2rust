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
    {
        let inf1: Value<std_basic_istringstream_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"abc").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                std_basic_istringstream_char__std_char_traits_char___std_allocator_char__::new_1(
                    { __tmp_0.as_pointer() },
                    None,
                )
            }));
        let inf2: Value<std_basic_istringstream_char__std_char_traits_char___std_allocator_char__> =
            Rc::new(RefCell::new({
                let __tmp_1: Value<Vec<i8>> = Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"def").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                }));
                std_basic_istringstream_char__std_char_traits_char___std_allocator_char__::new_1(
                    { __tmp_1.as_pointer() },
                    None,
                )
            }));
        let i1: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            std_istreambuf_iterator_char__std_char_traits_char__::new_1({ inf1.as_pointer() }),
        ));
        let i2: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            std_istreambuf_iterator_char__std_char_traits_char__::new_1({ inf2.as_pointer() }),
        ));
        let i3: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::null()));
        let i4: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::null()));
        let i5: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::<::std::fs::File>::null()));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i1.borrow()).clone();
                    (*i1.borrow()).equal(___b)
                })
            })
        );
        assert!(({ ({ (*i1.borrow()).equal((*i2.borrow()).clone(),) }) }));
        assert!(({ !({ (*i1.borrow()).equal((*i3.borrow()).clone(),) }) }));
        assert!(({ !({ (*i1.borrow()).equal((*i4.borrow()).clone(),) }) }));
        assert!(({ !({ (*i1.borrow()).equal((*i5.borrow()).clone(),) }) }));
        assert!(({ ({ (*i2.borrow()).equal((*i1.borrow()).clone(),) }) }));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i2.borrow()).clone();
                    (*i2.borrow()).equal(___b)
                })
            })
        );
        assert!(({ !({ (*i2.borrow()).equal((*i3.borrow()).clone(),) }) }));
        assert!(({ !({ (*i2.borrow()).equal((*i4.borrow()).clone(),) }) }));
        assert!(({ !({ (*i2.borrow()).equal((*i5.borrow()).clone(),) }) }));
        assert!(({ !({ (*i3.borrow()).equal((*i1.borrow()).clone(),) }) }));
        assert!(({ !({ (*i3.borrow()).equal((*i2.borrow()).clone(),) }) }));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i3.borrow()).clone();
                    (*i3.borrow()).equal(___b)
                })
            })
        );
        assert!(({ ({ (*i3.borrow()).equal((*i4.borrow()).clone(),) }) }));
        assert!(({ ({ (*i3.borrow()).equal((*i5.borrow()).clone(),) }) }));
        assert!(({ !({ (*i4.borrow()).equal((*i1.borrow()).clone(),) }) }));
        assert!(({ !({ (*i4.borrow()).equal((*i2.borrow()).clone(),) }) }));
        assert!(({ ({ (*i4.borrow()).equal((*i3.borrow()).clone(),) }) }));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i4.borrow()).clone();
                    (*i4.borrow()).equal(___b)
                })
            })
        );
        assert!(({ ({ (*i4.borrow()).equal((*i5.borrow()).clone(),) }) }));
        assert!(({ !({ (*i5.borrow()).equal((*i1.borrow()).clone(),) }) }));
        assert!(({ !({ (*i5.borrow()).equal((*i2.borrow()).clone(),) }) }));
        assert!(({ ({ (*i5.borrow()).equal((*i3.borrow()).clone(),) }) }));
        assert!(({ ({ (*i5.borrow()).equal((*i4.borrow()).clone(),) }) }));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i5.borrow()).clone();
                    (*i5.borrow()).equal(___b)
                })
            })
        );
    }
    {
        let inf1: Value<
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_2: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes =
                    Ptr::<i32>::from_string_literal(&[97 as i32, 98 as i32, 99 as i32, 0 as i32])
                        .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { __tmp_2.as_pointer() } , None , )
        }));
        let inf2: Value<
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = Rc::new(RefCell::new({
            let __tmp_3: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i32>::from_string_literal(&[
                    100 as i32, 101 as i32, 102 as i32, 0 as i32,
                ])
                .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            std_basic_istringstream_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { __tmp_3.as_pointer() } , None , )
        }));
        let i1: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            std_istreambuf_iterator_wchar_t__std_char_traits_wchar_t__::new_1({
                inf1.as_pointer()
            }),
        ));
        let i2: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            std_istreambuf_iterator_wchar_t__std_char_traits_wchar_t__::new_1({
                inf2.as_pointer()
            }),
        ));
        let i3: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::null()));
        let i4: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::null()));
        let i5: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::<::std::fs::File>::null()));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i1.borrow()).clone();
                    (*i1.borrow()).equal(___b)
                })
            })
        );
        assert!(({ ({ (*i1.borrow()).equal((*i2.borrow()).clone(),) }) }));
        assert!(({ !({ (*i1.borrow()).equal((*i3.borrow()).clone(),) }) }));
        assert!(({ !({ (*i1.borrow()).equal((*i4.borrow()).clone(),) }) }));
        assert!(({ !({ (*i1.borrow()).equal((*i5.borrow()).clone(),) }) }));
        assert!(({ ({ (*i2.borrow()).equal((*i1.borrow()).clone(),) }) }));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i2.borrow()).clone();
                    (*i2.borrow()).equal(___b)
                })
            })
        );
        assert!(({ !({ (*i2.borrow()).equal((*i3.borrow()).clone(),) }) }));
        assert!(({ !({ (*i2.borrow()).equal((*i4.borrow()).clone(),) }) }));
        assert!(({ !({ (*i2.borrow()).equal((*i5.borrow()).clone(),) }) }));
        assert!(({ !({ (*i3.borrow()).equal((*i1.borrow()).clone(),) }) }));
        assert!(({ !({ (*i3.borrow()).equal((*i2.borrow()).clone(),) }) }));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i3.borrow()).clone();
                    (*i3.borrow()).equal(___b)
                })
            })
        );
        assert!(({ ({ (*i3.borrow()).equal((*i4.borrow()).clone(),) }) }));
        assert!(({ ({ (*i3.borrow()).equal((*i5.borrow()).clone(),) }) }));
        assert!(({ !({ (*i4.borrow()).equal((*i1.borrow()).clone(),) }) }));
        assert!(({ !({ (*i4.borrow()).equal((*i2.borrow()).clone(),) }) }));
        assert!(({ ({ (*i4.borrow()).equal((*i3.borrow()).clone(),) }) }));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i4.borrow()).clone();
                    (*i4.borrow()).equal(___b)
                })
            })
        );
        assert!(({ ({ (*i4.borrow()).equal((*i5.borrow()).clone(),) }) }));
        assert!(({ !({ (*i5.borrow()).equal((*i1.borrow()).clone(),) }) }));
        assert!(({ !({ (*i5.borrow()).equal((*i2.borrow()).clone(),) }) }));
        assert!(({ ({ (*i5.borrow()).equal((*i3.borrow()).clone(),) }) }));
        assert!(({ ({ (*i5.borrow()).equal((*i4.borrow()).clone(),) }) }));
        assert!(
            ({
                ({
                    let ___b: Ptr<::std::fs::File> = (*i5.borrow()).clone();
                    (*i5.borrow()).equal(___b)
                })
            })
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
