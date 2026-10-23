#include <algorithm>
#include <cstddef>
#include <ios>
#include <sstream>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> using t1 = std::num_put<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::num_put<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::num_put<T1, T2>::iter_type;

template <typename T1, typename T2> std::num_put<T1, T2> f1(std::size_t a0) {
  return std::num_put<T1, T2>(a0);
}

template <typename T1, typename T2>
typename std::num_put<T1, T2>::iter_type
f2(const std::num_put<T1, T2> &a0, typename std::num_put<T1, T2>::iter_type a1,
   std::ios_base &a2, typename std::num_put<T1, T2>::char_type a3, bool a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
typename std::num_put<T1, T2>::iter_type
f3(const std::num_put<T1, T2> &a0, typename std::num_put<T1, T2>::iter_type a1,
   std::ios_base &a2, typename std::num_put<T1, T2>::char_type a3, long a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
typename std::num_put<T1, T2>::iter_type
f4(const std::num_put<T1, T2> &a0, typename std::num_put<T1, T2>::iter_type a1,
   std::ios_base &a2, typename std::num_put<T1, T2>::char_type a3,
   unsigned long a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
typename std::num_put<T1, T2>::iter_type
f5(const std::num_put<T1, T2> &a0, typename std::num_put<T1, T2>::iter_type a1,
   std::ios_base &a2, typename std::num_put<T1, T2>::char_type a3,
   long long a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
typename std::num_put<T1, T2>::iter_type
f6(const std::num_put<T1, T2> &a0, typename std::num_put<T1, T2>::iter_type a1,
   std::ios_base &a2, typename std::num_put<T1, T2>::char_type a3,
   unsigned long long a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
typename std::num_put<T1, T2>::iter_type
f7(const std::num_put<T1, T2> &a0, typename std::num_put<T1, T2>::iter_type a1,
   std::ios_base &a2, typename std::num_put<T1, T2>::char_type a3, double a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
typename std::num_put<T1, T2>::iter_type
f8(const std::num_put<T1, T2> &a0, typename std::num_put<T1, T2>::iter_type a1,
   std::ios_base &a2, typename std::num_put<T1, T2>::char_type a3,
   long double a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
typename std::num_put<T1, T2>::iter_type
f9(const std::num_put<T1, T2> &a0, typename std::num_put<T1, T2>::iter_type a1,
   std::ios_base &a2, typename std::num_put<T1, T2>::char_type a3,
   const void *a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2> const std::locale::id &f10() {
  return std::num_put<T1, T2>::id;
}
