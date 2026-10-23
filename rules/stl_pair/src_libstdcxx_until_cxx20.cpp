#include <algorithm>
#include <utility>

template <typename T1, typename T2>
bool f11(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f12(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f13(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f14(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f15(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f16(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 >= a1;
}
