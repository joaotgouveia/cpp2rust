#include <initializer_list>
#include <ranges>
#include <utility>
#include <vector>

template <typename T1, typename T2>
std::vector<bool, T1>
f233(std::from_range_t a0, T2 &&a1,
     const typename std::vector<bool, T1>::allocator_type &a2) {
  return std::vector<bool, T1>(std::move(a0), std::forward<T2>(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::vector<T1, T2>
f234(std::from_range_t a0, T3 &&a1,
     const typename std::vector<T1, T2>::allocator_type &a2) {
  return std::vector<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
