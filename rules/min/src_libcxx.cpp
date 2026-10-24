#include <algorithm>
#include <utility>

template <typename T1> const T1 &f1(const T1 &a0, const T1 &a1) {
  return std::min<T1>(a0, a1);
}

template <typename T1> T1 f2(std::initializer_list<T1> a0) {
  return std::min<T1>(std::move(a0));
}

template <typename T1, typename T2>
const T1 &f3(const T1 &a0, const T1 &a1, T2 a2) {
  return std::min<T1, T2>(a0, a1, std::move(a2));
}

template <typename T1, typename T2> T1 f4(std::initializer_list<T1> a0, T2 a1) {
  return std::min<T1, T2>(std::move(a0), std::move(a1));
}
