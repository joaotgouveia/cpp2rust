#include <filesystem>
#include <ostream>
#include <system_error>
#include <utility>

std::filesystem::directory_iterator f1(std::filesystem::directory_iterator a0) {
  return std::filesystem::begin(std::move(a0));
}

std::filesystem::recursive_directory_iterator
f2(std::filesystem::recursive_directory_iterator a0) {
  return std::filesystem::begin(std::move(a0));
}

using t1 = std::filesystem::directory_entry;

std::filesystem::directory_entry f3() {
  return std::filesystem::directory_entry();
}

std::filesystem::directory_entry
f4(const std::filesystem::directory_entry &a0) {
  return std::filesystem::directory_entry(a0);
}

std::filesystem::directory_entry f5(std::filesystem::directory_entry &&a0) {
  return std::filesystem::directory_entry(std::move(a0));
}

std::filesystem::directory_entry f6(const std::filesystem::path &a0) {
  return std::filesystem::directory_entry(a0);
}

std::filesystem::directory_entry f7(const std::filesystem::path &a0,
                                    std::error_code &a1) {
  return std::filesystem::directory_entry(a0, a1);
}

std::filesystem::directory_entry &
f8(std::filesystem::directory_entry &a0,
   const std::filesystem::directory_entry &a1) {
  return a0.operator=(a1);
}

std::filesystem::directory_entry &f9(std::filesystem::directory_entry &a0,
                                     std::filesystem::directory_entry &&a1) {
  return a0.operator=(std::move(a1));
}

void f10(std::filesystem::directory_entry &a0,
         const std::filesystem::path &a1) {
  return a0.assign(a1);
}

void f11(std::filesystem::directory_entry &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return a0.assign(a1, a2);
}

void f12(std::filesystem::directory_entry &a0,
         const std::filesystem::path &a1) {
  return a0.replace_filename(a1);
}

void f13(std::filesystem::directory_entry &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return a0.replace_filename(a1, a2);
}

void f14(std::filesystem::directory_entry &a0) { return a0.refresh(); }

void f15(std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.refresh(a1);
}

const std::filesystem::path &f16(const std::filesystem::directory_entry &a0) {
  return a0.path();
}

const std::filesystem::path &f17(const std::filesystem::directory_entry &a0) {
  return static_cast<const std::filesystem::path &>(a0);
}

bool f18(const std::filesystem::directory_entry &a0) { return a0.exists(); }

bool f19(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.exists(a1);
}

bool f20(const std::filesystem::directory_entry &a0) {
  return a0.is_block_file();
}

bool f21(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_block_file(a1);
}

bool f22(const std::filesystem::directory_entry &a0) {
  return a0.is_character_file();
}

bool f23(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_character_file(a1);
}

bool f24(const std::filesystem::directory_entry &a0) {
  return a0.is_directory();
}

bool f25(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_directory(a1);
}

bool f26(const std::filesystem::directory_entry &a0) { return a0.is_fifo(); }

bool f27(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_fifo(a1);
}

bool f28(const std::filesystem::directory_entry &a0) { return a0.is_other(); }

bool f29(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_other(a1);
}

bool f30(const std::filesystem::directory_entry &a0) {
  return a0.is_regular_file();
}

bool f31(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_regular_file(a1);
}

bool f32(const std::filesystem::directory_entry &a0) { return a0.is_socket(); }

bool f33(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_socket(a1);
}

bool f34(const std::filesystem::directory_entry &a0) { return a0.is_symlink(); }

bool f35(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_symlink(a1);
}

std::uintmax_t f36(const std::filesystem::directory_entry &a0) {
  return a0.file_size();
}

std::uintmax_t f37(const std::filesystem::directory_entry &a0,
                   std::error_code &a1) {
  return a0.file_size(a1);
}

std::uintmax_t f38(const std::filesystem::directory_entry &a0) {
  return a0.hard_link_count();
}

std::uintmax_t f39(const std::filesystem::directory_entry &a0,
                   std::error_code &a1) {
  return a0.hard_link_count(a1);
}

std::filesystem::file_time_type
f40(const std::filesystem::directory_entry &a0) {
  return a0.last_write_time();
}

std::filesystem::file_time_type f41(const std::filesystem::directory_entry &a0,
                                    std::error_code &a1) {
  return a0.last_write_time(a1);
}

std::filesystem::file_status f42(const std::filesystem::directory_entry &a0) {
  return a0.status();
}

std::filesystem::file_status f43(const std::filesystem::directory_entry &a0,
                                 std::error_code &a1) {
  return a0.status(a1);
}

std::filesystem::file_status f44(const std::filesystem::directory_entry &a0) {
  return a0.symlink_status();
}

std::filesystem::file_status f45(const std::filesystem::directory_entry &a0,
                                 std::error_code &a1) {
  return a0.symlink_status(a1);
}

bool f46(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator==(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f47(std::basic_ostream<T1, T2> &a0,
                                const std::filesystem::directory_entry &a1) {
  return operator<<(a0, a1);
}

using t2 = std::filesystem::directory_iterator;

using t3 = std::filesystem::directory_iterator::value_type;

using t4 = std::filesystem::directory_iterator::difference_type;

using t5 = std::filesystem::directory_iterator::pointer;

using t6 = std::filesystem::directory_iterator::reference;

using t7 = std::filesystem::directory_iterator::iterator_category;

std::filesystem::directory_iterator f48() {
  return std::filesystem::directory_iterator();
}

std::filesystem::directory_iterator f49(const std::filesystem::path &a0) {
  return std::filesystem::directory_iterator(a0);
}

std::filesystem::directory_iterator f50(const std::filesystem::path &a0,
                                        std::filesystem::directory_options a1) {
  return std::filesystem::directory_iterator(a0, a1);
}

std::filesystem::directory_iterator f51(const std::filesystem::path &a0,
                                        std::error_code &a1) {
  return std::filesystem::directory_iterator(a0, a1);
}

std::filesystem::directory_iterator f52(const std::filesystem::path &a0,
                                        std::filesystem::directory_options a1,
                                        std::error_code &a2) {
  return std::filesystem::directory_iterator(a0, a1, a2);
}

std::filesystem::directory_iterator
f53(const std::filesystem::directory_iterator &a0) {
  return std::filesystem::directory_iterator(a0);
}

std::filesystem::directory_iterator
f54(std::filesystem::directory_iterator &&a0) {
  return std::filesystem::directory_iterator(std::move(a0));
}

std::filesystem::directory_iterator &
f55(std::filesystem::directory_iterator &a0,
    const std::filesystem::directory_iterator &a1) {
  return a0.operator=(a1);
}

std::filesystem::directory_iterator &
f56(std::filesystem::directory_iterator &a0,
    std::filesystem::directory_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

const std::filesystem::directory_entry &
f57(const std::filesystem::directory_iterator &a0) {
  return a0.operator*();
}

const std::filesystem::directory_entry *
f58(const std::filesystem::directory_iterator &a0) {
  return a0.operator->();
}

std::filesystem::directory_iterator &
f59(std::filesystem::directory_iterator &a0) {
  return a0.operator++();
}

std::filesystem::directory_iterator &
f60(std::filesystem::directory_iterator &a0, std::error_code &a1) {
  return a0.increment(a1);
}

bool f61(const std::filesystem::directory_iterator &a0,
         const std::filesystem::directory_iterator &a1) {
  return operator==(a0, a1);
}

std::filesystem::directory_iterator
f62(std::filesystem::directory_iterator a0) {
  return std::filesystem::end(std::move(a0));
}

std::filesystem::recursive_directory_iterator
f63(std::filesystem::recursive_directory_iterator a0) {
  return std::filesystem::end(std::move(a0));
}

using t8 = std::filesystem::file_status;

std::filesystem::file_status f64() { return std::filesystem::file_status(); }

std::filesystem::file_status f65(std::filesystem::file_type a0,
                                 std::filesystem::perms a1) {
  return std::filesystem::file_status(a0, a1);
}

std::filesystem::file_status f66(const std::filesystem::file_status &a0) {
  return std::filesystem::file_status(a0);
}

std::filesystem::file_status f67(std::filesystem::file_status &&a0) {
  return std::filesystem::file_status(std::move(a0));
}

std::filesystem::file_status &f68(std::filesystem::file_status &a0,
                                  const std::filesystem::file_status &a1) {
  return a0.operator=(a1);
}

std::filesystem::file_status &f69(std::filesystem::file_status &a0,
                                  std::filesystem::file_status &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::file_type f70(const std::filesystem::file_status &a0) {
  return a0.type();
}

std::filesystem::perms f71(const std::filesystem::file_status &a0) {
  return a0.permissions();
}

void f72(std::filesystem::file_status &a0, std::filesystem::file_type a1) {
  return a0.type(a1);
}

void f73(std::filesystem::file_status &a0, std::filesystem::perms a1) {
  return a0.permissions(a1);
}

using t9 = std::filesystem::recursive_directory_iterator;

using t10 = std::filesystem::recursive_directory_iterator::value_type;

using t11 = std::filesystem::recursive_directory_iterator::difference_type;

using t12 = std::filesystem::recursive_directory_iterator::pointer;

using t13 = std::filesystem::recursive_directory_iterator::reference;

using t14 = std::filesystem::recursive_directory_iterator::iterator_category;

std::filesystem::recursive_directory_iterator f74() {
  return std::filesystem::recursive_directory_iterator();
}

std::filesystem::recursive_directory_iterator
f75(const std::filesystem::path &a0) {
  return std::filesystem::recursive_directory_iterator(a0);
}

std::filesystem::recursive_directory_iterator
f76(const std::filesystem::path &a0, std::filesystem::directory_options a1) {
  return std::filesystem::recursive_directory_iterator(a0, a1);
}

std::filesystem::recursive_directory_iterator
f77(const std::filesystem::path &a0, std::filesystem::directory_options a1,
    std::error_code &a2) {
  return std::filesystem::recursive_directory_iterator(a0, a1, a2);
}

std::filesystem::recursive_directory_iterator
f78(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::recursive_directory_iterator(a0, a1);
}

std::filesystem::recursive_directory_iterator
f79(const std::filesystem::recursive_directory_iterator &a0) {
  return std::filesystem::recursive_directory_iterator(a0);
}

std::filesystem::recursive_directory_iterator
f80(std::filesystem::recursive_directory_iterator &&a0) {
  return std::filesystem::recursive_directory_iterator(std::move(a0));
}

std::filesystem::directory_options
f81(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.options();
}

int f82(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.depth();
}

bool f83(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.recursion_pending();
}

const std::filesystem::directory_entry &
f84(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.operator*();
}

const std::filesystem::directory_entry *
f85(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.operator->();
}

std::filesystem::recursive_directory_iterator &
f86(std::filesystem::recursive_directory_iterator &a0,
    const std::filesystem::recursive_directory_iterator &a1) {
  return a0.operator=(a1);
}

std::filesystem::recursive_directory_iterator &
f87(std::filesystem::recursive_directory_iterator &a0,
    std::filesystem::recursive_directory_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::recursive_directory_iterator &
f88(std::filesystem::recursive_directory_iterator &a0) {
  return a0.operator++();
}

std::filesystem::recursive_directory_iterator &
f89(std::filesystem::recursive_directory_iterator &a0, std::error_code &a1) {
  return a0.increment(a1);
}

void f90(std::filesystem::recursive_directory_iterator &a0) { return a0.pop(); }

void f91(std::filesystem::recursive_directory_iterator &a0,
         std::error_code &a1) {
  return a0.pop(a1);
}

void f92(std::filesystem::recursive_directory_iterator &a0) {
  return a0.disable_recursion_pending();
}

bool f93(const std::filesystem::recursive_directory_iterator &a0,
         const std::filesystem::recursive_directory_iterator &a1) {
  return operator==(a0, a1);
}
