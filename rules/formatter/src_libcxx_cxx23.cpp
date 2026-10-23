#include <chrono>
#include <cstddef>
#include <format>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

void f216(std::formatter<char, char> &a0) { return a0.set_debug_format(); }

void f217(std::formatter<wchar_t, wchar_t> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
void f218(std::formatter<std::basic_string_view<T1, T2>, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>))
using t73 = std::formatter<T1, T2>;

template <typename T1, typename T2>
  requires(std::same_as<
              typename T1::__container,
              std::vector<bool, typename T1::__container::allocator_type>>)
using t74 = std::formatter<T1, T2>;

template <typename T1, typename T2, typename T3>
typename T3::iterator f219(std::formatter<T1, T2> &a0, T3 &a1)
  requires(
      std::same_as<typename T1::__container,
                   std::vector<bool, typename T1::__container::allocator_type>>)
{
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f220(const std::formatter<T1, T2> &a0, const T1 &a1,
                           T3 &a2)
  requires(
      std::same_as<typename T1::__container,
                   std::vector<bool, typename T1::__container::allocator_type>>)
{
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator f221(std::formatter<std::tuple<T2...>, T1> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator
f222(const std::formatter<std::tuple<T2...>, T1> &a0,
     std::conditional_t<(std::formattable<T2 const, T1> && ...),
                        const std::tuple<T2...> &, std::tuple<T2...> &>
         a1,
     T3 &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename... T2>
using t75 = std::formatter<std::pair<T2...>, T1>;

template <typename T1, typename... T2>
void f223(std::formatter<std::pair<T2...>, T1> &a0,
          std::basic_string_view<T1> a1) {
  return a0.set_separator(a1);
}

template <typename T1, typename... T2>
void f224(std::formatter<std::pair<T2...>, T1> &a0,
          std::basic_string_view<T1> a1, std::basic_string_view<T1> a2) {
  return a0.set_brackets(a1, a2);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator f225(std::formatter<std::pair<T2...>, T1> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator
f226(const std::formatter<std::pair<T2...>, T1> &a0,
     std::conditional_t<(std::formattable<T2 const, T1> && ...),
                        const std::pair<T2...> &, std::pair<T2...> &>
         a1,
     T3 &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator f227(std::formatter<std::queue<T2, T3>, T1> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator f228(std::formatter<std::stack<T2, T3>, T1> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}

template <typename T1, typename T2, typename T3>
void f229(std::formatter<std::basic_string<T1, T2, T3>, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename T5::iterator
f230(std::formatter<std::priority_queue<T2, T3, T4>, T1> &a0, T5 &a1) {
  return a0.template parse<T5>(a1);
}
