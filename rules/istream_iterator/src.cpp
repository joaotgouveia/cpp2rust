#include <istream>
#include <iterator>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::istream_iterator<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = typename std::istream_iterator<T1, T2, T3, T4>::char_type;

template <typename T1, typename T2, typename T3, typename T4>
using t3 = typename std::istream_iterator<T1, T2, T3, T4>::traits_type;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::istream_iterator<T1, T2, T3, T4>::istream_type;

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4> f1() {
  return std::istream_iterator<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4> f2(std::basic_istream<T2, T3> &a0) {
  return std::istream_iterator<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
const T1 &f3(const std::istream_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3, typename T4>
const T1 *f4(const std::istream_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4> &
f5(std::istream_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4>
f6(std::istream_iterator<T1, T2, T3, T4> &a0, int a1) {
  return a0.operator++(a1);
}
