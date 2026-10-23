#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1, typename T2> using t1 = std::common_iterator<T1, T2>;

template <typename T1, typename T2> std::common_iterator<T1, T2> f1() {
  return std::common_iterator<T1, T2>();
}

template <typename T1, typename T2> std::common_iterator<T1, T2> f2(T1 a0) {
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2> std::common_iterator<T1, T2> f3(T2 a0) {
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
decltype(auto) f4(std::common_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
decltype(auto) f5(const std::common_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f6(std::common_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
decltype(auto) f7(std::common_iterator<T1, T2> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::common_iterator<T1, T2> f8(const std::common_iterator<T3, T4> &a0) {
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::common_iterator<T1, T2> &f9(std::common_iterator<T1, T2> &a0,
                                 const std::common_iterator<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}
