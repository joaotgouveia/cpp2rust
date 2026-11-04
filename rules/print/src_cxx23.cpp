#include <format>
#include <print>
#include <string_view>
#include <utility>

template <typename... T1>
void f1(std::FILE *a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::print(a0, std::forward<std::format_string<T1...>>(a1),
                    std::forward<T1>(a2)...);
}

template <typename... T1> void f2(std::format_string<T1...> a0, T1 &&...a1) {
  return std::print(std::forward<std::format_string<T1...>>(a0),
                    std::forward<T1>(a1)...);
}

void f3() { return std::println(); }

void f4(std::FILE *a0) { return std::println(a0); }

template <typename... T1>
void f5(std::FILE *a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::println(a0, std::forward<std::format_string<T1...>>(a1),
                      std::forward<T1>(a2)...);
}

template <typename... T1> void f6(std::format_string<T1...> a0, T1 &&...a1) {
  return std::println(std::forward<std::format_string<T1...>>(a0),
                      std::forward<T1>(a1)...);
}
