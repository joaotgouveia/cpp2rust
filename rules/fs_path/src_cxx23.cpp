#include <algorithm>
#include <barrier>
#include <filesystem>
#include <ios>
#include <iosfwd>
#include <istream>
#include <string_view>
#include <system_error>
#include <utility>

template <typename T1> void f1(std::filesystem::path::iterator &a0, T1 a1) {
  return std::advance<T1>(a0, std::move(a1));
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

std::u8string f38(const std::filesystem::path &a0) { return a0.u8string(); }

std::u16string f39(const std::filesystem::path &a0) { return a0.u16string(); }

std::u32string f40(const std::filesystem::path &a0) { return a0.u32string(); }

std::string f41(const std::filesystem::path &a0) { return a0.generic_string(); }

std::wstring f42(const std::filesystem::path &a0) {
  return a0.generic_wstring();
}

std::u8string f43(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

std::u16string f44(const std::filesystem::path &a0) {
  return a0.generic_u16string();
}

std::u32string f45(const std::filesystem::path &a0) {
  return a0.generic_u32string();
}

int f46(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return a0.compare(a1);
}

int f47(const std::filesystem::path &a0,
        const std::filesystem::path::string_type &a1) {
  return a0.compare(a1);
}

int f48(const std::filesystem::path &a0,
        const std::filesystem::path::value_type *a1) {
  return a0.compare(a1);
}

int f49(const std::filesystem::path &a0,
        std::basic_string_view<std::filesystem::path::value_type> a1) {
  return a0.compare(std::move(a1));
}

std::filesystem::path f50(const std::filesystem::path &a0) {
  return a0.root_name();
}

std::filesystem::path f51(const std::filesystem::path &a0) {
  return a0.root_directory();
}

std::filesystem::path f52(const std::filesystem::path &a0) {
  return a0.root_path();
}

std::filesystem::path f53(const std::filesystem::path &a0) {
  return a0.relative_path();
}

std::filesystem::path f54(const std::filesystem::path &a0) {
  return a0.parent_path();
}

std::filesystem::path f55(const std::filesystem::path &a0) {
  return a0.filename();
}

std::filesystem::path f56(const std::filesystem::path &a0) { return a0.stem(); }

std::filesystem::path f57(const std::filesystem::path &a0) {
  return a0.extension();
}

bool f58(const std::filesystem::path &a0) { return a0.empty(); }

bool f59(const std::filesystem::path &a0) { return a0.has_root_name(); }

bool f60(const std::filesystem::path &a0) { return a0.has_root_directory(); }

bool f61(const std::filesystem::path &a0) { return a0.has_root_path(); }

bool f62(const std::filesystem::path &a0) { return a0.has_relative_path(); }

bool f63(const std::filesystem::path &a0) { return a0.has_parent_path(); }

bool f64(const std::filesystem::path &a0) { return a0.has_filename(); }

bool f65(const std::filesystem::path &a0) { return a0.has_stem(); }

bool f66(const std::filesystem::path &a0) { return a0.has_extension(); }

bool f67(const std::filesystem::path &a0) { return a0.is_absolute(); }

bool f68(const std::filesystem::path &a0) { return a0.is_relative(); }

std::filesystem::path f69(const std::filesystem::path &a0) {
  return a0.lexically_normal();
}

std::filesystem::path f70(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return a0.lexically_relative(a1);
}

std::filesystem::path f71(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return a0.lexically_proximate(a1);
}

std::filesystem::path::iterator f72(const std::filesystem::path &a0) {
  return a0.begin();
}

std::filesystem::path::iterator f73(const std::filesystem::path &a0) {
  return a0.end();
}

template <typename T1>
std::filesystem::path f74(const T1 &a0, std::filesystem::path::format a1) {
  return std::filesystem::path(a0, a1);
}

template <typename T1>
std::filesystem::path f75(T1 a0, T1 a1, std::filesystem::path::format a2) {
  return std::filesystem::path(std::move(a0), std::move(a1), a2);
}

template <typename T1>
std::filesystem::path f76(const T1 &a0, const std::locale &a1,
                          std::filesystem::path::format a2) {
  return std::filesystem::path(a0, a1, a2);
}

template <typename T1>
std::filesystem::path f77(T1 a0, T1 a1, const std::locale &a2,
                          std::filesystem::path::format a3) {
  return std::filesystem::path(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1>
std::filesystem::__detail::_Path<T1> &f78(std::filesystem::path &a0,
                                          const T1 &a1)
  requires(!(std::is_same<T1, std::filesystem::path>::value))
{
  return a0.template operator= <T1>(a1);
}

template <typename T1>
std::filesystem::__detail::_Path<T1> &f79(std::filesystem::path &a0,
                                          const T1 &a1) {
  return a0.template assign<T1>(a1);
}

template <typename T1>
std::filesystem::__detail::_Path2<T1> &f80(std::filesystem::path &a0, T1 a1,
                                           T1 a2) {
  return a0.template assign<T1>(std::move(a1), std::move(a2));
}

template <typename T1>
std::filesystem::__detail::_Path<T1> &f81(std::filesystem::path &a0,
                                          const T1 &a1)
  requires(!(std::is_same<T1, std::filesystem::path>::value))
{
  return a0.template operator/= <T1>(a1);
}

template <typename T1>
std::filesystem::__detail::_Path<T1> &f82(std::filesystem::path &a0,
                                          const T1 &a1) {
  return a0.template append<T1>(a1);
}

template <typename T1>
std::filesystem::__detail::_Path2<T1> &f83(std::filesystem::path &a0, T1 a1,
                                           T1 a2) {
  return a0.template append<T1>(std::move(a1), std::move(a2));
}

template <typename T1>
std::filesystem::__detail::_Path<T1> &f84(std::filesystem::path &a0,
                                          const T1 &a1)
  requires(!(std::is_same<T1, std::filesystem::path>::value) &&
           !(std::is_same<T1, std::filesystem::path::string_type>::value))
{
  return a0.template operator+= <T1>(a1);
}

template <typename T1>
std::filesystem::__detail::_Path2<T1 *> &f85(std::filesystem::path &a0, T1 a1)
  requires(
      !(std::is_same<T1, const std::filesystem::path &>::value) &&
      !(std::is_same<T1, const std::filesystem::path::string_type &>::value) &&
      !(std::is_same<T1, const std::filesystem::path::value_type *>::value) &&
      !(std::is_same<T1, std::filesystem::path::value_type>::value) &&
      !(std::is_same<T1, std::basic_string_view<
                             std::filesystem::path::value_type>>::value))
{
  return a0.template operator+= <T1>(std::move(a1));
}

template <typename T1>
std::filesystem::__detail::_Path<T1> &f86(std::filesystem::path &a0,
                                          const T1 &a1) {
  return a0.template concat<T1>(a1);
}

template <typename T1>
std::filesystem::__detail::_Path2<T1> &f87(std::filesystem::path &a0, T1 a1,
                                           T1 a2) {
  return a0.template concat<T1>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2 = std::char_traits<T1>,
          typename T3 = std::allocator<T1>>
std::basic_string<T1, T2, T3> f88(const std::filesystem::path &a0,
                                  const T3 &a1) {
  return a0.template string<T1, T2, T3>(a1);
}

template <typename T1, typename T2 = std::char_traits<T1>,
          typename T3 = std::allocator<T1>>
std::basic_string<T1, T2, T3> f89(const std::filesystem::path &a0,
                                  const T3 &a1) {
  return a0.template generic_string<T1, T2, T3>(a1);
}

std::filesystem::path f90(const std::filesystem::path &a0,
                          const std::filesystem::path &a1) {
  return operator/(a0, a1);
}

std::strong_ordering f91(const std::filesystem::path &a0,
                         const std::filesystem::path &a1) {
  return operator<=>(a0, a1);
}

bool f92(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f93(std::basic_istream<T1, T2> &a0,
                                std::filesystem::path &a1) {
  return operator>>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f94(std::basic_ostream<T1, T2> &a0,
                                const std::filesystem::path &a1) {
  return operator<<(a0, a1);
}

std::filesystem::path::value_type f95() {
  return std::filesystem::path::preferred_separator;
}

std::filesystem::path::format f96() {
  return std::filesystem::path::native_format;
}

std::filesystem::path::format f97() {
  return std::filesystem::path::generic_format;
}

std::filesystem::path::format f98() {
  return std::filesystem::path::auto_format;
}

void f99(std::filesystem::path &a0, std::filesystem::path &a1) {
  return std::filesystem::swap(a0, a1);
}

template <typename T1> std::filesystem::path f100(const T1 &a0) {
  return std::filesystem::u8path<T1>(a0);
}

template <typename T1> std::filesystem::path f101(T1 a0, T1 a1) {
  return std::filesystem::u8path<T1>(std::move(a0), std::move(a1));
}
