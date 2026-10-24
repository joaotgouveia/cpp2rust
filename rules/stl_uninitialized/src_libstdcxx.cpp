#include <deque>
#include <utility>

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::uninitialized_copy<T1, T2>(std::move(a0), std::move(a1),
                                         std::move(a2));
}

template <typename T1, typename T2, typename T3> T3 f2(T1 a0, T2 a1, T3 a2) {
  return std::uninitialized_copy_n<T1, T2, T3>(std::move(a0), std::move(a1),
                                               std::move(a2));
}

template <typename T1, typename T2> void f3(T1 a0, T1 a1, const T2 &a2) {
  return std::uninitialized_fill<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T1 f4(T1 a0, T2 a1, const T3 &a2) {
  return std::uninitialized_fill_n<T1, T2, T3>(std::move(a0), std::move(a1),
                                               a2);
}
