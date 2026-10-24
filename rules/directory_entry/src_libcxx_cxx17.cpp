#include <filesystem>
#include <ostream>
#include <system_error>
#include <utility>

using t1 = std::filesystem::__dir_element_proxy;

std::filesystem::directory_entry f1(std::filesystem::__dir_element_proxy &a0) {
  return a0.operator*();
}

using t2 = std::filesystem::directory_entry;

std::filesystem::directory_entry f2() {
  return std::filesystem::directory_entry();
}

std::filesystem::directory_entry
f3(const std::filesystem::directory_entry &a0) {
  return std::filesystem::directory_entry(a0);
}

std::filesystem::directory_entry f4(std::filesystem::directory_entry &&a0) {
  return std::filesystem::directory_entry(std::move(a0));
}

std::filesystem::directory_entry f5(const std::filesystem::path &a0) {
  return std::filesystem::directory_entry(a0);
}

std::filesystem::directory_entry f6(const std::filesystem::path &a0,
                                    std::error_code &a1) {
  return std::filesystem::directory_entry(a0, a1);
}

std::filesystem::directory_entry &
f7(std::filesystem::directory_entry &a0,
   const std::filesystem::directory_entry &a1) {
  return a0.operator=(a1);
}

std::filesystem::directory_entry &f8(std::filesystem::directory_entry &a0,
                                     std::filesystem::directory_entry &&a1) {
  return a0.operator=(std::move(a1));
}

void f9(std::filesystem::directory_entry &a0, const std::filesystem::path &a1) {
  return a0.assign(a1);
}

void f10(std::filesystem::directory_entry &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return a0.assign(a1, a2);
}

void f11(std::filesystem::directory_entry &a0,
         const std::filesystem::path &a1) {
  return a0.replace_filename(a1);
}

void f12(std::filesystem::directory_entry &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return a0.replace_filename(a1, a2);
}

void f13(std::filesystem::directory_entry &a0) { return a0.refresh(); }

void f14(std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.refresh(a1);
}

bool f15(const std::filesystem::directory_entry &a0) { return a0.exists(); }

bool f16(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.exists(a1);
}

bool f17(const std::filesystem::directory_entry &a0) {
  return a0.is_block_file();
}

bool f18(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_block_file(a1);
}

bool f19(const std::filesystem::directory_entry &a0) {
  return a0.is_character_file();
}

bool f20(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_character_file(a1);
}

bool f21(const std::filesystem::directory_entry &a0) {
  return a0.is_directory();
}

bool f22(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_directory(a1);
}

bool f23(const std::filesystem::directory_entry &a0) { return a0.is_fifo(); }

bool f24(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_fifo(a1);
}

bool f25(const std::filesystem::directory_entry &a0) { return a0.is_other(); }

bool f26(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_other(a1);
}

bool f27(const std::filesystem::directory_entry &a0) {
  return a0.is_regular_file();
}

bool f28(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_regular_file(a1);
}

bool f29(const std::filesystem::directory_entry &a0) { return a0.is_socket(); }

bool f30(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_socket(a1);
}

bool f31(const std::filesystem::directory_entry &a0) { return a0.is_symlink(); }

bool f32(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_symlink(a1);
}

std::uintmax_t f33(const std::filesystem::directory_entry &a0) {
  return a0.file_size();
}

std::uintmax_t f34(const std::filesystem::directory_entry &a0,
                   std::error_code &a1) {
  return a0.file_size(a1);
}

std::uintmax_t f35(const std::filesystem::directory_entry &a0) {
  return a0.hard_link_count();
}

std::uintmax_t f36(const std::filesystem::directory_entry &a0,
                   std::error_code &a1) {
  return a0.hard_link_count(a1);
}

std::filesystem::file_time_type
f37(const std::filesystem::directory_entry &a0) {
  return a0.last_write_time();
}

std::filesystem::file_time_type f38(const std::filesystem::directory_entry &a0,
                                    std::error_code &a1) {
  return a0.last_write_time(a1);
}

std::filesystem::file_status f39(const std::filesystem::directory_entry &a0) {
  return a0.status();
}

std::filesystem::file_status f40(const std::filesystem::directory_entry &a0,
                                 std::error_code &a1) {
  return a0.status(a1);
}

std::filesystem::file_status f41(const std::filesystem::directory_entry &a0) {
  return a0.symlink_status();
}

std::filesystem::file_status f42(const std::filesystem::directory_entry &a0,
                                 std::error_code &a1) {
  return a0.symlink_status(a1);
}

bool f43(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator==(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f44(std::basic_ostream<T1, T2> &a0,
                                const std::filesystem::directory_entry &a1) {
  return operator<<(a0, a1);
}
