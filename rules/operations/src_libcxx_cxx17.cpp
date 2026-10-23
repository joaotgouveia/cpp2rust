#include <exception>
#include <filesystem>
#include <system_error>
#include <utility>

std::filesystem::path f10(const std::filesystem::path &a0) {
  return std::filesystem::absolute(a0);
}

std::filesystem::path f11(const std::filesystem::path &a0,
                          std::error_code &a1) {
  return std::filesystem::absolute(a0, a1);
}

std::filesystem::path f12(const std::filesystem::path &a0) {
  return std::filesystem::canonical(a0);
}

std::filesystem::path f13(const std::filesystem::path &a0,
                          std::error_code &a1) {
  return std::filesystem::canonical(a0, a1);
}

void f14(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::copy(a0, a1);
}

void f15(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::filesystem::copy_options a2) {
  return std::filesystem::copy(a0, a1, a2);
}

void f16(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::filesystem::copy_options a2, std::error_code &a3) {
  return std::filesystem::copy(a0, a1, a2, a3);
}

void f17(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::copy(a0, a1, a2);
}

bool f18(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::copy_file(a0, a1);
}

bool f19(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::filesystem::copy_options a2) {
  return std::filesystem::copy_file(a0, a1, a2);
}

bool f20(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::filesystem::copy_options a2, std::error_code &a3) {
  return std::filesystem::copy_file(a0, a1, a2, a3);
}

bool f21(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::copy_file(a0, a1, a2);
}

void f22(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::copy_symlink(a0, a1);
}

void f23(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::copy_symlink(a0, a1, a2);
}

bool f24(const std::filesystem::path &a0) {
  return std::filesystem::create_directories(a0);
}

bool f25(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::create_directories(a0, a1);
}

bool f26(const std::filesystem::path &a0) {
  return std::filesystem::create_directory(a0);
}

bool f27(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::create_directory(a0, a1);
}

bool f28(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::create_directory(a0, a1);
}

bool f29(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::create_directory(a0, a1, a2);
}

void f30(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::create_directory_symlink(a0, a1);
}

void f31(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::create_directory_symlink(a0, a1, a2);
}

void f32(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::create_hard_link(a0, a1);
}

void f33(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::create_hard_link(a0, a1, a2);
}

void f34(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::create_symlink(a0, a1);
}

void f35(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::create_symlink(a0, a1, a2);
}

std::filesystem::path f36() { return std::filesystem::current_path(); }

std::filesystem::path f37(std::error_code &a0) {
  return std::filesystem::current_path(a0);
}

void f38(const std::filesystem::path &a0) {
  return std::filesystem::current_path(a0);
}

void f39(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::current_path(a0, a1);
}

bool f40(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::equivalent(a0, a1);
}

bool f41(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::equivalent(a0, a1, a2);
}

bool f42(std::filesystem::file_status a0) {
  return std::filesystem::exists(std::move(a0));
}

bool f43(const std::filesystem::path &a0) {
  return std::filesystem::exists(a0);
}

bool f44(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::exists(a0, a1);
}

std::uintmax_t f45(const std::filesystem::path &a0) {
  return std::filesystem::file_size(a0);
}

std::uintmax_t f46(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::file_size(a0, a1);
}

std::uintmax_t f47(const std::filesystem::path &a0) {
  return std::filesystem::hard_link_count(a0);
}

std::uintmax_t f48(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::hard_link_count(a0, a1);
}

bool f49(std::filesystem::file_status a0) {
  return std::filesystem::is_block_file(std::move(a0));
}

bool f50(const std::filesystem::path &a0) {
  return std::filesystem::is_block_file(a0);
}

bool f51(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_block_file(a0, a1);
}

bool f52(std::filesystem::file_status a0) {
  return std::filesystem::is_character_file(std::move(a0));
}

bool f53(const std::filesystem::path &a0) {
  return std::filesystem::is_character_file(a0);
}

bool f54(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_character_file(a0, a1);
}

bool f55(std::filesystem::file_status a0) {
  return std::filesystem::is_directory(std::move(a0));
}

bool f56(const std::filesystem::path &a0) {
  return std::filesystem::is_directory(a0);
}

bool f57(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_directory(a0, a1);
}

bool f58(const std::filesystem::path &a0) {
  return std::filesystem::is_empty(a0);
}

bool f59(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_empty(a0, a1);
}

bool f60(std::filesystem::file_status a0) {
  return std::filesystem::is_fifo(std::move(a0));
}

bool f61(const std::filesystem::path &a0) {
  return std::filesystem::is_fifo(a0);
}

bool f62(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_fifo(a0, a1);
}

bool f63(std::filesystem::file_status a0) {
  return std::filesystem::is_other(std::move(a0));
}

bool f64(const std::filesystem::path &a0) {
  return std::filesystem::is_other(a0);
}

bool f65(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_other(a0, a1);
}

bool f66(std::filesystem::file_status a0) {
  return std::filesystem::is_regular_file(std::move(a0));
}

bool f67(const std::filesystem::path &a0) {
  return std::filesystem::is_regular_file(a0);
}

bool f68(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_regular_file(a0, a1);
}

bool f69(std::filesystem::file_status a0) {
  return std::filesystem::is_socket(std::move(a0));
}

bool f70(const std::filesystem::path &a0) {
  return std::filesystem::is_socket(a0);
}

bool f71(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_socket(a0, a1);
}

bool f72(std::filesystem::file_status a0) {
  return std::filesystem::is_symlink(std::move(a0));
}

bool f73(const std::filesystem::path &a0) {
  return std::filesystem::is_symlink(a0);
}

bool f74(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::is_symlink(a0, a1);
}

std::filesystem::file_time_type f75(const std::filesystem::path &a0) {
  return std::filesystem::last_write_time(a0);
}

void f76(const std::filesystem::path &a0, std::filesystem::file_time_type a1) {
  return std::filesystem::last_write_time(a0, std::move(a1));
}

void f77(const std::filesystem::path &a0, std::filesystem::file_time_type a1,
         std::error_code &a2) {
  return std::filesystem::last_write_time(a0, std::move(a1), a2);
}

std::filesystem::file_time_type f78(const std::filesystem::path &a0,
                                    std::error_code &a1) {
  return std::filesystem::last_write_time(a0, a1);
}

void f79(const std::filesystem::path &a0, std::filesystem::perms a1,
         std::filesystem::perm_options a2) {
  return std::filesystem::permissions(a0, a1, a2);
}

void f80(const std::filesystem::path &a0, std::filesystem::perms a1,
         std::filesystem::perm_options a2, std::error_code &a3) {
  return std::filesystem::permissions(a0, a1, a2, a3);
}

void f81(const std::filesystem::path &a0, std::filesystem::perms a1,
         std::error_code &a2) {
  return std::filesystem::permissions(a0, a1, a2);
}

std::filesystem::path f82(const std::filesystem::path &a0,
                          std::error_code &a1) {
  return std::filesystem::proximate(a0, a1);
}

std::filesystem::path f83(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return std::filesystem::proximate(a0, a1);
}

std::filesystem::path f84(const std::filesystem::path &a0,
                          const std::filesystem::path &a1,
                          std::error_code &a2) {
  return std::filesystem::proximate(a0, a1, a2);
}

std::filesystem::path f85(const std::filesystem::path &a0) {
  return std::filesystem::read_symlink(a0);
}

std::filesystem::path f86(const std::filesystem::path &a0,
                          std::error_code &a1) {
  return std::filesystem::read_symlink(a0, a1);
}

std::filesystem::path f87(const std::filesystem::path &a0,
                          std::error_code &a1) {
  return std::filesystem::relative(a0, a1);
}

std::filesystem::path f88(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return std::filesystem::relative(a0, a1);
}

std::filesystem::path f89(const std::filesystem::path &a0,
                          const std::filesystem::path &a1,
                          std::error_code &a2) {
  return std::filesystem::relative(a0, a1, a2);
}

bool f90(const std::filesystem::path &a0) {
  return std::filesystem::remove(a0);
}

bool f91(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::remove(a0, a1);
}

std::uintmax_t f92(const std::filesystem::path &a0) {
  return std::filesystem::remove_all(a0);
}

std::uintmax_t f93(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::remove_all(a0, a1);
}

void f94(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return std::filesystem::rename(a0, a1);
}

void f95(const std::filesystem::path &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return std::filesystem::rename(a0, a1, a2);
}

void f96(const std::filesystem::path &a0, std::uintmax_t a1) {
  return std::filesystem::resize_file(a0, a1);
}

void f97(const std::filesystem::path &a0, std::uintmax_t a1,
         std::error_code &a2) {
  return std::filesystem::resize_file(a0, a1, a2);
}

std::filesystem::space_info f98(const std::filesystem::path &a0) {
  return std::filesystem::space(a0);
}

std::filesystem::space_info f99(const std::filesystem::path &a0,
                                std::error_code &a1) {
  return std::filesystem::space(a0, a1);
}

std::filesystem::file_status f100(const std::filesystem::path &a0) {
  return std::filesystem::status(a0);
}

std::filesystem::file_status f101(const std::filesystem::path &a0,
                                  std::error_code &a1) {
  return std::filesystem::status(a0, a1);
}

bool f102(std::filesystem::file_status a0) {
  return std::filesystem::status_known(std::move(a0));
}

std::filesystem::file_status f103(const std::filesystem::path &a0) {
  return std::filesystem::symlink_status(a0);
}

std::filesystem::file_status f104(const std::filesystem::path &a0,
                                  std::error_code &a1) {
  return std::filesystem::symlink_status(a0, a1);
}

std::filesystem::path f105() { return std::filesystem::temp_directory_path(); }

std::filesystem::path f106(std::error_code &a0) {
  return std::filesystem::temp_directory_path(a0);
}

std::filesystem::path f107(const std::filesystem::path &a0) {
  return std::filesystem::weakly_canonical(a0);
}

std::filesystem::path f108(const std::filesystem::path &a0,
                           std::error_code &a1) {
  return std::filesystem::weakly_canonical(a0, a1);
}
