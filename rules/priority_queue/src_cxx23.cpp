#include <algorithm>
#include <queue>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3> f15(std::from_range_t a0, T4 &&a1,
                                    const T3 &a2) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                         a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f16(std::priority_queue<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template push_range<T4>(std::forward<T4>(a1));
}
