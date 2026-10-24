#include <filesystem>
#include <ostream>
#include <system_error>
#include <utility>

using t1 = std::filesystem::__directory_iterator_proxy;

const std::filesystem::directory_entry &
f1(const std::filesystem::__directory_iterator_proxy &a0) {
  return a0.operator*();
}

std::filesystem::directory_entry
f2(std::filesystem::__directory_iterator_proxy &&a0) {
  return std::move(a0).operator*();
}

std::filesystem::__directory_iterator_proxy
f3(const std::filesystem::__directory_iterator_proxy &a0) {
  return std::filesystem::__directory_iterator_proxy(a0);
}

std::filesystem::__directory_iterator_proxy
f4(std::filesystem::__directory_iterator_proxy &&a0) {
  return std::filesystem::__directory_iterator_proxy(std::move(a0));
}

std::filesystem::__directory_iterator_proxy &
f5(std::filesystem::__directory_iterator_proxy &a0,
   const std::filesystem::__directory_iterator_proxy &a1) {
  return a0.operator=(a1);
}

std::filesystem::__directory_iterator_proxy &
f6(std::filesystem::__directory_iterator_proxy &a0,
   std::filesystem::__directory_iterator_proxy &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::directory_iterator f7(std::filesystem::directory_iterator a0) {
  return std::filesystem::begin(std::move(a0));
}

std::filesystem::recursive_directory_iterator
f8(std::filesystem::recursive_directory_iterator a0) {
  return std::filesystem::begin(std::move(a0));
}

using t2 = std::filesystem::directory_entry;

std::filesystem::directory_entry f9() {
  return std::filesystem::directory_entry();
}

std::filesystem::directory_entry
f10(const std::filesystem::directory_entry &a0) {
  return std::filesystem::directory_entry(a0);
}

std::filesystem::directory_entry f11(std::filesystem::directory_entry &&a0) {
  return std::filesystem::directory_entry(std::move(a0));
}

std::filesystem::directory_entry f12(const std::filesystem::path &a0) {
  return std::filesystem::directory_entry(a0);
}

std::filesystem::directory_entry f13(const std::filesystem::path &a0,
                                     std::error_code &a1) {
  return std::filesystem::directory_entry(a0, a1);
}

std::filesystem::directory_entry &
f14(std::filesystem::directory_entry &a0,
    const std::filesystem::directory_entry &a1) {
  return a0.operator=(a1);
}

std::filesystem::directory_entry &f15(std::filesystem::directory_entry &a0,
                                      std::filesystem::directory_entry &&a1) {
  return a0.operator=(std::move(a1));
}

void f16(std::filesystem::directory_entry &a0,
         const std::filesystem::path &a1) {
  return a0.assign(a1);
}

void f17(std::filesystem::directory_entry &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return a0.assign(a1, a2);
}

void f18(std::filesystem::directory_entry &a0,
         const std::filesystem::path &a1) {
  return a0.replace_filename(a1);
}

void f19(std::filesystem::directory_entry &a0, const std::filesystem::path &a1,
         std::error_code &a2) {
  return a0.replace_filename(a1, a2);
}

void f20(std::filesystem::directory_entry &a0) { return a0.refresh(); }

void f21(std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.refresh(a1);
}

const std::filesystem::path &f22(const std::filesystem::directory_entry &a0) {
  return a0.path();
}

const std::filesystem::path &f23(const std::filesystem::directory_entry &a0) {
  return static_cast<const std::filesystem::path &>(a0);
}

bool f24(const std::filesystem::directory_entry &a0) { return a0.exists(); }

bool f25(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.exists(a1);
}

bool f26(const std::filesystem::directory_entry &a0) {
  return a0.is_block_file();
}

bool f27(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_block_file(a1);
}

bool f28(const std::filesystem::directory_entry &a0) {
  return a0.is_character_file();
}

bool f29(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_character_file(a1);
}

bool f30(const std::filesystem::directory_entry &a0) {
  return a0.is_directory();
}

bool f31(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_directory(a1);
}

bool f32(const std::filesystem::directory_entry &a0) { return a0.is_fifo(); }

bool f33(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_fifo(a1);
}

bool f34(const std::filesystem::directory_entry &a0) { return a0.is_other(); }

bool f35(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_other(a1);
}

bool f36(const std::filesystem::directory_entry &a0) {
  return a0.is_regular_file();
}

bool f37(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_regular_file(a1);
}

bool f38(const std::filesystem::directory_entry &a0) { return a0.is_socket(); }

bool f39(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_socket(a1);
}

bool f40(const std::filesystem::directory_entry &a0) { return a0.is_symlink(); }

bool f41(const std::filesystem::directory_entry &a0, std::error_code &a1) {
  return a0.is_symlink(a1);
}

std::uintmax_t f42(const std::filesystem::directory_entry &a0) {
  return a0.file_size();
}

std::uintmax_t f43(const std::filesystem::directory_entry &a0,
                   std::error_code &a1) {
  return a0.file_size(a1);
}

std::uintmax_t f44(const std::filesystem::directory_entry &a0) {
  return a0.hard_link_count();
}

std::uintmax_t f45(const std::filesystem::directory_entry &a0,
                   std::error_code &a1) {
  return a0.hard_link_count(a1);
}

std::filesystem::file_time_type
f46(const std::filesystem::directory_entry &a0) {
  return a0.last_write_time();
}

std::filesystem::file_time_type f47(const std::filesystem::directory_entry &a0,
                                    std::error_code &a1) {
  return a0.last_write_time(a1);
}

std::filesystem::file_status f48(const std::filesystem::directory_entry &a0) {
  return a0.status();
}

std::filesystem::file_status f49(const std::filesystem::directory_entry &a0,
                                 std::error_code &a1) {
  return a0.status(a1);
}

std::filesystem::file_status f50(const std::filesystem::directory_entry &a0) {
  return a0.symlink_status();
}

std::filesystem::file_status f51(const std::filesystem::directory_entry &a0,
                                 std::error_code &a1) {
  return a0.symlink_status(a1);
}

bool f52(const std::filesystem::directory_entry &a0,
         const std::filesystem::directory_entry &a1) {
  return a0.operator==(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f53(std::basic_ostream<T1, T2> &a0,
                                const std::filesystem::directory_entry &a1) {
  return operator<<(a0, a1);
}

using t3 = std::filesystem::directory_iterator;

using t4 = std::filesystem::directory_iterator::value_type;

using t5 = std::filesystem::directory_iterator::difference_type;

using t6 = std::filesystem::directory_iterator::pointer;

using t7 = std::filesystem::directory_iterator::reference;

using t8 = std::filesystem::directory_iterator::iterator_category;

std::filesystem::directory_iterator f54() {
  return std::filesystem::directory_iterator();
}

std::filesystem::directory_iterator f55(const std::filesystem::path &a0) {
  return std::filesystem::directory_iterator(a0);
}

std::filesystem::directory_iterator f56(const std::filesystem::path &a0,
                                        std::filesystem::directory_options a1) {
  return std::filesystem::directory_iterator(a0, a1);
}

std::filesystem::directory_iterator f57(const std::filesystem::path &a0,
                                        std::error_code &a1) {
  return std::filesystem::directory_iterator(a0, a1);
}

std::filesystem::directory_iterator f58(const std::filesystem::path &a0,
                                        std::filesystem::directory_options a1,
                                        std::error_code &a2) {
  return std::filesystem::directory_iterator(a0, a1, a2);
}

std::filesystem::directory_iterator
f59(const std::filesystem::directory_iterator &a0) {
  return std::filesystem::directory_iterator(a0);
}

std::filesystem::directory_iterator
f60(std::filesystem::directory_iterator &&a0) {
  return std::filesystem::directory_iterator(std::move(a0));
}

std::filesystem::directory_iterator &
f61(std::filesystem::directory_iterator &a0,
    const std::filesystem::directory_iterator &a1) {
  return a0.operator=(a1);
}

std::filesystem::directory_iterator &
f62(std::filesystem::directory_iterator &a0,
    std::filesystem::directory_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

const std::filesystem::directory_entry &
f63(const std::filesystem::directory_iterator &a0) {
  return a0.operator*();
}

const std::filesystem::directory_entry *
f64(const std::filesystem::directory_iterator &a0) {
  return a0.operator->();
}

std::filesystem::directory_iterator &
f65(std::filesystem::directory_iterator &a0) {
  return a0.operator++();
}

std::filesystem::directory_iterator &
f66(std::filesystem::directory_iterator &a0, std::error_code &a1) {
  return a0.increment(a1);
}

std::filesystem::__directory_iterator_proxy
f67(std::filesystem::directory_iterator &a0, int a1) {
  return a0.operator++(a1);
}

bool f68(const std::filesystem::directory_iterator &a0,
         const std::filesystem::directory_iterator &a1) {
  return operator==(a0, a1);
}

std::filesystem::directory_iterator
f69(std::filesystem::directory_iterator a0) {
  return std::filesystem::end(std::move(a0));
}

std::filesystem::recursive_directory_iterator
f70(std::filesystem::recursive_directory_iterator a0) {
  return std::filesystem::end(std::move(a0));
}

using t9 = std::filesystem::file_status;

std::filesystem::file_status f71() { return std::filesystem::file_status(); }

std::filesystem::file_status f72(std::filesystem::file_type a0,
                                 std::filesystem::perms a1) {
  return std::filesystem::file_status(a0, a1);
}

std::filesystem::file_status f73(const std::filesystem::file_status &a0) {
  return std::filesystem::file_status(a0);
}

std::filesystem::file_status f74(std::filesystem::file_status &&a0) {
  return std::filesystem::file_status(std::move(a0));
}

std::filesystem::file_status &f75(std::filesystem::file_status &a0,
                                  const std::filesystem::file_status &a1) {
  return a0.operator=(a1);
}

std::filesystem::file_status &f76(std::filesystem::file_status &a0,
                                  std::filesystem::file_status &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::file_type f77(const std::filesystem::file_status &a0) {
  return a0.type();
}

std::filesystem::perms f78(const std::filesystem::file_status &a0) {
  return a0.permissions();
}

void f79(std::filesystem::file_status &a0, std::filesystem::file_type a1) {
  return a0.type(a1);
}

void f80(std::filesystem::file_status &a0, std::filesystem::perms a1) {
  return a0.permissions(a1);
}

using t10 = std::filesystem::recursive_directory_iterator;

using t11 = std::filesystem::recursive_directory_iterator::value_type;

using t12 = std::filesystem::recursive_directory_iterator::difference_type;

using t13 = std::filesystem::recursive_directory_iterator::pointer;

using t14 = std::filesystem::recursive_directory_iterator::reference;

using t15 = std::filesystem::recursive_directory_iterator::iterator_category;

std::filesystem::recursive_directory_iterator f81() {
  return std::filesystem::recursive_directory_iterator();
}

std::filesystem::recursive_directory_iterator
f82(const std::filesystem::path &a0) {
  return std::filesystem::recursive_directory_iterator(a0);
}

std::filesystem::recursive_directory_iterator
f83(const std::filesystem::path &a0, std::filesystem::directory_options a1) {
  return std::filesystem::recursive_directory_iterator(a0, a1);
}

std::filesystem::recursive_directory_iterator
f84(const std::filesystem::path &a0, std::filesystem::directory_options a1,
    std::error_code &a2) {
  return std::filesystem::recursive_directory_iterator(a0, a1, a2);
}

std::filesystem::recursive_directory_iterator
f85(const std::filesystem::path &a0, std::error_code &a1) {
  return std::filesystem::recursive_directory_iterator(a0, a1);
}

std::filesystem::recursive_directory_iterator
f86(const std::filesystem::recursive_directory_iterator &a0) {
  return std::filesystem::recursive_directory_iterator(a0);
}

std::filesystem::recursive_directory_iterator
f87(std::filesystem::recursive_directory_iterator &&a0) {
  return std::filesystem::recursive_directory_iterator(std::move(a0));
}

std::filesystem::directory_options
f88(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.options();
}

int f89(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.depth();
}

bool f90(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.recursion_pending();
}

const std::filesystem::directory_entry &
f91(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.operator*();
}

const std::filesystem::directory_entry *
f92(const std::filesystem::recursive_directory_iterator &a0) {
  return a0.operator->();
}

std::filesystem::recursive_directory_iterator &
f93(std::filesystem::recursive_directory_iterator &a0,
    const std::filesystem::recursive_directory_iterator &a1) {
  return a0.operator=(a1);
}

std::filesystem::recursive_directory_iterator &
f94(std::filesystem::recursive_directory_iterator &a0,
    std::filesystem::recursive_directory_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::recursive_directory_iterator &
f95(std::filesystem::recursive_directory_iterator &a0) {
  return a0.operator++();
}

std::filesystem::recursive_directory_iterator &
f96(std::filesystem::recursive_directory_iterator &a0, std::error_code &a1) {
  return a0.increment(a1);
}

std::filesystem::__directory_iterator_proxy
f97(std::filesystem::recursive_directory_iterator &a0, int a1) {
  return a0.operator++(a1);
}

void f98(std::filesystem::recursive_directory_iterator &a0) { return a0.pop(); }

void f99(std::filesystem::recursive_directory_iterator &a0,
         std::error_code &a1) {
  return a0.pop(a1);
}

void f100(std::filesystem::recursive_directory_iterator &a0) {
  return a0.disable_recursion_pending();
}

bool f101(const std::filesystem::recursive_directory_iterator &a0,
          const std::filesystem::recursive_directory_iterator &a1) {
  return operator==(a0, a1);
}
