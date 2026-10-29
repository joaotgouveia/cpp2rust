extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type LockFreeStatus = ();
pub const LockFreeStatus_unknown: LockFreeStatus = -1;
pub const LockFreeStatus_never: LockFreeStatus = 0;
pub const LockFreeStatus_sometimes: LockFreeStatus = 1;
pub const LockFreeStatus_always: LockFreeStatus = 2;
thread_local!(
    pub static value_0: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_1: Value<()> = Rc::new(RefCell::new(
        (value_0.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_char_ {}
thread_local!(
    pub static value_2: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_3: Value<()> = Rc::new(RefCell::new(
        (value_2.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_short_ {}
thread_local!(
    pub static value_4: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_5: Value<()> = Rc::new(RefCell::new(
        (value_4.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_int_ {}
thread_local!(
    pub static value_6: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_7: Value<()> = Rc::new(RefCell::new(
        (value_6.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_long_ {}
thread_local!(
    pub static value_8: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
            2_()
        } else {
            (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                2_()
            } else {
                (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                    2_()
                } else {
                    (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                        2_()
                    } else {
                        (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                            2_()
                        } else {
                            (if ((8usize == 8usize) && (8usize >= 8usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_9: Value<()> = Rc::new(RefCell::new(
        (value_8.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_void_ptr_ {}
thread_local!(
    pub static value_10: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_11: Value<()> = Rc::new(RefCell::new(
        (value_10.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_long_long_ {}
thread_local!(
    pub static value_12: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_13: Value<()> = Rc::new(RefCell::new(
        (value_12.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_float_ {}
thread_local!(
    pub static value_14: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
            2_()
        } else {
            (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                2_()
            } else {
                (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                    2_()
                } else {
                    (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                        2_()
                    } else {
                        (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                            2_()
                        } else {
                            (if ((8usize == 8usize) && (8usize >= 8usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_15: Value<()> = Rc::new(RefCell::new(
        (value_14.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_int_ptr_ {}
thread_local!(
    pub static value_16: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((1usize == ::std::mem::size_of::<()>()) && (1usize >= 1usize)) {
            2_()
        } else {
            (if ((1usize == ::std::mem::size_of::<()>()) && (1usize >= 1usize)) {
                2_()
            } else {
                (if ((1usize == ::std::mem::size_of::<()>()) && (1usize >= 1usize)) {
                    2_()
                } else {
                    (if ((1usize == ::std::mem::size_of::<()>()) && (1usize >= 1usize)) {
                        2_()
                    } else {
                        (if ((1usize == ::std::mem::size_of::<()>()) && (1usize >= 1usize)) {
                            2_()
                        } else {
                            (if ((1usize == 8usize) && (1usize >= 1usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_17: Value<()> = Rc::new(RefCell::new(
        (value_16.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_Empty_ {}
thread_local!(
    pub static value_18: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
            2_()
        } else {
            (if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
                2_()
            } else {
                (if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
                    2_()
                } else {
                    (if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
                        2_()
                    } else {
                        (if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
                            2_()
                        } else {
                            (if ((4usize == 8usize) && (4usize >= 4usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_19: Value<()> = Rc::new(RefCell::new(
        (value_18.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_OneInt_ {}
thread_local!(
    pub static value_20: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((8usize == ::std::mem::size_of::<()>()) && (4usize >= 8usize)) {
            2_()
        } else {
            (if ((8usize == ::std::mem::size_of::<()>()) && (4usize >= 8usize)) {
                2_()
            } else {
                (if ((8usize == ::std::mem::size_of::<()>()) && (4usize >= 8usize)) {
                    2_()
                } else {
                    (if ((8usize == ::std::mem::size_of::<()>()) && (4usize >= 8usize)) {
                        2_()
                    } else {
                        (if ((8usize == ::std::mem::size_of::<()>()) && (4usize >= 8usize)) {
                            2_()
                        } else {
                            (if ((8usize == 8usize) && (4usize >= 8usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_21: Value<()> = Rc::new(RefCell::new(
        (value_20.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_IntArr2_ {}
thread_local!(
    pub static value_22: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((12usize == ::std::mem::size_of::<()>()) && (4usize >= 12usize)) {
            2_()
        } else {
            (if ((12usize == ::std::mem::size_of::<()>()) && (4usize >= 12usize)) {
                2_()
            } else {
                (if ((12usize == ::std::mem::size_of::<()>()) && (4usize >= 12usize)) {
                    2_()
                } else {
                    (if ((12usize == ::std::mem::size_of::<()>()) && (4usize >= 12usize)) {
                        2_()
                    } else {
                        (if ((12usize == ::std::mem::size_of::<()>()) && (4usize >= 12usize)) {
                            2_()
                        } else {
                            (if ((12usize == 8usize) && (4usize >= 12usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_23: Value<()> = Rc::new(RefCell::new(
        (value_22.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_FloatArr3_ {}
thread_local!(
    pub static value_24: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
            2_()
        } else {
            (if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
                2_()
            } else {
                (if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
                    2_()
                } else {
                    (if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
                        2_()
                    } else {
                        (if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
                            2_()
                        } else {
                            (if ((16usize == 8usize) && (8usize >= 16usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_25: Value<()> = Rc::new(RefCell::new(
        (value_24.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_LLIArr2_ {}
thread_local!(
    pub static value_26: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((32usize == ::std::mem::size_of::<()>()) && (8usize >= 32usize)) {
            2_()
        } else {
            (if ((32usize == ::std::mem::size_of::<()>()) && (8usize >= 32usize)) {
                2_()
            } else {
                (if ((32usize == ::std::mem::size_of::<()>()) && (8usize >= 32usize)) {
                    2_()
                } else {
                    (if ((32usize == ::std::mem::size_of::<()>()) && (8usize >= 32usize)) {
                        2_()
                    } else {
                        (if ((32usize == ::std::mem::size_of::<()>()) && (8usize >= 32usize)) {
                            2_()
                        } else {
                            (if ((32usize == 8usize) && (8usize >= 32usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_27: Value<()> = Rc::new(RefCell::new(
        (value_26.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_LLIArr4_ {}
thread_local!(
    pub static value_28: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((64usize == ::std::mem::size_of::<()>()) && (8usize >= 64usize)) {
            2_()
        } else {
            (if ((64usize == ::std::mem::size_of::<()>()) && (8usize >= 64usize)) {
                2_()
            } else {
                (if ((64usize == ::std::mem::size_of::<()>()) && (8usize >= 64usize)) {
                    2_()
                } else {
                    (if ((64usize == ::std::mem::size_of::<()>()) && (8usize >= 64usize)) {
                        2_()
                    } else {
                        (if ((64usize == ::std::mem::size_of::<()>()) && (8usize >= 64usize)) {
                            2_()
                        } else {
                            (if ((64usize == 8usize) && (8usize >= 64usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_29: Value<()> = Rc::new(RefCell::new(
        (value_28.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_LLIArr8_ {}
thread_local!(
    pub static value_30: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((128usize == ::std::mem::size_of::<()>()) && (8usize >= 128usize)) {
            2_()
        } else {
            (if ((128usize == ::std::mem::size_of::<()>()) && (8usize >= 128usize)) {
                2_()
            } else {
                (if ((128usize == ::std::mem::size_of::<()>()) && (8usize >= 128usize)) {
                    2_()
                } else {
                    (if ((128usize == ::std::mem::size_of::<()>()) && (8usize >= 128usize)) {
                        2_()
                    } else {
                        (if ((128usize == ::std::mem::size_of::<()>()) && (8usize >= 128usize)) {
                            2_()
                        } else {
                            (if ((128usize == 8usize) && (8usize >= 128usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_31: Value<()> = Rc::new(RefCell::new(
        (value_30.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_LLIArr16_ {}
thread_local!(
    pub static value_32: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
            2_()
        } else {
            (if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
                2_()
            } else {
                (if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
                    2_()
                } else {
                    (if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
                        2_()
                    } else {
                        (if ((16usize == ::std::mem::size_of::<()>()) && (8usize >= 16usize)) {
                            2_()
                        } else {
                            (if ((16usize == 8usize) && (8usize >= 16usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_33: Value<()> = Rc::new(RefCell::new(
        (value_32.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_Padding_ {}
thread_local!(
    pub static value_34: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
            2_()
        } else {
            (if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
                2_()
            } else {
                (if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
                    2_()
                } else {
                    (if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
                        2_()
                    } else {
                        (if ((4usize == ::std::mem::size_of::<()>()) && (4usize >= 4usize)) {
                            2_()
                        } else {
                            (if ((4usize == 8usize) && (4usize >= 4usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_35: Value<()> = Rc::new(RefCell::new(
        (value_34.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_IntFloat_ {}
thread_local!(
    pub static value_36: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<CharEnumClass_37>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<CharEnumClass_37>()
                >= ::std::mem::size_of::<CharEnumClass_37>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<CharEnumClass_37>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<CharEnumClass_37>()
                    >= ::std::mem::size_of::<CharEnumClass_37>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<CharEnumClass_37>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<CharEnumClass_37>()
                        >= ::std::mem::size_of::<CharEnumClass_37>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<CharEnumClass_37>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<CharEnumClass_37>()
                            >= ::std::mem::size_of::<CharEnumClass_37>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<CharEnumClass_37>()
                            == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<CharEnumClass_37>()
                                >= ::std::mem::size_of::<CharEnumClass_37>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<CharEnumClass_37>() == 8usize)
                                && (::std::mem::align_of::<CharEnumClass_37>()
                                    >= ::std::mem::size_of::<CharEnumClass_37>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_38: Value<()> = Rc::new(RefCell::new(
        (value_36.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
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
pub fn operator_eq_39(x: Ptr<UserAtomicType>, y: Ptr<UserAtomicType>) -> () {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct UserAtomicType {
    #[offset(0)]
    pub i: (),
}
impl UserAtomicType {
    pub fn new(d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| 0_());
        Self { i: d }
    }
}
impl CppEq for UserAtomicType {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_39(__lhs.clone(), __rhs.clone())
    }
}
impl Default for UserAtomicType {
    fn default() -> Self {
        { UserAtomicType::new(None) }
    }
}
pub fn operator_eq_40(x: Ptr<LargeUserAtomicType>, y: Ptr<LargeUserAtomicType>) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 128_()) {
        if ({ (array_elem!(x, a, i).read()) } != { (array_elem!(y, a, i).read()) }) {
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(512)]
pub struct LargeUserAtomicType {
    #[offset(0)]
    #[byte_size(512)]
    pub a: Box<[()]>,
}
impl LargeUserAtomicType {
    pub fn new(d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| 0_());
        let __this: Value<LargeUserAtomicType> = Rc::new(RefCell::new(Self {
            a: (0..128).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<LargeUserAtomicType> = __this.as_pointer();
        'loop_: for e in IterRange::new(
            (array_field_ptr!(this, a) as Ptr<()>),
            (array_field_ptr!(this, a) as Ptr<()>).offset((128_()) as isize),
        ) {
            let e: Ptr<()> = (e).clone();
            let __rhs = d.postfix_inc();
            e.write(__rhs);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for LargeUserAtomicType {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_40(__lhs.clone(), __rhs.clone())
    }
}
impl Default for LargeUserAtomicType {
    fn default() -> Self {
        { LargeUserAtomicType::new(None) }
    }
}
pub fn check_always_lock_free_41(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_42.with(|rc| *rc.borrow());
    {
        let mut known_status: LockFreeStatus = value_0.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () = unimplemented!();
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_43(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_44.with(|rc| *rc.borrow());
    {
        let mut known_status: LockFreeStatus = value_4.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () = unimplemented!();
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_45(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_46.with(|rc| *rc.borrow());
    {
        let mut known_status: LockFreeStatus = value_12.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            ({ ((*a.upgrade().deref()) as std___atomic_ref_float__false__true_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_47(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_48.with(|rc| *rc.borrow());
    {
        let mut known_status: LockFreeStatus = value_14.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () = ({
            ((*a.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_).is_lock_free()
        });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_49(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_50.with(|rc| *rc.borrow());
    {
        let mut known_status: LockFreeStatus = value_16.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            ({ ((*a.upgrade().deref()) as std___atomic_ref_Empty__false__false_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_51(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_52.with(|rc| *rc.borrow());
    {
        let mut known_status: LockFreeStatus = value_18.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () =
            ({ ((*a.upgrade().deref()) as std___atomic_ref_OneInt__false__false_).is_lock_free() });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_53(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_54.with(|rc| *rc.borrow());
    {}
    {
        let mut is_lock_free: () = ({
            ((*a.upgrade().deref()) as std___atomic_ref_IntArr2__false__false_).is_lock_free()
        });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_55(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_56.with(|rc| *rc.borrow());
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_57(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_58.with(|rc| *rc.borrow());
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_59(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_60.with(|rc| *rc.borrow());
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_61(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_62.with(|rc| *rc.borrow());
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_63(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_64.with(|rc| *rc.borrow());
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_65(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_66.with(|rc| *rc.borrow());
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_67(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_68.with(|rc| *rc.borrow());
    {
        let mut known_status: LockFreeStatus = value_34.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () = ({
            ((*a.upgrade().deref()) as std___atomic_ref_IntFloat__false__false_).is_lock_free()
        });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_69(a: Ptr<()>) {
    let mut is_always_lock_free: () = is_always_lock_free_70.with(|rc| *rc.borrow());
    {
        let mut known_status: LockFreeStatus = value_36.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            unimplemented!();
        }
    }
    {
        let mut is_lock_free: () = ({
            ((*a.upgrade().deref()) as std___atomic_ref_CharEnumClass__false__false_).is_lock_free()
        });
        unimplemented!();
    }
    const _: () = assert!(true, "");;
}
pub fn test_71() {
    let mut c: () = ('x' as ());
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_always_lock_free_41(_a.as_pointer())
    });
    let i: Value<()> = Rc::new(RefCell::new(0_()));
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_always_lock_free_43(_a.as_pointer())
    });
    let mut f: () = 0_f32;
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_always_lock_free_45(_a.as_pointer())
    });
    let mut p: Ptr<()> = (i.as_pointer());
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_always_lock_free_47(_a.as_pointer())
    });
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: Empty_72 = <Empty_72>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_49(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: OneInt_73 = <OneInt_73>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_51(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: IntArr2_74 = <IntArr2_74>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_53(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: FloatArr3_75 = <FloatArr3_75>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_55(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: LLIArr2_76 = <LLIArr2_76>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_57(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: LLIArr4_77 = <LLIArr4_77>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_59(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: LLIArr8_78 = <LLIArr8_78>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_61(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: LLIArr16_79 = <LLIArr16_79>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_63(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: Padding_80 = <Padding_80>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_65(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let obj: Value<IntFloat_81> = Rc::new(RefCell::new(<IntFloat_81>::default()));
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_67(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut obj: CharEnumClass_37 = <CharEnumClass_37>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_69(a.as_pointer()) });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty_72 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OneInt_73 {
    #[offset(0)]
    pub i: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct IntArr2_74 {
    #[offset(0)]
    #[byte_size(8)]
    pub i: Box<[()]>,
}
impl Default for IntArr2_74 {
    fn default() -> Self {
        {
            IntArr2_74 {
                i: (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct FloatArr3_75 {
    #[offset(0)]
    #[byte_size(12)]
    pub i: Box<[()]>,
}
impl Default for FloatArr3_75 {
    fn default() -> Self {
        {
            FloatArr3_75 {
                i: (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct LLIArr2_76 {
    #[offset(0)]
    #[byte_size(16)]
    pub i: Box<[()]>,
}
impl Default for LLIArr2_76 {
    fn default() -> Self {
        {
            LLIArr2_76 {
                i: (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct LLIArr4_77 {
    #[offset(0)]
    #[byte_size(32)]
    pub i: Box<[()]>,
}
impl Default for LLIArr4_77 {
    fn default() -> Self {
        {
            LLIArr4_77 {
                i: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(64)]
pub struct LLIArr8_78 {
    #[offset(0)]
    #[byte_size(64)]
    pub i: Box<[()]>,
}
impl Default for LLIArr8_78 {
    fn default() -> Self {
        {
            LLIArr8_78 {
                i: (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(128)]
pub struct LLIArr16_79 {
    #[offset(0)]
    #[byte_size(128)]
    pub i: Box<[()]>,
}
impl Default for LLIArr16_79 {
    fn default() -> Self {
        {
            LLIArr16_79 {
                i: (0..16).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Padding_80 {
    #[offset(0)]
    pub c: (),
    #[offset(8)]
    pub i: (),
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct IntFloat_81 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl IntFloat_81 {
    pub fn i(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn f(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for IntFloat_81 {
    fn default() -> Self {
        IntFloat_81 {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
pub type CharEnumClass_37 = ();
pub const CharEnumClass_37_foo: CharEnumClass_37 = 0;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_71() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = value_0.with(|_| ());
    let _ = status_known_1.with(|_| ());
    let _ = value_2.with(|_| ());
    let _ = status_known_3.with(|_| ());
    let _ = value_4.with(|_| ());
    let _ = status_known_5.with(|_| ());
    let _ = value_6.with(|_| ());
    let _ = status_known_7.with(|_| ());
    let _ = value_8.with(|_| ());
    let _ = status_known_9.with(|_| ());
    let _ = value_10.with(|_| ());
    let _ = status_known_11.with(|_| ());
    let _ = value_12.with(|_| ());
    let _ = status_known_13.with(|_| ());
    let _ = value_14.with(|_| ());
    let _ = status_known_15.with(|_| ());
    let _ = value_16.with(|_| ());
    let _ = status_known_17.with(|_| ());
    let _ = value_18.with(|_| ());
    let _ = status_known_19.with(|_| ());
    let _ = value_20.with(|_| ());
    let _ = status_known_21.with(|_| ());
    let _ = value_22.with(|_| ());
    let _ = status_known_23.with(|_| ());
    let _ = value_24.with(|_| ());
    let _ = status_known_25.with(|_| ());
    let _ = value_26.with(|_| ());
    let _ = status_known_27.with(|_| ());
    let _ = value_28.with(|_| ());
    let _ = status_known_29.with(|_| ());
    let _ = value_30.with(|_| ());
    let _ = status_known_31.with(|_| ());
    let _ = value_32.with(|_| ());
    let _ = status_known_33.with(|_| ());
    let _ = value_34.with(|_| ());
    let _ = status_known_35.with(|_| ());
    let _ = value_36.with(|_| ());
    let _ = status_known_38.with(|_| ());
}
