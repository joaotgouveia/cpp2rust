#include <filesystem>
#include <system_error>
#include <utility>

std::filesystem::path f1(const std::filesystem::path &a0) {
  return std::filesystem::absolute(a0);
}

std::filesystem::path f2(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::absolute(a0, a1);
}

std::filesystem::path f3(const std::filesystem::path &a0) {
  return std::filesystem::canonical(a0);
}

std::filesystem::path f4(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::canonical(a0, a1);
}

void f5(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::copy(a0, a1);
}

void f6(const std::filesystem::path &a0, const std::filesystem::path &a1,
        std::error_code &a2) {
  return std::filesystem::copy(a0, a1, a2);
}

bool f7(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::copy_file(a0, a1);
}

bool f8(const std::filesystem::path &a0, const std::filesystem::path &a1,
        std::error_code &a2) {
  return std::filesystem::copy_file(a0, a1, a2);
}

void f9(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::copy_symlink(a0, a1);
}

void f10(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::copy_symlink(a0, a1, a2);
}

bool f11(const std::filesystem::path &a0) {
  return std::filesystem::create_directories(a0);
}

bool f12(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::create_directories(a0, a1);
}

bool f13(const std::filesystem::path &a0) {
  return std::filesystem::create_directory(a0);
}

bool f14(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::create_directory(a0, a1);
}

bool f15(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::create_directory(a0, a1);
}

bool f16(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::create_directory(a0, a1, a2);
}

void f17(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::create_directory_symlink(a0, a1);
}

void f18(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::create_directory_symlink(a0, a1, a2);
}

void f19(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::create_hard_link(a0, a1);
}

void f20(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::create_hard_link(a0, a1, a2);
}

void f21(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::create_symlink(a0, a1);
}

void f22(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::create_symlink(a0, a1, a2);
}

std::filesystem::path f23(std::error_code &a0) {
  return std::filesystem::current_path(a0);
}

void f24(const std::filesystem::path &a0) {
  return std::filesystem::current_path(a0);
}

void f25(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::current_path(a0, a1);
}

bool f26(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::equivalent(a0, a1);
}

bool f27(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::equivalent(a0, a1, a2);
}

bool f28(std::filesystem::file_status a0) {
  return std::filesystem::exists(std::move(a0));
}

bool f29(const std::filesystem::path &a0) {
  return std::filesystem::exists(a0);
}

bool f30(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::exists(a0, a1);
}

bool f31(std::filesystem::file_status a0) {
  return std::filesystem::is_block_file(std::move(a0));
}

bool f32(const std::filesystem::path &a0) {
  return std::filesystem::is_block_file(a0);
}

bool f33(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_block_file(a0, a1);
}

bool f34(std::filesystem::file_status a0) {
  return std::filesystem::is_character_file(std::move(a0));
}

bool f35(const std::filesystem::path &a0) {
  return std::filesystem::is_character_file(a0);
}

bool f36(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_character_file(a0, a1);
}

bool f37(std::filesystem::file_status a0) {
  return std::filesystem::is_directory(std::move(a0));
}

bool f38(const std::filesystem::path &a0) {
  return std::filesystem::is_directory(a0);
}

bool f39(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_directory(a0, a1);
}

bool f40(const std::filesystem::path &a0) {
  return std::filesystem::is_empty(a0);
}

bool f41(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_empty(a0, a1);
}

bool f42(std::filesystem::file_status a0) {
  return std::filesystem::is_fifo(std::move(a0));
}

bool f43(const std::filesystem::path &a0) {
  return std::filesystem::is_fifo(a0);
}

bool f44(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_fifo(a0, a1);
}

bool f45(std::filesystem::file_status a0) {
  return std::filesystem::is_other(std::move(a0));
}

bool f46(const std::filesystem::path &a0) {
  return std::filesystem::is_other(a0);
}

bool f47(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_other(a0, a1);
}

bool f48(std::filesystem::file_status a0) {
  return std::filesystem::is_regular_file(std::move(a0));
}

bool f49(const std::filesystem::path &a0) {
  return std::filesystem::is_regular_file(a0);
}

bool f50(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_regular_file(a0, a1);
}

bool f51(std::filesystem::file_status a0) {
  return std::filesystem::is_socket(std::move(a0));
}

bool f52(const std::filesystem::path &a0) {
  return std::filesystem::is_socket(a0);
}

bool f53(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_socket(a0, a1);
}

bool f54(std::filesystem::file_status a0) {
  return std::filesystem::is_symlink(std::move(a0));
}

bool f55(const std::filesystem::path &a0) {
  return std::filesystem::is_symlink(a0);
}

bool f56(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_symlink(a0, a1);
}

void f57(const std::filesystem::path &a0, std::filesystem::file_time_type a1) {
  return std::filesystem::last_write_time(a0, std::move(a1));
}

void f58(const std::filesystem::path &a0, std::filesystem::file_time_type a1,
         std::error_code &a2) {
  return std::filesystem::last_write_time(a0, std::move(a1), a2);
}

void f59(const std::filesystem::path &a0, std::filesystem::perms a1,
         std::filesystem::perm_options a2) {
  return std::filesystem::permissions(a0, a1, a2);
}

void f60(const std::filesystem::path &a0, std::filesystem::perms a1,
         std::error_code &a2) {
  return std::filesystem::permissions(a0, a1, a2);
}

std::filesystem::path f61(const std::filesystem::path &a0,
                          std::error_code &a1) {
  return std::filesystem::proximate(a0, a1);
}

std::filesystem::path f62(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return std::filesystem::proximate(a0, a1);
}

std::filesystem::path f63(const std::filesystem::path &a0) {
  return std::filesystem::read_symlink(a0);
}

std::filesystem::path f64(const std::filesystem::path &a0,
                          std::error_code &a1) {
  return std::filesystem::read_symlink(a0, a1);
}

std::filesystem::path f65(const std::filesystem::path &a0,
                          std::error_code &a1) {
  return std::filesystem::relative(a0, a1);
}

std::filesystem::path f66(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return std::filesystem::relative(a0, a1);
}

bool f67(const std::filesystem::path &a0) {
  return std::filesystem::remove(a0);
}

void f68(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::rename(a0, a1);
}

void f69(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::rename(a0, a1, a2);
}

void f70(const std::filesystem::path &a0, std::uintmax_t a1) {
  return std::filesystem::resize_file(a0, a1);
}

void f71(const std::filesystem::path &a0, std::uintmax_t a1,
         std::error_code &a2) {
  return std::filesystem::resize_file(a0, a1, a2);
}

std::filesystem::space_info f72(const std::filesystem::path &a0) {
  return std::filesystem::space(a0);
}

std::filesystem::space_info f73(const std::filesystem::path &a0,
                                std::error_code &a1) {
  return std::filesystem::space(a0, a1);
}

bool f74(std::filesystem::file_status a0) {
  return std::filesystem::status_known(std::move(a0));
}

std::filesystem::path f75() { return std::filesystem::temp_directory_path(); }

std::filesystem::path f76(std::error_code &a0) {
  return std::filesystem::temp_directory_path(a0);
}

std::filesystem::path f77(const std::filesystem::path &a0) {
  return std::filesystem::weakly_canonical(a0);
}

std::filesystem::path f78(const std::filesystem::path &a0,
                          std::error_code &a1) {
  return std::filesystem::weakly_canonical(a0, a1);
}
