#include <algorithm>
#include <type_traits>
#include <utility>

template <typename T1, typename T2> bool f8(T1 a0, T2 a1) {
  return std::cmp_equal<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f9(T1 a0, T2 a1) {
  return std::cmp_greater<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f10(T1 a0, T2 a1) {
  return std::cmp_greater_equal<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f11(T1 a0, T2 a1) {
  return std::cmp_less<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f12(T1 a0, T2 a1) {
  return std::cmp_less_equal<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f13(T1 a0, T2 a1) {
  return std::cmp_not_equal<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f14(T2 a0) {
  return std::in_range<T1, T2>(std::move(a0));
}
