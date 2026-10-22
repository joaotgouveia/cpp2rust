#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <regex>
#include <utility>

template <typename T1> using t1 = std::regex_traits<T1>;

template <typename T1> using t2 = typename std::regex_traits<T1>::char_type;

template <typename T1> using t3 = typename std::regex_traits<T1>::string_type;

template <typename T1> using t4 = typename std::regex_traits<T1>::locale_type;

template <typename T1> std::regex_traits<T1> f1() {
  return std::regex_traits<T1>();
}

template <typename T1>
std::size_t f2(const typename std::regex_traits<T1>::char_type *a0) {
  return std::regex_traits<T1>::length(a0);
}

template <typename T1>
typename std::regex_traits<T1>::char_type
f3(const std::regex_traits<T1> &a0,
   typename std::regex_traits<T1>::char_type a1) {
  return a0.translate(a1);
}

template <typename T1>
typename std::regex_traits<T1>::char_type
f4(const std::regex_traits<T1> &a0,
   typename std::regex_traits<T1>::char_type a1) {
  return a0.translate_nocase(a1);
}

template <typename T1>
typename std::regex_traits<T1>::locale_type
f5(std::regex_traits<T1> &a0, typename std::regex_traits<T1>::locale_type a1) {
  return a0.imbue(a1);
}

template <typename T1>
typename std::regex_traits<T1>::locale_type
f6(const std::regex_traits<T1> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2>
typename std::regex_traits<T1>::string_type f7(const std::regex_traits<T1> &a0,
                                               T2 a1, T2 a2) {
  return a0.template transform<T2>(a1, a2);
}

template <typename T1, typename T2>
typename std::regex_traits<T1>::string_type f8(const std::regex_traits<T1> &a0,
                                               T2 a1, T2 a2) {
  return a0.template transform_primary<T2>(a1, a2);
}

template <typename T1, typename T2>
typename std::regex_traits<T1>::string_type f9(const std::regex_traits<T1> &a0,
                                               T2 a1, T2 a2) {
  return a0.template lookup_collatename<T2>(a1, a2);
}

template <typename T1, typename T2>
typename std::regex_traits<T1>::char_class_type
f10(const std::regex_traits<T1> &a0, T2 a1, T2 a2, bool a3) {
  return a0.template lookup_classname<T2>(a1, a2, a3);
}

template <typename T1>
using t5 = typename std::regex_traits<T1>::string_type::traits_type;

template <typename T1>
using t6 = typename std::regex_traits<T1>::string_type::value_type;

template <typename T1>
using t7 = typename std::regex_traits<T1>::string_type::allocator_type;

template <typename T1>
using t8 = typename std::regex_traits<T1>::string_type::size_type;

template <typename T1>
using t9 = typename std::regex_traits<T1>::string_type::difference_type;

template <typename T1>
using t10 = typename std::regex_traits<T1>::string_type::reference;

template <typename T1>
using t11 = typename std::regex_traits<T1>::string_type::const_reference;

template <typename T1>
using t12 = typename std::regex_traits<T1>::string_type::pointer;

template <typename T1>
using t13 = typename std::regex_traits<T1>::string_type::const_pointer;
