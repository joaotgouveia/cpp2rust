// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1001(a0: i32) -> i32 {
    if (0..=255).contains(&a0) {
        (a0 as u8).to_ascii_uppercase() as i32
    } else {
        a0
    }
}

unsafe fn f1003(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_alphanumeric()) { 8 } else { 0 }
}

unsafe fn f1004(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_alphabetic()) { 1024 } else { 0 }
}

unsafe fn f1005(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && (__c == 0x20 || __c == 0x09) { 1 } else { 0 }
}

unsafe fn f1006(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_control()) { 2 } else { 0 }
}

unsafe fn f1007(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_digit()) { 2048 } else { 0 }
}

unsafe fn f1008(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_graphic()) { 32768 } else { 0 }
}

unsafe fn f1009(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_lowercase()) { 512 } else { 0 }
}

unsafe fn f1010(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((0x20..=0x7e).contains(&__c)) { 16384 } else { 0 }
}

unsafe fn f1011(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_punctuation()) { 4 } else { 0 }
}

unsafe fn f1012(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && (__c == 0x20 || (0x09..=0x0d).contains(&__c)) { 8192 } else { 0 }
}

unsafe fn f1013(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_uppercase()) { 256 } else { 0 }
}

unsafe fn f1014(a0: i32) -> i32 {
    let __c = a0;
    if (0..=0x7f).contains(&__c) && ((__c as u8).is_ascii_hexdigit()) { 4096 } else { 0 }
}

unsafe fn f1002(a0: i32) -> i32 {
    if (0..=255).contains(&a0) {
        (a0 as u8).to_ascii_lowercase() as i32
    } else {
        a0
    }
}
