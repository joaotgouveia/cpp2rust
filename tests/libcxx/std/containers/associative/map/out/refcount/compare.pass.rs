extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
impl Key {
    pub fn new(_a0: Ptr<i32>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, ptr)]
#[byte_size(1)]
pub struct Key {}
impl CppLess for Key {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        KeyImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Key {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(KeyImpl::operator_lt(__lhs, __rhs.clone()))
            && !(KeyImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
impl CppMove for Key {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0() -> bool {
    {
        let m_empty: Value<Map<Key, (Value<Key>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
        let m_contains: Value<Map<Key, (Value<Key>, Value<i32>)>> =
            Rc::new(RefCell::new(Map::new()));
        Map::subscript_move(
            m_contains.as_pointer(),
            Rc::new(RefCell::new({
                let __tmp_0: Value<i32> = Rc::new(RefCell::new(0));
                Key::new({ __tmp_0.as_pointer() })
            }))
            .as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(42);
        let it: Value<BidirectionalIterator<(Value<Key>, Value<i32>)>> = Rc::new(RefCell::new(
            match (Rc::new(RefCell::new({
                let __tmp_2: Value<i32> = Rc::new(RefCell::new(0));
                Key::new({ __tmp_2.as_pointer() })
            }))
            .as_pointer(),)
            {
                (__tmp_1,) => m_empty
                    .as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.find(&(__tmp_1).clone())),
            },
        ));
        assert!(
            (*it.borrow())
                == m_empty
                    .as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.end())
        );
        (*it.borrow_mut()) = match (Rc::new(RefCell::new({
            let __tmp_4: Value<i32> = Rc::new(RefCell::new(0));
            Key::new({ __tmp_4.as_pointer() })
        }))
        .as_pointer(),)
        {
            (__tmp_3,) => m_contains
                .as_pointer()
                .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.find(&(__tmp_3).clone())),
        };
        assert!(
            (*it.borrow())
                != m_contains
                    .as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.end())
        );
    }
    {
        let map: Value<Map<Key, (Value<Key>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
        let result: Value<(
            Value<BidirectionalIterator<(Value<Key>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new(
            ({
                let ___x: Value<(Value<Key>, Value<i32>)> = Rc::new(RefCell::new((
                    Rc::new(RefCell::new({
                        let __tmp_5: Value<i32> = Rc::new(RefCell::new(0));
                        Key::new({ __tmp_5.as_pointer() })
                    })),
                    Rc::new(RefCell::new(42)),
                )));
                (*map.borrow()).insert_55_struct_std_pair_struct_Key__int_(___x.as_pointer())
            }),
        ));
        assert!(({ ((*(*result.borrow()).1.borrow()) as bool) }));
        assert!(
            ({
                ((*(*(*(*result.borrow()).0.borrow())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .borrow())
                    == 42)
            })
        );
        let result2: Value<(
            Value<BidirectionalIterator<(Value<Key>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new(
            ({
                let ___x: Value<(Value<Key>, Value<i32>)> = Rc::new(RefCell::new((
                    Rc::new(RefCell::new({
                        let __tmp_6: Value<i32> = Rc::new(RefCell::new(0));
                        Key::new({ __tmp_6.as_pointer() })
                    })),
                    Rc::new(RefCell::new(43)),
                )));
                (*map.borrow()).insert_55_struct_std_pair_struct_Key__int_(___x.as_pointer())
            }),
        ));
        assert!(({ !(*(*result2.borrow()).1.borrow()) }));
        assert!(
            ((Map::subscript_move(
                map.as_pointer(),
                Rc::new(RefCell::new({
                    let __tmp_7: Value<i32> = Rc::new(RefCell::new(0));
                    Key::new({ __tmp_7.as_pointer() })
                }))
                .as_pointer(),
                |__k| cpp_move!(__k)
            )
            .read())
                == 42)
        );
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
    ({ test_0() });
    return 0;
}
pub trait KeyImpl {
    fn operator_lt(&self, _a0: Ptr<Key>) -> bool;
}
impl KeyImpl for Ptr<Key> {
    fn operator_lt(&self, _a0: Ptr<Key>) -> bool {
        return false;
    }
}
pub fn __cpp2rust_init_globals() {}
