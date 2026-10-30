extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<Vec<Vec<Vec<i8>>>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __init = Vec::new();
        (*v.borrow_mut()).push(__init)
    };
    {
        let __init = Vec::new();
        (*v.borrow_mut()).push(__init)
    };
    let row: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"bc").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    v.decay()
        .offset(0_usize as isize)
        .with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.push((*row.borrow()).clone()));
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"def").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    v.decay()
        .offset(1_usize as isize)
        .with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.push((*row.borrow()).clone()));
    {
        let __init = Vec::new();
        v.decay()
            .offset(1_usize as isize)
            .with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.push(__init))
    };
    {
        let __a1 = {
            let mut __r = (*v
                .decay()
                .offset(0_usize as isize)
                .decay()
                .decay()
                .offset(1_usize as isize)
                .upgrade()
                .deref())
            .clone();
            __r.pop();
            Ptr::<i8>::from_string_literal(b"x").with_c_str(|__s| __r.extend_from_slice(__s));
            __r.push(0_i8);
            __r
        };
        v.decay()
            .offset(1_usize as isize)
            .decay()
            .to_last()
            .with_mut(|__v: &mut Vec<Vec<i8>>| __v.push(__a1))
    };
    assert!(((*v.decay().offset(0_usize as isize).decay().upgrade().deref()).len() == 2_usize));
    assert!(
        ((*v.decay().offset(1_usize as isize).decay().upgrade().deref()).len() == 3_usize)
            && (Ptr::<i8>::from_string_literal(b"def").with_c_str(|__s| (*v
                .decay()
                .offset(1_usize as isize)
                .decay()
                .decay()
                .offset(2_usize as isize)
                .upgrade()
                .deref())[..(*v
                .decay()
                .offset(1_usize as isize)
                .decay()
                .decay()
                .offset(2_usize as isize)
                .upgrade()
                .deref())
            .len()
            .saturating_sub(1)]
                == *__s))
    );
    assert!(Ptr::<i8>::from_string_literal(b"bcx").with_c_str(|__s| {
        (*v.decay()
            .offset(1_usize as isize)
            .decay()
            .to_last()
            .decay()
            .offset(0_usize as isize)
            .upgrade()
            .deref())[..(*v
            .decay()
            .offset(1_usize as isize)
            .decay()
            .to_last()
            .decay()
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    let copy: Value<Vec<Vec<Vec<Vec<i8>>>>> = Rc::new(RefCell::new((*v.borrow()).clone()));
    copy.decay()
        .offset(0_usize as isize)
        .decay()
        .decay()
        .offset(0_usize as isize)
        .write({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"z").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        });
    if 0_usize as usize
        >= (*v
            .decay()
            .offset(0_usize as isize)
            .decay()
            .decay()
            .offset(1_usize as isize)
            .upgrade()
            .deref())
        .len()
        .saturating_sub(1)
    {
        panic!("out of bounds access")
    } else {
        v.decay()
            .offset(0_usize as isize)
            .decay()
            .decay()
            .offset(1_usize as isize)
            .decay()
            .offset(0_usize as isize)
    }
    .write(('B' as i8));
    assert!(
        (Ptr::<i8>::from_string_literal(b"a").with_c_str(|__s| (*v
            .decay()
            .offset(0_usize as isize)
            .decay()
            .decay()
            .offset(0_usize as isize)
            .upgrade()
            .deref())[..(*v
            .decay()
            .offset(0_usize as isize)
            .decay()
            .decay()
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .len()
        .saturating_sub(1)]
            == *__s))
            && (Ptr::<i8>::from_string_literal(b"z").with_c_str(|__s| (*copy
                .decay()
                .offset(0_usize as isize)
                .decay()
                .decay()
                .offset(0_usize as isize)
                .upgrade()
                .deref())[..(*copy
                .decay()
                .offset(0_usize as isize)
                .decay()
                .decay()
                .offset(0_usize as isize)
                .upgrade()
                .deref())
            .len()
            .saturating_sub(1)]
                == *__s))
    );
    assert!(
        (Ptr::<i8>::from_string_literal(b"Bc").with_c_str(|__s| (*v
            .decay()
            .offset(0_usize as isize)
            .decay()
            .decay()
            .offset(1_usize as isize)
            .upgrade()
            .deref())[..(*v
            .decay()
            .offset(0_usize as isize)
            .decay()
            .decay()
            .offset(1_usize as isize)
            .upgrade()
            .deref())
        .len()
        .saturating_sub(1)]
            == *__s))
            && (Ptr::<i8>::from_string_literal(b"bc").with_c_str(|__s| (*copy
                .decay()
                .offset(0_usize as isize)
                .decay()
                .decay()
                .offset(1_usize as isize)
                .upgrade()
                .deref())[..(*copy
                .decay()
                .offset(0_usize as isize)
                .decay()
                .decay()
                .offset(1_usize as isize)
                .upgrade()
                .deref())
            .len()
            .saturating_sub(1)]
                == *__s))
    );
    v.decay()
        .offset(1_usize as isize)
        .with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.remove(0));
    assert!(Ptr::<i8>::from_string_literal(b"bcx").with_c_str(|__s| {
        (*v.decay()
            .offset(1_usize as isize)
            .decay()
            .decay()
            .offset(0_usize as isize)
            .upgrade()
            .deref())[..(*v
            .decay()
            .offset(1_usize as isize)
            .decay()
            .decay()
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(
        ((*copy
            .decay()
            .offset(1_usize as isize)
            .decay()
            .upgrade()
            .deref())
        .len()
            == 3_usize)
    );
    assert!(((*row.borrow()).len() == 3_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
