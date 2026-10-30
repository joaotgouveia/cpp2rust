extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub unsafe fn sum(&self) -> i32 {
        return ((self.x) + (self.y));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub values: Vec<i32>,
    pub points: Vec<Point>,
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            values: std::mem::take(&mut (*_a0).values),
            points: std::mem::take(&mut (*_a0).points),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.values = std::mem::take(&mut (*_a0).values);
        self.points = std::mem::take(&mut (*_a0).points);
        return &mut (*(self as *mut Holder));
    }
}
pub unsafe fn push_and_index_0(v: *mut Vec<i32>) -> i32 {
    {
        let __a1 = 42;
        (*v).push(__a1)
    };
    return (((*v).len() as i32) - (1));
}
pub unsafe fn sum_ref_1(v: *const Vec<i32>) -> i32 {
    let mut s: i32 = 0;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*v).len())) {
        s += (*(&(*v)).as_ptr().add(i as usize));
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = Vec::from([1, 2, 3]);
    (*(&mut v).as_mut_ptr().add(0_usize as usize)) = 10;
    assert!(((*(&mut v).as_mut_ptr().add(0_usize as usize)) == (10)));
    (*(&mut v).as_mut_ptr().add(1_usize as usize)) += 5;
    (*(&mut v).as_mut_ptr().add(2_usize as usize)).postfix_inc();
    assert!(
        ((*(&mut v).as_mut_ptr().add(1_usize as usize)) == (7))
            && ((*(&mut v).as_mut_ptr().add(2_usize as usize)) == (4))
    );
    (*(&mut v).as_mut_ptr().add(0_usize as usize)) = (*(&mut v).as_mut_ptr().add(1_usize as usize));
    assert!(((*(&mut v).as_mut_ptr().add(0_usize as usize)) == (7)));
    (*(&mut v).as_mut_ptr().add(1_usize as usize)) = 0;
    assert!(
        ((*(&mut v)
            .as_mut_ptr()
            .add(((*(&mut v).as_mut_ptr().add(1_usize as usize)) as usize) as usize))
            == (7))
    );
    let mut i: i32 = 0;
    (*(&mut v)
        .as_mut_ptr()
        .add((i.postfix_inc() as usize) as usize)) = 3;
    assert!(((i) == (1)) && ((*(&mut v).as_mut_ptr().add(0_usize as usize)) == (3)));
    assert!(
        ((*(&mut v)
            .as_mut_ptr()
            .add(((unsafe { push_and_index_0(&mut v,) }) as usize) as usize))
            == (42))
    );
    (*(&mut v)
        .as_mut_ptr()
        .add(((unsafe { push_and_index_0(&mut v) }) as usize) as usize)) = 5;
    assert!(((v.len()) == (5_usize)) && ((*(&mut v).as_mut_ptr().add(4_usize as usize)) == (5)));
    let mut p: *mut i32 = (&mut (*(&mut v).as_mut_ptr().add(2_usize as usize)) as *mut i32);
    (*p) = 9;
    assert!(((*(&mut v).as_mut_ptr().add(2_usize as usize)) == (9)));
    assert!(((unsafe { sum_ref_1(&v,) }) == (((((3) + (0)) + (9)) + (42)) + (5))));
    let mut h: Holder = <Holder>::default();
    {
        let __a0 = 2_usize as usize;
        h.values.resize_with(__a0, || 0_i32)
    };
    (*(&mut h.values).as_mut_ptr().add(1_usize as usize)) = 6;
    {
        let __a1 = Point { x: 1, y: 2 };
        h.points.push(__a1)
    };
    (*(&mut h.points).as_mut_ptr().add(0_usize as usize)).y = 5;
    assert!(((*(&mut h.values).as_mut_ptr().add(1_usize as usize)) == (6)));
    assert!(
        ((unsafe { Point::sum(&(*(&mut h.points).as_mut_ptr().add(0_usize as usize)),) }) == (6))
    );
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    (*(&mut (*hp).values).as_mut_ptr().add(0_usize as usize)) =
        ((*(&mut (*hp).values).as_mut_ptr().add(1_usize as usize)) + (1));
    (*(&mut (*hp).points).as_mut_ptr().add(0_usize as usize)).x =
        (*(&mut (*hp).values).as_mut_ptr().add(0_usize as usize));
    assert!(
        ((*(&mut h.values).as_mut_ptr().add(0_usize as usize)) == (7))
            && (((*(&mut h.points).as_mut_ptr().add(0_usize as usize)).x) == (7))
    );
    let mut q: Point = (*(&mut h.points).as_mut_ptr().add(0_usize as usize));
    q.x = 0;
    assert!((((*(&mut h.points).as_mut_ptr().add(0_usize as usize)).x) == (7)));
    let mut grid: Vec<Vec<i32>> = Vec::new();
    {
        let __a1 = vec![0; 3_usize as usize];
        grid.push(__a1)
    };
    {
        let __a1 = vec![0; 3_usize as usize];
        grid.push(__a1)
    };
    (*(&mut (*(&mut grid).as_mut_ptr().add(1_usize as usize)))
        .as_mut_ptr()
        .add(2_usize as usize)) = 8;
    assert!(
        ((*(&mut (*(&mut grid).as_mut_ptr().add(1_usize as usize)))
            .as_mut_ptr()
            .add(2_usize as usize))
            == (8))
            && ((*(&mut (*(&mut grid).as_mut_ptr().add(0_usize as usize)))
                .as_mut_ptr()
                .add(2_usize as usize))
                == (0))
    );
    (*(&mut (*(&mut grid).as_mut_ptr().add(0_usize as usize)))
        .as_mut_ptr()
        .add(1_usize as usize)) = (*(&mut (*(&mut grid).as_mut_ptr().add(1_usize as usize)))
        .as_mut_ptr()
        .add(2_usize as usize));
    (*(&mut (*(&mut grid).as_mut_ptr().add(1_usize as usize)))
        .as_mut_ptr()
        .add(0_usize as usize)) += (*(&mut (*(&mut grid).as_mut_ptr().add(0_usize as usize)))
        .as_mut_ptr()
        .add(1_usize as usize));
    assert!(
        ((*(&mut (*(&mut grid).as_mut_ptr().add(0_usize as usize)))
            .as_mut_ptr()
            .add(1_usize as usize))
            == (8))
            && ((*(&mut (*(&mut grid).as_mut_ptr().add(1_usize as usize)))
                .as_mut_ptr()
                .add(0_usize as usize))
                == (8))
    );
    let mut a: Vec<i32> = Vec::from([4, 5, 6]);
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut a;
        __v.as_mut_ptr().add(__idx)
    }) = ((*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut a;
        __v.as_mut_ptr().add(__idx)
    }) + (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<i32> = &mut a;
        __v.as_mut_ptr().add(__idx)
    }));
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut a;
            __v.as_mut_ptr().add(__idx)
        }) == (10))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
