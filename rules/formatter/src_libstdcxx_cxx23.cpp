#include <algorithm>
#include <barrier>
#include <chrono>
#include <format>
#include <queue>
#include <stack>
#include <stacktrace>
#include <string_view>
#include <thread>
#include <utility>

using t43 = std::formatter<std::stacktrace_entry>;

const std::basic_string_view<char>::value_type *
f97(std::formatter<std::stacktrace_entry> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

std::formatter<std::stacktrace_entry> f98() {
  return std::formatter<std::stacktrace_entry>();
}

std::formatter<std::stacktrace_entry>
f99(const std::formatter<std::stacktrace_entry> &a0) {
  return std::formatter<std::stacktrace_entry>(a0);
}

std::formatter<std::stacktrace_entry>
f100(std::formatter<std::stacktrace_entry> &&a0) {
  return std::formatter<std::stacktrace_entry>(std::move(a0));
}

std::formatter<std::stacktrace_entry> &
f101(std::formatter<std::stacktrace_entry> &a0,
     const std::formatter<std::stacktrace_entry> &a1) {
  return a0.operator=(a1);
}

std::formatter<std::stacktrace_entry> &
f102(std::formatter<std::stacktrace_entry> &a0,
     std::formatter<std::stacktrace_entry> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::basic_format_context<T1, char>::iterator
f103(const std::formatter<std::stacktrace_entry> &a0,
     const std::stacktrace_entry &a1, std::basic_format_context<T1, char> &a2) {
  return a0.template format<T1>(a1, a2);
}

using t44 = std::formatter<const char *, wchar_t>;

using t45 = std::formatter<char *, wchar_t>;

template <typename T1>
  requires(std::is_pointer_v<std::thread::native_handle_type> ||
           std::is_integral_v<std::thread::native_handle_type>)
using t46 = std::formatter<std::thread::id, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f104(std::formatter<std::thread::id, T1> &a0,
     std::basic_format_parse_context<T1> &a1)
  requires(std::is_pointer_v<std::thread::native_handle_type> ||
           std::is_integral_v<std::thread::native_handle_type>)
{
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f105(const std::formatter<std::thread::id, T1> &a0, std::thread::id a1,
     std::basic_format_context<T2, T1> &a2)
  requires(std::is_pointer_v<std::thread::native_handle_type> ||
           std::is_integral_v<std::thread::native_handle_type>)
{
  return a0.template format<T2>(std::move(a1), a2);
}

template <typename T1> void f106(std::formatter<T1 *, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1> using t47 = std::formatter<std::basic_stacktrace<T1>>;

template <typename T1>
const std::basic_string_view<char>::value_type *
f107(std::formatter<std::basic_stacktrace<T1>> &a0,
     std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, char>::iterator
f108(const std::formatter<std::basic_stacktrace<T1>> &a0,
     const std::basic_stacktrace<T1> &a1,
     std::basic_format_context<T2, char> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
void f109(std::formatter<std::basic_string_view<char, T1>, char> &a0) {
  return a0.set_debug_format();
}

template <typename T1>
using t48 = std::formatter<std::basic_string_view<char, T1>, wchar_t>;

template <typename T1>
void f110(std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0) {
  return a0.set_debug_format();
}

template <typename T1> void f111(std::formatter<T1, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
void f112(std::formatter<std::basic_string<char, T1, T2>, char> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
using t49 = std::formatter<std::basic_string<char, T1, T2>, wchar_t>;

template <typename T1, typename T2>
void f113(std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
  requires((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
using t50 = std::formatter<T1, T2>;

template <typename T1, typename T2>
std::formatter<T1, T2> f114()
  requires((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
{
  return std::formatter<T1, T2>();
}

template <typename T1, typename T2>
void f115(std::formatter<T1, T2> &a0, std::basic_string_view<T2> a1)
  requires((std::format_kind<T1> == std::range_format::sequence)) &&
          ((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.set_separator(std::move(a1));
}

template <typename T1, typename T2>
void f116(std::formatter<T1, T2> &a0, std::basic_string_view<T2> a1,
          std::basic_string_view<T2> a2)
  requires((std::format_kind<T1> == std::range_format::sequence)) &&
          ((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.set_brackets(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f117(std::formatter<T1, T2> &a0, std::basic_format_parse_context<T2> &a1)
  requires((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.parse(a1);
}

template <typename T1, typename... T2>
typename std::basic_format_parse_context<T1>::iterator
f118(std::formatter<std::tuple<T2...>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
using t51 = std::formatter<std::pair<T2, T3>, T1>;

template <typename T1, typename T2, typename T3>
void f119(std::formatter<std::pair<T2, T3>, T1> &a0,
          std::basic_string_view<T1> a1) {
  return a0.set_separator(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f120(std::formatter<std::pair<T2, T3>, T1> &a0,
          std::basic_string_view<T1> a1, std::basic_string_view<T1> a2) {
  return a0.set_brackets(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T1>::iterator
f121(std::formatter<std::pair<T2, T3>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T1>::iterator
f122(std::formatter<std::queue<T2, T3>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T1>::iterator
f123(std::formatter<std::stack<T2, T3>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_parse_context<T1>::iterator
f124(std::formatter<std::priority_queue<T2, T3, T4>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}
