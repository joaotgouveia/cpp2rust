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
        let t: Value<std_regex_traits_char_> =
            Rc::new(RefCell::new(<std_regex_traits_char_>::default()));
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"w").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"alnum").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"alpha").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"blank").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"cntrl").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('\n' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"digit").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('\n' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"graph").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('\n' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"lower").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('\n' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"print").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('\n' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"punct").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('\n' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"space").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('\n' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"upper").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('\n' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"xdigit").to_c_bytes();
            __bytes.push(0_i8);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('\n' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('_' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('a' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('Z' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('5' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((' ' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('-' as i8), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_char___RegexMask = ({
                        let ___first: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i8> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype(('@' as i8), ___f)
                })
            })
        );
    }
    {
        let t: Value<std_regex_traits_wchar_t_> =
            Rc::new(RefCell::new(<std_regex_traits_wchar_t_>::default()));
        let s: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[119 as i32, 0 as i32]).to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                97 as i32, 108 as i32, 110 as i32, 117 as i32, 109 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                97 as i32, 108 as i32, 112 as i32, 104 as i32, 97 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                98 as i32, 108 as i32, 97 as i32, 110 as i32, 107 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                99 as i32, 110 as i32, 116 as i32, 114 as i32, 108 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((10 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                100 as i32, 105 as i32, 103 as i32, 105 as i32, 116 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((10 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                103 as i32, 114 as i32, 97 as i32, 112 as i32, 104 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((10 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                108 as i32, 111 as i32, 119 as i32, 101 as i32, 114 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((10 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                112 as i32, 114 as i32, 105 as i32, 110 as i32, 116 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((10 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                112 as i32, 117 as i32, 110 as i32, 99 as i32, 116 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((10 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                115 as i32, 112 as i32, 97 as i32, 99 as i32, 101 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((10 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                117 as i32, 112 as i32, 112 as i32, 101 as i32, 114 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((10 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
        {
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                120 as i32, 100 as i32, 105 as i32, 103 as i32, 105 as i32, 116 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            s.as_pointer().write(__bytes);
            s.as_pointer()
        };
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((10 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((95 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((97 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((90 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                ({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((53 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((32 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((45 as i32), ___f)
                })
            })
        );
        assert!(
            ({
                !({
                    let ___f: std_regex_traits_wchar_t___RegexMask = ({
                        let ___first: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay());
                        let ___last: RandomAccessIterator<i32> =
                            RandomAccessIterator::Vec(s.decay().to_last());
                        (*t.borrow()).lookup_classname(___first, ___last, None)
                    });
                    (*t.borrow()).isctype((64 as i32), ___f)
                })
            })
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
