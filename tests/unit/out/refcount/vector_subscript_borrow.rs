extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static next_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn bump_1(mut v: Ptr<std_vector_int__std_allocator_int__>, mut x: ()) -> () {
    {
        let __rhs = x;
        elem!((v), 0_()).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (*next_0.with(Value::clone).borrow_mut()).postfix_inc();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new({
        let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
        std_vector_int__std_allocator_int__::new_1({ 4_() }, { __tmp_0.as_pointer() }, None)
    }));
    let items: Value<std_vector_unsigned_int__std_allocator_unsigned_int__> = Rc::new(
        RefCell::new(std_vector_unsigned_int__std_allocator_unsigned_int__::new_1({ 3_() }, None)),
    );
    let mut i: () = 0_();
    'loop_: while (i < ({ (*items.borrow()).size() })) {
        let __rhs = (i).wrapping_mul(2_());
        (*items.borrow_mut())[i] = __rhs;
        i.prefix_inc();
    }
    {
        let rhs_0 = ({ (*items.borrow())[1_()] }).wrapping_add(5_());
        (*items.borrow_mut())[1_()] = rhs_0
    };
    (*items.borrow_mut())[2_()].postfix_inc();
    unimplemented!();
    elem!(v.as_pointer(), { (*v.borrow())[0_()] }).write(3_());
    let __rhs = ({ (*v.borrow())[1_()] } + 4_());
    elem!(v.as_pointer(), { (*v.borrow())[1_()] }).write(__rhs);
    unimplemented!();
    let mut p: Ptr<()> = (v.as_pointer().offset(0_()));
    elem!(v.as_pointer(), (p.read())).write(9_());
    unimplemented!();
    elem!(v.as_pointer(), ({ bump_1((v.as_pointer()), 10_(),) })).write(2_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = next_0.with(|_| ());
}
