#include "../common/init.h"
#include "../common/traits.h"
#include <format>
#include <iosfwd>
#include <ostream>
#include <print>
#include <string_view>
#include <utility>

template <typename... T1>
void f11(std::ostream &a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::print<T1...>(a0, a1, std::forward<T1>(a2)...);
}

void f12(std::ostream &a0) { return std::println(a0); }

template <typename... T1>
void f13(std::ostream &a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::println<T1...>(a0, a1, std::forward<T1>(a2)...);
}

void f14(std::ostream &a0, std::string_view a1, std::format_args a2) {
  return std::vprint_nonunicode(a0, std::move(a1), std::move(a2));
}

void f15(std::ostream &a0, std::string_view a1, std::format_args a2) {
  return std::vprint_unicode(a0, std::move(a1), std::move(a2));
}
