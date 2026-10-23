#include <format>
#include <iosfwd>
#include <ostream>
#include <print>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1>
void f11(std::ostream &a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::print<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename T1 = void> void f12(std::ostream &a0) {
  return std::println(a0);
}

template <typename T1 = void> void f13(std::FILE *a0) {
  return std::println(a0);
}

template <typename T1 = void> void f14() { return std::println(); }

template <typename... T1>
void f15(std::ostream &a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::println<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename T1 = void>
void f16(std::string_view a0, std::format_args a1) {
  return std::vprint_nonunicode(std::move(a0), std::move(a1));
}

template <typename T1 = void>
void f17(std::ostream &a0, std::string_view a1, std::format_args a2) {
  return std::vprint_nonunicode(a0, std::move(a1), std::move(a2));
}

template <typename T1 = void>
void f18(std::FILE *a0, std::string_view a1, std::format_args a2) {
  return std::vprint_nonunicode(a0, std::move(a1), std::move(a2));
}

template <typename T1 = void>
void f19(std::string_view a0, std::format_args a1) {
  return std::vprint_unicode(std::move(a0), std::move(a1));
}

template <typename T1 = void>
void f20(std::ostream &a0, std::string_view a1, std::format_args a2) {
  return std::vprint_unicode(a0, std::move(a1), std::move(a2));
}

template <typename T1 = void>
void f21(std::FILE *a0, std::string_view a1, std::format_args a2) {
  return std::vprint_unicode(a0, std::move(a1), std::move(a2));
}
