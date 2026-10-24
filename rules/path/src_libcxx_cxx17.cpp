#include <cstddef>
#include <filesystem>
#include <istream>
#include <ostream>
#include <string.h>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

std::size_t f1(const std::filesystem::path &a0) {
  return std::filesystem::hash_value(a0);
}

using t1 = std::filesystem::path;

using t2 = std::filesystem::path::value_type;

using t3 = std::filesystem::path::string_type;

using t4 = std::filesystem::path::format;

using t5 = std::filesystem::path::iterator;

using t6 = std::filesystem::path::const_iterator;

std::filesystem::path f2() { return std::filesystem::path(); }

std::filesystem::path f3(const std::filesystem::path &a0) {
  return std::filesystem::path(a0);
}

std::filesystem::path f4(std::filesystem::path &&a0) {
  return std::filesystem::path(std::move(a0));
}

std::filesystem::path f5(std::filesystem::path::string_type &&a0,
                         std::filesystem::path::format a1) {
  return std::filesystem::path(std::move(a0), a1);
}

std::filesystem::path &f6(std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return a0.operator=(a1);
}

std::filesystem::path &f7(std::filesystem::path &a0,
                          std::filesystem::path &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::path &f8(std::filesystem::path &a0,
                          std::filesystem::path::string_type &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::path &f9(std::filesystem::path &a0,
                          std::filesystem::path::string_type &&a1) {
  return a0.assign(std::move(a1));
}

std::filesystem::path &f10(std::filesystem::path &a0,
                           const std::filesystem::path &a1) {
  return a0.operator/=(a1);
}

std::filesystem::path &f11(std::filesystem::path &a0,
                           const std::filesystem::path &a1) {
  return a0.operator+=(a1);
}

std::filesystem::path &f12(std::filesystem::path &a0,
                           const std::filesystem::path::string_type &a1) {
  return a0.operator+=(a1);
}

std::filesystem::path &
f13(std::filesystem::path &a0,
    std::basic_string_view<std::filesystem::path::value_type> a1) {
  return a0.operator+=(std::move(a1));
}

std::filesystem::path &f14(std::filesystem::path &a0,
                           const std::filesystem::path::value_type *a1) {
  return a0.operator+=(a1);
}

std::filesystem::path &f15(std::filesystem::path &a0,
                           std::filesystem::path::value_type a1) {
  return a0.operator+=(a1);
}

void f16(std::filesystem::path &a0) { return a0.clear(); }

std::filesystem::path &f17(std::filesystem::path &a0) {
  return a0.make_preferred();
}

std::filesystem::path &f18(std::filesystem::path &a0) {
  return a0.remove_filename();
}

std::filesystem::path &f19(std::filesystem::path &a0,
                           const std::filesystem::path &a1) {
  return a0.replace_filename(a1);
}

std::filesystem::path &f20(std::filesystem::path &a0,
                           const std::filesystem::path &a1) {
  return a0.replace_extension(a1);
}

void f21(std::filesystem::path &a0, std::filesystem::path &a1) {
  return a0.swap(a1);
}

const std::filesystem::path::string_type &f22(const std::filesystem::path &a0) {
  return a0.native();
}

const std::filesystem::path::value_type *f23(const std::filesystem::path &a0) {
  return a0.c_str();
}

std::filesystem::path::string_type f24(const std::filesystem::path &a0) {
  return static_cast<std::filesystem::path::string_type>(a0);
}

std::string f25(const std::filesystem::path &a0) { return a0.string(); }

std::wstring f26(const std::filesystem::path &a0) { return a0.wstring(); }

std::u16string f27(const std::filesystem::path &a0) { return a0.u16string(); }

std::u32string f28(const std::filesystem::path &a0) { return a0.u32string(); }

std::string f29(const std::filesystem::path &a0) { return a0.generic_string(); }

std::wstring f30(const std::filesystem::path &a0) {
  return a0.generic_wstring();
}

std::u16string f31(const std::filesystem::path &a0) {
  return a0.generic_u16string();
}

std::u32string f32(const std::filesystem::path &a0) {
  return a0.generic_u32string();
}

int f33(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0.compare(a1);
}

int f34(const std::filesystem::path &a0,
        const std::filesystem::path::string_type &a1) {
  return a0.compare(a1);
}

int f35(const std::filesystem::path &a0,
        std::basic_string_view<std::filesystem::path::value_type> a1) {
  return a0.compare(std::move(a1));
}

int f36(const std::filesystem::path &a0,
        const std::filesystem::path::value_type *a1) {
  return a0.compare(a1);
}

std::filesystem::path f37(const std::filesystem::path &a0) {
  return a0.root_name();
}

std::filesystem::path f38(const std::filesystem::path &a0) {
  return a0.root_directory();
}

std::filesystem::path f39(const std::filesystem::path &a0) {
  return a0.root_path();
}

std::filesystem::path f40(const std::filesystem::path &a0) {
  return a0.relative_path();
}

std::filesystem::path f41(const std::filesystem::path &a0) {
  return a0.parent_path();
}

std::filesystem::path f42(const std::filesystem::path &a0) {
  return a0.filename();
}

std::filesystem::path f43(const std::filesystem::path &a0) { return a0.stem(); }

std::filesystem::path f44(const std::filesystem::path &a0) {
  return a0.extension();
}

bool f45(const std::filesystem::path &a0) { return a0.empty(); }

bool f46(const std::filesystem::path &a0) { return a0.has_root_name(); }

bool f47(const std::filesystem::path &a0) { return a0.has_root_directory(); }

bool f48(const std::filesystem::path &a0) { return a0.has_root_path(); }

bool f49(const std::filesystem::path &a0) { return a0.has_relative_path(); }

bool f50(const std::filesystem::path &a0) { return a0.has_parent_path(); }

bool f51(const std::filesystem::path &a0) { return a0.has_filename(); }

bool f52(const std::filesystem::path &a0) { return a0.has_stem(); }

bool f53(const std::filesystem::path &a0) { return a0.has_extension(); }

bool f54(const std::filesystem::path &a0) { return a0.is_absolute(); }

bool f55(const std::filesystem::path &a0) { return a0.is_relative(); }

std::filesystem::path f56(const std::filesystem::path &a0) {
  return a0.lexically_normal();
}

std::filesystem::path f57(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return a0.lexically_relative(a1);
}

std::filesystem::path f58(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return a0.lexically_proximate(a1);
}

std::filesystem::path::iterator f59(const std::filesystem::path &a0) {
  return a0.begin();
}

std::filesystem::path::iterator f60(const std::filesystem::path &a0) {
  return a0.end();
}

template <typename T1, typename T2 = std::__enable_if_t<
                           std::filesystem::__is_pathable<T1>::value, void>>
std::filesystem::path f61(const T1 &a0, std::filesystem::path::format a1) {
  return std::filesystem::path(a0, a1);
}

template <typename T1>
std::filesystem::path f62(T1 a0, T1 a1, std::filesystem::path::format a2) {
  return std::filesystem::path(a0, a1, a2);
}

template <typename T1>
std::__enable_if_t<std::filesystem::__is_pathable<T1>::value,
                   std::filesystem::path &>
f63(std::filesystem::path &a0, const T1 &a1) {
  return a0.template assign<T1>(a1);
}

template <typename T1>
std::filesystem::path &f64(std::filesystem::path &a0, T1 a1, T1 a2) {
  return a0.template assign<T1>(a1, a2);
}

template <typename T1>
std::__enable_if_t<std::filesystem::__is_pathable<T1>::value,
                   std::filesystem::path &>
f65(std::filesystem::path &a0, const T1 &a1) {
  return a0.template append<T1>(a1);
}

template <typename T1>
std::filesystem::path &f66(std::filesystem::path &a0, T1 a1, T1 a2) {
  return a0.template append<T1>(a1, a2);
}

template <typename T1>
std::__enable_if_t<std::filesystem::__is_pathable<T1>::value,
                   std::filesystem::path &>
f67(std::filesystem::path &a0, const T1 &a1) {
  return a0.template concat<T1>(a1);
}

template <typename T1>
std::filesystem::path &f68(std::filesystem::path &a0, T1 a1, T1 a2) {
  return a0.template concat<T1>(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f69(const std::filesystem::path &a0,
                                  const T3 &a1) {
  return a0.template string<T1, T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f70(const std::filesystem::path &a0,
                                  const T3 &a1) {
  return a0.template generic_string<T1, T2, T3>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f71(std::basic_istream<T1, T2> &a0,
                                std::filesystem::path &a1) {
  return operator>>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f72(std::basic_ostream<T1, T2> &a0,
                                const std::filesystem::path &a1) {
  return operator<<(a0, a1);
}

std::filesystem::path f73(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return operator/(a0, a1);
}

bool f74(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator==(a0, a1);
}

std::filesystem::path::value_type f75() {
  return std::filesystem::path::preferred_separator;
}

std::filesystem::path::format f76() {
  return std::filesystem::path::auto_format;
}

std::filesystem::path::format f77() {
  return std::filesystem::path::native_format;
}

std::filesystem::path::format f78() {
  return std::filesystem::path::generic_format;
}

void f79(std::filesystem::path &a0, std::filesystem::path &a1) {
  return std::filesystem::swap(a0, a1);
}
