#include <format>
#include <iosfwd>
#include <ostream>
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

template <typename... T1>
void f3(std::FILE *a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::println<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1> void f4(std::format_string<T1...> a0, T1 &&...a1) {
  return std::println<T1...>(a0, std::forward<T1>(a1)...);
}
