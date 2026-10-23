#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5> f104(
    std::from_range_t a0, T6 &&a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2);
}
