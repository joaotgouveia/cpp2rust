extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<Vec<Vec<Vec<libc::c_char>>>> = Vec::new();
    {
        let __init = Vec::new();
        v.push(__init)
    };
    {
        let __init = Vec::new();
        v.push(__init)
    };
    let mut row: Vec<Vec<libc::c_char>> = Vec::new();
    {
        let __a1 = {
            let __s = c"a".as_ptr();
            std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
                .to_vec()
        };
        row.push(__a1)
    };
    {
        let __a1 = {
            let __s = c"bc".as_ptr();
            std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
                .to_vec()
        };
        row.push(__a1)
    };
    (*(&mut v).as_mut_ptr().add(0_usize as usize)).push((row).clone());
    {
        let __a1 = {
            let __s = c"def".as_ptr();
            std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
                .to_vec()
        };
        row.push(__a1)
    };
    (*(&mut v).as_mut_ptr().add(1_usize as usize)).push((row).clone());
    {
        let __init = Vec::new();
        (*(&mut v).as_mut_ptr().add(1_usize as usize)).push(__init)
    };
    {
        let __a1 = {
            let mut __tmp2 = (*(&mut (*((*(&mut v).as_mut_ptr().add(0_usize as usize))
                .first_mut()
                .unwrap())))
                .as_mut_ptr()
                .add(1_usize as usize))
            .clone();
            __tmp2.pop();
            let __from = c"x".as_ptr();
            __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                __from,
                (0..).position(|i| *__from.add(i) == 0).unwrap(),
            ));
            __tmp2.push(0);
            __tmp2
        };
        (*((*(&mut v).as_mut_ptr().add(1_usize as usize))
            .last_mut()
            .unwrap()))
        .push(__a1)
    };
    assert!(
        (((*((*(&mut v).as_mut_ptr().add(0_usize as usize))
            .first_mut()
            .unwrap()))
        .len())
            == (2_usize))
    );
    assert!(
        (((*((*(&mut v).as_mut_ptr().add(1_usize as usize))
            .first_mut()
            .unwrap()))
        .len())
            == (3_usize))
            && ((*(&mut (*((*(&mut v).as_mut_ptr().add(1_usize as usize))
                .first_mut()
                .unwrap())))
                .as_mut_ptr()
                .add(2_usize as usize))
                == {
                    let __s = c"def".as_ptr();
                    std::slice::from_raw_parts(
                        __s,
                        (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1,
                    )
                    .to_vec()
                })
    );
    assert!(
        (*(&mut (*((*(&mut v).as_mut_ptr().add(1_usize as usize))
            .last_mut()
            .unwrap())))
            .as_mut_ptr()
            .add(0_usize as usize))
            == {
                let __s = c"bcx".as_ptr();
                std::slice::from_raw_parts(
                    __s,
                    (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1,
                )
                .to_vec()
            }
    );
    let mut copy: Vec<Vec<Vec<Vec<libc::c_char>>>> = v.clone();
    (*(&mut (*((*(&mut copy).as_mut_ptr().add(0_usize as usize))
        .first_mut()
        .unwrap())))
        .as_mut_ptr()
        .add(0_usize as usize)) = {
        let __s = c"z".as_ptr();
        std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
            .to_vec()
    };
    (*if 0_usize as usize
        >= (*(&mut (*((*(&mut v).as_mut_ptr().add(0_usize as usize))
            .first_mut()
            .unwrap())))
            .as_mut_ptr()
            .add(1_usize as usize))
        .len()
            - 1
    {
        panic!("out of bounds access")
    } else {
        (&mut (*(&mut (*((*(&mut v).as_mut_ptr().add(0_usize as usize))
            .first_mut()
            .unwrap())))
            .as_mut_ptr()
            .add(1_usize as usize))[0_usize as usize] as *mut libc::c_char)
    }) = ('B' as libc::c_char);
    assert!(
        ((*(&mut (*((*(&mut v).as_mut_ptr().add(0_usize as usize))
            .first_mut()
            .unwrap())))
            .as_mut_ptr()
            .add(0_usize as usize))
            == {
                let __s = c"a".as_ptr();
                std::slice::from_raw_parts(
                    __s,
                    (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1,
                )
                .to_vec()
            })
            && ((*(&mut (*((*(&mut copy).as_mut_ptr().add(0_usize as usize))
                .first_mut()
                .unwrap())))
                .as_mut_ptr()
                .add(0_usize as usize))
                == {
                    let __s = c"z".as_ptr();
                    std::slice::from_raw_parts(
                        __s,
                        (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1,
                    )
                    .to_vec()
                })
    );
    assert!(
        ((*(&mut (*((*(&mut v).as_mut_ptr().add(0_usize as usize))
            .first_mut()
            .unwrap())))
            .as_mut_ptr()
            .add(1_usize as usize))
            == {
                let __s = c"Bc".as_ptr();
                std::slice::from_raw_parts(
                    __s,
                    (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1,
                )
                .to_vec()
            })
            && ((*(&mut (*((*(&mut copy).as_mut_ptr().add(0_usize as usize))
                .first_mut()
                .unwrap())))
                .as_mut_ptr()
                .add(1_usize as usize))
                == {
                    let __s = c"bc".as_ptr();
                    std::slice::from_raw_parts(
                        __s,
                        (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1,
                    )
                    .to_vec()
                })
    );
    (*(&mut v).as_mut_ptr().add(1_usize as usize)).remove(0);
    assert!(
        (*(&mut (*((*(&mut v).as_mut_ptr().add(1_usize as usize))
            .first_mut()
            .unwrap())))
            .as_mut_ptr()
            .add(0_usize as usize))
            == {
                let __s = c"bcx".as_ptr();
                std::slice::from_raw_parts(
                    __s,
                    (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1,
                )
                .to_vec()
            }
    );
    assert!(
        (((*((*(&mut copy).as_mut_ptr().add(1_usize as usize))
            .first_mut()
            .unwrap()))
        .len())
            == (3_usize))
    );
    assert!(((row.len()) == (3_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
