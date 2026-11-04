extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_const_container_0(c: Ptr<Vec<i32>>) {
    assert!(({ ({ data_1((c).clone(),) }) } == { c.decay() }));
}
pub fn test_const_container_2(c: Ptr<Vec<i32>>) {
    assert!(({ ({ data_3((c).clone(),) }) } == { c.decay() }));
}
pub fn test_const_container_4(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "");;
    assert!(
        ({
            ({ ({ data_5((*c.upgrade().deref()).clone()) }) } == {
                ({ (*c.upgrade().deref()).begin() })
            })
        })
    );
}
pub fn test_container_6(c: Ptr<Vec<i32>>) {
    assert!(({ ({ data_7((c).clone(),) }) } == { c.decay() }));
}
pub fn test_container_8(c: Ptr<Vec<i32>>) {
    assert!(({ ({ data_9((c).clone(),) }) } == { c.decay() }));
}
pub fn test_container_10(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "");;
    assert!(
        ({
            ({ ({ data_5((*c.upgrade().deref()).clone()) }) } == {
                ({ (*c.upgrade().deref()).begin() })
            })
        })
    );
}
pub fn test_const_array_11(array: Ptr<i32>) {
    const _: () = assert!(true, "");;
    assert!(({ ({ data_12((array).clone(),) }) } == { ((array).offset((0) as isize)) }));
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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(
        std::array::from_fn::<_, 1, _>(|_| 0_i32).to_vec(),
    ));
    a.decay().offset(0_usize as isize).write(3);
    let il: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([4])));
    ({ test_container_6(v.as_pointer()) });
    ({ test_container_8(a.as_pointer()) });
    ({ test_container_10(il.as_pointer()) });
    ({ test_const_container_0(v.as_pointer()) });
    ({ test_const_container_2(a.as_pointer()) });
    ({ test_const_container_4(il.as_pointer()) });
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({
            Ptr::<i8>::from_string_literal(b"ABC")
        }),
    ));
    ({ test_container_13(sv.as_pointer()) });
    ({ test_const_container_14(sv.as_pointer()) });
    ({ test_const_array_11(arrA_15.with(|v| v.as_pointer())) });
    return 0;
}
thread_local!(
    static arrA_15: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
);
pub fn __cpp2rust_init_globals() {}
