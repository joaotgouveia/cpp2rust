#include <numeric>
#include <utility>

template <typename T1, typename T2, typename T3>
T3 f1(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::inner_product<T1, T2, T3>(std::move(a0), std::move(a1),
                                        std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f2(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::inner_product<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), std::move(a3),
                                                std::move(a4), std::move(a5));
}
