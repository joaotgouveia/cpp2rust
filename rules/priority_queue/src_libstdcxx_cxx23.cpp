#include <algorithm>
#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3> f33(std::from_range_t a0, T4 &&a1, const T3 &a2,
                                    const T5 &a3) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                         a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3> f34(std::from_range_t a0, T4 &&a1,
                                    const T5 &a2) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                         a2);
}
