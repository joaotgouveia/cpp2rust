#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <map>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4>
f121(std::from_range_t a0, T5 &&a1,
     const typename std::map<T1, T2, T3, T4>::key_compare &a2,
     const typename std::map<T1, T2, T3, T4>::allocator_type &a3) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4>
f122(std::from_range_t a0, T5 &&a1,
     const typename std::map<T1, T2, T3, T4>::allocator_type &a2) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2);
}
