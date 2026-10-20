#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2> using t1 = std::pair<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::pair<T1, T2>::first_type;

template <typename T1, typename T2>
using t3 = typename std::pair<T1, T2>::second_type;

template <typename T1, typename T2>
std::pair<T1, T2> f1(const std::pair<T1, T2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2>
std::pair<T1, T2> f2(std::pair<T1, T2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f3(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename... T3, typename... T4>
std::pair<T1, T2> f4(std::piecewise_construct_t a0, std::tuple<T3...> a1,
                     std::tuple<T4...> a2) {
  return std::pair<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2> std::pair<T1, T2> f5() {
  return std::pair<T1, T2>();
}

template <typename T1, typename T2>
std::pair<T1, T2> f6(const T1 &a0, const T2 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f7(const std::pair<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f8(T3 &&a0, T4 &&a1) {
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f9(std::pair<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> &f10(std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> &f11(std::pair<T1, T2> &a0, std::pair<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}
