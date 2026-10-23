#include <filesystem>
#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

void f1(const std::filesystem::path &a0, const std::filesystem::path &a1,
        std::filesystem::copy_options a2) {
  return std::filesystem::copy(a0, a1, a2);
}

void f2(const std::filesystem::path &a0, const std::filesystem::path &a1,
        std::filesystem::copy_options a2, std::error_code &a3) {
  return std::filesystem::copy(a0, a1, a2, a3);
}

bool f3(const std::filesystem::path &a0, const std::filesystem::path &a1,
        std::filesystem::copy_options a2) {
  return std::filesystem::copy_file(a0, a1, a2);
}

bool f4(const std::filesystem::path &a0, const std::filesystem::path &a1,
        std::filesystem::copy_options a2, std::error_code &a3) {
  return std::filesystem::copy_file(a0, a1, a2, a3);
}

using t1 = std::filesystem::copy_options;

std::filesystem::copy_options f5() {
  return std::filesystem::copy_options::none;
}

std::filesystem::copy_options f6() {
  return std::filesystem::copy_options::skip_existing;
}

std::filesystem::copy_options f7() {
  return std::filesystem::copy_options::overwrite_existing;
}

std::filesystem::copy_options f8() {
  return std::filesystem::copy_options::update_existing;
}

std::filesystem::copy_options f9() {
  return std::filesystem::copy_options::recursive;
}

std::filesystem::copy_options f10() {
  return std::filesystem::copy_options::copy_symlinks;
}

std::filesystem::copy_options f11() {
  return std::filesystem::copy_options::skip_symlinks;
}

std::filesystem::copy_options f12() {
  return std::filesystem::copy_options::directories_only;
}

std::filesystem::copy_options f13() {
  return std::filesystem::copy_options::create_symlinks;
}

std::filesystem::copy_options f14() {
  return std::filesystem::copy_options::create_hard_links;
}

std::filesystem::path f15() { return std::filesystem::current_path(); }

using t2 = std::filesystem::directory_options;

std::filesystem::directory_options f16() {
  return std::filesystem::directory_options::none;
}

std::filesystem::directory_options f17() {
  return std::filesystem::directory_options::follow_directory_symlink;
}

std::filesystem::directory_options f18() {
  return std::filesystem::directory_options::skip_permission_denied;
}

std::uintmax_t f19(const std::filesystem::path &a0) {
  return std::filesystem::file_size(a0);
}

std::uintmax_t f20(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::file_size(a0, a1);
}

using t3 = std::filesystem::file_time_type;

using t4 = std::filesystem::file_type;

std::filesystem::file_type f21() { return std::filesystem::file_type::none; }

std::filesystem::file_type f22() {
  return std::filesystem::file_type::not_found;
}

std::filesystem::file_type f23() { return std::filesystem::file_type::regular; }

std::filesystem::file_type f24() {
  return std::filesystem::file_type::directory;
}

std::filesystem::file_type f25() { return std::filesystem::file_type::symlink; }

std::filesystem::file_type f26() { return std::filesystem::file_type::block; }

std::filesystem::file_type f27() {
  return std::filesystem::file_type::character;
}

std::filesystem::file_type f28() { return std::filesystem::file_type::fifo; }

std::filesystem::file_type f29() { return std::filesystem::file_type::socket; }

std::filesystem::file_type f30() { return std::filesystem::file_type::unknown; }

std::uintmax_t f31(const std::filesystem::path &a0) {
  return std::filesystem::hard_link_count(a0);
}

std::uintmax_t f32(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::hard_link_count(a0, a1);
}

std::filesystem::file_time_type f33(const std::filesystem::path &a0) {
  return std::filesystem::last_write_time(a0);
}

std::filesystem::file_time_type f34(const std::filesystem::path &a0,
                                    std::error_code &a1) {
  return std::filesystem::last_write_time(a0, a1);
}

std::filesystem::copy_options f35(std::filesystem::copy_options a0,
                                  std::filesystem::copy_options a1) {
  return a0 & a1;
}

std::filesystem::directory_options f36(std::filesystem::directory_options a0,
                                       std::filesystem::directory_options a1) {
  return a0 & a1;
}

std::filesystem::perm_options f37(std::filesystem::perm_options a0,
                                  std::filesystem::perm_options a1) {
  return a0 & a1;
}

std::filesystem::perms f38(std::filesystem::perms a0,
                           std::filesystem::perms a1) {
  return a0 & a1;
}

std::filesystem::copy_options &f39(std::filesystem::copy_options &a0,
                                   std::filesystem::copy_options a1) {
  return std::filesystem::operator&=(a0, a1);
}

std::filesystem::directory_options &f40(std::filesystem::directory_options &a0,
                                        std::filesystem::directory_options a1) {
  return std::filesystem::operator&=(a0, a1);
}

std::filesystem::perm_options &f41(std::filesystem::perm_options &a0,
                                   std::filesystem::perm_options a1) {
  return std::filesystem::operator&=(a0, a1);
}

std::filesystem::perms &f42(std::filesystem::perms &a0,
                            std::filesystem::perms a1) {
  return std::filesystem::operator&=(a0, a1);
}

std::filesystem::copy_options f43(std::filesystem::copy_options a0,
                                  std::filesystem::copy_options a1) {
  return a0 ^ a1;
}

std::filesystem::directory_options f44(std::filesystem::directory_options a0,
                                       std::filesystem::directory_options a1) {
  return a0 ^ a1;
}

std::filesystem::perm_options f45(std::filesystem::perm_options a0,
                                  std::filesystem::perm_options a1) {
  return a0 ^ a1;
}

std::filesystem::perms f46(std::filesystem::perms a0,
                           std::filesystem::perms a1) {
  return a0 ^ a1;
}

std::filesystem::copy_options &f47(std::filesystem::copy_options &a0,
                                   std::filesystem::copy_options a1) {
  return std::filesystem::operator^=(a0, a1);
}

std::filesystem::directory_options &f48(std::filesystem::directory_options &a0,
                                        std::filesystem::directory_options a1) {
  return std::filesystem::operator^=(a0, a1);
}

std::filesystem::perm_options &f49(std::filesystem::perm_options &a0,
                                   std::filesystem::perm_options a1) {
  return std::filesystem::operator^=(a0, a1);
}

std::filesystem::perms &f50(std::filesystem::perms &a0,
                            std::filesystem::perms a1) {
  return std::filesystem::operator^=(a0, a1);
}

std::filesystem::copy_options f51(std::filesystem::copy_options a0,
                                  std::filesystem::copy_options a1) {
  return a0 | a1;
}

std::filesystem::directory_options f52(std::filesystem::directory_options a0,
                                       std::filesystem::directory_options a1) {
  return a0 | a1;
}

std::filesystem::perm_options f53(std::filesystem::perm_options a0,
                                  std::filesystem::perm_options a1) {
  return a0 | a1;
}

std::filesystem::perms f54(std::filesystem::perms a0,
                           std::filesystem::perms a1) {
  return a0 | a1;
}

std::filesystem::copy_options &f55(std::filesystem::copy_options &a0,
                                   std::filesystem::copy_options a1) {
  return std::filesystem::operator|=(a0, a1);
}

std::filesystem::directory_options &f56(std::filesystem::directory_options &a0,
                                        std::filesystem::directory_options a1) {
  return std::filesystem::operator|=(a0, a1);
}

std::filesystem::perm_options &f57(std::filesystem::perm_options &a0,
                                   std::filesystem::perm_options a1) {
  return std::filesystem::operator|=(a0, a1);
}

std::filesystem::perms &f58(std::filesystem::perms &a0,
                            std::filesystem::perms a1) {
  return std::filesystem::operator|=(a0, a1);
}

std::filesystem::copy_options f59(std::filesystem::copy_options a0) {
  return ~a0;
}

std::filesystem::directory_options f60(std::filesystem::directory_options a0) {
  return ~a0;
}

std::filesystem::perm_options f61(std::filesystem::perm_options a0) {
  return ~a0;
}

std::filesystem::perms f62(std::filesystem::perms a0) { return ~a0; }

using t5 = std::filesystem::perm_options;

std::filesystem::perm_options f63() {
  return std::filesystem::perm_options::replace;
}

std::filesystem::perm_options f64() {
  return std::filesystem::perm_options::add;
}

std::filesystem::perm_options f65() {
  return std::filesystem::perm_options::remove;
}

std::filesystem::perm_options f66() {
  return std::filesystem::perm_options::nofollow;
}

void f67(const std::filesystem::path &a0, std::filesystem::perms a1,
         std::filesystem::perm_options a2, std::error_code &a3) {
  return std::filesystem::permissions(a0, a1, a2, a3);
}

using t6 = std::filesystem::perms;

std::filesystem::perms f68() { return std::filesystem::perms::none; }

std::filesystem::perms f69() { return std::filesystem::perms::owner_read; }

std::filesystem::perms f70() { return std::filesystem::perms::owner_write; }

std::filesystem::perms f71() { return std::filesystem::perms::owner_exec; }

std::filesystem::perms f72() { return std::filesystem::perms::owner_all; }

std::filesystem::perms f73() { return std::filesystem::perms::group_read; }

std::filesystem::perms f74() { return std::filesystem::perms::group_write; }

std::filesystem::perms f75() { return std::filesystem::perms::group_exec; }

std::filesystem::perms f76() { return std::filesystem::perms::group_all; }

std::filesystem::perms f77() { return std::filesystem::perms::others_read; }

std::filesystem::perms f78() { return std::filesystem::perms::others_write; }

std::filesystem::perms f79() { return std::filesystem::perms::others_exec; }

std::filesystem::perms f80() { return std::filesystem::perms::others_all; }

std::filesystem::perms f81() { return std::filesystem::perms::all; }

std::filesystem::perms f82() { return std::filesystem::perms::set_uid; }

std::filesystem::perms f83() { return std::filesystem::perms::set_gid; }

std::filesystem::perms f84() { return std::filesystem::perms::sticky_bit; }

std::filesystem::perms f85() { return std::filesystem::perms::mask; }

std::filesystem::perms f86() { return std::filesystem::perms::unknown; }

std::filesystem::path f87(const std::filesystem::path &a0,
                          const std::filesystem::path &a1,
                          std::error_code &a2) {
  return std::filesystem::proximate(a0, a1, a2);
}

std::filesystem::path f88(const std::filesystem::path &a0,
                          const std::filesystem::path &a1,
                          std::error_code &a2) {
  return std::filesystem::relative(a0, a1, a2);
}

bool f89(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::remove(a0, a1);
}

std::uintmax_t f90(const std::filesystem::path &a0) {
  return std::filesystem::remove_all(a0);
}

std::uintmax_t f91(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::remove_all(a0, a1);
}

using t7 = std::filesystem::space_info;

std::filesystem::space_info f92() { return std::filesystem::space_info(); }

std::filesystem::space_info f93(const std::filesystem::space_info &a0) {
  return std::filesystem::space_info(a0);
}

std::filesystem::space_info f94(std::filesystem::space_info &&a0) {
  return std::filesystem::space_info(std::move(a0));
}

std::filesystem::space_info &f95(std::filesystem::space_info &a0,
                                 const std::filesystem::space_info &a1) {
  return a0.operator=(a1);
}

std::filesystem::space_info &f96(std::filesystem::space_info &a0,
                                 std::filesystem::space_info &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::file_status f97(const std::filesystem::path &a0) {
  return std::filesystem::status(a0);
}

std::filesystem::file_status f98(const std::filesystem::path &a0,
                                 std::error_code &a1) {
  return std::filesystem::status(a0, a1);
}

std::filesystem::file_status f99(const std::filesystem::path &a0) {
  return std::filesystem::symlink_status(a0);
}

std::filesystem::file_status f100(const std::filesystem::path &a0,
                                  std::error_code &a1) {
  return std::filesystem::symlink_status(a0, a1);
}
