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
impl CppEq for Key {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        KeyImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for Key {
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
    let map : Value<std_unordered_map_Key__int__std_hash_Key___std_equal_to_Key___std_allocator_std_pair_const_Key__int___ > = Rc::new(RefCell::new(<std_unordered_map_Key__int__std_hash_Key___std_equal_to_Key___std_allocator_std_pair_const_Key__int___  >::default() )) ;
    let it: Value<std___detail__Node_iterator_std_pair_const_Key__int___false__true_> =
        Rc::new(RefCell::new(
            ({
                let ___x: Value<Key> = Rc::new(RefCell::new({
                    let __tmp_0: Value<i32> = Rc::new(RefCell::new(0));
                    Key::new({ __tmp_0.as_pointer() })
                }));
                (*map.borrow()).find_68(___x.as_pointer())
            }),
        ));
    assert!((*it.borrow()) == ({ (*map.borrow()).end_30() }));
    let result: Value<(
        Value<std___detail__Node_iterator_std_pair_const_Key__int___false__true_>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        ({
            let ___x: Value<(Value<Key>, Value<i32>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new({
                    let __tmp_1: Value<i32> = Rc::new(RefCell::new(0));
                    Key::new({ __tmp_1.as_pointer() })
                })),
                Rc::new(RefCell::new(42)),
            )));
            (*map.borrow()).insert_45_struct_std_pair_struct_Key__int_(___x.as_pointer())
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
    return 0;
}
pub trait KeyImpl {
    fn operator_eq(&self, _a0: Ptr<Key>) -> bool;
}
impl KeyImpl for Ptr<Key> {
    fn operator_eq(&self, _a0: Ptr<Key>) -> bool {
        return true;
    }
}
pub fn __cpp2rust_init_globals() {}
