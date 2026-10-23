#include <regex>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::sub_match<T1>;

template <typename T1> using t2 = typename std::sub_match<T1>::value_type;

template <typename T1> using t3 = typename std::sub_match<T1>::difference_type;

template <typename T1> using t4 = typename std::sub_match<T1>::iterator;

template <typename T1> using t5 = typename std::sub_match<T1>::string_type;

template <typename T1> std::sub_match<T1> f1() { return std::sub_match<T1>(); }

template <typename T1>
typename std::sub_match<T1>::difference_type f2(const std::sub_match<T1> &a0) {
  return a0.length();
}

template <typename T1>
typename std::sub_match<T1>::string_type f3(const std::sub_match<T1> &a0) {
  return static_cast<typename std::sub_match<T1>::string_type>(a0);
}

template <typename T1>
typename std::sub_match<T1>::string_type f4(const std::sub_match<T1> &a0) {
  return a0.str();
}

template <typename T1>
int f5(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return a0.compare(a1);
}

template <typename T1>
int f6(const std::sub_match<T1> &a0,
       const typename std::sub_match<T1>::string_type &a1) {
  return a0.compare(a1);
}

template <typename T1>
int f7(const std::sub_match<T1> &a0,
       const typename std::sub_match<T1>::value_type *a1) {
  return a0.compare(a1);
}

template <typename T1> void f8(std::sub_match<T1> &a0, std::sub_match<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1>
using t6 = typename std::sub_match<T1>::string_type::traits_type;

template <typename T1>
using t7 = typename std::sub_match<T1>::string_type::value_type;

template <typename T1>
using t8 = typename std::sub_match<T1>::string_type::allocator_type;

template <typename T1>
using t9 = typename std::sub_match<T1>::string_type::size_type;

template <typename T1>
using t10 = typename std::sub_match<T1>::string_type::difference_type;

template <typename T1>
using t11 = typename std::sub_match<T1>::string_type::reference;

template <typename T1>
using t12 = typename std::sub_match<T1>::string_type::const_reference;

template <typename T1>
using t13 = typename std::sub_match<T1>::string_type::pointer;

template <typename T1>
using t14 = typename std::sub_match<T1>::string_type::const_pointer;
