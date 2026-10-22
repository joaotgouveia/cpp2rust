#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <locale>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::wstring_convert<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = typename std::wstring_convert<T1, T2, T3, T4>::byte_string;

template <typename T1, typename T2, typename T3, typename T4>
using t3 = typename std::wstring_convert<T1, T2, T3, T4>::wide_string;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::wstring_convert<T1, T2, T3, T4>::state_type;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::wstring_convert<T1, T2, T3, T4>::int_type;

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::wide_string
f1(std::wstring_convert<T1, T2, T3, T4> &a0, char a1) {
  return a0.from_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::wide_string
f2(std::wstring_convert<T1, T2, T3, T4> &a0, const char *a1) {
  return a0.from_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::wide_string
f3(std::wstring_convert<T1, T2, T3, T4> &a0,
   const typename std::wstring_convert<T1, T2, T3, T4>::byte_string &a1) {
  return a0.from_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::wide_string
f4(std::wstring_convert<T1, T2, T3, T4> &a0, const char *a1, const char *a2) {
  return a0.from_bytes(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::byte_string
f5(std::wstring_convert<T1, T2, T3, T4> &a0, T2 a1) {
  return a0.to_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::byte_string
f6(std::wstring_convert<T1, T2, T3, T4> &a0, const T2 *a1) {
  return a0.to_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::byte_string
f7(std::wstring_convert<T1, T2, T3, T4> &a0,
   const typename std::wstring_convert<T1, T2, T3, T4>::wide_string &a1) {
  return a0.to_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::byte_string
f8(std::wstring_convert<T1, T2, T3, T4> &a0, const T2 *a1, const T2 *a2) {
  return a0.to_bytes(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::size_t f9(const std::wstring_convert<T1, T2, T3, T4> &a0) {
  return a0.converted();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::state_type
f10(const std::wstring_convert<T1, T2, T3, T4> &a0) {
  return a0.state();
}

template <typename T1, typename T2, typename T3, typename T4>
using t6 =
    typename std::wstring_convert<T1, T2, T3, T4>::byte_string::traits_type;

template <typename T1, typename T2, typename T3, typename T4>
using t7 =
    typename std::wstring_convert<T1, T2, T3, T4>::byte_string::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t8 =
    typename std::wstring_convert<T1, T2, T3, T4>::byte_string::allocator_type;

template <typename T1, typename T2, typename T3, typename T4>
using t9 =
    typename std::wstring_convert<T1, T2, T3, T4>::byte_string::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t10 =
    typename std::wstring_convert<T1, T2, T3, T4>::byte_string::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t11 =
    typename std::wstring_convert<T1, T2, T3, T4>::byte_string::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t12 =
    typename std::wstring_convert<T1, T2, T3, T4>::byte_string::const_reference;

template <typename T1, typename T2, typename T3, typename T4>
using t13 = typename std::wstring_convert<T1, T2, T3, T4>::byte_string::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t14 =
    typename std::wstring_convert<T1, T2, T3, T4>::byte_string::const_pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t15 =
    typename std::wstring_convert<T1, T2, T3, T4>::wide_string::traits_type;

template <typename T1, typename T2, typename T3, typename T4>
using t16 =
    typename std::wstring_convert<T1, T2, T3, T4>::wide_string::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t17 =
    typename std::wstring_convert<T1, T2, T3, T4>::wide_string::allocator_type;

template <typename T1, typename T2, typename T3, typename T4>
using t18 =
    typename std::wstring_convert<T1, T2, T3, T4>::wide_string::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t19 =
    typename std::wstring_convert<T1, T2, T3, T4>::wide_string::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t20 =
    typename std::wstring_convert<T1, T2, T3, T4>::wide_string::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t21 =
    typename std::wstring_convert<T1, T2, T3, T4>::wide_string::const_reference;

template <typename T1, typename T2, typename T3, typename T4>
using t22 = typename std::wstring_convert<T1, T2, T3, T4>::wide_string::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t23 =
    typename std::wstring_convert<T1, T2, T3, T4>::wide_string::const_pointer;
