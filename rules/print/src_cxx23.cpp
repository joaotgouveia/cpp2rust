#include "../common/init.h"
#include "../common/traits.h"
#include <format>
#include <print>
#include <string_view>
#include <utility>

template <typename... T1>
void f1(std::FILE *a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::print<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1> void f2(std::format_string<T1...> a0, T1 &&...a1) {
  return std::print<T1...>(a0, std::forward<T1>(a1)...);
}

void f3() { return std::println(); }

void f4(std::FILE *a0) { return std::println(a0); }

template <typename... T1>
void f5(std::FILE *a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::println<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1> void f6(std::format_string<T1...> a0, T1 &&...a1) {
  return std::println<T1...>(a0, std::forward<T1>(a1)...);
}

void f7(std::string_view a0, std::format_args a1) {
  return std::vprint_nonunicode(std::move(a0), std::move(a1));
}

void f8(std::FILE *a0, std::string_view a1, std::format_args a2) {
  return std::vprint_nonunicode(a0, std::move(a1), std::move(a2));
}

void f9(std::string_view a0, std::format_args a1) {
  return std::vprint_unicode(std::move(a0), std::move(a1));
}

void f10(std::FILE *a0, std::string_view a1, std::format_args a2) {
  return std::vprint_unicode(a0, std::move(a1), std::move(a2));
}
