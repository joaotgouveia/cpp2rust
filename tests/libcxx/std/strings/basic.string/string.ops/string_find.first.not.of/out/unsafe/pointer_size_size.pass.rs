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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub unsafe fn test_4(s: *const (), mut str: *const (), mut pos: (), mut n: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5))) {
        unimplemented!();
    }
}
pub unsafe fn test_6(s: *const (), mut str: *const (), mut pos: (), mut n: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7))) {
        unimplemented!();
    }
}
pub unsafe fn test0_8() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"irkhs".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"kante".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"oknlr".as_ptr(),
            0_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"pcdro".as_ptr(),
            0_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"bnrpe".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jtdaefblso".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"oselktgbca".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"eqgaplhckj".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"bjahtcmnlp".as_ptr(),
            0_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"hjlcmgpket".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"htaobedqikfplcgjsmrn".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"hpqiarojkcdlsgnmfetb".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dfkaprhjloqetcsimnbg".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ihqrfebgadntlpmjksoc".as_ptr(),
            0_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ngtjfcalbseiqrphmkdo".as_ptr(),
            0_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"lbtqd".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"tboim".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"slcer".as_ptr(),
            1_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"cbjfs".as_ptr(),
            1_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"aqibs".as_ptr(),
            1_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"gtfblmqinc".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"mkqpbtdalg".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"kphatlimcd".as_ptr(),
            1_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"pblasqogic".as_ptr(),
            1_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"arosdhcfme".as_ptr(),
            1_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"blkhjeogicatqfnpdmsr".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"bmhineprjcoadgstflqk".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"djkqcmetslnghpbarfoi".as_ptr(),
            1_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"lgokshjtpbemarcdqnfi".as_ptr(),
            1_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"bqjhtkfepimcnsgrlado".as_ptr(),
            1_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"eaint".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"binja".as_ptr() },
            None,
        );
        test_4(&mut _s, c"gfsrt".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"latkm".as_ptr() },
            None,
        );
        test_4(&mut _s, c"pfsoc".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lecfr".as_ptr() },
            None,
        );
        test_4(&mut _s, c"tpflm".as_ptr(), 0_(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"eqkst".as_ptr() },
            None,
        );
        test_4(&mut _s, c"sgkec".as_ptr(), 0_(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cdafr".as_ptr() },
            None,
        );
        test_4(&mut _s, c"romds".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"prbhe".as_ptr() },
            None,
        );
        test_4(&mut _s, c"qhjistlgmr".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lbisk".as_ptr() },
            None,
        );
        test_4(&mut _s, c"pedfirsglo".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hrlpd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"aqcoslgrmk".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ehmja".as_ptr() },
            None,
        );
        test_4(&mut _s, c"dabckmepqj".as_ptr(), 0_(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mhqgd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"pqscrjthli".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tgklq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kfphdcsjqmobliagtren".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bocjs".as_ptr() },
            None,
        );
        test_4(&mut _s, c"rokpefncljibsdhqtagm".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"grbsd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"afionmkphlebtcjqsgrd".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ofjqr".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"aenmqplidhkofrjbctsg".as_ptr(),
            0_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"btlfi".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"osjmbtcadhiklegrpqnf".as_ptr(),
            0_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"clrgb".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tjmek".as_ptr() },
            None,
        );
        test_4(&mut _s, c"osmia".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bgstp".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ckonl".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hstrk".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ilcaj".as_ptr(), 1_(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kmspj".as_ptr() },
            None,
        );
        test_4(&mut _s, c"lasiq".as_ptr(), 1_(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tjboh".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kfqmr".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ilbcj".as_ptr() },
            None,
        );
        test_4(&mut _s, c"klnitfaobg".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jkngf".as_ptr() },
            None,
        );
        test_4(&mut _s, c"gjhmdlqikp".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gfcql".as_ptr() },
            None,
        );
        test_4(&mut _s, c"skbgtahqej".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dqtlg".as_ptr() },
            None,
        );
        test_4(&mut _s, c"bjsdgtlpkf".as_ptr(), 1_(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bthpg".as_ptr() },
            None,
        );
        test_4(&mut _s, c"bjgfmnlkio".as_ptr(), 1_(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dgsnq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"lbhepotfsjdqigcnamkr".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rmfhp".as_ptr() },
            None,
        );
        test_4(&mut _s, c"tebangckmpsrqdlfojhi".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jfdam".as_ptr() },
            None,
        );
        test_4(&mut _s, c"joflqbdkhtegimscpanr".as_ptr(), 1_(), 10_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"edapb".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"adpmcohetfbsrjinlqkg".as_ptr(),
            1_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"brfsm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"iacldqjpfnogbsrhmetk".as_ptr(),
            1_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ndrhl".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mrecp".as_ptr() },
            None,
        );
        test_4(&mut _s, c"otkgb".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qlasf".as_ptr() },
            None,
        );
        test_4(&mut _s, c"cqsjl".as_ptr(), 2_(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"smaqd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"dpifl".as_ptr(), 2_(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hjeni".as_ptr() },
            None,
        );
        test_4(&mut _s, c"oapht".as_ptr(), 2_(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ocmfj".as_ptr() },
            None,
        );
        test_4(&mut _s, c"cifts".as_ptr(), 2_(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hmftq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"nmsckbgalo".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fklad".as_ptr() },
            None,
        );
        test_4(&mut _s, c"tpksqhamle".as_ptr(), 2_(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dirnm".as_ptr() },
            None,
        );
        test_4(&mut _s, c"tpdrchmkji".as_ptr(), 2_(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hrgdc".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ijagfkblst".as_ptr(), 2_(), 9_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ifakg".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kpocsignjb".as_ptr(), 2_(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ebrgd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"pecqtkjsnbdrialgmohf".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rcjml".as_ptr() },
            None,
        );
        test_4(&mut _s, c"aiortphfcmkjebgsndql".as_ptr(), 2_(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"peqmt".as_ptr() },
            None,
        );
        test_4(&mut _s, c"sdbkeamglhipojqftrcn".as_ptr(), 2_(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"frehn".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ljqncehgmfktroapidbs".as_ptr(),
            2_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tqolf".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"rtcfodilamkbenjghqps".as_ptr(),
            2_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cjgao".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kjplq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"mabns".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"herni".as_ptr() },
            None,
        );
        test_4(&mut _s, c"bdnrp".as_ptr(), 4_(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tadrb".as_ptr() },
            None,
        );
        test_4(&mut _s, c"scidp".as_ptr(), 4_(), 2_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pkfeo".as_ptr() },
            None,
        );
        test_4(&mut _s, c"agbjl".as_ptr(), 4_(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hoser".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jfmpr".as_ptr(),
            4_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kgrsp".as_ptr() },
            None,
        );
        test_4(&mut _s, c"rbpefghsmj".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pgejb".as_ptr() },
            None,
        );
        test_4(&mut _s, c"apsfntdoqc".as_ptr(), 4_(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"thlnq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ndkjeisgcl".as_ptr(), 4_(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"nbmit".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"rnfpqatdeo".as_ptr(),
            4_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jgmib".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"bntjlqrfik".as_ptr(),
            4_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ncrfj".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kcrtmpolnaqejghsfdbi".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ncsik".as_ptr() },
            None,
        );
        test_4(&mut _s, c"lobheanpkmqidsrtcfgj".as_ptr(), 4_(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"sgbfh".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"athdkljcnreqbgpmisof".as_ptr(),
            4_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dktbn".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"qkdmjialrscpbhefgont".as_ptr(),
            4_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fthqm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dmasojntqleribkgfchp".as_ptr(),
            4_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"klopi".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dajhn".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"psthd".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jbgno".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"rpmjd".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hkjae".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dfsmk".as_ptr(),
            5_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
}
pub unsafe fn test0_9() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"irkhs".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"kante".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"oknlr".as_ptr(),
            0_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"pcdro".as_ptr(),
            0_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"bnrpe".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jtdaefblso".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"oselktgbca".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"eqgaplhckj".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"bjahtcmnlp".as_ptr(),
            0_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"hjlcmgpket".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"htaobedqikfplcgjsmrn".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"hpqiarojkcdlsgnmfetb".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dfkaprhjloqetcsimnbg".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ihqrfebgadntlpmjksoc".as_ptr(),
            0_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ngtjfcalbseiqrphmkdo".as_ptr(),
            0_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"lbtqd".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"tboim".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"slcer".as_ptr(),
            1_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"cbjfs".as_ptr(),
            1_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"aqibs".as_ptr(),
            1_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"gtfblmqinc".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"mkqpbtdalg".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"kphatlimcd".as_ptr(),
            1_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"pblasqogic".as_ptr(),
            1_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"arosdhcfme".as_ptr(),
            1_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"blkhjeogicatqfnpdmsr".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"bmhineprjcoadgstflqk".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"djkqcmetslnghpbarfoi".as_ptr(),
            1_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"lgokshjtpbemarcdqnfi".as_ptr(),
            1_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"bqjhtkfepimcnsgrlado".as_ptr(),
            1_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"eaint".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"binja".as_ptr() },
            None,
        );
        test_6(&mut _s, c"gfsrt".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"latkm".as_ptr() },
            None,
        );
        test_6(&mut _s, c"pfsoc".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lecfr".as_ptr() },
            None,
        );
        test_6(&mut _s, c"tpflm".as_ptr(), 0_(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"eqkst".as_ptr() },
            None,
        );
        test_6(&mut _s, c"sgkec".as_ptr(), 0_(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cdafr".as_ptr() },
            None,
        );
        test_6(&mut _s, c"romds".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"prbhe".as_ptr() },
            None,
        );
        test_6(&mut _s, c"qhjistlgmr".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lbisk".as_ptr() },
            None,
        );
        test_6(&mut _s, c"pedfirsglo".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hrlpd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"aqcoslgrmk".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ehmja".as_ptr() },
            None,
        );
        test_6(&mut _s, c"dabckmepqj".as_ptr(), 0_(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mhqgd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"pqscrjthli".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tgklq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kfphdcsjqmobliagtren".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bocjs".as_ptr() },
            None,
        );
        test_6(&mut _s, c"rokpefncljibsdhqtagm".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"grbsd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"afionmkphlebtcjqsgrd".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ofjqr".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"aenmqplidhkofrjbctsg".as_ptr(),
            0_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"btlfi".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"osjmbtcadhiklegrpqnf".as_ptr(),
            0_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"clrgb".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tjmek".as_ptr() },
            None,
        );
        test_6(&mut _s, c"osmia".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bgstp".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ckonl".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hstrk".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ilcaj".as_ptr(), 1_(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kmspj".as_ptr() },
            None,
        );
        test_6(&mut _s, c"lasiq".as_ptr(), 1_(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tjboh".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kfqmr".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ilbcj".as_ptr() },
            None,
        );
        test_6(&mut _s, c"klnitfaobg".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jkngf".as_ptr() },
            None,
        );
        test_6(&mut _s, c"gjhmdlqikp".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gfcql".as_ptr() },
            None,
        );
        test_6(&mut _s, c"skbgtahqej".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dqtlg".as_ptr() },
            None,
        );
        test_6(&mut _s, c"bjsdgtlpkf".as_ptr(), 1_(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bthpg".as_ptr() },
            None,
        );
        test_6(&mut _s, c"bjgfmnlkio".as_ptr(), 1_(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dgsnq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"lbhepotfsjdqigcnamkr".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rmfhp".as_ptr() },
            None,
        );
        test_6(&mut _s, c"tebangckmpsrqdlfojhi".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jfdam".as_ptr() },
            None,
        );
        test_6(&mut _s, c"joflqbdkhtegimscpanr".as_ptr(), 1_(), 10_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"edapb".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"adpmcohetfbsrjinlqkg".as_ptr(),
            1_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"brfsm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"iacldqjpfnogbsrhmetk".as_ptr(),
            1_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ndrhl".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mrecp".as_ptr() },
            None,
        );
        test_6(&mut _s, c"otkgb".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qlasf".as_ptr() },
            None,
        );
        test_6(&mut _s, c"cqsjl".as_ptr(), 2_(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"smaqd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"dpifl".as_ptr(), 2_(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hjeni".as_ptr() },
            None,
        );
        test_6(&mut _s, c"oapht".as_ptr(), 2_(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ocmfj".as_ptr() },
            None,
        );
        test_6(&mut _s, c"cifts".as_ptr(), 2_(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hmftq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"nmsckbgalo".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fklad".as_ptr() },
            None,
        );
        test_6(&mut _s, c"tpksqhamle".as_ptr(), 2_(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dirnm".as_ptr() },
            None,
        );
        test_6(&mut _s, c"tpdrchmkji".as_ptr(), 2_(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hrgdc".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ijagfkblst".as_ptr(), 2_(), 9_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ifakg".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kpocsignjb".as_ptr(), 2_(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ebrgd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"pecqtkjsnbdrialgmohf".as_ptr(), 2_(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rcjml".as_ptr() },
            None,
        );
        test_6(&mut _s, c"aiortphfcmkjebgsndql".as_ptr(), 2_(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"peqmt".as_ptr() },
            None,
        );
        test_6(&mut _s, c"sdbkeamglhipojqftrcn".as_ptr(), 2_(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"frehn".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ljqncehgmfktroapidbs".as_ptr(),
            2_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tqolf".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"rtcfodilamkbenjghqps".as_ptr(),
            2_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cjgao".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kjplq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"mabns".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"herni".as_ptr() },
            None,
        );
        test_6(&mut _s, c"bdnrp".as_ptr(), 4_(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tadrb".as_ptr() },
            None,
        );
        test_6(&mut _s, c"scidp".as_ptr(), 4_(), 2_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pkfeo".as_ptr() },
            None,
        );
        test_6(&mut _s, c"agbjl".as_ptr(), 4_(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hoser".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jfmpr".as_ptr(),
            4_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kgrsp".as_ptr() },
            None,
        );
        test_6(&mut _s, c"rbpefghsmj".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pgejb".as_ptr() },
            None,
        );
        test_6(&mut _s, c"apsfntdoqc".as_ptr(), 4_(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"thlnq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ndkjeisgcl".as_ptr(), 4_(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"nbmit".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"rnfpqatdeo".as_ptr(),
            4_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jgmib".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"bntjlqrfik".as_ptr(),
            4_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ncrfj".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kcrtmpolnaqejghsfdbi".as_ptr(), 4_(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ncsik".as_ptr() },
            None,
        );
        test_6(&mut _s, c"lobheanpkmqidsrtcfgj".as_ptr(), 4_(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"sgbfh".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"athdkljcnreqbgpmisof".as_ptr(),
            4_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dktbn".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"qkdmjialrscpbhefgont".as_ptr(),
            4_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fthqm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dmasojntqleribkgfchp".as_ptr(),
            4_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"klopi".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dajhn".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"psthd".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jbgno".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"rpmjd".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hkjae".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dfsmk".as_ptr(),
            5_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
}
pub unsafe fn test1_10() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gbhqo".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"skqne".as_ptr(),
            5_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ktdor".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"kipnf".as_ptr(),
            5_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ldprn".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"hmrnqdgifl".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"egmjk".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"fsmjcdairn".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"armql".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"pcdgltbrfj".as_ptr(),
            5_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cdhjo".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"aekfctpirg".as_ptr(),
            5_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jcons".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ledihrsgpf".as_ptr(),
            5_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cbrkp".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"mqcklahsbtirgopefndj".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fhgna".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"kmlthaoqgecrnpdbjfis".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ejfcd".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"sfhbamcdptojlkrenqgi".as_ptr(),
            5_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kqjhe".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"pbniofmcedrkhlstgaqj".as_ptr(),
            5_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pbdjl".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"mongjratcskbhqiepfdl".as_ptr(),
            5_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gajqn".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"stedk".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"hrnat".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tjkaf".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"gsqdt".as_ptr(),
            6_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dthpe".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"bspkd".as_ptr(),
            6_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"klhde".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ohcmb".as_ptr(),
            6_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bhlki".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"heatr".as_ptr(),
            6_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lqmoh".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"pmblckedfn".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mtqin".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"aceqmsrbik".as_ptr(),
            6_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dpqbr".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"lmbtdehjrn".as_ptr(),
            6_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kdhmo".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"teqmcrlgib".as_ptr(),
            6_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jblqp".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"njolbmspac".as_ptr(),
            6_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qmjgl".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"pofnhidklamecrbqjgst".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rothp".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jbhckmtgrqnosafedpli".as_ptr(),
            6_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ghknq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dobntpmqklicsahgjerf".as_ptr(),
            6_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"eopfi".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"tpdshainjkbfoemlrgcq".as_ptr(),
            6_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dsnmg".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"oldpfgeakrnitscbjmqh".as_ptr(),
            6_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jnkrfhotgl".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dltjfngbko".as_ptr() },
            None,
        );
        test_4(&mut _s, c"rqegt".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bmjlpkiqde".as_ptr() },
            None,
        );
        test_4(&mut _s, c"dashm".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"skrflobnqm".as_ptr() },
            None,
        );
        test_4(&mut _s, c"jqirk".as_ptr(), 0_(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jkpldtshrm".as_ptr() },
            None,
        );
        test_4(&mut _s, c"rckeg".as_ptr(), 0_(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ghasdbnjqo".as_ptr() },
            None,
        );
        test_4(&mut _s, c"jscie".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"igrkhpbqjt".as_ptr() },
            None,
        );
        test_4(&mut _s, c"efsphndliq".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ikthdgcamf".as_ptr() },
            None,
        );
        test_4(&mut _s, c"gdicosleja".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pcofgeniam".as_ptr() },
            None,
        );
        test_4(&mut _s, c"qcpjibosfl".as_ptr(), 0_(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rlfjgesqhc".as_ptr() },
            None,
        );
        test_4(&mut _s, c"lrhmefnjcq".as_ptr(), 0_(), 9_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"itphbqsker".as_ptr() },
            None,
        );
        test_4(&mut _s, c"dtablcrseo".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"skjafcirqm".as_ptr() },
            None,
        );
        test_4(&mut _s, c"apckjsftedbhgomrnilq".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tcqomarsfd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"pcbrgflehjtiadnsokqm".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rocfeldqpk".as_ptr() },
            None,
        );
        test_4(&mut _s, c"nsiadegjklhobrmtqcpf".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cfpegndlkt".as_ptr() },
            None,
        );
        test_4(&mut _s, c"cpmajdqnolikhgsbretf".as_ptr(), 0_(), 19_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fqbtnkeasj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jcflkntmgiqrphdosaeb".as_ptr(),
            0_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"shbcqnmoar".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bdoshlmfin".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ontrs".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"khfrebnsgq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"pfkna".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"getcrsaoji".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ekosa".as_ptr(), 1_(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fjiknedcpq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"anqhk".as_ptr(), 1_(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tkejgnafrm".as_ptr() },
            None,
        );
        test_4(&mut _s, c"jekca".as_ptr(), 1_(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jnakolqrde".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ikemsjgacf".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lcjptsmgbe".as_ptr() },
            None,
        );
        test_4(&mut _s, c"arolgsjkhm".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"itfsmcjorl".as_ptr() },
            None,
        );
        test_4(&mut _s, c"oftkbldhre".as_ptr(), 1_(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"omchkfrjea".as_ptr() },
            None,
        );
        test_4(&mut _s, c"gbkqdoeftl".as_ptr(), 1_(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cigfqkated".as_ptr() },
            None,
        );
        test_4(&mut _s, c"sqcflrgtim".as_ptr(), 1_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tscenjikml".as_ptr() },
            None,
        );
        test_4(&mut _s, c"fmhbkislrjdpanogqcet".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qcpaemsinf".as_ptr() },
            None,
        );
        test_4(&mut _s, c"rnioadktqlgpbcjsmhef".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gltkojeipd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"oakgtnldpsefihqmjcbr".as_ptr(), 1_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qistfrgnmp".as_ptr() },
            None,
        );
        test_4(&mut _s, c"gbnaelosidmcjqktfhpr".as_ptr(), 1_(), 19_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bdnpfcqaem".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"akbripjhlosndcmqgfet".as_ptr(),
            1_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ectnhskflp".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fgtianblpq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"pijag".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mfeqklirnh".as_ptr() },
            None,
        );
        test_4(&mut _s, c"jrckd".as_ptr(), 5_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"astedncjhk".as_ptr() },
            None,
        );
        test_4(&mut _s, c"qcloh".as_ptr(), 5_(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fhlqgcajbr".as_ptr() },
            None,
        );
        test_4(&mut _s, c"thlmp".as_ptr(), 5_(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"epfhocmdng".as_ptr() },
            None,
        );
        test_4(&mut _s, c"qidmo".as_ptr(), 5_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"apcnsibger".as_ptr() },
            None,
        );
        test_4(&mut _s, c"lnegpsjqrd".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"aqkocrbign".as_ptr() },
            None,
        );
        test_4(&mut _s, c"rjqdablmfs".as_ptr(), 5_(), 1_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ijsmdtqgce".as_ptr() },
            None,
        );
        test_4(&mut _s, c"enkgpbsjaq".as_ptr(), 5_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"clobgsrken".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kdsgoaijfh".as_ptr(), 5_(), 9_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jbhcfposld".as_ptr() },
            None,
        );
        test_4(&mut _s, c"trfqgmckbe".as_ptr(), 5_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"oqnpblhide".as_ptr() },
            None,
        );
        test_4(&mut _s, c"igetsracjfkdnpoblhqm".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lroeasctif".as_ptr() },
            None,
        );
        test_4(&mut _s, c"nqctfaogirshlekbdjpm".as_ptr(), 5_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bpjlgmiedh".as_ptr() },
            None,
        );
        test_4(&mut _s, c"csehfgomljdqinbartkp".as_ptr(), 5_(), 10_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pamkeoidrj".as_ptr() },
            None,
        );
        test_4(&mut _s, c"qahoegcmplkfsjbdnitr".as_ptr(), 5_(), 19_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"espogqbthk".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dpteiajrqmsognhlfbkc".as_ptr(),
            5_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"shoiedtcjb".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ebcinjgads".as_ptr() },
            None,
        );
        test_4(&mut _s, c"tqbnh".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dqmregkcfl".as_ptr() },
            None,
        );
        test_4(&mut _s, c"akmle".as_ptr(), 9_(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ngcrieqajf".as_ptr() },
            None,
        );
        test_4(&mut _s, c"iqfkm".as_ptr(), 9_(), 2_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qosmilgnjb".as_ptr() },
            None,
        );
        test_4(&mut _s, c"tqjsr".as_ptr(), 9_(), 4_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ikabsjtdfl".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jplqg".as_ptr(),
            9_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ersmicafdh".as_ptr() },
            None,
        );
        test_4(&mut _s, c"oilnrbcgtj".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fdnplotmgh".as_ptr() },
            None,
        );
        test_4(&mut _s, c"morkglpesn".as_ptr(), 9_(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fdbicojerm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dmicerngat".as_ptr(),
            9_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mbtafndjcq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"radgeskbtc".as_ptr(), 9_(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mlenkpfdtc".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ljikprsmqo".as_ptr(), 9_(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ahlcifdqgs".as_ptr() },
            None,
        );
        test_4(&mut _s, c"trqihkcgsjamfdbolnpe".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bgjemaltks".as_ptr() },
            None,
        );
        test_4(&mut _s, c"lqmthbsrekajgnofcipd".as_ptr(), 9_(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pdhslbqrfc".as_ptr() },
            None,
        );
        test_4(&mut _s, c"jtalmedribkgqsopcnfh".as_ptr(), 9_(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dirhtsnjkc".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"spqfoiclmtagejbndkrh".as_ptr(),
            9_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dlroktbcja".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"nmotklspigjrdhcfaebq".as_ptr(),
            9_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ncjpmaekbs".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hlbosgmrak".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"hpmsd".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pqfhsgilen".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"qnpor".as_ptr(),
            10_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gqtjsbdckh".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"otdma".as_ptr(),
            10_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cfkqpjlegi".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"efhjg".as_ptr(),
            10_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"beanrfodgj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"odpte".as_ptr(),
            10_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"adtkqpbjfi".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"bctdgfmolr".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"iomkfthagj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"oaklidrbqg".as_ptr(),
            10_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
}
pub unsafe fn test1_11() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gbhqo".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"skqne".as_ptr(),
            5_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ktdor".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"kipnf".as_ptr(),
            5_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ldprn".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"hmrnqdgifl".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"egmjk".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"fsmjcdairn".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"armql".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"pcdgltbrfj".as_ptr(),
            5_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cdhjo".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"aekfctpirg".as_ptr(),
            5_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jcons".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ledihrsgpf".as_ptr(),
            5_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cbrkp".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"mqcklahsbtirgopefndj".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fhgna".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"kmlthaoqgecrnpdbjfis".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ejfcd".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"sfhbamcdptojlkrenqgi".as_ptr(),
            5_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kqjhe".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"pbniofmcedrkhlstgaqj".as_ptr(),
            5_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pbdjl".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"mongjratcskbhqiepfdl".as_ptr(),
            5_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gajqn".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"stedk".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"hrnat".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tjkaf".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"gsqdt".as_ptr(),
            6_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dthpe".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"bspkd".as_ptr(),
            6_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"klhde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ohcmb".as_ptr(),
            6_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bhlki".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"heatr".as_ptr(),
            6_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lqmoh".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"pmblckedfn".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mtqin".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"aceqmsrbik".as_ptr(),
            6_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dpqbr".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"lmbtdehjrn".as_ptr(),
            6_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kdhmo".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"teqmcrlgib".as_ptr(),
            6_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jblqp".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"njolbmspac".as_ptr(),
            6_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qmjgl".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"pofnhidklamecrbqjgst".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rothp".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jbhckmtgrqnosafedpli".as_ptr(),
            6_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ghknq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dobntpmqklicsahgjerf".as_ptr(),
            6_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"eopfi".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"tpdshainjkbfoemlrgcq".as_ptr(),
            6_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dsnmg".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"oldpfgeakrnitscbjmqh".as_ptr(),
            6_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jnkrfhotgl".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dltjfngbko".as_ptr() },
            None,
        );
        test_6(&mut _s, c"rqegt".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bmjlpkiqde".as_ptr() },
            None,
        );
        test_6(&mut _s, c"dashm".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"skrflobnqm".as_ptr() },
            None,
        );
        test_6(&mut _s, c"jqirk".as_ptr(), 0_(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jkpldtshrm".as_ptr() },
            None,
        );
        test_6(&mut _s, c"rckeg".as_ptr(), 0_(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ghasdbnjqo".as_ptr() },
            None,
        );
        test_6(&mut _s, c"jscie".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"igrkhpbqjt".as_ptr() },
            None,
        );
        test_6(&mut _s, c"efsphndliq".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ikthdgcamf".as_ptr() },
            None,
        );
        test_6(&mut _s, c"gdicosleja".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pcofgeniam".as_ptr() },
            None,
        );
        test_6(&mut _s, c"qcpjibosfl".as_ptr(), 0_(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rlfjgesqhc".as_ptr() },
            None,
        );
        test_6(&mut _s, c"lrhmefnjcq".as_ptr(), 0_(), 9_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"itphbqsker".as_ptr() },
            None,
        );
        test_6(&mut _s, c"dtablcrseo".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"skjafcirqm".as_ptr() },
            None,
        );
        test_6(&mut _s, c"apckjsftedbhgomrnilq".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tcqomarsfd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"pcbrgflehjtiadnsokqm".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rocfeldqpk".as_ptr() },
            None,
        );
        test_6(&mut _s, c"nsiadegjklhobrmtqcpf".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cfpegndlkt".as_ptr() },
            None,
        );
        test_6(&mut _s, c"cpmajdqnolikhgsbretf".as_ptr(), 0_(), 19_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fqbtnkeasj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jcflkntmgiqrphdosaeb".as_ptr(),
            0_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"shbcqnmoar".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bdoshlmfin".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ontrs".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"khfrebnsgq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"pfkna".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"getcrsaoji".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ekosa".as_ptr(), 1_(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fjiknedcpq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"anqhk".as_ptr(), 1_(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tkejgnafrm".as_ptr() },
            None,
        );
        test_6(&mut _s, c"jekca".as_ptr(), 1_(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jnakolqrde".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ikemsjgacf".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lcjptsmgbe".as_ptr() },
            None,
        );
        test_6(&mut _s, c"arolgsjkhm".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"itfsmcjorl".as_ptr() },
            None,
        );
        test_6(&mut _s, c"oftkbldhre".as_ptr(), 1_(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"omchkfrjea".as_ptr() },
            None,
        );
        test_6(&mut _s, c"gbkqdoeftl".as_ptr(), 1_(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cigfqkated".as_ptr() },
            None,
        );
        test_6(&mut _s, c"sqcflrgtim".as_ptr(), 1_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tscenjikml".as_ptr() },
            None,
        );
        test_6(&mut _s, c"fmhbkislrjdpanogqcet".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qcpaemsinf".as_ptr() },
            None,
        );
        test_6(&mut _s, c"rnioadktqlgpbcjsmhef".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gltkojeipd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"oakgtnldpsefihqmjcbr".as_ptr(), 1_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qistfrgnmp".as_ptr() },
            None,
        );
        test_6(&mut _s, c"gbnaelosidmcjqktfhpr".as_ptr(), 1_(), 19_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bdnpfcqaem".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"akbripjhlosndcmqgfet".as_ptr(),
            1_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ectnhskflp".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fgtianblpq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"pijag".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mfeqklirnh".as_ptr() },
            None,
        );
        test_6(&mut _s, c"jrckd".as_ptr(), 5_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"astedncjhk".as_ptr() },
            None,
        );
        test_6(&mut _s, c"qcloh".as_ptr(), 5_(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fhlqgcajbr".as_ptr() },
            None,
        );
        test_6(&mut _s, c"thlmp".as_ptr(), 5_(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"epfhocmdng".as_ptr() },
            None,
        );
        test_6(&mut _s, c"qidmo".as_ptr(), 5_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"apcnsibger".as_ptr() },
            None,
        );
        test_6(&mut _s, c"lnegpsjqrd".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"aqkocrbign".as_ptr() },
            None,
        );
        test_6(&mut _s, c"rjqdablmfs".as_ptr(), 5_(), 1_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ijsmdtqgce".as_ptr() },
            None,
        );
        test_6(&mut _s, c"enkgpbsjaq".as_ptr(), 5_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"clobgsrken".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kdsgoaijfh".as_ptr(), 5_(), 9_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jbhcfposld".as_ptr() },
            None,
        );
        test_6(&mut _s, c"trfqgmckbe".as_ptr(), 5_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"oqnpblhide".as_ptr() },
            None,
        );
        test_6(&mut _s, c"igetsracjfkdnpoblhqm".as_ptr(), 5_(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lroeasctif".as_ptr() },
            None,
        );
        test_6(&mut _s, c"nqctfaogirshlekbdjpm".as_ptr(), 5_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bpjlgmiedh".as_ptr() },
            None,
        );
        test_6(&mut _s, c"csehfgomljdqinbartkp".as_ptr(), 5_(), 10_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pamkeoidrj".as_ptr() },
            None,
        );
        test_6(&mut _s, c"qahoegcmplkfsjbdnitr".as_ptr(), 5_(), 19_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"espogqbthk".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dpteiajrqmsognhlfbkc".as_ptr(),
            5_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"shoiedtcjb".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ebcinjgads".as_ptr() },
            None,
        );
        test_6(&mut _s, c"tqbnh".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dqmregkcfl".as_ptr() },
            None,
        );
        test_6(&mut _s, c"akmle".as_ptr(), 9_(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ngcrieqajf".as_ptr() },
            None,
        );
        test_6(&mut _s, c"iqfkm".as_ptr(), 9_(), 2_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qosmilgnjb".as_ptr() },
            None,
        );
        test_6(&mut _s, c"tqjsr".as_ptr(), 9_(), 4_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ikabsjtdfl".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jplqg".as_ptr(),
            9_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ersmicafdh".as_ptr() },
            None,
        );
        test_6(&mut _s, c"oilnrbcgtj".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fdnplotmgh".as_ptr() },
            None,
        );
        test_6(&mut _s, c"morkglpesn".as_ptr(), 9_(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fdbicojerm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dmicerngat".as_ptr(),
            9_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mbtafndjcq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"radgeskbtc".as_ptr(), 9_(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mlenkpfdtc".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ljikprsmqo".as_ptr(), 9_(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ahlcifdqgs".as_ptr() },
            None,
        );
        test_6(&mut _s, c"trqihkcgsjamfdbolnpe".as_ptr(), 9_(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bgjemaltks".as_ptr() },
            None,
        );
        test_6(&mut _s, c"lqmthbsrekajgnofcipd".as_ptr(), 9_(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pdhslbqrfc".as_ptr() },
            None,
        );
        test_6(&mut _s, c"jtalmedribkgqsopcnfh".as_ptr(), 9_(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dirhtsnjkc".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"spqfoiclmtagejbndkrh".as_ptr(),
            9_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dlroktbcja".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"nmotklspigjrdhcfaebq".as_ptr(),
            9_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ncjpmaekbs".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hlbosgmrak".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"hpmsd".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pqfhsgilen".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"qnpor".as_ptr(),
            10_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gqtjsbdckh".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"otdma".as_ptr(),
            10_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cfkqpjlegi".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"efhjg".as_ptr(),
            10_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"beanrfodgj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"odpte".as_ptr(),
            10_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"adtkqpbjfi".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"bctdgfmolr".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"iomkfthagj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"oaklidrbqg".as_ptr(),
            10_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
}
pub unsafe fn test2_12() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"sdpcilonqj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dnjfsagktr".as_ptr(),
            10_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gtfbdkqeml".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"nejaktmiqg".as_ptr(),
            10_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bmeqgcdorj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"pjqonlebsf".as_ptr(),
            10_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"etqlcanmob".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dshmnbtolcjepgaikfqr".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"roqmkbdtia".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"iogfhpabtjkqlrnemcds".as_ptr(),
            10_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kadsithljf".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ngridfabjsecpqltkmoh".as_ptr(),
            10_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"sgtkpbfdmh".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"athmknplcgofrqejsdib".as_ptr(),
            10_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qgmetnabkl".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ldobhmqcafnjtkeisgrp".as_ptr(),
            10_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cqjohampgd".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hobitmpsan".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"aocjb".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tjehkpsalm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jbrnk".as_ptr(),
            11_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ngfbojitcl".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"tqedg".as_ptr(),
            11_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rcfkdbhgjo".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"nqskp".as_ptr(),
            11_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qghptonrea".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"eaqkl".as_ptr(),
            11_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hnprfgqjdl".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"reaoicljqm".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hlmgabenti".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"lsftgajqpm".as_ptr(),
            11_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ofcjanmrbs".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"rlpfogmits".as_ptr(),
            11_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jqedtkornm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"shkncmiaqj".as_ptr(),
            11_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rfedlasjmg".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"fpnatrhqgs".as_ptr(),
            11_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"talpqjsgkm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"sjclemqhnpdbgikarfot".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lrkcbtqpie".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"otcmedjikgsfnqbrhpla".as_ptr(),
            11_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cipogdskjf".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"bonsaefdqiprkhlgtjcm".as_ptr(),
            11_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"nqedcojahi".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"egpscmahijlfnkrodqtb".as_ptr(),
            11_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hefnrkmctj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"kmqbfepjthgilscrndoa".as_ptr(),
            11_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"atqirnmekfjolhpdsgcb".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"echfkmlpribjnqsaogtd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"prboq".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qnhiftdgcleajbpkrosm".as_ptr() },
            None,
        );
        test_4(&mut _s, c"fjcqh".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"chamfknorbedjitgslpq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"fmosa".as_ptr(), 0_(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"njhqpibfmtlkaecdrgso".as_ptr() },
            None,
        );
        test_4(&mut _s, c"qdbok".as_ptr(), 0_(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ebnghfsqkprmdcljoiat".as_ptr() },
            None,
        );
        test_4(&mut _s, c"amslg".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"letjomsgihfrpqbkancd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"smpltjneqb".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"nblgoipcrqeaktshjdmf".as_ptr() },
            None,
        );
        test_4(&mut _s, c"flitskrnge".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cehkbngtjoiflqapsmrd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"pgqihmlbef".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mignapfoklbhcqjetdrs".as_ptr() },
            None,
        );
        test_4(&mut _s, c"cfpdqjtgsb".as_ptr(), 0_(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ceatbhlsqjgpnokfrmdi".as_ptr() },
            None,
        );
        test_4(&mut _s, c"htpsiaflom".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ocihkjgrdelpfnmastqb".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kpjfiaceghsrdtlbnomq".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"noelgschdtbrjfmiqkap".as_ptr() },
            None,
        );
        test_4(&mut _s, c"qhtbomidljgafneksprc".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dkclqfombepritjnghas".as_ptr() },
            None,
        );
        test_4(&mut _s, c"nhtjobkcefldimpsaqgr".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"miklnresdgbhqcojftap".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"prabcjfqnoeskilmtgdh".as_ptr(),
            0_(),
            19_(),
            11_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"htbcigojaqmdkfrnlsep".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dtrgmchilkasqoebfpjn".as_ptr(),
            0_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"febhmqtjanokscdirpgl".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"loakbsqjpcrdhftniegm".as_ptr() },
            None,
        );
        test_4(&mut _s, c"sqome".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"reagphsqflbitdcjmkno".as_ptr() },
            None,
        );
        test_4(&mut _s, c"smfte".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jitlfrqemsdhkopncabg".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ciboh".as_ptr(), 1_(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mhtaepscdnrjqgbkifol".as_ptr() },
            None,
        );
        test_4(&mut _s, c"haois".as_ptr(), 1_(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"tocesrfmnglpbjihqadk".as_ptr() },
            None,
        );
        test_4(&mut _s, c"abfki".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lpfmctjrhdagneskbqoi".as_ptr() },
            None,
        );
        test_4(&mut _s, c"frdkocntmq".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lsmqaepkdhncirbtjfgo".as_ptr() },
            None,
        );
        test_4(&mut _s, c"oasbpedlnr".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"epoiqmtldrabnkjhcfsg".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kltqmhgand".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"emgasrilpknqojhtbdcf".as_ptr() },
            None,
        );
        test_4(&mut _s, c"gdtfjchpmr".as_ptr(), 1_(), 9_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hnfiagdpcklrjetqbsom".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ponmcqblet".as_ptr(), 1_(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"nsdfebgajhmtricpoklq".as_ptr() },
            None,
        );
        test_4(&mut _s, c"sgphqdnofeiklatbcmjr".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"atjgfsdlpobmeiqhncrk".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ljqprsmigtfoneadckbh".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"sitodfgnrejlahcbmqkp".as_ptr() },
            None,
        );
        test_4(&mut _s, c"ligeojhafnkmrcsqtbdp".as_ptr(), 1_(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fraghmbiceknltjpqosd".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"lsimqfnjarbopedkhcgt".as_ptr(),
            1_(),
            19_(),
            13_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pmafenlhqtdbkirjsogc".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"abedmfjlghniorcqptks".as_ptr(),
            1_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pihgmoeqtnakrjslcbfd".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gjdkeprctqblnhiafsom".as_ptr() },
            None,
        );
        test_4(&mut _s, c"hqtoa".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mkpnblfdsahrcqijteog".as_ptr() },
            None,
        );
        test_4(&mut _s, c"cahif".as_ptr(), 10_(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gckarqnelodfjhmbptis".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kehis".as_ptr(), 10_(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gqpskidtbclomahnrjfe".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kdlmh".as_ptr(), 10_(), 4_(), 11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pkldjsqrfgitbhmaecno".as_ptr() },
            None,
        );
        test_4(&mut _s, c"paeql".as_ptr(), 10_(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"aftsijrbeklnmcdqhgop".as_ptr() },
            None,
        );
        test_4(&mut _s, c"aghoqiefnb".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mtlgdrhafjkbiepqnsoc".as_ptr() },
            None,
        );
        test_4(&mut _s, c"jrbqaikpdo".as_ptr(), 10_(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pqgirnaefthokdmbsclj".as_ptr() },
            None,
        );
        test_4(&mut _s, c"smjonaeqcl".as_ptr(), 10_(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kpdbgjmtherlsfcqoina".as_ptr() },
            None,
        );
        test_4(&mut _s, c"eqbdrkcfah".as_ptr(), 10_(), 9_(), 11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jrlbothiknqmdgcfasep".as_ptr() },
            None,
        );
        test_4(&mut _s, c"kapmsienhf".as_ptr(), 10_(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mjogldqferckabinptsh".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jpqotrlenfcsbhkaimdg".as_ptr(),
            10_(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"apoklnefbhmgqcdrisjt".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jlbmhnfgtcqprikeados".as_ptr(),
            10_(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ifeopcnrjbhkdgatmqls".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"stgbhfmdaljnpqoicker".as_ptr(),
            10_(),
            10_(),
            11_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ckqhaiesmjdnrgolbtpf".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"oihcetflbjagdsrkmqpn".as_ptr(),
            10_(),
            19_(),
            11_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bnlgapfimcoterskqdjh".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"adtclebmnpjsrqfkigoh".as_ptr(),
            10_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kgdlrobpmjcthqsafeni".as_ptr() },
            None,
        );
        test_4(&mut _s, c"".as_ptr(), 19_(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dfkechomjapgnslbtqir".as_ptr() },
            None,
        );
        test_4(&mut _s, c"beafg".as_ptr(), 19_(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rloadknfbqtgmhcsipje".as_ptr() },
            None,
        );
        test_4(&mut _s, c"iclat".as_ptr(), 19_(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mgjhkolrnadqbpetcifs".as_ptr() },
            None,
        );
        test_4(&mut _s, c"rkhnf".as_ptr(), 19_(), 2_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"cmlfakiojdrgtbsphqen".as_ptr() },
            None,
        );
        test_4(&mut _s, c"clshq".as_ptr(), 19_(), 4_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kghbfipeomsntdalrqjc".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dtcoj".as_ptr(),
            19_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"eldiqckrnmtasbghjfpo".as_ptr() },
            None,
        );
        test_4(&mut _s, c"rqosnjmfth".as_ptr(), 19_(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abqjcfedgotihlnspkrm".as_ptr() },
            None,
        );
        test_4(&mut _s, c"siatdfqglh".as_ptr(), 19_(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qfbadrtjsimkolcenhpg".as_ptr() },
            None,
        );
        test_4(&mut _s, c"mrlshtpgjq".as_ptr(), 19_(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abseghclkjqifmtodrnp".as_ptr() },
            None,
        );
        test_4(&mut _s, c"adlcskgqjt".as_ptr(), 19_(), 9_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ibmsnlrjefhtdokacqpg".as_ptr() },
            None,
        );
        test_4(&mut _s, c"drshcjknaf".as_ptr(), 19_(), 10_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mrkfciqjebaponsthldg".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"etsaqroinghpkjdlfcbm".as_ptr(),
            19_(),
            0_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mjkticdeoqshpalrfbgn".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"sgepdnkqliambtrocfhj".as_ptr(),
            19_(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rqnoclbdejgiphtfsakm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"nlmcjaqgbsortfdihkpe".as_ptr(),
            19_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"plkqbhmtfaeodjcrsing".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"racfnpmosldibqkghjet".as_ptr(),
            19_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"oegalhmstjrfickpbndq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"fjhdsctkqeiolagrnmbp".as_ptr(),
            19_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rdtgjcaohpblniekmsfq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ofkqbnjetrmsaidphglc".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ejanp".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"grkpahljcftesdmonqib".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"odife".as_ptr(),
            20_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"jimlgbhfqkteospardcn".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"okaqd".as_ptr(),
            20_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"gftenihpmslrjkqadcob".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"lcdbi".as_ptr(),
            20_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"bmhldogtckrfsanijepq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"fsqbj".as_ptr(),
            20_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"nfqkrpjdesabgtlcmoih".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"bigdomnplq".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"focalnrpiqmdkstehbjg".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"apiblotgcd".as_ptr(),
            20_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rhqdspkmebiflcotnjga".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"acfhdenops".as_ptr(),
            20_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rahdtmsckfboqlpniegj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jopdeamcrk".as_ptr(),
            20_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fbkeiopclstmdqranjhg".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"trqncbkgmh".as_ptr(),
            20_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"lifhpdgmbconstjeqark".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"tomglrkencbsfjqpihda".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
}
pub unsafe fn test2_13() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"sdpcilonqj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dnjfsagktr".as_ptr(),
            10_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gtfbdkqeml".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"nejaktmiqg".as_ptr(),
            10_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bmeqgcdorj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"pjqonlebsf".as_ptr(),
            10_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"etqlcanmob".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dshmnbtolcjepgaikfqr".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"roqmkbdtia".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"iogfhpabtjkqlrnemcds".as_ptr(),
            10_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kadsithljf".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ngridfabjsecpqltkmoh".as_ptr(),
            10_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"sgtkpbfdmh".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"athmknplcgofrqejsdib".as_ptr(),
            10_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qgmetnabkl".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ldobhmqcafnjtkeisgrp".as_ptr(),
            10_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cqjohampgd".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hobitmpsan".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"aocjb".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tjehkpsalm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jbrnk".as_ptr(),
            11_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ngfbojitcl".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"tqedg".as_ptr(),
            11_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rcfkdbhgjo".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"nqskp".as_ptr(),
            11_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qghptonrea".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"eaqkl".as_ptr(),
            11_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hnprfgqjdl".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"reaoicljqm".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hlmgabenti".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"lsftgajqpm".as_ptr(),
            11_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ofcjanmrbs".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"rlpfogmits".as_ptr(),
            11_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jqedtkornm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"shkncmiaqj".as_ptr(),
            11_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rfedlasjmg".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"fpnatrhqgs".as_ptr(),
            11_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"talpqjsgkm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"sjclemqhnpdbgikarfot".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lrkcbtqpie".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"otcmedjikgsfnqbrhpla".as_ptr(),
            11_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cipogdskjf".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"bonsaefdqiprkhlgtjcm".as_ptr(),
            11_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"nqedcojahi".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"egpscmahijlfnkrodqtb".as_ptr(),
            11_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hefnrkmctj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"kmqbfepjthgilscrndoa".as_ptr(),
            11_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"atqirnmekfjolhpdsgcb".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"echfkmlpribjnqsaogtd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"prboq".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qnhiftdgcleajbpkrosm".as_ptr() },
            None,
        );
        test_6(&mut _s, c"fjcqh".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"chamfknorbedjitgslpq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"fmosa".as_ptr(), 0_(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"njhqpibfmtlkaecdrgso".as_ptr() },
            None,
        );
        test_6(&mut _s, c"qdbok".as_ptr(), 0_(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ebnghfsqkprmdcljoiat".as_ptr() },
            None,
        );
        test_6(&mut _s, c"amslg".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"letjomsgihfrpqbkancd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"smpltjneqb".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"nblgoipcrqeaktshjdmf".as_ptr() },
            None,
        );
        test_6(&mut _s, c"flitskrnge".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cehkbngtjoiflqapsmrd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"pgqihmlbef".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mignapfoklbhcqjetdrs".as_ptr() },
            None,
        );
        test_6(&mut _s, c"cfpdqjtgsb".as_ptr(), 0_(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ceatbhlsqjgpnokfrmdi".as_ptr() },
            None,
        );
        test_6(&mut _s, c"htpsiaflom".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ocihkjgrdelpfnmastqb".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kpjfiaceghsrdtlbnomq".as_ptr(), 0_(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"noelgschdtbrjfmiqkap".as_ptr() },
            None,
        );
        test_6(&mut _s, c"qhtbomidljgafneksprc".as_ptr(), 0_(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dkclqfombepritjnghas".as_ptr() },
            None,
        );
        test_6(&mut _s, c"nhtjobkcefldimpsaqgr".as_ptr(), 0_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"miklnresdgbhqcojftap".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"prabcjfqnoeskilmtgdh".as_ptr(),
            0_(),
            19_(),
            11_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"htbcigojaqmdkfrnlsep".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dtrgmchilkasqoebfpjn".as_ptr(),
            0_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"febhmqtjanokscdirpgl".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"loakbsqjpcrdhftniegm".as_ptr() },
            None,
        );
        test_6(&mut _s, c"sqome".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"reagphsqflbitdcjmkno".as_ptr() },
            None,
        );
        test_6(&mut _s, c"smfte".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jitlfrqemsdhkopncabg".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ciboh".as_ptr(), 1_(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mhtaepscdnrjqgbkifol".as_ptr() },
            None,
        );
        test_6(&mut _s, c"haois".as_ptr(), 1_(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"tocesrfmnglpbjihqadk".as_ptr() },
            None,
        );
        test_6(&mut _s, c"abfki".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lpfmctjrhdagneskbqoi".as_ptr() },
            None,
        );
        test_6(&mut _s, c"frdkocntmq".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lsmqaepkdhncirbtjfgo".as_ptr() },
            None,
        );
        test_6(&mut _s, c"oasbpedlnr".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"epoiqmtldrabnkjhcfsg".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kltqmhgand".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"emgasrilpknqojhtbdcf".as_ptr() },
            None,
        );
        test_6(&mut _s, c"gdtfjchpmr".as_ptr(), 1_(), 9_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hnfiagdpcklrjetqbsom".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ponmcqblet".as_ptr(), 1_(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"nsdfebgajhmtricpoklq".as_ptr() },
            None,
        );
        test_6(&mut _s, c"sgphqdnofeiklatbcmjr".as_ptr(), 1_(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"atjgfsdlpobmeiqhncrk".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ljqprsmigtfoneadckbh".as_ptr(), 1_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"sitodfgnrejlahcbmqkp".as_ptr() },
            None,
        );
        test_6(&mut _s, c"ligeojhafnkmrcsqtbdp".as_ptr(), 1_(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fraghmbiceknltjpqosd".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"lsimqfnjarbopedkhcgt".as_ptr(),
            1_(),
            19_(),
            13_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pmafenlhqtdbkirjsogc".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"abedmfjlghniorcqptks".as_ptr(),
            1_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pihgmoeqtnakrjslcbfd".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gjdkeprctqblnhiafsom".as_ptr() },
            None,
        );
        test_6(&mut _s, c"hqtoa".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mkpnblfdsahrcqijteog".as_ptr() },
            None,
        );
        test_6(&mut _s, c"cahif".as_ptr(), 10_(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gckarqnelodfjhmbptis".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kehis".as_ptr(), 10_(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gqpskidtbclomahnrjfe".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kdlmh".as_ptr(), 10_(), 4_(), 11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pkldjsqrfgitbhmaecno".as_ptr() },
            None,
        );
        test_6(&mut _s, c"paeql".as_ptr(), 10_(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"aftsijrbeklnmcdqhgop".as_ptr() },
            None,
        );
        test_6(&mut _s, c"aghoqiefnb".as_ptr(), 10_(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mtlgdrhafjkbiepqnsoc".as_ptr() },
            None,
        );
        test_6(&mut _s, c"jrbqaikpdo".as_ptr(), 10_(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pqgirnaefthokdmbsclj".as_ptr() },
            None,
        );
        test_6(&mut _s, c"smjonaeqcl".as_ptr(), 10_(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kpdbgjmtherlsfcqoina".as_ptr() },
            None,
        );
        test_6(&mut _s, c"eqbdrkcfah".as_ptr(), 10_(), 9_(), 11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jrlbothiknqmdgcfasep".as_ptr() },
            None,
        );
        test_6(&mut _s, c"kapmsienhf".as_ptr(), 10_(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mjogldqferckabinptsh".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jpqotrlenfcsbhkaimdg".as_ptr(),
            10_(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"apoklnefbhmgqcdrisjt".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jlbmhnfgtcqprikeados".as_ptr(),
            10_(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ifeopcnrjbhkdgatmqls".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"stgbhfmdaljnpqoicker".as_ptr(),
            10_(),
            10_(),
            11_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ckqhaiesmjdnrgolbtpf".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"oihcetflbjagdsrkmqpn".as_ptr(),
            10_(),
            19_(),
            11_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bnlgapfimcoterskqdjh".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"adtclebmnpjsrqfkigoh".as_ptr(),
            10_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kgdlrobpmjcthqsafeni".as_ptr() },
            None,
        );
        test_6(&mut _s, c"".as_ptr(), 19_(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dfkechomjapgnslbtqir".as_ptr() },
            None,
        );
        test_6(&mut _s, c"beafg".as_ptr(), 19_(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rloadknfbqtgmhcsipje".as_ptr() },
            None,
        );
        test_6(&mut _s, c"iclat".as_ptr(), 19_(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mgjhkolrnadqbpetcifs".as_ptr() },
            None,
        );
        test_6(&mut _s, c"rkhnf".as_ptr(), 19_(), 2_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"cmlfakiojdrgtbsphqen".as_ptr() },
            None,
        );
        test_6(&mut _s, c"clshq".as_ptr(), 19_(), 4_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kghbfipeomsntdalrqjc".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dtcoj".as_ptr(),
            19_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"eldiqckrnmtasbghjfpo".as_ptr() },
            None,
        );
        test_6(&mut _s, c"rqosnjmfth".as_ptr(), 19_(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abqjcfedgotihlnspkrm".as_ptr() },
            None,
        );
        test_6(&mut _s, c"siatdfqglh".as_ptr(), 19_(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qfbadrtjsimkolcenhpg".as_ptr() },
            None,
        );
        test_6(&mut _s, c"mrlshtpgjq".as_ptr(), 19_(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abseghclkjqifmtodrnp".as_ptr() },
            None,
        );
        test_6(&mut _s, c"adlcskgqjt".as_ptr(), 19_(), 9_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ibmsnlrjefhtdokacqpg".as_ptr() },
            None,
        );
        test_6(&mut _s, c"drshcjknaf".as_ptr(), 19_(), 10_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mrkfciqjebaponsthldg".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"etsaqroinghpkjdlfcbm".as_ptr(),
            19_(),
            0_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mjkticdeoqshpalrfbgn".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"sgepdnkqliambtrocfhj".as_ptr(),
            19_(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rqnoclbdejgiphtfsakm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"nlmcjaqgbsortfdihkpe".as_ptr(),
            19_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"plkqbhmtfaeodjcrsing".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"racfnpmosldibqkghjet".as_ptr(),
            19_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"oegalhmstjrfickpbndq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"fjhdsctkqeiolagrnmbp".as_ptr(),
            19_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rdtgjcaohpblniekmsfq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ofkqbnjetrmsaidphglc".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ejanp".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"grkpahljcftesdmonqib".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"odife".as_ptr(),
            20_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"jimlgbhfqkteospardcn".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"okaqd".as_ptr(),
            20_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"gftenihpmslrjkqadcob".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"lcdbi".as_ptr(),
            20_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"bmhldogtckrfsanijepq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"fsqbj".as_ptr(),
            20_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"nfqkrpjdesabgtlcmoih".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"bigdomnplq".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"focalnrpiqmdkstehbjg".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"apiblotgcd".as_ptr(),
            20_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rhqdspkmebiflcotnjga".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"acfhdenops".as_ptr(),
            20_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rahdtmsckfboqlpniegj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jopdeamcrk".as_ptr(),
            20_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fbkeiopclstmdqranjhg".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"trqncbkgmh".as_ptr(),
            20_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"lifhpdgmbconstjeqark".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"tomglrkencbsfjqpihda".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
}
pub unsafe fn test3_14() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pboqganrhedjmltsicfk".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"gbkhdnpoietfcmrslajq".as_ptr(),
            20_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"klchabsimetjnqgorfpd".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"rtfnmbsglkjaichoqedp".as_ptr(),
            20_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"sirfgmjqhctndbklaepo".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ohkmdpfqbsacrtjnlgei".as_ptr(),
            20_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rlbdsiceaonqjtfpghkm".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"dlbrteoisgphmkncajfq".as_ptr(),
            20_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ecgdanriptblhjfqskom".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"fdmiarlpgcskbhoteqjn".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"sjrlo".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"rlbstjqopignecmfadkh".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"qjpor".as_ptr(),
            21_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"grjpqmbshektdolcafni".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"odhfn".as_ptr(),
            21_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"sakfcohtqnibprjmlged".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"qtfin".as_ptr(),
            21_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"mjtdglasihqpocebrfkn".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"hpqfo".as_ptr(),
            21_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"okaplfrntghqbmeicsdj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"fabmertkos".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"sahngemrtcjidqbklfpo".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"brqtgkmaej".as_ptr(),
            21_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"dlmsipcnekhbgoaftqjr".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"nfrdeihsgl".as_ptr(),
            21_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"ahegrmqnoiklpfsdbcjt".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"hlfrosekpi".as_ptr(),
            21_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hdsjbnmlegtkqripacof".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"atgbkrjdsm".as_ptr(),
            21_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"pcnedrfjihqbalkgtoms".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"blnrptjgqmaifsdkhoec".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qjidealmtpskrbfhocng".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"ctpmdahebfqjgknloris".as_ptr(),
            21_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"qeindtagmokpfhsclrbj".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"apnkeqthrmlbfodiscgj".as_ptr(),
            21_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"kpfegbjhsrnodltqciam".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"jdgictpframeoqlsbknh".as_ptr(),
            21_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"hnbrcplsjfgiktoedmaq".as_ptr() },
            None,
        );
        test_4(
            &mut _s,
            c"qprlsfojamgndekthibc".as_ptr(),
            21_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
}
pub unsafe fn test3_15() {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pboqganrhedjmltsicfk".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"gbkhdnpoietfcmrslajq".as_ptr(),
            20_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"klchabsimetjnqgorfpd".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"rtfnmbsglkjaichoqedp".as_ptr(),
            20_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"sirfgmjqhctndbklaepo".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ohkmdpfqbsacrtjnlgei".as_ptr(),
            20_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rlbdsiceaonqjtfpghkm".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"dlbrteoisgphmkncajfq".as_ptr(),
            20_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ecgdanriptblhjfqskom".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"fdmiarlpgcskbhoteqjn".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"sjrlo".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"rlbstjqopignecmfadkh".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"qjpor".as_ptr(),
            21_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"grjpqmbshektdolcafni".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"odhfn".as_ptr(),
            21_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"sakfcohtqnibprjmlged".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"qtfin".as_ptr(),
            21_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"mjtdglasihqpocebrfkn".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"hpqfo".as_ptr(),
            21_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"okaplfrntghqbmeicsdj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"fabmertkos".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"sahngemrtcjidqbklfpo".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"brqtgkmaej".as_ptr(),
            21_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"dlmsipcnekhbgoaftqjr".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"nfrdeihsgl".as_ptr(),
            21_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"ahegrmqnoiklpfsdbcjt".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"hlfrosekpi".as_ptr(),
            21_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hdsjbnmlegtkqripacof".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"atgbkrjdsm".as_ptr(),
            21_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"pcnedrfjihqbalkgtoms".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"blnrptjgqmaifsdkhoec".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qjidealmtpskrbfhocng".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"ctpmdahebfqjgknloris".as_ptr(),
            21_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"qeindtagmokpfhsclrbj".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"apnkeqthrmlbfodiscgj".as_ptr(),
            21_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"kpfegbjhsrnodltqciam".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"jdgictpframeoqlsbknh".as_ptr(),
            21_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"hnbrcplsjfgiktoedmaq".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            c"qprlsfojamgndekthibc".as_ptr(),
            21_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
}
pub unsafe fn test_string_16() {
    (unsafe { test0_8() });
    (unsafe { test1_10() });
    (unsafe { test2_12() });
    (unsafe { test3_14() });
}
pub unsafe fn test_string_17() {
    (unsafe { test0_9() });
    (unsafe { test1_11() });
    (unsafe { test2_13() });
    (unsafe { test3_15() });
}
pub unsafe fn test_18() -> () {
    (unsafe { test_string_16() });
    (unsafe { test_string_17() });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_18() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
