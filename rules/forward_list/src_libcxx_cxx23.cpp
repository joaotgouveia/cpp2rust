#include "../common/init.h"
#include "../common/traits.h"
#include <forward_list>
#include <initializer_list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::forward_list<T1, T2>
f105(std::from_range_t a0, T3 &&a1,
     const typename std::forward_list<T1, T2>::allocator_type &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
