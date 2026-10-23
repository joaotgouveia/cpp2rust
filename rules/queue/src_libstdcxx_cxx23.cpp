#include <algorithm>
#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::queue<T1, T2> f23(T3 a0, T3 a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T3>,
          typename T6 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::queue<T1, T2> f24(T3 a0, T3 a1, const T4 &a2) {
  return std::queue<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::queue<T1, T2> f25(std::from_range_t a0, T3 &&a1, const T4 &a2) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
