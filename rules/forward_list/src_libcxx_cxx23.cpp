#include <forward_list>
#include <initializer_list>
#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::forward_list<T1, T2>
f102(std::from_range_t a0, T3 &&a1,
     const typename std::forward_list<T1, T2>::allocator_type &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
