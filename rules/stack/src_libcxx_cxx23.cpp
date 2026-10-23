#include <compare>
#include <ranges>
#include <stack>
#include <utility>

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::stack<T1, T2> f43(T3 a0, T3 a1) {
  return std::stack<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T5 = 0,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T6 = 0>
std::stack<T1, T2> f44(T3 a0, T3 a1, const T4 &a2) {
  return std::stack<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::stack<T1, T2> f45(std::from_range_t a0, T3 &&a1, const T4 &a2) {
  return std::stack<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
