extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type LockFreeStatus = i32;
pub const LockFreeStatus_unknown: LockFreeStatus = -1;
pub const LockFreeStatus_never: LockFreeStatus = 0;
pub const LockFreeStatus_sometimes: LockFreeStatus = 1;
pub const LockFreeStatus_always: LockFreeStatus = 2;
thread_local!(
    pub static value_0: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i8>() == 8usize)
                                && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_1: Value<bool> = Rc::new(RefCell::new(
        (value_0.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_char_ {}
thread_local!(
    pub static value_2: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i16>() == 8usize)
                                && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_3: Value<bool> = Rc::new(RefCell::new(
        (value_2.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_short_ {}
thread_local!(
    pub static value_4: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i32>() == 8usize)
                                && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_5: Value<bool> = Rc::new(RefCell::new(
        (value_4.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_int_ {}
thread_local!(
    pub static value_6: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i64>() == 8usize)
                                && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_7: Value<bool> = Rc::new(RefCell::new(
        (value_6.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_long_ {}
thread_local!(
    pub static value_8: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((8usize == ::std::mem::size_of::<i8>()) && (8usize >= 8usize)) {
            2
        } else {
            (if ((8usize == ::std::mem::size_of::<i16>()) && (8usize >= 8usize)) {
                2
            } else {
                (if ((8usize == ::std::mem::size_of::<i32>()) && (8usize >= 8usize)) {
                    2
                } else {
                    (if ((8usize == ::std::mem::size_of::<i64>()) && (8usize >= 8usize)) {
                        2
                    } else {
                        (if ((8usize == ::std::mem::size_of::<i64>()) && (8usize >= 8usize)) {
                            2
                        } else {
                            (if ((8usize == 8usize) && (8usize >= 8usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_9: Value<bool> = Rc::new(RefCell::new(
        (value_8.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_void_ptr_ {}
thread_local!(
    pub static value_10: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i64>() == 8usize)
                                && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_11: Value<bool> = Rc::new(RefCell::new(
        (value_10.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_long_long_ {}
thread_local!(
    pub static value_12: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<f32>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<f32>() >= ::std::mem::size_of::<f32>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<f32>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<f32>() >= ::std::mem::size_of::<f32>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<f32>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<f32>() >= ::std::mem::size_of::<f32>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<f32>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<f32>() >= ::std::mem::size_of::<f32>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<f32>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<f32>() >= ::std::mem::size_of::<f32>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<f32>() == 8usize)
                                && (::std::mem::align_of::<f32>() >= ::std::mem::size_of::<f32>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_13: Value<bool> = Rc::new(RefCell::new(
        (value_12.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_float_ {}
thread_local!(
    pub static value_14: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((8usize == ::std::mem::size_of::<i8>()) && (8usize >= 8usize)) {
            2
        } else {
            (if ((8usize == ::std::mem::size_of::<i16>()) && (8usize >= 8usize)) {
                2
            } else {
                (if ((8usize == ::std::mem::size_of::<i32>()) && (8usize >= 8usize)) {
                    2
                } else {
                    (if ((8usize == ::std::mem::size_of::<i64>()) && (8usize >= 8usize)) {
                        2
                    } else {
                        (if ((8usize == ::std::mem::size_of::<i64>()) && (8usize >= 8usize)) {
                            2
                        } else {
                            (if ((8usize == 8usize) && (8usize >= 8usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_15: Value<bool> = Rc::new(RefCell::new(
        (value_14.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_int_ptr_ {}
thread_local!(
    pub static value_16: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((1usize == ::std::mem::size_of::<i8>()) && (1usize >= 1usize)) {
            2
        } else {
            (if ((1usize == ::std::mem::size_of::<i16>()) && (1usize >= 1usize)) {
                2
            } else {
                (if ((1usize == ::std::mem::size_of::<i32>()) && (1usize >= 1usize)) {
                    2
                } else {
                    (if ((1usize == ::std::mem::size_of::<i64>()) && (1usize >= 1usize)) {
                        2
                    } else {
                        (if ((1usize == ::std::mem::size_of::<i64>()) && (1usize >= 1usize)) {
                            2
                        } else {
                            (if ((1usize == 8usize) && (1usize >= 1usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_17: Value<bool> = Rc::new(RefCell::new(
        (value_16.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_Empty_ {}
thread_local!(
    pub static value_18: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((4usize == ::std::mem::size_of::<i8>()) && (4usize >= 4usize)) {
            2
        } else {
            (if ((4usize == ::std::mem::size_of::<i16>()) && (4usize >= 4usize)) {
                2
            } else {
                (if ((4usize == ::std::mem::size_of::<i32>()) && (4usize >= 4usize)) {
                    2
                } else {
                    (if ((4usize == ::std::mem::size_of::<i64>()) && (4usize >= 4usize)) {
                        2
                    } else {
                        (if ((4usize == ::std::mem::size_of::<i64>()) && (4usize >= 4usize)) {
                            2
                        } else {
                            (if ((4usize == 8usize) && (4usize >= 4usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_19: Value<bool> = Rc::new(RefCell::new(
        (value_18.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_OneInt_ {}
thread_local!(
    pub static value_20: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((8usize == ::std::mem::size_of::<i8>()) && (4usize >= 8usize)) {
            2
        } else {
            (if ((8usize == ::std::mem::size_of::<i16>()) && (4usize >= 8usize)) {
                2
            } else {
                (if ((8usize == ::std::mem::size_of::<i32>()) && (4usize >= 8usize)) {
                    2
                } else {
                    (if ((8usize == ::std::mem::size_of::<i64>()) && (4usize >= 8usize)) {
                        2
                    } else {
                        (if ((8usize == ::std::mem::size_of::<i64>()) && (4usize >= 8usize)) {
                            2
                        } else {
                            (if ((8usize == 8usize) && (4usize >= 8usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_21: Value<bool> = Rc::new(RefCell::new(
        (value_20.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_IntArr2_ {}
thread_local!(
    pub static value_22: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((12usize == ::std::mem::size_of::<i8>()) && (4usize >= 12usize)) {
            2
        } else {
            (if ((12usize == ::std::mem::size_of::<i16>()) && (4usize >= 12usize)) {
                2
            } else {
                (if ((12usize == ::std::mem::size_of::<i32>()) && (4usize >= 12usize)) {
                    2
                } else {
                    (if ((12usize == ::std::mem::size_of::<i64>()) && (4usize >= 12usize)) {
                        2
                    } else {
                        (if ((12usize == ::std::mem::size_of::<i64>()) && (4usize >= 12usize)) {
                            2
                        } else {
                            (if ((12usize == 8usize) && (4usize >= 12usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_23: Value<bool> = Rc::new(RefCell::new(
        (value_22.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_FloatArr3_ {}
thread_local!(
    pub static value_24: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((16usize == ::std::mem::size_of::<i8>()) && (8usize >= 16usize)) {
            2
        } else {
            (if ((16usize == ::std::mem::size_of::<i16>()) && (8usize >= 16usize)) {
                2
            } else {
                (if ((16usize == ::std::mem::size_of::<i32>()) && (8usize >= 16usize)) {
                    2
                } else {
                    (if ((16usize == ::std::mem::size_of::<i64>()) && (8usize >= 16usize)) {
                        2
                    } else {
                        (if ((16usize == ::std::mem::size_of::<i64>()) && (8usize >= 16usize)) {
                            2
                        } else {
                            (if ((16usize == 8usize) && (8usize >= 16usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_25: Value<bool> = Rc::new(RefCell::new(
        (value_24.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_LLIArr2_ {}
thread_local!(
    pub static value_26: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((32usize == ::std::mem::size_of::<i8>()) && (8usize >= 32usize)) {
            2
        } else {
            (if ((32usize == ::std::mem::size_of::<i16>()) && (8usize >= 32usize)) {
                2
            } else {
                (if ((32usize == ::std::mem::size_of::<i32>()) && (8usize >= 32usize)) {
                    2
                } else {
                    (if ((32usize == ::std::mem::size_of::<i64>()) && (8usize >= 32usize)) {
                        2
                    } else {
                        (if ((32usize == ::std::mem::size_of::<i64>()) && (8usize >= 32usize)) {
                            2
                        } else {
                            (if ((32usize == 8usize) && (8usize >= 32usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_27: Value<bool> = Rc::new(RefCell::new(
        (value_26.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_LLIArr4_ {}
thread_local!(
    pub static value_28: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((64usize == ::std::mem::size_of::<i8>()) && (8usize >= 64usize)) {
            2
        } else {
            (if ((64usize == ::std::mem::size_of::<i16>()) && (8usize >= 64usize)) {
                2
            } else {
                (if ((64usize == ::std::mem::size_of::<i32>()) && (8usize >= 64usize)) {
                    2
                } else {
                    (if ((64usize == ::std::mem::size_of::<i64>()) && (8usize >= 64usize)) {
                        2
                    } else {
                        (if ((64usize == ::std::mem::size_of::<i64>()) && (8usize >= 64usize)) {
                            2
                        } else {
                            (if ((64usize == 8usize) && (8usize >= 64usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_29: Value<bool> = Rc::new(RefCell::new(
        (value_28.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_LLIArr8_ {}
thread_local!(
    pub static value_30: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((128usize == ::std::mem::size_of::<i8>()) && (8usize >= 128usize)) {
            2
        } else {
            (if ((128usize == ::std::mem::size_of::<i16>()) && (8usize >= 128usize)) {
                2
            } else {
                (if ((128usize == ::std::mem::size_of::<i32>()) && (8usize >= 128usize)) {
                    2
                } else {
                    (if ((128usize == ::std::mem::size_of::<i64>()) && (8usize >= 128usize)) {
                        2
                    } else {
                        (if ((128usize == ::std::mem::size_of::<i64>()) && (8usize >= 128usize)) {
                            2
                        } else {
                            (if ((128usize == 8usize) && (8usize >= 128usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_31: Value<bool> = Rc::new(RefCell::new(
        (value_30.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_LLIArr16_ {}
thread_local!(
    pub static value_32: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((16usize == ::std::mem::size_of::<i8>()) && (8usize >= 16usize)) {
            2
        } else {
            (if ((16usize == ::std::mem::size_of::<i16>()) && (8usize >= 16usize)) {
                2
            } else {
                (if ((16usize == ::std::mem::size_of::<i32>()) && (8usize >= 16usize)) {
                    2
                } else {
                    (if ((16usize == ::std::mem::size_of::<i64>()) && (8usize >= 16usize)) {
                        2
                    } else {
                        (if ((16usize == ::std::mem::size_of::<i64>()) && (8usize >= 16usize)) {
                            2
                        } else {
                            (if ((16usize == 8usize) && (8usize >= 16usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_33: Value<bool> = Rc::new(RefCell::new(
        (value_32.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_Padding_ {}
thread_local!(
    pub static value_34: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((4usize == ::std::mem::size_of::<i8>()) && (4usize >= 4usize)) {
            2
        } else {
            (if ((4usize == ::std::mem::size_of::<i16>()) && (4usize >= 4usize)) {
                2
            } else {
                (if ((4usize == ::std::mem::size_of::<i32>()) && (4usize >= 4usize)) {
                    2
                } else {
                    (if ((4usize == ::std::mem::size_of::<i64>()) && (4usize >= 4usize)) {
                        2
                    } else {
                        (if ((4usize == ::std::mem::size_of::<i64>()) && (4usize >= 4usize)) {
                            2
                        } else {
                            (if ((4usize == 8usize) && (4usize >= 4usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_35: Value<bool> = Rc::new(RefCell::new(
        (value_34.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_IntFloat_ {}
thread_local!(
    pub static value_36: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<CharEnumClass_37>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<CharEnumClass_37>()
                >= ::std::mem::size_of::<CharEnumClass_37>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<CharEnumClass_37>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<CharEnumClass_37>()
                    >= ::std::mem::size_of::<CharEnumClass_37>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<CharEnumClass_37>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<CharEnumClass_37>()
                        >= ::std::mem::size_of::<CharEnumClass_37>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<CharEnumClass_37>()
                        == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<CharEnumClass_37>()
                            >= ::std::mem::size_of::<CharEnumClass_37>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<CharEnumClass_37>()
                            == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<CharEnumClass_37>()
                                >= ::std::mem::size_of::<CharEnumClass_37>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<CharEnumClass_37>() == 8usize)
                                && (::std::mem::align_of::<CharEnumClass_37>()
                                    >= ::std::mem::size_of::<CharEnumClass_37>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_38: Value<bool> = Rc::new(RefCell::new(
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
pub fn operator_eq_39(x: Ptr<UserAtomicType>, y: Ptr<UserAtomicType>) -> bool {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct UserAtomicType {
    #[offset(0)]
    pub i: i32,
}
impl UserAtomicType {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
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
pub fn operator_eq_40(x: Ptr<LargeUserAtomicType>, y: Ptr<LargeUserAtomicType>) -> bool {
    let mut i: i32 = 0;
    'loop_: while (i < 128) {
        if ({ (array_elem!(x, a, i).read()) } != { (array_elem!(y, a, i).read()) }) {
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new, some_value)]
#[byte_size(512)]
pub struct LargeUserAtomicType {
    #[offset(0)]
    #[byte_size(512)]
    pub a: Box<[i32]>,
}
impl LargeUserAtomicType {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        let __this: Value<LargeUserAtomicType> = Rc::new(RefCell::new(Self {
            a: (0..128).map(|_| 0_i32).collect::<Box<[i32]>>(),
        }));
        let this: Ptr<LargeUserAtomicType> = __this.as_pointer();
        'loop_: for e in IterRange::new(
            (array_field_ptr!(this, a) as Ptr<i32>),
            (array_field_ptr!(this, a) as Ptr<i32>).offset((128_i64) as isize),
        ) {
            let e: Ptr<i32> = (e).clone();
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
    let mut is_always_lock_free: bool = unimplemented!();
    {
        let mut known_status: LockFreeStatus = value_0.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            assert!(
                (unimplemented!())
                    && (!(Ptr::<i8>::from_string_literal(
                        b"is_lock_free() is inconsistent with known lock-free status"
                    ))
                    .is_null())
            );
        }
    }
    {
        let mut is_lock_free: bool = unimplemented!();
        assert!((is_lock_free as bool));
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_42(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {
        let mut known_status: LockFreeStatus = value_4.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            assert!(
                (unimplemented!())
                    && (!(Ptr::<i8>::from_string_literal(
                        b"is_lock_free() is inconsistent with known lock-free status"
                    ))
                    .is_null())
            );
        }
    }
    {
        let mut is_lock_free: bool = unimplemented!();
        assert!((is_lock_free as bool));
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_43(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {
        let mut known_status: LockFreeStatus = value_12.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            assert!(
                (unimplemented!())
                    && (!(Ptr::<i8>::from_string_literal(
                        b"is_lock_free() is inconsistent with known lock-free status"
                    ))
                    .is_null())
            );
        }
    }
    {
        let mut is_lock_free: bool = unimplemented!();
        assert!((is_lock_free as bool));
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_44(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {
        let mut known_status: LockFreeStatus = value_14.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            assert!(
                (unimplemented!())
                    && (!(Ptr::<i8>::from_string_literal(
                        b"is_lock_free() is inconsistent with known lock-free status"
                    ))
                    .is_null())
            );
        }
    }
    {
        let mut is_lock_free: bool = unimplemented!();
        assert!((is_lock_free as bool));
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_45(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {
        let mut known_status: LockFreeStatus = value_16.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            assert!(
                (unimplemented!())
                    && (!(Ptr::<i8>::from_string_literal(
                        b"is_lock_free() is inconsistent with known lock-free status"
                    ))
                    .is_null())
            );
        }
    }
    {
        let mut is_lock_free: bool = unimplemented!();
        assert!((is_lock_free as bool));
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_46(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {
        let mut known_status: LockFreeStatus = value_18.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            assert!(
                (unimplemented!())
                    && (!(Ptr::<i8>::from_string_literal(
                        b"is_lock_free() is inconsistent with known lock-free status"
                    ))
                    .is_null())
            );
        }
    }
    {
        let mut is_lock_free: bool = unimplemented!();
        assert!((is_lock_free as bool));
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_47(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {}
    {
        let mut is_lock_free: bool = unimplemented!();
        assert!((is_lock_free as bool));
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_48(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_49(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_50(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_51(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_52(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_53(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {}
    {}
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_54(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {
        let mut known_status: LockFreeStatus = value_34.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            assert!(
                (unimplemented!())
                    && (!(Ptr::<i8>::from_string_literal(
                        b"is_lock_free() is inconsistent with known lock-free status"
                    ))
                    .is_null())
            );
        }
    }
    {
        let mut is_lock_free: bool = unimplemented!();
        assert!((is_lock_free as bool));
    }
    const _: () = assert!(true, "");;
}
pub fn check_always_lock_free_55(a: Ptr<()>) {
    let mut is_always_lock_free: bool = unimplemented!();
    {
        let mut known_status: LockFreeStatus = value_36.with(|rc| *rc.borrow());
        {
            const _: () = assert!(true, "is_always_lock_free");;
            assert!(
                (unimplemented!())
                    && (!(Ptr::<i8>::from_string_literal(
                        b"is_lock_free() is inconsistent with known lock-free status"
                    ))
                    .is_null())
            );
        }
    }
    {
        let mut is_lock_free: bool = unimplemented!();
        assert!((is_lock_free as bool));
    }
    const _: () = assert!(true, "");;
}
pub fn test_56() {
    let mut c: i8 = ('x' as i8);
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_always_lock_free_41(_a.as_pointer())
    });
    let i: Value<i32> = Rc::new(RefCell::new(0));
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_always_lock_free_42(_a.as_pointer())
    });
    let mut f: f32 = 0_f32;
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_always_lock_free_43(_a.as_pointer())
    });
    let mut p: Ptr<i32> = (i.as_pointer());
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        check_always_lock_free_44(_a.as_pointer())
    });
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: Empty_57 = <Empty_57>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_45(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: OneInt_58 = <OneInt_58>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_46(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: IntArr2_59 = <IntArr2_59>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_47(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: FloatArr3_60 = <FloatArr3_60>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_48(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: LLIArr2_61 = <LLIArr2_61>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_49(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: LLIArr4_62 = <LLIArr4_62>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_50(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: LLIArr8_63 = <LLIArr8_63>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_51(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: LLIArr16_64 = <LLIArr16_64>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_52(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: Padding_65 = <Padding_65>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_53(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let obj: Value<IntFloat_66> = Rc::new(RefCell::new(<IntFloat_66>::default()));
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_54(a.as_pointer()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut obj: CharEnumClass_37 = <CharEnumClass_37>::default();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_always_lock_free_55(a.as_pointer()) });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty_57 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OneInt_58 {
    #[offset(0)]
    pub i: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct IntArr2_59 {
    #[offset(0)]
    #[byte_size(8)]
    pub i: Box<[i32]>,
}
impl Default for IntArr2_59 {
    fn default() -> Self {
        {
            IntArr2_59 {
                i: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct FloatArr3_60 {
    #[offset(0)]
    #[byte_size(12)]
    pub i: Box<[f32]>,
}
impl Default for FloatArr3_60 {
    fn default() -> Self {
        {
            FloatArr3_60 {
                i: (0..3).map(|_| 0_f32).collect::<Box<[f32]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct LLIArr2_61 {
    #[offset(0)]
    #[byte_size(16)]
    pub i: Box<[i64]>,
}
impl Default for LLIArr2_61 {
    fn default() -> Self {
        {
            LLIArr2_61 {
                i: (0..2).map(|_| 0_i64).collect::<Box<[i64]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct LLIArr4_62 {
    #[offset(0)]
    #[byte_size(32)]
    pub i: Box<[i64]>,
}
impl Default for LLIArr4_62 {
    fn default() -> Self {
        {
            LLIArr4_62 {
                i: (0..4).map(|_| 0_i64).collect::<Box<[i64]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(64)]
pub struct LLIArr8_63 {
    #[offset(0)]
    #[byte_size(64)]
    pub i: Box<[i64]>,
}
impl Default for LLIArr8_63 {
    fn default() -> Self {
        {
            LLIArr8_63 {
                i: (0..8).map(|_| 0_i64).collect::<Box<[i64]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(128)]
pub struct LLIArr16_64 {
    #[offset(0)]
    #[byte_size(128)]
    pub i: Box<[i64]>,
}
impl Default for LLIArr16_64 {
    fn default() -> Self {
        {
            LLIArr16_64 {
                i: (0..16).map(|_| 0_i64).collect::<Box<[i64]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Padding_65 {
    #[offset(0)]
    pub c: i8,
    #[offset(8)]
    pub i: i64,
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct IntFloat_66 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl IntFloat_66 {
    pub fn i(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn f(this: Ptr<Self>) -> Ptr<f32> {
        this.reinterpret_cast()
    }
}
impl Default for IntFloat_66 {
    fn default() -> Self {
        IntFloat_66 {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
pub type CharEnumClass_37 = i8;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_56() });
    return 0;
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
