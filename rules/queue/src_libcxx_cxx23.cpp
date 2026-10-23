#include <compare>
#include <queue>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::queue<T1, T2> f47(T3 a0, T3 a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T5 = 0,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T6 = 0>
std::queue<T1, T2> f48(T3 a0, T3 a1, const T4 &a2) {
  return std::queue<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::queue<T1, T2> f49(std::from_range_t a0, T3 &&a1, const T4 &a2) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
