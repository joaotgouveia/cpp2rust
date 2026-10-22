#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::list<T1, T2> f106(std::from_range_t a0, T3 &&a1,
                       const typename std::list<T1, T2>::allocator_type &a2) {
  return std::list<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
