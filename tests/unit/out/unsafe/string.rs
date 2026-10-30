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
    let mut empty: Vec<libc::c_char> = vec![libc::c_char::default()];
    assert!(((empty.len() - 1) == (0_usize)));
    let mut s1: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"hello".as_ptr(),
        libcc2rs::c_str_len(c"hello".as_ptr()) + 1,
    )
    .to_vec();
    assert!(((s1.len() - 1) == (5_usize)));
    assert!(((s1.len() - 1) == (s1.len() - 1)));
    assert!(
        (((*(&mut s1).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('h' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut s1).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('e' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut s1).as_mut_ptr().add(2_usize as usize)) as i32)
            == (('l' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut s1).as_mut_ptr().add(3_usize as usize)) as i32)
            == (('l' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut s1).as_mut_ptr().add(4_usize as usize)) as i32)
            == (('o' as libc::c_char) as i32))
    );
    assert!(
        s1 == std::slice::from_raw_parts(
            c"hello".as_ptr(),
            libcc2rs::c_str_len(c"hello".as_ptr()) + 1
        )
        .to_vec()
    );
    assert!(
        !(s1 == std::slice::from_raw_parts(
            c"world".as_ptr(),
            libcc2rs::c_str_len(c"world".as_ptr()) + 1
        )
        .to_vec())
    );
    assert!(
        !(!(s1
            == std::slice::from_raw_parts(
                c"hello".as_ptr(),
                libcc2rs::c_str_len(c"hello".as_ptr()) + 1
            )
            .to_vec()))
    );
    let mut p1: *const libc::c_char = (s1.as_mut_ptr()).cast_const();
    assert!((((*p1.offset((0) as isize)) as i32) == (('h' as libc::c_char) as i32)));
    assert!((((*p1.offset((1) as isize)) as i32) == (('e' as libc::c_char) as i32)));
    assert!((((*p1.offset((2) as isize)) as i32) == (('l' as libc::c_char) as i32)));
    assert!((((*p1.offset((3) as isize)) as i32) == (('l' as libc::c_char) as i32)));
    assert!((((*p1.offset((4) as isize)) as i32) == (('o' as libc::c_char) as i32)));
    let mut s2: Vec<libc::c_char> = vec![('a' as libc::c_char); (10_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    let mut p2: *const libc::c_char = (s2.as_mut_ptr()).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s2.len() - 1)) {
        assert!(
            (((*p2.offset((i) as isize)) as i32) == (('a' as libc::c_char) as i32))
                && (((*(&mut s2).as_mut_ptr().add((i as usize) as usize)) as i32)
                    == (('a' as libc::c_char) as i32))
        );
        i.prefix_inc();
    }
    assert!(((s2.len() - 1) == (10_usize)));
    assert!(((s2.len() - 1) == (s2.len() - 1)));
    (*(&mut s2).as_mut_ptr().add(0_usize as usize)) = ('b' as libc::c_char);
    (*(&mut s2).as_mut_ptr().add(1_usize as usize)) = ('c' as libc::c_char);
    assert!(
        (((*(&mut s2).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut s2).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('c' as libc::c_char) as i32))
    );
    let mut i: u32 = 2_u32;
    'loop_: while ((i as usize) < (s2.len() - 1)) {
        assert!(
            (((*p2.offset((i) as isize)) as i32) == (('a' as libc::c_char) as i32))
                && (((*(&mut s2).as_mut_ptr().add((i as usize) as usize)) as i32)
                    == (('a' as libc::c_char) as i32))
        );
        i.prefix_inc();
    }
    let mut s3: Vec<libc::c_char> = {
        let mut __tmp1 = s2
            [(2_usize) as usize..::std::cmp::min((2_usize + 5_usize) as usize, s2.len() - 1)]
            .to_vec();
        __tmp1.push((0 as libc::c_char));
        __tmp1
    };
    assert!(((s3.len() - 1) == (5_usize)));
    assert!(((s3.len() - 1) == (s3.len() - 1)));
    let mut p3: *const libc::c_char = (s3.as_mut_ptr()).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s3.len() - 1)) {
        assert!(
            (((*p3.offset((i) as isize)) as i32)
                == ((*(&mut s3).as_mut_ptr().add((i as usize) as usize)) as i32))
        );
        i.prefix_inc();
    }
    let mut s4: Vec<libc::c_char> = {
        let mut __tmp1 = s1[(1_usize) as usize
            ..::std::cmp::min(
                (1_usize + {
                    let __n = s1.len().saturating_sub(1);
                    let __end = usize::MAX.saturating_add(1).min(__n);
                    let __lookup = ::std::slice::from_raw_parts(
                        c"l".as_ptr(),
                        libcc2rs::c_str_len(c"l".as_ptr()),
                    );
                    match s1
                        .iter()
                        .take(__end)
                        .rposition(|__c| __lookup.contains(__c))
                    {
                        Some(__idx) => __idx,
                        None => usize::MAX,
                    }
                }) as usize,
                s1.len() - 1,
            )]
            .to_vec();
        __tmp1.push((0 as libc::c_char));
        __tmp1
    };
    assert!(((s4.len() - 1) == (3_usize)));
    assert!(((s4.len() - 1) == (s4.len() - 1)));
    let mut p4: *const libc::c_char = (s4.as_mut_ptr()).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s4.len() - 1)) {
        assert!(
            (((*p4.offset((i) as isize)) as i32)
                == ((*(&mut s4).as_mut_ptr().add((i as usize) as usize)) as i32))
        );
        i.prefix_inc();
    }
    let mut s5: Vec<libc::c_char> = {
        let mut __tmp2 = s1.clone();
        __tmp2.pop();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            c", world".as_ptr(),
            libcc2rs::c_str_len(c", world".as_ptr()),
        ));
        __tmp2.push((0 as libc::c_char));
        __tmp2
    };
    assert!(((s5.len() - 1) == (12_usize)));
    assert!(((s5.len() - 1) == (s5.len() - 1)));
    let mut p5: *const libc::c_char = (s5.as_mut_ptr()).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s5.len() - 1)) {
        assert!(
            (((*p5.offset((i) as isize)) as i32)
                == ((*(&mut s5).as_mut_ptr().add((i as usize) as usize)) as i32))
        );
        i.prefix_inc();
    }
    let mut arr: [libc::c_char; 7] = [
        ('b' as libc::c_char),
        ('a' as libc::c_char),
        ('r' as libc::c_char),
        (' ' as libc::c_char),
        ('f' as libc::c_char),
        ('o' as libc::c_char),
        ('o' as libc::c_char),
    ];
    let mut string: Vec<libc::c_char> =
        std::slice::from_raw_parts((arr.as_mut_ptr()).cast_const(), 3_usize as usize)
            .to_vec()
            .iter()
            .copied()
            .chain(std::iter::once((0 as libc::c_char)))
            .collect();
    assert!(((string.len() - 1) == (3_usize)));
    assert!(
        (((*(&mut string).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut string).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut string).as_mut_ptr().add(2_usize as usize)) as i32)
            == (('r' as libc::c_char) as i32))
    );
    assert!(
        string
            == std::slice::from_raw_parts(
                c"bar".as_ptr(),
                libcc2rs::c_str_len(c"bar".as_ptr()) + 1
            )
            .to_vec()
    );
    {
        string.pop();
        string.resize((3_usize) as usize, (0 as libc::c_char));
        string.push((0 as libc::c_char))
    };
    assert!(((string.len() - 1) == (3_usize)));
    assert!(
        (((*(&mut string).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut string).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut string).as_mut_ptr().add(2_usize as usize)) as i32)
            == (('r' as libc::c_char) as i32))
    );
    assert!(
        string
            == std::slice::from_raw_parts(
                c"bar".as_ptr(),
                libcc2rs::c_str_len(c"bar".as_ptr()) + 1
            )
            .to_vec()
    );
    {
        string.pop();
        string.resize((5_usize) as usize, (0 as libc::c_char));
        string.push((0 as libc::c_char))
    };
    assert!(((string.len() - 1) == (5_usize)));
    assert!(
        (((*(&mut string).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut string).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut string).as_mut_ptr().add(2_usize as usize)) as i32)
            == (('r' as libc::c_char) as i32))
    );
    assert!((((*(&mut string).as_mut_ptr().add(3_usize as usize)) as i32) == (0)));
    assert!((((*(&mut string).as_mut_ptr().add(4_usize as usize)) as i32) == (0)));
    (*(&mut string).as_mut_ptr().add(3_usize as usize)) = ('a' as libc::c_char);
    (*(&mut string).as_mut_ptr().add(4_usize as usize)) = ('b' as libc::c_char);
    assert!(
        (((*(&mut string).as_mut_ptr().add(3_usize as usize)) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut string).as_mut_ptr().add(4_usize as usize)) as i32)
            == (('b' as libc::c_char) as i32))
    );
    (*(&mut string).as_mut_ptr().add(3_usize as usize)) = (0 as libc::c_char);
    (*(&mut string).as_mut_ptr().add(4_usize as usize)) = (0 as libc::c_char);
    {
        string.pop();
        string.resize((4_usize) as usize, (0 as libc::c_char));
        string.push((0 as libc::c_char))
    };
    assert!(((string.len() - 1) == (4_usize)));
    assert!(
        (((*(&mut string).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut string).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut string).as_mut_ptr().add(2_usize as usize)) as i32)
            == (('r' as libc::c_char) as i32))
    );
    assert!((((*(&mut string).as_mut_ptr().add(3_usize as usize)) as i32) == (0)));
    let mut result: Vec<libc::c_char> = {
        let mut __tmp2 = string.clone();
        __tmp2.pop();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            c" foo".as_ptr(),
            libcc2rs::c_str_len(c" foo".as_ptr()),
        ));
        __tmp2.push((0 as libc::c_char));
        __tmp2
    };
    assert!(((result.len() - 1) == (8_usize)));
    assert!(
        (((*(&mut result).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut result).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut result).as_mut_ptr().add(2_usize as usize)) as i32)
            == (('r' as libc::c_char) as i32))
    );
    assert!((((*(&mut result).as_mut_ptr().add(3_usize as usize)) as i32) == (0)));
    assert!(
        (((*(&mut result).as_mut_ptr().add(4_usize as usize)) as i32)
            == ((' ' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut result).as_mut_ptr().add(5_usize as usize)) as i32)
            == (('f' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut result).as_mut_ptr().add(6_usize as usize)) as i32)
            == (('o' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut result).as_mut_ptr().add(7_usize as usize)) as i32)
            == (('o' as libc::c_char) as i32))
    );
    let mut substr_0: Vec<libc::c_char> = {
        let mut __tmp1 = result
            [(5_usize) as usize..::std::cmp::min((5_usize + 3_usize) as usize, result.len() - 1)]
            .to_vec();
        __tmp1.push((0 as libc::c_char));
        __tmp1
    };
    assert!(((substr_0.len() - 1) == (3_usize)));
    assert!(
        (((*(&mut substr_0).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('f' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut substr_0).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('o' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut substr_0).as_mut_ptr().add(2_usize as usize)) as i32)
            == (('o' as libc::c_char) as i32))
    );
    let mut substr_1: Vec<libc::c_char> = {
        let mut __tmp1 = result
            [(0_usize) as usize..::std::cmp::min((0_usize + 5_usize) as usize, result.len() - 1)]
            .to_vec();
        __tmp1.push((0 as libc::c_char));
        __tmp1
    };
    assert!(((substr_1.len() - 1) == (5_usize)));
    assert!(
        (((*(&mut substr_1).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut substr_1).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut substr_1).as_mut_ptr().add(2_usize as usize)) as i32)
            == (('r' as libc::c_char) as i32))
    );
    assert!((((*(&mut substr_1).as_mut_ptr().add(3_usize as usize)) as i32) == (0)));
    assert!(
        (((*(&mut substr_1).as_mut_ptr().add(4_usize as usize)) as i32)
            == ((' ' as libc::c_char) as i32))
    );
    let mut substr_2: Vec<libc::c_char> = {
        let mut __tmp1 = result
            [(0_usize) as usize..::std::cmp::min((0_usize + 15_usize) as usize, result.len() - 1)]
            .to_vec();
        __tmp1.push((0 as libc::c_char));
        __tmp1
    };
    assert!(((substr_2.len() - 1) == (8_usize)));
    assert!(
        (((*(&mut substr_2).as_mut_ptr().add(0_usize as usize)) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut substr_2).as_mut_ptr().add(1_usize as usize)) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut substr_2).as_mut_ptr().add(2_usize as usize)) as i32)
            == (('r' as libc::c_char) as i32))
    );
    assert!((((*(&mut substr_2).as_mut_ptr().add(3_usize as usize)) as i32) == (0)));
    assert!(
        (((*(&mut substr_2).as_mut_ptr().add(4_usize as usize)) as i32)
            == ((' ' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut substr_2).as_mut_ptr().add(5_usize as usize)) as i32)
            == (('f' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut substr_2).as_mut_ptr().add(6_usize as usize)) as i32)
            == (('o' as libc::c_char) as i32))
    );
    assert!(
        (((*(&mut substr_2).as_mut_ptr().add(7_usize as usize)) as i32)
            == (('o' as libc::c_char) as i32))
    );
    let mut pos: usize = {
        let __n = result.len().saturating_sub(1);
        let __end = usize::MAX.saturating_add(1).min(__n);
        let __lookup =
            ::std::slice::from_raw_parts(c"b".as_ptr(), libcc2rs::c_str_len(c"b".as_ptr()));
        match result
            .iter()
            .take(__end)
            .rposition(|__c| __lookup.contains(__c))
        {
            Some(__idx) => __idx,
            None => usize::MAX,
        }
    };
    assert!(((pos) == (0_usize)));
    pos = {
        let __n = result.len().saturating_sub(1);
        let __end = usize::MAX.saturating_add(1).min(__n);
        let __lookup =
            ::std::slice::from_raw_parts(c"f".as_ptr(), libcc2rs::c_str_len(c"f".as_ptr()));
        match result
            .iter()
            .take(__end)
            .rposition(|__c| __lookup.contains(__c))
        {
            Some(__idx) => __idx,
            None => usize::MAX,
        }
    };
    assert!(((pos) == (5_usize)));
    pos = {
        let __n = result.len().saturating_sub(1);
        let __end = usize::MAX.saturating_add(1).min(__n);
        let __lookup =
            ::std::slice::from_raw_parts(c"o".as_ptr(), libcc2rs::c_str_len(c"o".as_ptr()));
        match result
            .iter()
            .take(__end)
            .rposition(|__c| __lookup.contains(__c))
        {
            Some(__idx) => __idx,
            None => usize::MAX,
        }
    };
    assert!(((pos) == (7_usize)));
    pos = {
        let __n = result.len().saturating_sub(1);
        let __end = usize::MAX.saturating_add(1).min(__n);
        let __lookup =
            ::std::slice::from_raw_parts(c"x".as_ptr(), libcc2rs::c_str_len(c"x".as_ptr()));
        match result
            .iter()
            .take(__end)
            .rposition(|__c| __lookup.contains(__c))
        {
            Some(__idx) => __idx,
            None => usize::MAX,
        }
    };
    assert!(((pos) == ((-1_i64 as u64) as usize)));
    pos = {
        let __hay = &result[..result.len() - 1];
        let __needle =
            ::std::slice::from_raw_parts(c"f".as_ptr(), libcc2rs::c_str_len(c"f".as_ptr()));
        (0_usize..=__hay.len().saturating_sub(__needle.len()))
            .find(|&__i| __hay[__i..].starts_with(__needle))
            .unwrap_or(usize::MAX)
    };
    assert!(((pos) == (5_usize)));
    pos = {
        let __hay = &result[..result.len() - 1];
        let __needle =
            ::std::slice::from_raw_parts(c"oo".as_ptr(), libcc2rs::c_str_len(c"oo".as_ptr()));
        (0_usize..=__hay.len().saturating_sub(__needle.len()))
            .find(|&__i| __hay[__i..].starts_with(__needle))
            .unwrap_or(usize::MAX)
    };
    assert!(((pos) == (6_usize)));
    pos = {
        let __hay = &result[..result.len() - 1];
        let __needle =
            ::std::slice::from_raw_parts(c"x".as_ptr(), libcc2rs::c_str_len(c"x".as_ptr()));
        (0_usize..=__hay.len().saturating_sub(__needle.len()))
            .find(|&__i| __hay[__i..].starts_with(__needle))
            .unwrap_or(usize::MAX)
    };
    assert!(((pos) == ((-1_i64 as u64) as usize)));
    let mut string_to_cast: Vec<libc::c_char> =
        std::slice::from_raw_parts(c"cast".as_ptr(), libcc2rs::c_str_len(c"cast".as_ptr()) + 1)
            .to_vec();
    let mut output_data: *mut u8 =
        ((&mut (*(&mut string_to_cast).as_mut_ptr().add(0_usize as usize)) as *mut libc::c_char)
            as *mut libc::c_char as *mut u8);
    assert!((((*output_data) as i32) == (('c' as libc::c_char) as i32)));
    assert!((((*output_data.offset((1) as isize)) as i32) == (('a' as libc::c_char) as i32)));
    assert!((((*output_data.offset((2) as isize)) as i32) == (('s' as libc::c_char) as i32)));
    assert!((((*output_data.offset((3) as isize)) as i32) == (('t' as libc::c_char) as i32)));
    let mut diff: isize = (-3_i32 as isize);
    assert!(((diff) < (0_isize)));
    diff += 6_isize;
    assert!(((diff) == (3_isize)));
    let mut t0: usize = (s1.len() - 1);
    let mut t1: usize = (t0).wrapping_add(((*p1) as usize));
    let mut t2: usize = (t1).wrapping_add((s2.len() - 1));
    let mut t3: usize = (t2).wrapping_add((s3.len() - 1));
    let mut t4: usize = (t3).wrapping_add((s4.len() - 1));
    assert!((((t4).wrapping_add((s5.len() - 1))) == (139_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
