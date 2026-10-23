#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multiset<T1, T2, T3, T4>
f84(std::from_range_t a0, T5 &&a1,
    const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
        &a2) {
  return std::unordered_multiset<T1, T2, T3, T4>(std::move(a0),
                                                 std::forward<T5>(a1), a2);
}
