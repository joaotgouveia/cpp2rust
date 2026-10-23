#include <utility>

template <typename T1, typename T2> bool f1(T1 a0, T2 a1) {
  return std::cmp_equal<T1, T2>(a0, a1);
}

template <typename T1, typename T2> bool f2(T1 a0, T2 a1) {
  return std::cmp_greater<T1, T2>(a0, a1);
}

template <typename T1, typename T2> bool f3(T1 a0, T2 a1) {
  return std::cmp_greater_equal<T1, T2>(a0, a1);
}

template <typename T1, typename T2> bool f4(T1 a0, T2 a1) {
  return std::cmp_less<T1, T2>(a0, a1);
}

template <typename T1, typename T2> bool f5(T1 a0, T2 a1) {
  return std::cmp_less_equal<T1, T2>(a0, a1);
}

template <typename T1, typename T2> bool f6(T1 a0, T2 a1) {
  return std::cmp_not_equal<T1, T2>(a0, a1);
}

template <typename T1, typename T2> bool f7(T2 a0) {
  return std::in_range<T1, T2>(a0);
}
