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

using t42 = std::formatter<std::stacktrace_entry>;

const std::basic_string_view<char>::value_type *
f94(std::formatter<std::stacktrace_entry> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

std::formatter<std::stacktrace_entry> f95() {
  return std::formatter<std::stacktrace_entry>();
}

std::formatter<std::stacktrace_entry>
f96(const std::formatter<std::stacktrace_entry> &a0) {
  return std::formatter<std::stacktrace_entry>(a0);
}

std::formatter<std::stacktrace_entry>
f97(std::formatter<std::stacktrace_entry> &&a0) {
  return std::formatter<std::stacktrace_entry>(std::move(a0));
}

std::formatter<std::stacktrace_entry> &
f98(std::formatter<std::stacktrace_entry> &a0,
    const std::formatter<std::stacktrace_entry> &a1) {
  return a0.operator=(a1);
}

std::formatter<std::stacktrace_entry> &
f99(std::formatter<std::stacktrace_entry> &a0,
    std::formatter<std::stacktrace_entry> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::basic_format_context<T1, char>::iterator
f100(const std::formatter<std::stacktrace_entry> &a0,
     const std::stacktrace_entry &a1, std::basic_format_context<T1, char> &a2) {
  return a0.template format<T1>(a1, a2);
}

using t43 = std::formatter<const char *, wchar_t>;

using t44 = std::formatter<char *, wchar_t>;

template <typename T1> using t45 = std::formatter<std::thread::id, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f101(std::formatter<std::thread::id, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f102(const std::formatter<std::thread::id, T1> &a0, std::thread::id a1,
     std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(std::move(a1), a2);
}

template <typename T1> void f103(std::formatter<T1 *, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1> using t46 = std::formatter<std::basic_stacktrace<T1>>;

template <typename T1>
const std::basic_string_view<char>::value_type *
f104(std::formatter<std::basic_stacktrace<T1>> &a0,
     std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, char>::iterator
f105(const std::formatter<std::basic_stacktrace<T1>> &a0,
     const std::basic_stacktrace<T1> &a1,
     std::basic_format_context<T2, char> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
void f106(std::formatter<std::basic_string_view<char, T1>, char> &a0) {
  return a0.set_debug_format();
}

template <typename T1>
using t47 = std::formatter<std::basic_string_view<char, T1>, wchar_t>;

template <typename T1>
void f107(std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0) {
  return a0.set_debug_format();
}

template <typename T1> void f108(std::formatter<T1, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
void f109(std::formatter<std::basic_string<char, T1, T2>, char> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
using t48 = std::formatter<std::basic_string<char, T1, T2>, wchar_t>;

template <typename T1, typename T2>
void f110(std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
void f111(std::formatter<T1, T2> &a0, std::basic_string_view<T2> a1) {
  return a0.set_separator(a1);
}

template <typename T1, typename T2>
void f112(std::formatter<T1, T2> &a0, std::basic_string_view<T2> a1,
          std::basic_string_view<T2> a2) {
  return a0.set_brackets(a1, a2);
}

template <typename T1, typename... T2>
typename std::basic_format_parse_context<T1>::iterator
f113(std::formatter<std::tuple<T2...>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
using t49 = std::formatter<std::pair<T2, T3>, T1>;

template <typename T1, typename T2, typename T3>
void f114(std::formatter<std::pair<T2, T3>, T1> &a0,
          std::basic_string_view<T1> a1) {
  return a0.set_separator(a1);
}

template <typename T1, typename T2, typename T3>
void f115(std::formatter<std::pair<T2, T3>, T1> &a0,
          std::basic_string_view<T1> a1, std::basic_string_view<T1> a2) {
  return a0.set_brackets(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T1>::iterator
f116(std::formatter<std::pair<T2, T3>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T1>::iterator
f117(std::formatter<std::queue<T2, T3>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T1>::iterator
f118(std::formatter<std::stack<T2, T3>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_parse_context<T1>::iterator
f119(std::formatter<std::priority_queue<T2, T3, T4>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}
