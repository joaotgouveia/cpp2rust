#include <algorithm>
#include <utility>

template <typename T1> void f1(T1 a0, T1 a1, T1 a2) {
  return std::nth_element<T1>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> void f2(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::nth_element<T1, T2>(std::move(a0), std::move(a1), std::move(a2),
                                  std::move(a3));
}
