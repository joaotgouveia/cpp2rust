#include <algorithm>
#include <cstddef>
#include <format>
#include <ios>
#include <utility>

template <typename T1, typename T2>
using t1 = std::basic_format_context<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_format_context<T1, T2>::iterator;

template <typename T1, typename T2>
using t3 = typename std::basic_format_context<T1, T2>::char_type;

template <typename T1, typename T2>
std::basic_format_arg<std::basic_format_context<T1, T2>>
f1(const std::basic_format_context<T1, T2> &a0, std::size_t a1) {
  return a0.arg(a1);
}

template <typename T1, typename T2>
std::locale f2(std::basic_format_context<T1, T2> &a0) {
  return a0.locale();
}

template <typename T1, typename T2>
typename std::basic_format_context<T1, T2>::iterator
f3(std::basic_format_context<T1, T2> &a0) {
  return a0.out();
}

template <typename T1, typename T2>
void f4(std::basic_format_context<T1, T2> &a0,
        typename std::basic_format_context<T1, T2>::iterator a1) {
  return a0.advance_to(a1);
}
