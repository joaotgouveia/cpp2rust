#include <numeric>
#include <utility>

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::accumulate(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
T2 f2(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::accumulate(std::move(a0), std::move(a1), std::move(a2),
                         std::move(a3));
}

template <typename T1, typename T2> T2 f3(T1 a0, T1 a1, T2 a2) {
  return std::adjacent_difference(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
T2 f4(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::adjacent_difference(std::move(a0), std::move(a1), std::move(a2),
                                  std::move(a3));
}

template <typename T1, typename T2, typename T3>
T3 f5(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::inner_product(std::move(a0), std::move(a1), std::move(a2),
                            std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f6(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::inner_product(std::move(a0), std::move(a1), std::move(a2),
                            std::move(a3), std::move(a4), std::move(a5));
}

template <typename T1, typename T2> void f7(T1 a0, T1 a1, T2 a2) {
  return std::iota(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> T2 f8(T1 a0, T1 a1, T2 a2) {
  return std::partial_sum(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
T2 f9(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::partial_sum(std::move(a0), std::move(a1), std::move(a2),
                          std::move(a3));
}
