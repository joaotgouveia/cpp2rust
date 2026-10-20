#include "../common/init.h"
#include "../common/traits.h"
#include <ostream>
#include <regex>
#include <string>
#include <utility>

template <typename T1>
bool f27(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return a0 != a1;
}

template <typename T1>
bool f28(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return a0 != a1;
}

template <typename T1>
bool f29(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f30(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f31(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f32(const std::match_results<T1, T2> &a0,
         const std::match_results<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f33(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f34(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f35(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return a0 < a1;
}

template <typename T1>
bool f36(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return a0 < a1;
}

template <typename T1>
bool f37(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return a0 < a1;
}

template <typename T1>
bool f38(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return a0 < a1;
}

template <typename T1>
bool f39(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f40(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f41(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return a0 < a1;
}

template <typename T1>
bool f42(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f43(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return a0 <= a1;
}

template <typename T1>
bool f44(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f45(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f46(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f47(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f48(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f49(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return a0 == a1;
}

template <typename T1>
bool f50(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f51(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return a0 == a1;
}

template <typename T1>
bool f52(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return a0 > a1;
}

template <typename T1>
bool f53(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return a0 > a1;
}

template <typename T1>
bool f54(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return a0 > a1;
}

template <typename T1>
bool f55(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return a0 > a1;
}

template <typename T1>
bool f56(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f57(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f58(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return a0 > a1;
}

template <typename T1>
bool f59(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return a0 >= a1;
}

template <typename T1>
bool f60(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return a0 >= a1;
}

template <typename T1>
bool f61(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return a0 >= a1;
}

template <typename T1>
bool f62(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return a0 >= a1;
}

template <typename T1>
bool f63(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3>
bool f64(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3>
bool f65(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return a0 >= a1;
}
