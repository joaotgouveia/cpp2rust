#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <initializer_list>
#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_regex<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_regex<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::basic_regex<T1, T2>::traits_type;

template <typename T1, typename T2>
using t4 = typename std::basic_regex<T1, T2>::string_type;

template <typename T1, typename T2>
using t5 = typename std::basic_regex<T1, T2>::flag_type;

template <typename T1, typename T2>
using t6 = typename std::basic_regex<T1, T2>::locale_type;

template <typename T1, typename T2> std::basic_regex<T1, T2> f1() {
  return std::basic_regex<T1, T2>();
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f2(std::basic_regex<T1, T2> &a0,
                             const std::basic_regex<T1, T2> &a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
std::basic_regex<T1, T2> &f3(std::basic_regex<T1, T2> &a0,
                             std::basic_regex<T1, T2> &&a1) {
  return a0.assign(std::move(a1));
}

template <typename T1, typename T2>
unsigned int f4(const std::basic_regex<T1, T2> &a0) {
  return a0.mark_count();
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type
f5(const std::basic_regex<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::locale_type
f6(std::basic_regex<T1, T2> &a0,
   typename std::basic_regex<T1, T2>::locale_type a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::locale_type
f7(const std::basic_regex<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2>
void f8(std::basic_regex<T1, T2> &a0, std::basic_regex<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_regex<T1, T2> f9(T3 a0, T3 a1,
                            typename std::basic_regex<T1, T2>::flag_type a2) {
  return std::basic_regex<T1, T2>(a0, a1, a2);
}
