#include <chrono>
#include <cstddef>
#include <format>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

void f226(std::formatter<char, char> &a0) { return a0.set_debug_format(); }

void f227(std::formatter<wchar_t, wchar_t> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
void f228(std::formatter<std::basic_string_view<T1, T2>, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>))
using t76 = std::formatter<T1, T2>;

template <typename T1, typename T2>
void f229(std::formatter<T1, T2> &a0, std::basic_string_view<T2> a1)
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>)) &&
          (std::format_kind<T1> == (std::range_format)3)
{
  return a0.set_separator(std::move(a1));
}

template <typename T1, typename T2>
void f230(std::formatter<T1, T2> &a0, std::basic_string_view<T2> a1,
          std::basic_string_view<T2> a2)
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>)) &&
          (std::format_kind<T1> == (std::range_format)3)
{
  return a0.set_brackets(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f231(std::formatter<T1, T2> &a0, T3 &a1)
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>)) &&
          (std::format_kind<T1> == (std::range_format)3)
{
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f232(const std::formatter<T1, T2> &a0,
     typename std::conditional<std::__const_formattable_range<T1, T2>, T1 const,
                               T1>::type &a1,
     T3 &a2)
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>)) &&
          (std::format_kind<T1> == (std::range_format)3)
{
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f233(std::formatter<T1, T2> &a0, T3 &a1)
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>)) &&
          (std::format_kind<T1> == (std::range_format)1)
{
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f234(const std::formatter<T1, T2> &a0,
     typename std::conditional<std::__const_formattable_range<T1, T2>, T1 const,
                               T1>::type &a1,
     T3 &a2)
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>)) &&
          (std::format_kind<T1> == (std::range_format)1)
{
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f235(std::formatter<T1, T2> &a0, T3 &a1)
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>)) &&
          (std::format_kind<T1> == (std::range_format)2)
{
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator
f236(const std::formatter<T1, T2> &a0,
     typename std::conditional<std::__const_formattable_range<T1, T2>, T1 const,
                               T1>::type &a1,
     T3 &a2)
  requires((std::format_kind<T1> != std::range_format::disabled &&
            std::formattable<std::ranges::range_reference_t<T1>, T2>)) &&
          (std::format_kind<T1> == (std::range_format)2)
{
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
  requires(std::same_as<
              typename T1::__container,
              std::vector<bool, typename T1::__container::allocator_type>>)
using t77 = std::formatter<T1, T2>;

template <typename T1, typename T2, typename T3>
typename T3::iterator f237(std::formatter<T1, T2> &a0, T3 &a1)
  requires(
      std::same_as<typename T1::__container,
                   std::vector<bool, typename T1::__container::allocator_type>>)
{
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f238(const std::formatter<T1, T2> &a0, const T1 &a1,
                           T3 &a2)
  requires(
      std::same_as<typename T1::__container,
                   std::vector<bool, typename T1::__container::allocator_type>>)
{
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator f239(std::formatter<std::tuple<T2...>, T1> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator
f240(const std::formatter<std::tuple<T2...>, T1> &a0,
     std::conditional_t<(std::formattable<T2 const, T1> && ...),
                        const std::tuple<T2...> &, std::tuple<T2...> &>
         a1,
     T3 &a2) {
  return a0.template format<T3>(std::move(a1), a2);
}

template <typename T1, typename... T2>
using t78 = std::formatter<std::pair<T2...>, T1>;

template <typename T1, typename... T2>
void f241(std::formatter<std::pair<T2...>, T1> &a0,
          std::basic_string_view<T1> a1) {
  return a0.set_separator(std::move(a1));
}

template <typename T1, typename... T2>
void f242(std::formatter<std::pair<T2...>, T1> &a0,
          std::basic_string_view<T1> a1, std::basic_string_view<T1> a2) {
  return a0.set_brackets(std::move(a1), std::move(a2));
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator f243(std::formatter<std::pair<T2...>, T1> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename... T2, typename T3>
typename T3::iterator
f244(const std::formatter<std::pair<T2...>, T1> &a0,
     std::conditional_t<(std::formattable<T2 const, T1> && ...),
                        const std::pair<T2...> &, std::pair<T2...> &>
         a1,
     T3 &a2) {
  return a0.template format<T3>(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator f245(std::formatter<std::queue<T2, T3>, T1> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f246(const std::formatter<std::queue<T2, T3>, T1> &a0,
     typename std::conditional<
         std::is_const_v<std::__fmt_maybe_const<
             typename std::queue<T2, T3>::container_type, T1>>,
         std::queue<T2, T3> const, std::queue<T2, T3>>::type &a1,
     T4 &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator f247(std::formatter<std::stack<T2, T3>, T1> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f248(const std::formatter<std::stack<T2, T3>, T1> &a0,
     typename std::conditional<
         std::is_const_v<std::__fmt_maybe_const<
             typename std::stack<T2, T3>::container_type, T1>>,
         std::stack<T2, T3> const, std::stack<T2, T3>>::type &a1,
     T4 &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f249(std::formatter<std::basic_string<T1, T2, T3>, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename T5::iterator
f250(std::formatter<std::priority_queue<T2, T3, T4>, T1> &a0, T5 &a1) {
  return a0.template parse<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename T5::iterator
f251(const std::formatter<std::priority_queue<T2, T3, T4>, T1> &a0,
     typename std::conditional<
         std::is_const_v<std::__fmt_maybe_const<
             typename std::priority_queue<T2, T3, T4>::container_type, T1>>,
         std::priority_queue<T2, T3, T4> const,
         std::priority_queue<T2, T3, T4>>::type &a1,
     T5 &a2) {
  return a0.template format<T5>(a1, a2);
}
