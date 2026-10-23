extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type LockFreeStatus = ();
pub const LockFreeStatus_unknown: LockFreeStatus = -1;
pub const LockFreeStatus_never: LockFreeStatus = 0;
pub const LockFreeStatus_sometimes: LockFreeStatus = 1;
pub const LockFreeStatus_always: LockFreeStatus = 2;
pub static mut value_0: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_0)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_char_ {}
pub static mut value_2: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_2)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_short_ {}
pub static mut value_4: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_4)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_int_ {}
pub static mut value_6: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_6)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_long_ {}
pub static mut value_8: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_8)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_void_ptr_ {}
pub static mut value_10: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<()>())
                                    >= (::std::mem::size_of::<()>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_11: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_10)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_long_long_ {}
pub static mut value_12: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<()>())
                                    >= (::std::mem::size_of::<()>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_13: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_12)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_float_ {}
pub static mut value_14: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<*mut ()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<*mut ()>()) >= (::std::mem::size_of::<*mut ()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<*mut ()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<*mut ()>()) >= (::std::mem::size_of::<*mut ()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<*mut ()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<*mut ()>()) >= (::std::mem::size_of::<*mut ()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<*mut ()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<*mut ()>())
                            >= (::std::mem::size_of::<*mut ()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<*mut ()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<*mut ()>())
                                >= (::std::mem::size_of::<*mut ()>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<*mut ()>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<*mut ()>())
                                    >= (::std::mem::size_of::<*mut ()>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_15: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_14)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_int_ptr_ {}
pub static mut value_16: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<Empty_17>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<Empty_17>()) >= (::std::mem::size_of::<Empty_17>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<Empty_17>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<Empty_17>()) >= (::std::mem::size_of::<Empty_17>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<Empty_17>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<Empty_17>())
                        >= (::std::mem::size_of::<Empty_17>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<Empty_17>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<Empty_17>())
                            >= (::std::mem::size_of::<Empty_17>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<Empty_17>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<Empty_17>())
                                >= (::std::mem::size_of::<Empty_17>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<Empty_17>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<Empty_17>())
                                    >= (::std::mem::size_of::<Empty_17>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_18: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_16)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_Empty_ {}
pub static mut value_19: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<OneInt_20>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<OneInt_20>()) >= (::std::mem::size_of::<OneInt_20>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<OneInt_20>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<OneInt_20>()) >= (::std::mem::size_of::<OneInt_20>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<OneInt_20>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<OneInt_20>())
                        >= (::std::mem::size_of::<OneInt_20>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<OneInt_20>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<OneInt_20>())
                            >= (::std::mem::size_of::<OneInt_20>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<OneInt_20>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<OneInt_20>())
                                >= (::std::mem::size_of::<OneInt_20>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<OneInt_20>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<OneInt_20>())
                                    >= (::std::mem::size_of::<OneInt_20>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_21: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_19)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_OneInt_ {}
pub static mut value_22: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<IntArr2_23>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<IntArr2_23>()) >= (::std::mem::size_of::<IntArr2_23>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<IntArr2_23>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<IntArr2_23>())
                    >= (::std::mem::size_of::<IntArr2_23>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<IntArr2_23>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<IntArr2_23>())
                        >= (::std::mem::size_of::<IntArr2_23>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<IntArr2_23>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<IntArr2_23>())
                            >= (::std::mem::size_of::<IntArr2_23>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<IntArr2_23>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<IntArr2_23>())
                                >= (::std::mem::size_of::<IntArr2_23>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<IntArr2_23>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<IntArr2_23>())
                                    >= (::std::mem::size_of::<IntArr2_23>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_24: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_22)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_IntArr2_ {}
pub static mut value_25: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<FloatArr3_26>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<FloatArr3_26>())
                >= (::std::mem::size_of::<FloatArr3_26>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<FloatArr3_26>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<FloatArr3_26>())
                    >= (::std::mem::size_of::<FloatArr3_26>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<FloatArr3_26>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<FloatArr3_26>())
                        >= (::std::mem::size_of::<FloatArr3_26>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<FloatArr3_26>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<FloatArr3_26>())
                            >= (::std::mem::size_of::<FloatArr3_26>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<FloatArr3_26>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<FloatArr3_26>())
                                >= (::std::mem::size_of::<FloatArr3_26>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<FloatArr3_26>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<FloatArr3_26>())
                                    >= (::std::mem::size_of::<FloatArr3_26>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_27: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_25)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_FloatArr3_ {}
pub static mut value_28: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<LLIArr2_29>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<LLIArr2_29>()) >= (::std::mem::size_of::<LLIArr2_29>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<LLIArr2_29>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<LLIArr2_29>())
                    >= (::std::mem::size_of::<LLIArr2_29>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<LLIArr2_29>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<LLIArr2_29>())
                        >= (::std::mem::size_of::<LLIArr2_29>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<LLIArr2_29>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<LLIArr2_29>())
                            >= (::std::mem::size_of::<LLIArr2_29>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<LLIArr2_29>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<LLIArr2_29>())
                                >= (::std::mem::size_of::<LLIArr2_29>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<LLIArr2_29>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<LLIArr2_29>())
                                    >= (::std::mem::size_of::<LLIArr2_29>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_30: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_28)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_LLIArr2_ {}
pub static mut value_31: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<LLIArr4_32>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<LLIArr4_32>()) >= (::std::mem::size_of::<LLIArr4_32>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<LLIArr4_32>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<LLIArr4_32>())
                    >= (::std::mem::size_of::<LLIArr4_32>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<LLIArr4_32>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<LLIArr4_32>())
                        >= (::std::mem::size_of::<LLIArr4_32>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<LLIArr4_32>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<LLIArr4_32>())
                            >= (::std::mem::size_of::<LLIArr4_32>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<LLIArr4_32>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<LLIArr4_32>())
                                >= (::std::mem::size_of::<LLIArr4_32>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<LLIArr4_32>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<LLIArr4_32>())
                                    >= (::std::mem::size_of::<LLIArr4_32>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_33: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_31)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_LLIArr4_ {}
pub static mut value_34: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<LLIArr8_35>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<LLIArr8_35>()) >= (::std::mem::size_of::<LLIArr8_35>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<LLIArr8_35>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<LLIArr8_35>())
                    >= (::std::mem::size_of::<LLIArr8_35>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<LLIArr8_35>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<LLIArr8_35>())
                        >= (::std::mem::size_of::<LLIArr8_35>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<LLIArr8_35>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<LLIArr8_35>())
                            >= (::std::mem::size_of::<LLIArr8_35>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<LLIArr8_35>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<LLIArr8_35>())
                                >= (::std::mem::size_of::<LLIArr8_35>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<LLIArr8_35>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<LLIArr8_35>())
                                    >= (::std::mem::size_of::<LLIArr8_35>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_36: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_34)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_LLIArr8_ {}
pub static mut value_37: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<LLIArr16_38>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<LLIArr16_38>()) >= (::std::mem::size_of::<LLIArr16_38>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<LLIArr16_38>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<LLIArr16_38>())
                    >= (::std::mem::size_of::<LLIArr16_38>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<LLIArr16_38>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<LLIArr16_38>())
                        >= (::std::mem::size_of::<LLIArr16_38>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<LLIArr16_38>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<LLIArr16_38>())
                            >= (::std::mem::size_of::<LLIArr16_38>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<LLIArr16_38>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<LLIArr16_38>())
                                >= (::std::mem::size_of::<LLIArr16_38>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<LLIArr16_38>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<LLIArr16_38>())
                                    >= (::std::mem::size_of::<LLIArr16_38>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_39: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_37)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_LLIArr16_ {}
pub static mut value_40: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<Padding_41>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<Padding_41>()) >= (::std::mem::size_of::<Padding_41>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<Padding_41>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<Padding_41>())
                    >= (::std::mem::size_of::<Padding_41>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<Padding_41>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<Padding_41>())
                        >= (::std::mem::size_of::<Padding_41>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<Padding_41>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<Padding_41>())
                            >= (::std::mem::size_of::<Padding_41>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<Padding_41>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<Padding_41>())
                                >= (::std::mem::size_of::<Padding_41>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<Padding_41>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<Padding_41>())
                                    >= (::std::mem::size_of::<Padding_41>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_42: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_40)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_Padding_ {}
pub static mut value_43: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<IntFloat_44>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<IntFloat_44>()) >= (::std::mem::size_of::<IntFloat_44>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<IntFloat_44>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<IntFloat_44>())
                    >= (::std::mem::size_of::<IntFloat_44>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<IntFloat_44>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<IntFloat_44>())
                        >= (::std::mem::size_of::<IntFloat_44>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<IntFloat_44>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<IntFloat_44>())
                            >= (::std::mem::size_of::<IntFloat_44>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<IntFloat_44>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<IntFloat_44>())
                                >= (::std::mem::size_of::<IntFloat_44>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<IntFloat_44>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<IntFloat_44>())
                                    >= (::std::mem::size_of::<IntFloat_44>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_45: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_43)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_IntFloat_ {}
pub static mut value_46: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<CharEnumClass_47>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<CharEnumClass_47>())
                >= (::std::mem::size_of::<CharEnumClass_47>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<CharEnumClass_47>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<CharEnumClass_47>())
                    >= (::std::mem::size_of::<CharEnumClass_47>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<CharEnumClass_47>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<CharEnumClass_47>())
                        >= (::std::mem::size_of::<CharEnumClass_47>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<CharEnumClass_47>())
                        == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<CharEnumClass_47>())
                            >= (::std::mem::size_of::<CharEnumClass_47>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<CharEnumClass_47>())
                            == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<CharEnumClass_47>())
                                >= (::std::mem::size_of::<CharEnumClass_47>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<CharEnumClass_47>())
                                == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<CharEnumClass_47>())
                                    >= (::std::mem::size_of::<CharEnumClass_47>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_48: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_46)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_CharEnumClass_ {}
const _: () = assert!(true, "LockFreeStatusInfo<char>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<short>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<int>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<long>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<void*>::status_known");
const _: () = assert!(
    true,
    "alignof(long long) == sizeof(long long) ? LockFreeStatusInfo<long long>::status_known : true"
);
const _: () = assert!(
    true,
    "LockFreeStatusInfo<char>::value == LockFreeStatus::always"
);
const _: () = assert!(
    true,
    "LockFreeStatusInfo<short>::value == LockFreeStatus::always"
);
const _: () = assert!(
    true,
    "LockFreeStatusInfo<int>::value == LockFreeStatus::always"
);
pub unsafe fn operator_eq_49(x: *const UserAtomicType, y: *const UserAtomicType) -> () {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct UserAtomicType {
    pub i: (),
}
impl UserAtomicType {
    pub unsafe fn new(mut d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { i: d };
        this
    }
}
impl std::cmp::PartialEq for UserAtomicType {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_49(
                self as *const UserAtomicType,
                other as *const UserAtomicType,
            )
        }
    }
}
impl std::cmp::Eq for UserAtomicType {}
impl Default for UserAtomicType {
    fn default() -> Self {
        unsafe { UserAtomicType::new(None) }
    }
}
pub unsafe fn operator_eq_50(x: *const LargeUserAtomicType, y: *const LargeUserAtomicType) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (128_())) {
        if (((*x).a[(i) as usize]) != ((*y).a[(i) as usize])) {
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct LargeUserAtomicType {
    pub a: [(); 128],
}
impl LargeUserAtomicType {
    pub unsafe fn new(mut d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self {
            a: [unimplemented!(); 128],
        };
        'loop_: for e in IterRange::new(
            this.a.as_mut_ptr(),
            this.a.as_mut_ptr().offset((128_()) as isize),
        ) {
            let e: *mut () = &mut (*e);
            (*e) = d.postfix_inc();
        }
        this
    }
}
impl std::cmp::PartialEq for LargeUserAtomicType {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_50(
                self as *const LargeUserAtomicType,
                other as *const LargeUserAtomicType,
            )
        }
    }
}
impl std::cmp::Eq for LargeUserAtomicType {}
impl Default for LargeUserAtomicType {
    fn default() -> Self {
        unsafe { LargeUserAtomicType::new(None) }
    }
}
pub unsafe fn check_always_lock_free_51(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_52));
    {
        let known_status: LockFreeStatus =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut value_0));
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            (unsafe { ((*a) as std___atomic_ref_base_char_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_53(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_54));
    {
        let known_status: LockFreeStatus =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut value_4));
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () = (unsafe { ((*a) as std___atomic_ref_base_int_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_55(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_56));
    {
        let known_status: LockFreeStatus =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut value_12));
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            (unsafe { ((*a) as std___atomic_ref_base_float_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_57(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_58));
    {
        let known_status: LockFreeStatus =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut value_14));
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            (unsafe { ((*a) as std___atomic_ref_base_int_ptr_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_59(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_60));
    {
        let known_status: LockFreeStatus =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut value_16));
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            (unsafe { ((*a) as std___atomic_ref_base_Empty_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_61(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_62));
    {
        let known_status: LockFreeStatus =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut value_19));
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            (unsafe { ((*a) as std___atomic_ref_base_OneInt_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_63(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_64));
    {}
    {
        let mut is_lock_free: () =
            (unsafe { ((*a) as std___atomic_ref_base_IntArr2_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_65(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_66));
    {}
    {}
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_67(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_68));
    {}
    {}
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_69(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_70));
    {}
    {}
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_71(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_72));
    {}
    {}
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_73(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_74));
    {}
    {}
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_75(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_76));
    {}
    {}
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_77(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_78));
    {
        let known_status: LockFreeStatus =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut value_43));
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            (unsafe { ((*a) as std___atomic_ref_base_IntFloat_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub unsafe fn check_always_lock_free_79(a: *const ()) {
    let is_always_lock_free: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut is_always_lock_free_80));
    {
        let known_status: LockFreeStatus =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut value_46));
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            (unsafe { ((*a) as std___atomic_ref_base_CharEnumClass_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub unsafe fn test_81() {
    let mut c: () = ('x' as ());
    (unsafe {
        let mut _a: () = unimplemented!();
        check_always_lock_free_51(&mut _a)
    });
    let mut i: () = 0_();
    (unsafe {
        let mut _a: () = unimplemented!();
        check_always_lock_free_53(&mut _a)
    });
    let mut f: () = 0_f32;
    (unsafe {
        let mut _a: () = unimplemented!();
        check_always_lock_free_55(&mut _a)
    });
    let mut p: *mut () = (&mut i as *mut ());
    (unsafe {
        let mut _a: () = unimplemented!();
        check_always_lock_free_57(&mut _a)
    });
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: Empty_17 = <Empty_17>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_59(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: OneInt_20 = <OneInt_20>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_61(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: IntArr2_23 = <IntArr2_23>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_63(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: FloatArr3_26 = <FloatArr3_26>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_65(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: LLIArr2_29 = <LLIArr2_29>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_67(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: LLIArr4_32 = <LLIArr4_32>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_69(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: LLIArr8_35 = <LLIArr8_35>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_71(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: LLIArr16_38 = <LLIArr16_38>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_73(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: Padding_41 = <Padding_41>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_75(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: IntFloat_44 = <IntFloat_44>::default();
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_77(&a) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: CharEnumClass_47 = CharEnumClass_47_foo;
        let mut a: () = unimplemented!();
        (unsafe { check_always_lock_free_79(&a) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty_17 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OneInt_20 {
    pub i: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntArr2_23 {
    pub i: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FloatArr3_26 {
    pub i: [(); 3],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LLIArr2_29 {
    pub i: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LLIArr4_32 {
    pub i: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LLIArr8_35 {
    pub i: [(); 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LLIArr16_38 {
    pub i: [(); 16],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Padding_41 {
    pub c: (),
    pub i: (),
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union IntFloat_44 {
    pub i: (),
    pub f: (),
}
impl Default for IntFloat_44 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type CharEnumClass_47 = ();
pub const CharEnumClass_47_foo: CharEnumClass_47 = 0;
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
    (unsafe { test_81() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_0);
    std::cell::LazyCell::force(&*&raw const status_known_1);
    std::cell::LazyCell::force(&*&raw const value_2);
    std::cell::LazyCell::force(&*&raw const status_known_3);
    std::cell::LazyCell::force(&*&raw const value_4);
    std::cell::LazyCell::force(&*&raw const status_known_5);
    std::cell::LazyCell::force(&*&raw const value_6);
    std::cell::LazyCell::force(&*&raw const status_known_7);
    std::cell::LazyCell::force(&*&raw const value_8);
    std::cell::LazyCell::force(&*&raw const status_known_9);
    std::cell::LazyCell::force(&*&raw const value_10);
    std::cell::LazyCell::force(&*&raw const status_known_11);
    std::cell::LazyCell::force(&*&raw const value_12);
    std::cell::LazyCell::force(&*&raw const status_known_13);
    std::cell::LazyCell::force(&*&raw const value_14);
    std::cell::LazyCell::force(&*&raw const status_known_15);
    std::cell::LazyCell::force(&*&raw const value_16);
    std::cell::LazyCell::force(&*&raw const status_known_18);
    std::cell::LazyCell::force(&*&raw const value_19);
    std::cell::LazyCell::force(&*&raw const status_known_21);
    std::cell::LazyCell::force(&*&raw const value_22);
    std::cell::LazyCell::force(&*&raw const status_known_24);
    std::cell::LazyCell::force(&*&raw const value_25);
    std::cell::LazyCell::force(&*&raw const status_known_27);
    std::cell::LazyCell::force(&*&raw const value_28);
    std::cell::LazyCell::force(&*&raw const status_known_30);
    std::cell::LazyCell::force(&*&raw const value_31);
    std::cell::LazyCell::force(&*&raw const status_known_33);
    std::cell::LazyCell::force(&*&raw const value_34);
    std::cell::LazyCell::force(&*&raw const status_known_36);
    std::cell::LazyCell::force(&*&raw const value_37);
    std::cell::LazyCell::force(&*&raw const status_known_39);
    std::cell::LazyCell::force(&*&raw const value_40);
    std::cell::LazyCell::force(&*&raw const status_known_42);
    std::cell::LazyCell::force(&*&raw const value_43);
    std::cell::LazyCell::force(&*&raw const status_known_45);
    std::cell::LazyCell::force(&*&raw const value_46);
    std::cell::LazyCell::force(&*&raw const status_known_48);
}
