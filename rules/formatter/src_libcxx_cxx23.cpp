#include <chrono>
#include <cstddef>
#include <format>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

void f209(std::formatter<char, char> &a0) { return a0.set_debug_format(); }

void f210(std::formatter<wchar_t, wchar_t> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
void f211(std::formatter<std::basic_string_view<T1, T2>, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator f212(std::formatter<std::tuple<T2...>, T1> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator
f213(const std::formatter<std::tuple<T2...>, T1> &a0,
     std::conditional_t<(std::formattable<T2 const, T1> && ...),
                        const std::tuple<T2...> &, std::tuple<T2...> &>
         a1,
     T3 &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename... T2>
using t70 = std::formatter<std::pair<T2...>, T1>;

template <typename T1, typename... T2>
void f214(std::formatter<std::pair<T2...>, T1> &a0,
          std::basic_string_view<T1> a1) {
  return a0.set_separator(a1);
}

template <typename T1, typename... T2>
void f215(std::formatter<std::pair<T2...>, T1> &a0,
          std::basic_string_view<T1> a1, std::basic_string_view<T1> a2) {
  return a0.set_brackets(a1, a2);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator f216(std::formatter<std::pair<T2...>, T1> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator
f217(const std::formatter<std::pair<T2...>, T1> &a0,
     std::conditional_t<(std::formattable<T2 const, T1> && ...),
                        const std::pair<T2...> &, std::pair<T2...> &>
         a1,
     T3 &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator f218(std::formatter<std::queue<T2, T3>, T1> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator f219(std::formatter<std::stack<T2, T3>, T1> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}

template <typename T1, typename T2, typename T3>
void f220(std::formatter<std::basic_string<T1, T2, T3>, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename T5::iterator
f221(std::formatter<std::priority_queue<T2, T3, T4>, T1> &a0, T5 &a1) {
  return a0.template parse<T5>(a1);
}
