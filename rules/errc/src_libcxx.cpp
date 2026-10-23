#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::errc;

std::errc f1() { return std::errc::address_family_not_supported; }

std::errc f2() { return std::errc::address_in_use; }

std::errc f3() { return std::errc::address_not_available; }

std::errc f4() { return std::errc::already_connected; }

std::errc f5() { return std::errc::argument_list_too_long; }

std::errc f6() { return std::errc::argument_out_of_domain; }

std::errc f7() { return std::errc::bad_address; }

std::errc f8() { return std::errc::bad_file_descriptor; }

std::errc f9() { return std::errc::bad_message; }

std::errc f10() { return std::errc::broken_pipe; }

std::errc f11() { return std::errc::connection_aborted; }

std::errc f12() { return std::errc::connection_already_in_progress; }

std::errc f13() { return std::errc::connection_refused; }

std::errc f14() { return std::errc::connection_reset; }

std::errc f15() { return std::errc::cross_device_link; }

std::errc f16() { return std::errc::destination_address_required; }

std::errc f17() { return std::errc::device_or_resource_busy; }

std::errc f18() { return std::errc::directory_not_empty; }

std::errc f19() { return std::errc::executable_format_error; }

std::errc f20() { return std::errc::file_exists; }

std::errc f21() { return std::errc::file_too_large; }

std::errc f22() { return std::errc::filename_too_long; }

std::errc f23() { return std::errc::function_not_supported; }

std::errc f24() { return std::errc::host_unreachable; }

std::errc f25() { return std::errc::identifier_removed; }

std::errc f26() { return std::errc::illegal_byte_sequence; }

std::errc f27() { return std::errc::inappropriate_io_control_operation; }

std::errc f28() { return std::errc::interrupted; }

std::errc f29() { return std::errc::invalid_argument; }

std::errc f30() { return std::errc::invalid_seek; }

std::errc f31() { return std::errc::io_error; }

std::errc f32() { return std::errc::is_a_directory; }

std::errc f33() { return std::errc::message_size; }

std::errc f34() { return std::errc::network_down; }

std::errc f35() { return std::errc::network_reset; }

std::errc f36() { return std::errc::network_unreachable; }

std::errc f37() { return std::errc::no_buffer_space; }

std::errc f38() { return std::errc::no_child_process; }

std::errc f39() { return std::errc::no_link; }

std::errc f40() { return std::errc::no_lock_available; }

std::errc f41() { return std::errc::no_message; }

std::errc f42() { return std::errc::no_protocol_option; }

std::errc f43() { return std::errc::no_space_on_device; }

std::errc f44() { return std::errc::no_such_device_or_address; }

std::errc f45() { return std::errc::no_such_device; }

std::errc f46() { return std::errc::no_such_file_or_directory; }

std::errc f47() { return std::errc::no_such_process; }

std::errc f48() { return std::errc::not_a_directory; }

std::errc f49() { return std::errc::not_a_socket; }

std::errc f50() { return std::errc::not_connected; }

std::errc f51() { return std::errc::not_enough_memory; }

std::errc f52() { return std::errc::not_supported; }

std::errc f53() { return std::errc::operation_canceled; }

std::errc f54() { return std::errc::operation_in_progress; }

std::errc f55() { return std::errc::operation_not_permitted; }

std::errc f56() { return std::errc::operation_not_supported; }

std::errc f57() { return std::errc::operation_would_block; }

std::errc f58() { return std::errc::owner_dead; }

std::errc f59() { return std::errc::permission_denied; }

std::errc f60() { return std::errc::protocol_error; }

std::errc f61() { return std::errc::protocol_not_supported; }

std::errc f62() { return std::errc::read_only_file_system; }

std::errc f63() { return std::errc::resource_deadlock_would_occur; }

std::errc f64() { return std::errc::resource_unavailable_try_again; }

std::errc f65() { return std::errc::result_out_of_range; }

std::errc f66() { return std::errc::state_not_recoverable; }

std::errc f67() { return std::errc::text_file_busy; }

std::errc f68() { return std::errc::timed_out; }

std::errc f69() { return std::errc::too_many_files_open_in_system; }

std::errc f70() { return std::errc::too_many_files_open; }

std::errc f71() { return std::errc::too_many_links; }

std::errc f72() { return std::errc::too_many_symbolic_link_levels; }

std::errc f73() { return std::errc::value_too_large; }

std::errc f74() { return std::errc::wrong_protocol_type; }
