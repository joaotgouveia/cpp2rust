#include <algorithm>
#include <utility>

template <typename T1, typename T2> bool f14(T1 a0, T2 a1) {
  return std::cmp_equal<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f15(T1 a0, T2 a1) {
  return std::cmp_greater<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f16(T1 a0, T2 a1) {
  return std::cmp_greater_equal<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f17(T1 a0, T2 a1) {
  return std::cmp_less<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f18(T1 a0, T2 a1) {
  return std::cmp_less_equal<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f19(T1 a0, T2 a1) {
  return std::cmp_not_equal<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f20(T2 a0) {
  return std::in_range<T1, T2>(std::move(a0));
}
