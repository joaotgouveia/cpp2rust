extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::address_family_not_supported) == EAFNOSUPPORT"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::address_in_use) == EADDRINUSE"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::address_not_available) == EADDRNOTAVAIL"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::already_connected) == EISCONN"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::argument_list_too_long) == E2BIG"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::argument_out_of_domain) == EDOM"
    );;
    const _: () = assert!(true, "static_cast<int>(std::errc::bad_address) == EFAULT");;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::bad_file_descriptor) == EBADF"
    );;
    const _: () = assert!(true, "static_cast<int>(std::errc::bad_message) == EBADMSG");;
    const _: () = assert!(true, "static_cast<int>(std::errc::broken_pipe) == EPIPE");;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::connection_aborted) == ECONNABORTED"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::connection_already_in_progress) == EALREADY"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::connection_refused) == ECONNREFUSED"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::connection_reset) == ECONNRESET"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::cross_device_link) == EXDEV"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::destination_address_required) == EDESTADDRREQ"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::device_or_resource_busy) == EBUSY"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::directory_not_empty) == ENOTEMPTY"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::executable_format_error) == ENOEXEC"
    );;
    const _: () = assert!(true, "static_cast<int>(std::errc::file_exists) == EEXIST");;
    const _: () = assert!(true, "static_cast<int>(std::errc::file_too_large) == EFBIG");;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::filename_too_long) == ENAMETOOLONG"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::function_not_supported) == ENOSYS"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::host_unreachable) == EHOSTUNREACH"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::identifier_removed) == EIDRM"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::illegal_byte_sequence) == EILSEQ"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::inappropriate_io_control_operation) == ENOTTY"
    );;
    const _: () = assert!(true, "static_cast<int>(std::errc::interrupted) == EINTR");;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::invalid_argument) == EINVAL"
    );;
    const _: () = assert!(true, "static_cast<int>(std::errc::invalid_seek) == ESPIPE");;
    const _: () = assert!(true, "static_cast<int>(std::errc::io_error) == EIO");;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::is_a_directory) == EISDIR"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::message_size) == EMSGSIZE"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::network_down) == ENETDOWN"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::network_reset) == ENETRESET"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::network_unreachable) == ENETUNREACH"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::no_buffer_space) == ENOBUFS"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::no_child_process) == ECHILD"
    );;
    const _: () = assert!(true, "static_cast<int>(std::errc::no_link) == ENOLINK");;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::no_lock_available) == ENOLCK"
    );;
    const _: () = assert!(true, "static_cast<int>(std::errc::no_message) == ENOMSG");;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::no_protocol_option) == ENOPROTOOPT"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::no_space_on_device) == ENOSPC"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::no_such_device_or_address) == ENXIO"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::no_such_device) == ENODEV"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::no_such_file_or_directory) == ENOENT"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::no_such_process) == ESRCH"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::not_a_directory) == ENOTDIR"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::not_a_socket) == ENOTSOCK"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::not_connected) == ENOTCONN"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::not_enough_memory) == ENOMEM"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::not_supported) == ENOTSUP"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::operation_canceled) == ECANCELED"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::operation_in_progress) == EINPROGRESS"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::operation_not_permitted) == EPERM"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::operation_not_supported) == EOPNOTSUPP"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::operation_would_block) == EWOULDBLOCK"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::owner_dead) == EOWNERDEAD"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::permission_denied) == EACCES"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::protocol_error) == EPROTO"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::protocol_not_supported) == EPROTONOSUPPORT"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::read_only_file_system) == EROFS"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::resource_deadlock_would_occur) == EDEADLK"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::resource_unavailable_try_again) == EAGAIN"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::result_out_of_range) == ERANGE"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::state_not_recoverable) == ENOTRECOVERABLE"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::text_file_busy) == ETXTBSY"
    );;
    const _: () = assert!(true, "static_cast<int>(std::errc::timed_out) == ETIMEDOUT");;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::too_many_files_open_in_system) == ENFILE"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::too_many_files_open) == EMFILE"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::too_many_links) == EMLINK"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::too_many_symbolic_link_levels) == ELOOP"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::value_too_large) == EOVERFLOW"
    );;
    const _: () = assert!(
        true,
        "static_cast<int>(std::errc::wrong_protocol_type) == EPROTOTYPE"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
