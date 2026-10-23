#include <algorithm>
#include <cstddef>
#include <ios>
#include <sstream>
#include <utility>

template <typename T1, typename T2> using t1 = std::num_get<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::num_get<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::num_get<T1, T2>::iter_type;

template <typename T1, typename T2> std::num_get<T1, T2> f1(std::size_t a0) {
  return std::num_get<T1, T2>(a0);
}

template <typename T1, typename T2>
T2 f2(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, bool &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f3(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, long &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f4(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, unsigned short &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f5(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, unsigned int &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f6(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, unsigned long &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f7(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, long long &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f8(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, unsigned long long &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f9(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, float &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f10(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
       std::ios_base::iostate &a4, double &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f11(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
       std::ios_base::iostate &a4, long double &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f12(const std::num_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
       std::ios_base::iostate &a4, void *&a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2> const std::locale::id &f13() {
  return std::num_get<T1, T2>::id;
}
