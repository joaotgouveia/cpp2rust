extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct NaturalParameters {
    m_data: Vec<f64>,
}
impl NaturalParameters {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            m_data: (0..(2_usize) as usize).map(|_| 0_f64).collect::<Vec<_>>(),
        };
        this
    }
    pub unsafe fn begin(&self) -> UnsafeRandomAccessIterator<f64> {
        return self.m_data.begin();
    }
    pub unsafe fn end(&self) -> UnsafeRandomAccessIterator<f64> {
        return self.m_data.begin();
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        _a0: *const NaturalParameters,
    ) -> *mut NaturalParameters {
        return &mut (*(self as *mut NaturalParameters));
    }
    pub unsafe fn move_from(_a0: *mut NaturalParameters) -> Self {
        let mut this = Self {
            m_data: std::mem::take(&mut (*_a0).m_data),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NaturalParameters) -> *mut NaturalParameters {
        self.m_data = std::mem::take(&mut (*_a0).m_data);
        return &mut (*(self as *mut NaturalParameters));
    }
}
impl Default for NaturalParameters {
    fn default() -> Self {
        unsafe { NaturalParameters::new() }
    }
}
pub unsafe fn operator_add_0(
    a: *const NaturalParameters,
    b: *const NaturalParameters,
) -> NaturalParameters {
    let mut tmp: NaturalParameters = (*a).clone();
    return (*(unsafe {
        let _arg0: *const NaturalParameters = b;
        NaturalParameters::operator_add_assign(&mut tmp, _arg0)
    }))
    .clone();
}
pub unsafe fn test01_1() {
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (1000_usize)) {
        let mut ChildrenNP: Vec<NaturalParameters> = (0..(1000_usize) as usize)
            .map(|_| <NaturalParameters>::default())
            .collect::<Vec<_>>();
        let mut init: NaturalParameters = NaturalParameters::new();
        let mut NP: NaturalParameters = unimplemented!();
        i.prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
