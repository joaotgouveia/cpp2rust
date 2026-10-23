#include <algorithm>
#include <filesystem>
#include <ios>
#include <string>
#include <system_error>
#include <utility>

template <typename T1> void f1(std::filesystem::path::iterator &a0, T1 a1) {
  return std::advance<T1>(a0, a1);
}

std::ptrdiff_t f2(std::filesystem::path::iterator a0,
                  std::filesystem::path::iterator a1) {
  return std::distance(std::move(a0), std::move(a1));
}

using t1 = std::filesystem::filesystem_error;

std::filesystem::filesystem_error f3(const std::string &a0,
                                     std::error_code a1) {
  return std::filesystem::filesystem_error(a0, std::move(a1));
}

std::filesystem::filesystem_error
f4(const std::string &a0, const std::filesystem::path &a1, std::error_code a2) {
  return std::filesystem::filesystem_error(a0, a1, std::move(a2));
}

std::filesystem::filesystem_error f5(const std::string &a0,
                                     const std::filesystem::path &a1,
                                     const std::filesystem::path &a2,
                                     std::error_code a3) {
  return std::filesystem::filesystem_error(a0, a1, a2, std::move(a3));
}

std::filesystem::filesystem_error
f6(const std::filesystem::filesystem_error &a0) {
  return std::filesystem::filesystem_error(a0);
}

std::filesystem::filesystem_error &
f7(std::filesystem::filesystem_error &a0,
   const std::filesystem::filesystem_error &a1) {
  return a0.operator=(a1);
}

const std::filesystem::path &f8(const std::filesystem::filesystem_error &a0) {
  return a0.path1();
}

const std::filesystem::path &f9(const std::filesystem::filesystem_error &a0) {
  return a0.path2();
}

const char *f10(const std::filesystem::filesystem_error &a0) {
  return a0.what();
}

const std::error_code &f11(const std::filesystem::filesystem_error &a0) {
  return a0.code();
}

std::size_t f12(const std::filesystem::path &a0) {
  return std::filesystem::hash_value(a0);
}

using t2 = std::filesystem::path;

using t3 = std::filesystem::path::value_type;

using t4 = std::filesystem::path::string_type;

using t5 = std::filesystem::path::format;

using t6 = std::filesystem::path::iterator;

using t7 = std::filesystem::path::const_iterator;

std::filesystem::path f13() { return std::filesystem::path(); }

std::filesystem::path f14(const std::filesystem::path &a0) {
  return std::filesystem::path(a0);
}

std::filesystem::path f15(std::filesystem::path &&a0) {
  return std::filesystem::path(std::move(a0));
}

std::filesystem::path f16(std::filesystem::path::string_type &&a0,
                          std::filesystem::path::format a1) {
  return std::filesystem::path(std::move(a0), a1);
}

std::filesystem::path &f17(std::filesystem::path &a0,
                           const std::filesystem::path &a1) {
  return a0.operator=(a1);
}

std::filesystem::path &f18(std::filesystem::path &a0,
                           std::filesystem::path &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::path &f19(std::filesystem::path &a0,
                           std::filesystem::path::string_type &&a1) {
  return a0.operator=(std::move(a1));
}

std::filesystem::path &f20(std::filesystem::path &a0,
                           std::filesystem::path::string_type &&a1) {
  return a0.assign(std::move(a1));
}

std::filesystem::path &f21(std::filesystem::path &a0,
                           const std::filesystem::path &a1) {
  return a0.operator/=(a1);
}

std::filesystem::path &f22(std::filesystem::path &a0,
                           const std::filesystem::path &a1) {
  return a0.operator+=(a1);
}

std::filesystem::path &f23(std::filesystem::path &a0,
                           const std::filesystem::path::string_type &a1) {
  return a0.operator+=(a1);
}

std::filesystem::path &f24(std::filesystem::path &a0,
                           const std::filesystem::path::value_type *a1) {
  return a0.operator+=(a1);
}

std::filesystem::path &f25(std::filesystem::path &a0,
                           std::filesystem::path::value_type a1) {
  return a0.operator+=(a1);
}

std::filesystem::path &
f26(std::filesystem::path &a0,
    std::basic_string_view<std::filesystem::path::value_type> a1) {
  return a0.operator+=(std::move(a1));
}

void f27(std::filesystem::path &a0) { return a0.clear(); }

std::filesystem::path &f28(std::filesystem::path &a0) {
  return a0.make_preferred();
}

std::filesystem::path &f29(std::filesystem::path &a0) {
  return a0.remove_filename();
}

std::filesystem::path &f30(std::filesystem::path &a0,
                           const std::filesystem::path &a1) {
  return a0.replace_filename(a1);
}

std::filesystem::path &f31(std::filesystem::path &a0,
                           const std::filesystem::path &a1) {
  return a0.replace_extension(a1);
}

void f32(std::filesystem::path &a0, std::filesystem::path &a1) {
  return a0.swap(a1);
}

const std::filesystem::path::string_type &f33(const std::filesystem::path &a0) {
  return a0.native();
}

const std::filesystem::path::value_type *f34(const std::filesystem::path &a0) {
  return a0.c_str();
}

std::filesystem::path::string_type f35(const std::filesystem::path &a0) {
  return static_cast<std::filesystem::path::string_type>(a0);
}

std::string f36(const std::filesystem::path &a0) { return a0.string(); }

std::wstring f37(const std::filesystem::path &a0) { return a0.wstring(); }

std::u16string f38(const std::filesystem::path &a0) { return a0.u16string(); }

std::u32string f39(const std::filesystem::path &a0) { return a0.u32string(); }

std::string f40(const std::filesystem::path &a0) { return a0.generic_string(); }

std::wstring f41(const std::filesystem::path &a0) {
  return a0.generic_wstring();
}

std::u16string f42(const std::filesystem::path &a0) {
  return a0.generic_u16string();
}

std::u32string f43(const std::filesystem::path &a0) {
  return a0.generic_u32string();
}

int f44(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0.compare(a1);
}

int f45(const std::filesystem::path &a0,
        const std::filesystem::path::string_type &a1) {
  return a0.compare(a1);
}

int f46(const std::filesystem::path &a0,
        const std::filesystem::path::value_type *a1) {
  return a0.compare(a1);
}

int f47(const std::filesystem::path &a0,
        std::basic_string_view<std::filesystem::path::value_type> a1) {
  return a0.compare(std::move(a1));
}

std::filesystem::path f48(const std::filesystem::path &a0) {
  return a0.root_name();
}

std::filesystem::path f49(const std::filesystem::path &a0) {
  return a0.root_directory();
}

std::filesystem::path f50(const std::filesystem::path &a0) {
  return a0.root_path();
}

std::filesystem::path f51(const std::filesystem::path &a0) {
  return a0.relative_path();
}

std::filesystem::path f52(const std::filesystem::path &a0) {
  return a0.parent_path();
}

std::filesystem::path f53(const std::filesystem::path &a0) {
  return a0.filename();
}

std::filesystem::path f54(const std::filesystem::path &a0) { return a0.stem(); }

std::filesystem::path f55(const std::filesystem::path &a0) {
  return a0.extension();
}

bool f56(const std::filesystem::path &a0) { return a0.empty(); }

bool f57(const std::filesystem::path &a0) { return a0.has_root_name(); }

bool f58(const std::filesystem::path &a0) { return a0.has_root_directory(); }

bool f59(const std::filesystem::path &a0) { return a0.has_root_path(); }

bool f60(const std::filesystem::path &a0) { return a0.has_relative_path(); }

bool f61(const std::filesystem::path &a0) { return a0.has_parent_path(); }

bool f62(const std::filesystem::path &a0) { return a0.has_filename(); }

bool f63(const std::filesystem::path &a0) { return a0.has_stem(); }

bool f64(const std::filesystem::path &a0) { return a0.has_extension(); }

bool f65(const std::filesystem::path &a0) { return a0.is_absolute(); }

bool f66(const std::filesystem::path &a0) { return a0.is_relative(); }

std::filesystem::path f67(const std::filesystem::path &a0) {
  return a0.lexically_normal();
}

std::filesystem::path f68(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return a0.lexically_relative(a1);
}

std::filesystem::path f69(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return a0.lexically_proximate(a1);
}

std::filesystem::path::iterator f70(const std::filesystem::path &a0) {
  return a0.begin();
}

std::filesystem::path::iterator f71(const std::filesystem::path &a0) {
  return a0.end();
}

template <typename T1, typename T2 = std::filesystem::__detail::_Path<T1>>
std::filesystem::path f72(const T1 &a0, std::filesystem::path::format a1) {
  return std::filesystem::path(a0, a1);
}

template <typename T1, typename T2 = std::filesystem::__detail::_Path2<T1>>
std::filesystem::path f73(T1 a0, T1 a1, std::filesystem::path::format a2) {
  return std::filesystem::path(a0, a1, a2);
}

template <typename T1, typename T2 = std::filesystem::__detail::_Path<T1>,
          typename T3 = std::filesystem::__detail::__value_type_is_char<T1>>
std::filesystem::path f74(const T1 &a0, const std::locale &a1,
                          std::filesystem::path::format a2) {
  return std::filesystem::path(a0, a1, a2);
}

template <typename T1, typename T2 = std::filesystem::__detail::_Path2<T1>,
          typename T3 = std::filesystem::__detail::__value_type_is_char<T1>>
std::filesystem::path f75(T1 a0, T1 a1, const std::locale &a2,
                          std::filesystem::path::format a3) {
  return std::filesystem::path(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f76(const std::filesystem::path &a0,
                                  const T3 &a1) {
  return a0.template string<T1, T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f77(const std::filesystem::path &a0,
                                  const T3 &a1) {
  return a0.template generic_string<T1, T2, T3>(a1);
}

std::filesystem::path f78(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return operator/(a0, a1);
}

bool f79(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator==(a0, a1);
}

std::filesystem::path::value_type f80() {
  return std::filesystem::path::preferred_separator;
}

void f81(std::filesystem::path &a0, std::filesystem::path &a1) {
  return std::filesystem::swap(a0, a1);
}

template <typename T1, typename T2 = std::filesystem::__detail::_Path<T1>,
          typename T3 =
              std::filesystem::__detail::__value_type_is_char_or_char8_t<T1>>
std::filesystem::path f82(const T1 &a0) {
  return std::filesystem::u8path<T1>(a0);
}

template <typename T1, typename T2 = std::filesystem::__detail::_Path2<T1>,
          typename T3 =
              std::filesystem::__detail::__value_type_is_char_or_char8_t<T1>>
std::filesystem::path f83(T1 a0, T1 a1) {
  return std::filesystem::u8path<T1>(a0, a1);
}
