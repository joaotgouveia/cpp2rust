#include <algorithm>
#include <utility>

template <typename T1> void f1(T1 a0, T1 a1) {
  return std::make_heap<T1>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> void f2(T1 a0, T1 a1, T2 a2) {
  return std::make_heap<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}
