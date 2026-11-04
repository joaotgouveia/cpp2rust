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
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub fn eq(mut c1: i8, mut c2: i8) -> bool {
        return ((c1 as i32) == (c2 as i32));
    }
    pub fn find(s: Ptr<i8>, mut n: u64, a: Ptr<i8>) -> Ptr<i8> {
        let s: Value<Ptr<i8>> = Rc::new(RefCell::new(s));
        'loop_: while (n != 0) {
            if ({
                let _c1: i8 = ((*s.borrow()).read());
                let _c2: i8 = (a.read());
                constexpr_char_traits_char_::eq(_c1, _c2)
            }) {
                return (*s.borrow()).clone();
            }
            (*s.borrow_mut()).prefix_inc();
            n.prefix_dec();
        }
        return Ptr::<i8>::null();
    }
}
impl CppMove for constexpr_char_traits_char_ {
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
    {
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                0_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                1_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcde")
                })),
            );
            test_0(_s.as_pointer(), ('c' as i8), 0_u64, 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcde")
                })),
            );
            test_0(_s.as_pointer(), ('c' as i8), 1_u64, 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcde")
                })),
            );
            test_0(_s.as_pointer(), ('c' as i8), 2_u64, 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcde")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                4_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcde")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                5_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcde")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                6_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcde")
                })),
            );
            test_0(_s.as_pointer(), ('c' as i8), 0_u64, 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcde")
                })),
            );
            test_0(_s.as_pointer(), ('c' as i8), 1_u64, 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcde")
                })),
            );
            test_0(_s.as_pointer(), ('c' as i8), 5_u64, 7_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcde")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                9_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcde")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                10_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcde")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                11_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")
                })),
            );
            test_0(_s.as_pointer(), ('c' as i8), 0_u64, 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")
                })),
            );
            test_0(_s.as_pointer(), ('c' as i8), 1_u64, 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")
                })),
            );
            test_0(_s.as_pointer(), ('c' as i8), 10_u64, 12_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                19_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                20_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('c' as i8),
                21_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"")
                })),
            );
            test_2(_s.as_pointer(), ('c' as i8), npos_1.with(|rc| *rc.borrow()))
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcde")
                })),
            );
            test_2(_s.as_pointer(), ('c' as i8), 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcde")
                })),
            );
            test_2(_s.as_pointer(), ('c' as i8), 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")
                })),
            );
            test_2(_s.as_pointer(), ('c' as i8), 2_u64)
        });
    }
    {
        let sv1: Value<std_basic_string_view_char__constexpr_char_traits_char__> = Rc::new(
            RefCell::new(<std_basic_string_view_char__constexpr_char_traits_char__>::default()),
        );
        let sv2: Value<std_basic_string_view_char__constexpr_char_traits_char__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char__constexpr_char_traits_char__::new_1(
                    { Ptr::<i8>::from_string_literal(b"abcde") },
                    { 5_u64 },
                ),
            ));
        const _: () = assert!(true, "sv1.find(\'c\', 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find(\'c\', 1) == SV::npos");;
        const _: () = assert!(true, "sv2.find(\'c\', 0) == 2");;
        const _: () = assert!(true, "sv2.find(\'c\', 1) == 2");;
        const _: () = assert!(true, "sv2.find(\'c\', 2) == 2");;
        const _: () = assert!(true, "sv2.find(\'c\', 3) == SV::npos");;
        const _: () = assert!(true, "sv2.find(\'c\', 4) == SV::npos");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
