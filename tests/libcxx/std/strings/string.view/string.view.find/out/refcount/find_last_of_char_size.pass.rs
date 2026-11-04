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
                ('m' as i8),
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
                ('m' as i8),
                1_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"kitcj")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                0_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"qkamf")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                1_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"nhmko")
                })),
            );
            test_0(_s.as_pointer(), ('m' as i8), 2_u64, 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"tpsaf")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                4_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"lahfb")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                5_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"irkhs")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                6_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"gmfhdaipsr")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                0_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"kantesmpgj")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                1_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"odaftiegpm")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                5_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"oknlrstdpi")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                9_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"eolhfgpjqk")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                10_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"pcdrofikas")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                11_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"nbatdlmekrgcfqsophij")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                0_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"bnrpehidofmqtcksjgla")
                })),
            );
            test_0(
                _s.as_pointer(),
                ('m' as i8),
                1_u64,
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"jdmciepkaqgotsrfnhlb")
                })),
            );
            test_0(_s.as_pointer(), ('m' as i8), 10_u64, 2_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"jtdaefblsokrmhpgcnqi")
                })),
            );
            test_0(_s.as_pointer(), ('m' as i8), 19_u64, 12_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"hkbgspofltajcnedqmri")
                })),
            );
            test_0(_s.as_pointer(), ('m' as i8), 20_u64, 17_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"oselktgbcapndfjihrmq")
                })),
            );
            test_0(_s.as_pointer(), ('m' as i8), 21_u64, 18_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"")
                })),
            );
            test_2(_s.as_pointer(), ('m' as i8), npos_1.with(|rc| *rc.borrow()))
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"csope")
                })),
            );
            test_2(_s.as_pointer(), ('m' as i8), npos_1.with(|rc| *rc.borrow()))
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"gfsmthlkon")
                })),
            );
            test_2(_s.as_pointer(), ('m' as i8), 3_u64)
        });
        ({
            let _s: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(
                RefCell::new(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"laenfsbridchgotmkqpj")
                })),
            );
            test_2(_s.as_pointer(), ('m' as i8), 15_u64)
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
        const _: () = assert!(true, "sv1.find_last_of(\'i\', 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_last_of(\'i\', 1) == SV::npos");;
        const _: () = assert!(true, "sv2.find_last_of(\'a\', 0) == 0");;
        const _: () = assert!(true, "sv2.find_last_of(\'a\', 1) == 0");;
        const _: () = assert!(true, "sv2.find_last_of(\'e\', 5) == 4");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
