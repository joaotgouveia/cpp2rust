#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::copy<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> T2 f2(T1 a0, T1 a1, T2 a2) {
  return std::copy_backward<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> bool f3(T1 a0, T1 a1, T2 a2) {
  return std::equal<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f4(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::equal<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2> void f5(T1 a0, T1 a1, const T2 &a2) {
  return std::fill<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T1 f6(T1 a0, T2 a1, const T3 &a2) {
  return std::fill_n<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2> bool f7(T1 a0, T1 a1, T2 a2) {
  return std::is_permutation<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> void f8(T1 a0, T2 a1) {
  return std::iter_swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2> bool f9(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::lexicographical_compare<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f10(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::lexicographical_compare<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2> T1 f11(T1 a0, T1 a1, const T2 &a2) {
  return std::lower_bound<T1, T2>(a0, a1, a2);
}

template <typename T1> const T1 &f12(const T1 &a0, const T1 &a1) {
  return std::max<T1>(a0, a1);
}

template <typename T1, typename T2>
const T1 &f13(const T1 &a0, const T1 &a1, T2 a2) {
  return std::max<T1, T2>(a0, a1, a2);
}

template <typename T1> const T1 &f14(const T1 &a0, const T1 &a1) {
  return std::min<T1>(a0, a1);
}

template <typename T1, typename T2>
const T1 &f15(const T1 &a0, const T1 &a1, T2 a2) {
  return std::min<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> std::pair<T1, T2> f16(T1 a0, T1 a1, T2 a2) {
  return std::mismatch<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f17(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::mismatch<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2> T2 f18(T1 a0, T1 a1, T2 a2) {
  return std::move<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> T2 f19(T1 a0, T1 a1, T2 a2) {
  return std::move_backward<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T1 f20(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::search<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2> T2 f21(T1 a0, T1 a1, T2 a2) {
  return std::swap_ranges<T1, T2>(a0, a1, a2);
}
