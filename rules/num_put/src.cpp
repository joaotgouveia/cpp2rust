#include <algorithm>
#include <cstddef>
#include <ios>
#include <sstream>
#include <utility>

template <typename T1, typename T2> using t1 = std::num_put<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::num_put<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::num_put<T1, T2>::iter_type;

template <typename T1, typename T2> std::num_put<T1, T2> f1(std::size_t a0) {
  return std::num_put<T1, T2>(a0);
}

template <typename T1, typename T2>
T2 f2(const std::num_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      bool a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
T2 f3(const std::num_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      long a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
T2 f4(const std::num_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      unsigned long a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
T2 f5(const std::num_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      long long a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
T2 f6(const std::num_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      unsigned long long a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
T2 f7(const std::num_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      double a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
T2 f8(const std::num_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      long double a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2>
T2 f9(const std::num_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      const void *a4) {
  return a0.put(a1, a2, a3, a4);
}

template <typename T1, typename T2> const std::locale::id &f10() {
  return std::num_put<T1, T2>::id;
}
