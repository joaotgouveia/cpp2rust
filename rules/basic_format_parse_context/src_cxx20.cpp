#include <algorithm>
#include <cstddef>
#include <format>
#include <string_view>
#include <utility>

template <typename T1> using t1 = std::basic_format_parse_context<T1>;

template <typename T1>
using t2 = typename std::basic_format_parse_context<T1>::char_type;

template <typename T1>
using t3 = typename std::basic_format_parse_context<T1>::const_iterator;

template <typename T1>
using t4 = typename std::basic_format_parse_context<T1>::iterator;

template <typename T1>
typename std::basic_format_parse_context<T1>::const_iterator
f1(const std::basic_format_parse_context<T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::const_iterator
f2(const std::basic_format_parse_context<T1> &a0) {
  return a0.end();
}

template <typename T1>
void f3(std::basic_format_parse_context<T1> &a0,
        typename std::basic_format_parse_context<T1>::const_iterator a1) {
  return a0.advance_to(
      std::forward<
          typename std::basic_format_parse_context<T1>::const_iterator>(a1));
}

template <typename T1> std::size_t f4(std::basic_format_parse_context<T1> &a0) {
  return a0.next_arg_id();
}

template <typename T1>
void f5(std::basic_format_parse_context<T1> &a0, std::size_t a1) {
  return a0.check_arg_id(a1);
}

template <typename T1>
std::basic_format_parse_context<T1> f6(std::basic_string_view<T1> a0,
                                       std::size_t a1) {
  return std::basic_format_parse_context<T1>(
      std::forward<std::basic_string_view<T1>>(a0), a1);
}
