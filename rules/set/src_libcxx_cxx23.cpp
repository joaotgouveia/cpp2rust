#include <initializer_list>
#include <ranges>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3>
f114(std::from_range_t a0, T4 &&a1,
     const typename std::set<T1, T2, T3>::key_compare &a2,
     const typename std::set<T1, T2, T3>::allocator_type &a3) {
  return std::set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3>
f115(std::from_range_t a0, T4 &&a1,
     const typename std::set<T1, T2, T3>::allocator_type &a2) {
  return std::set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}
