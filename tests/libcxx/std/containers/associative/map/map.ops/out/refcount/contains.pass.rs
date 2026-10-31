extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(
    mut bad: i8,
    args_1: (Value<i8>, Value<i32>),
    args_2: (Value<i8>, Value<i32>),
    args_3: (Value<i8>, Value<i32>),
    args_4: (Value<i8>, Value<i32>),
) -> bool {
    let args_1: Value<(Value<i8>, Value<i32>)> = Rc::new(RefCell::new(args_1));
    let args_2: Value<(Value<i8>, Value<i32>)> = Rc::new(RefCell::new(args_2));
    let args_3: Value<(Value<i8>, Value<i32>)> = Rc::new(RefCell::new(args_3));
    let args_4: Value<(Value<i8>, Value<i32>)> = Rc::new(RefCell::new(args_4));
    let map: Value<Map<i8, (Value<i8>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let pairs: Value<Box<[(Value<i8>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
        {
            let __a0 = (*args_1.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_2.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_3.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_4.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
    ])));
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i32>)>),
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i32>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i8>, Value<i32>)> = (p).clone();
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i32>)>),
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i32>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i8>, Value<i32>)> = (p).clone();
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub fn test_1(
    mut bad: i8,
    args_1: (Value<i8>, Value<i8>),
    args_2: (Value<i8>, Value<i8>),
    args_3: (Value<i8>, Value<i8>),
    args_4: (Value<i8>, Value<i8>),
) -> bool {
    let args_1: Value<(Value<i8>, Value<i8>)> = Rc::new(RefCell::new(args_1));
    let args_2: Value<(Value<i8>, Value<i8>)> = Rc::new(RefCell::new(args_2));
    let args_3: Value<(Value<i8>, Value<i8>)> = Rc::new(RefCell::new(args_3));
    let args_4: Value<(Value<i8>, Value<i8>)> = Rc::new(RefCell::new(args_4));
    let map: Value<Map<i8, (Value<i8>, Value<i8>)>> = Rc::new(RefCell::new(Map::new()));
    let pairs: Value<Box<[(Value<i8>, Value<i8>)]>> = Rc::new(RefCell::new(Box::new([
        {
            let __a0 = (*args_1.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_2.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_3.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_4.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
    ])));
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i8>)>),
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i8>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i8>, Value<i8>)> = (p).clone();
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i8>)>),
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i8>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i8>, Value<i8>)> = (p).clone();
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub fn test_2(
    mut bad: i32,
    args_1: (Value<i32>, Value<E>),
    args_2: (Value<i32>, Value<E>),
    args_3: (Value<i32>, Value<E>),
    args_4: (Value<i32>, Value<E>),
) -> bool {
    let args_1: Value<(Value<i32>, Value<E>)> = Rc::new(RefCell::new(args_1));
    let args_2: Value<(Value<i32>, Value<E>)> = Rc::new(RefCell::new(args_2));
    let args_3: Value<(Value<i32>, Value<E>)> = Rc::new(RefCell::new(args_3));
    let args_4: Value<(Value<i32>, Value<E>)> = Rc::new(RefCell::new(args_4));
    let map: Value<Map<i32, (Value<i32>, Value<E>)>> = Rc::new(RefCell::new(Map::new()));
    let pairs: Value<Box<[(Value<i32>, Value<E>)]>> = Rc::new(RefCell::new(Box::new([
        {
            let __a0 = (*args_1.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new(<E as CppFrom<E>>::cpp_from(
                    __a0.1.as_pointer(),
                ))),
            )
        },
        {
            let __a0 = (*args_2.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new(<E as CppFrom<E>>::cpp_from(
                    __a0.1.as_pointer(),
                ))),
            )
        },
        {
            let __a0 = (*args_3.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new(<E as CppFrom<E>>::cpp_from(
                    __a0.1.as_pointer(),
                ))),
            )
        },
        {
            let __a0 = (*args_4.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new(<E as CppFrom<E>>::cpp_from(
                    __a0.1.as_pointer(),
                ))),
            )
        },
    ])));
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i32>, Value<E>)>),
        (pairs.as_pointer() as Ptr<(Value<i32>, Value<E>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i32>, Value<E>)> = (p).clone();
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i32>, Value<E>)>),
        (pairs.as_pointer() as Ptr<(Value<i32>, Value<E>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i32>, Value<E>)> = (p).clone();
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub fn test_3(
    mut bad: i8,
    args_1: (Value<i8>, Value<i32>),
    args_2: (Value<i8>, Value<i32>),
    args_3: (Value<i8>, Value<i32>),
    args_4: (Value<i8>, Value<i32>),
) -> bool {
    let args_1: Value<(Value<i8>, Value<i32>)> = Rc::new(RefCell::new(args_1));
    let args_2: Value<(Value<i8>, Value<i32>)> = Rc::new(RefCell::new(args_2));
    let args_3: Value<(Value<i8>, Value<i32>)> = Rc::new(RefCell::new(args_3));
    let args_4: Value<(Value<i8>, Value<i32>)> = Rc::new(RefCell::new(args_4));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let pairs: Value<Box<[(Value<i8>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
        {
            let __a0 = (*args_1.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_2.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_3.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_4.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
    ])));
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i32>)>),
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i32>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i8>, Value<i32>)> = (p).clone();
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i32>)>),
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i32>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i8>, Value<i32>)> = (p).clone();
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub fn test_4(
    mut bad: i8,
    args_1: (Value<i8>, Value<i8>),
    args_2: (Value<i8>, Value<i8>),
    args_3: (Value<i8>, Value<i8>),
    args_4: (Value<i8>, Value<i8>),
) -> bool {
    let args_1: Value<(Value<i8>, Value<i8>)> = Rc::new(RefCell::new(args_1));
    let args_2: Value<(Value<i8>, Value<i8>)> = Rc::new(RefCell::new(args_2));
    let args_3: Value<(Value<i8>, Value<i8>)> = Rc::new(RefCell::new(args_3));
    let args_4: Value<(Value<i8>, Value<i8>)> = Rc::new(RefCell::new(args_4));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let pairs: Value<Box<[(Value<i8>, Value<i8>)]>> = Rc::new(RefCell::new(Box::new([
        {
            let __a0 = (*args_1.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_2.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_3.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
        {
            let __a0 = (*args_4.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
            )
        },
    ])));
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i8>)>),
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i8>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i8>, Value<i8>)> = (p).clone();
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i8>)>),
        (pairs.as_pointer() as Ptr<(Value<i8>, Value<i8>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i8>, Value<i8>)> = (p).clone();
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
pub fn test_5(
    mut bad: i32,
    args_1: (Value<i32>, Value<E>),
    args_2: (Value<i32>, Value<E>),
    args_3: (Value<i32>, Value<E>),
    args_4: (Value<i32>, Value<E>),
) -> bool {
    let args_1: Value<(Value<i32>, Value<E>)> = Rc::new(RefCell::new(args_1));
    let args_2: Value<(Value<i32>, Value<E>)> = Rc::new(RefCell::new(args_2));
    let args_3: Value<(Value<i32>, Value<E>)> = Rc::new(RefCell::new(args_3));
    let args_4: Value<(Value<i32>, Value<E>)> = Rc::new(RefCell::new(args_4));
    let map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let pairs: Value<Box<[(Value<i32>, Value<E>)]>> = Rc::new(RefCell::new(Box::new([
        {
            let __a0 = (*args_1.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new(<E as CppFrom<E>>::cpp_from(
                    __a0.1.as_pointer(),
                ))),
            )
        },
        {
            let __a0 = (*args_2.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new(<E as CppFrom<E>>::cpp_from(
                    __a0.1.as_pointer(),
                ))),
            )
        },
        {
            let __a0 = (*args_3.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new(<E as CppFrom<E>>::cpp_from(
                    __a0.1.as_pointer(),
                ))),
            )
        },
        {
            let __a0 = (*args_4.borrow()).clone();
            (
                Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
                Rc::new(RefCell::new(<E as CppFrom<E>>::cpp_from(
                    __a0.1.as_pointer(),
                ))),
            )
        },
    ])));
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i32>, Value<E>)>),
        (pairs.as_pointer() as Ptr<(Value<i32>, Value<E>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i32>, Value<E>)> = (p).clone();
        unimplemented!();
    }
    'loop_: for p in IterRange::new(
        (pairs.as_pointer() as Ptr<(Value<i32>, Value<E>)>),
        (pairs.as_pointer() as Ptr<(Value<i32>, Value<E>)>).offset((4_i64) as isize),
    ) {
        let p: Ptr<(Value<i32>, Value<E>)> = (p).clone();
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct E {
    #[offset(0)]
    pub a: i32,
    #[offset(8)]
    pub b: f64,
    #[offset(16)]
    pub c: i8,
}
impl Default for E {
    fn default() -> Self {
        {
            E {
                a: 1,
                b: 1_f64,
                c: 1_i8,
            }
        }
    }
}
pub fn test_6() -> bool {
    {
        ({
            let _args_1: (Value<i8>, Value<i32>) = (
                Rc::new(RefCell::new(('a' as i8))),
                Rc::new(RefCell::new(10)),
            );
            let _args_2: (Value<i8>, Value<i32>) = (
                Rc::new(RefCell::new(('b' as i8))),
                Rc::new(RefCell::new(11)),
            );
            let _args_3: (Value<i8>, Value<i32>) = (
                Rc::new(RefCell::new(('c' as i8))),
                Rc::new(RefCell::new(12)),
            );
            let _args_4: (Value<i8>, Value<i32>) = (
                Rc::new(RefCell::new(('d' as i8))),
                Rc::new(RefCell::new(13)),
            );
            test_0(('e' as i8), _args_1, _args_2, _args_3, _args_4)
        });
        ({
            let _args_1: (Value<i8>, Value<i8>) = (
                Rc::new(RefCell::new(('a' as i8))),
                Rc::new(RefCell::new(('a' as i8))),
            );
            let _args_2: (Value<i8>, Value<i8>) = (
                Rc::new(RefCell::new(('b' as i8))),
                Rc::new(RefCell::new(('a' as i8))),
            );
            let _args_3: (Value<i8>, Value<i8>) = (
                Rc::new(RefCell::new(('c' as i8))),
                Rc::new(RefCell::new(('a' as i8))),
            );
            let _args_4: (Value<i8>, Value<i8>) = (
                Rc::new(RefCell::new(('d' as i8))),
                Rc::new(RefCell::new(('b' as i8))),
            );
            test_1(('e' as i8), _args_1, _args_2, _args_3, _args_4)
        });
        ({
            let _args_1: (Value<i32>, Value<E>) = (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new(<E>::default())),
            );
            let _args_2: (Value<i32>, Value<E>) = (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new(<E>::default())),
            );
            let _args_3: (Value<i32>, Value<E>) = (
                Rc::new(RefCell::new(3)),
                Rc::new(RefCell::new(<E>::default())),
            );
            let _args_4: (Value<i32>, Value<E>) = (
                Rc::new(RefCell::new(4)),
                Rc::new(RefCell::new(<E>::default())),
            );
            test_2(-1_i32, _args_1, _args_2, _args_3, _args_4)
        });
    }
    {
        ({
            let _args_1: (Value<i8>, Value<i32>) = (
                Rc::new(RefCell::new(('a' as i8))),
                Rc::new(RefCell::new(10)),
            );
            let _args_2: (Value<i8>, Value<i32>) = (
                Rc::new(RefCell::new(('b' as i8))),
                Rc::new(RefCell::new(11)),
            );
            let _args_3: (Value<i8>, Value<i32>) = (
                Rc::new(RefCell::new(('c' as i8))),
                Rc::new(RefCell::new(12)),
            );
            let _args_4: (Value<i8>, Value<i32>) = (
                Rc::new(RefCell::new(('d' as i8))),
                Rc::new(RefCell::new(13)),
            );
            test_3(('e' as i8), _args_1, _args_2, _args_3, _args_4)
        });
        ({
            let _args_1: (Value<i8>, Value<i8>) = (
                Rc::new(RefCell::new(('a' as i8))),
                Rc::new(RefCell::new(('a' as i8))),
            );
            let _args_2: (Value<i8>, Value<i8>) = (
                Rc::new(RefCell::new(('b' as i8))),
                Rc::new(RefCell::new(('a' as i8))),
            );
            let _args_3: (Value<i8>, Value<i8>) = (
                Rc::new(RefCell::new(('c' as i8))),
                Rc::new(RefCell::new(('a' as i8))),
            );
            let _args_4: (Value<i8>, Value<i8>) = (
                Rc::new(RefCell::new(('d' as i8))),
                Rc::new(RefCell::new(('b' as i8))),
            );
            test_4(('e' as i8), _args_1, _args_2, _args_3, _args_4)
        });
        ({
            let _args_1: (Value<i32>, Value<E>) = (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new(<E>::default())),
            );
            let _args_2: (Value<i32>, Value<E>) = (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new(<E>::default())),
            );
            let _args_3: (Value<i32>, Value<E>) = (
                Rc::new(RefCell::new(3)),
                Rc::new(RefCell::new(<E>::default())),
            );
            let _args_4: (Value<i32>, Value<E>) = (
                Rc::new(RefCell::new(4)),
                Rc::new(RefCell::new(<E>::default())),
            );
            test_5(-1_i32, _args_1, _args_2, _args_3, _args_4)
        });
    }
    return true;
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
    ({ test_6() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
