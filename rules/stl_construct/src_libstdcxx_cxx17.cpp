#include <algorithm>
#include <utility>

template <typename T1> void f1(T1 a0, T1 a1) {
  return std::destroy<T1>(a0, a1);
}

template <typename T1> void f2(T1 *a0) { return std::destroy_at<T1>(a0); }

template <typename T1, typename T2> T1 f3(T1 a0, T2 a1) {
  return std::destroy_n<T1, T2>(a0, a1);
}
