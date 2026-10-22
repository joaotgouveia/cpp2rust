#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <ranges>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f92(std::from_range_t a0, T6 &&a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal &a4,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a5) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<T6>(a1), a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f93(std::from_range_t a0, T6 &&a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f94(std::from_range_t a0, T6 &&a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<T6>(a1), a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f95(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}
