#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
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
typename std::num_get<T1, T2>::iter_type
f2(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
   typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
   std::ios_base::iostate &a4, bool &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f3(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
   typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
   std::ios_base::iostate &a4, long &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f4(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
   typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
   std::ios_base::iostate &a4, unsigned short &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f5(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
   typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
   std::ios_base::iostate &a4, unsigned int &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f6(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
   typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
   std::ios_base::iostate &a4, unsigned long &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f7(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
   typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
   std::ios_base::iostate &a4, long long &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f8(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
   typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
   std::ios_base::iostate &a4, unsigned long long &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f9(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
   typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
   std::ios_base::iostate &a4, float &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f10(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
    typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
    std::ios_base::iostate &a4, double &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f11(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
    typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
    std::ios_base::iostate &a4, long double &a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
typename std::num_get<T1, T2>::iter_type
f12(const std::num_get<T1, T2> &a0, typename std::num_get<T1, T2>::iter_type a1,
    typename std::num_get<T1, T2>::iter_type a2, std::ios_base &a3,
    std::ios_base::iostate &a4, void *&a5) {
  return a0.get(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2> const std::locale::id &f13() {
  return std::num_get<T1, T2>::id;
}
